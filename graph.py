import networkx as nx
from tri import Triangle
from typing import List, Iterable, Set
from collections import deque
from functools import lru_cache
from matplotlib import pyplot
import turtle

def regularize_id(id, angle=False) -> str:
	if isinstance(id, str):
		id = id.split(" ")
	elif not isinstance(id, list) and isinstance(id, Iterable):
		id = list(id)
	if hasattr(id[0], "id"):
		id = [point.id for point in id]
	if len(id) == 3 and angle:
		vertex = id.pop(1)
		id.sort()
		id.insert(1, vertex)
	else:
		id.sort()
	return " ".join(id)

@lru_cache
def edges_from_tri(tri: str, regularize=True) -> List[str]:
	points = deque(tri.split(" "))
	endpoints = points.copy()
	endpoints.rotate(-1)
	return [regularize_id(edge) if regularize else " ".join(edge) for edge in zip(points, endpoints)]

@lru_cache
def angles_from_tri(tri: str) -> List[str]:
	points = deque(tri.split(" "))
	points.rotate(-1)
	angles = []
	for _ in range(3):
		angle = regularize_id(points, angle=True)
		angles.append(angle)
		points.rotate(-1)
	return angles

# store all assigned values
# Allow accessing three points as a
# triangle which you activate a solve on
# then reassign to the graph.
# Values to solve (or target values)
# are a list of IDs which are looked up
# to see if the figure has been solved.

class TriGraph():
	def __init__(self):
		self._graph = nx.Graph()
		self._points = {}
		self._edges = {}
		self._angles = {}
		self._tris: Set[str] = set()
	
	def _add_point(self, id: str):
		if not id in self._points:
			self._points[id] = None
	
	def _add_edge(self, id: str):
		for point in id.split(" "):
			self._add_point(point)

		if not id in self._edges:
			self._edges[id] = None

	def _add_tri(self, id: str):
		for edge in edges_from_tri(id):
			self._add_edge(regularize_id(edge, angle=True))
		
		for angle in angles_from_tri(id):
			if not angle in self._angles:
				self._angles[angle] = None
		
		if not id in self._tris:
			self._tris.add(id)

	def __getitem__(self, indices):
		indices = regularize_id(indices, angle=True)

		num = indices.count(" ") + 1

		match num:
			case 1:
				return self._points[indices]
			case 2:
				return self._edges[indices]
			case 3:
				return self._angles[indices]
	
	def __setitem__(self, indices, value):
		indices = regularize_id(indices, angle=True)

		num = indices.count(" ") + 1

		match num:
			case 1:
				self._add_point(indices)
				self._points[indices] = value
			case 2:
				self._add_edge(indices)
				self._edges[indices] = value
			case 3:
				self._add_tri(regularize_id(indices))
				self._angles[indices] = value
	
	def _get_tri(self, id) -> Triangle:
		id = regularize_id(id)
		
		angle_ids = angles_from_tri(id)
		edge_ids = edges_from_tri(id)

		angles = [self[angle_id] for angle_id in angle_ids]
		edges = [self[edge_id] for edge_id in edge_ids]

		return Triangle(angles, edges)
	
	def _solve_tri(self, id):
		id = regularize_id(id)

		tri = self._get_tri(id)
		
		angle_ids = angles_from_tri(id)
		edge_ids = edges_from_tri(id)
		
		try:
			tri.solve()
		except ValueError:
			return

		for angle_id, edge_id, angle, edge in zip(angle_ids, edge_ids, tri.angles, tri.edges):
			self[angle_id] = angle
			self[edge_id] = edge


	@property
	def solved(self):
		return all([self._get_tri(id).solved for id in self._tris])


	def solve(self):
		iter_diff = True
		while iter_diff:
			iter_diff = False
			for id in self._tris:
				if self._get_tri(id).solved:
					continue
				try:
					self._solve_tri(id)
					iter_diff = True
				except Exception as e:
					print(e)

		if not self.solved:
			raise ValueError(f"Figure not solved.\n{self._angles}\n{self._edges}")

	# shared edges need to be traversed in opposite orders to prevent overlap
	def connect(self):
		edge_tris = {}
		graph = nx.Graph()

		for id in self._tris:
			edges = edges_from_tri(id)

			# is the edge traversed in regular (alphabetical) order
			# if two edges are traversed in the same order, they must
			# turn opposite directions so they don't overlap.
			# if two edges are traversed in opposite order, they must
			# turn the same direction so they don't overlap.

			# the use of a combined node allows direction to be determined
			# using 2-coloring of the resulting bipartite graph
			# this needs to be done because the last edge is traversed
			# in the opposite order from the others

			# TODO: Consider using the first point as a vertex and
			# drawing the other two with respect to that to create an
			# unordered system
			combo_node = " ".join(edges[0:2])
			inv_node = f"{id}: {edges[2]}"
			graph.add_edge(combo_node, inv_node)
			
			for i, edge in enumerate(edges):
				if edge not in edge_tris:
					edge_tris[edge] = [id]
				else:
					tris = edge_tris[edge]
					connecting = inv_node if i == 2 else combo_node
					for tri in tris:
						i_edges = edges_from_tri(tri)
						recieving = " ".join(i_edges[0:2]) if edge in tri else f"{tri}: {i_edges[2]}"
						graph.add_edge(connecting, recieving)
					edge_tris[edge].append(id)


		edge_tris = {edge: tris for edge, tris in edge_tris.items() if len(tris) > 1}

		if not nx.is_bipartite(graph):
			raise ValueError("Direction graph is not bipartite. Three or more triangles share an edge.")

		coloring = nx.greedy_color(graph)

		tri_dir = {}
		for tri in self._tris:
			node = " ".join(edges_from_tri(tri)[0:2])

			tri_dir[tri] = coloring[node]

		print(tri_dir)
		
		return tri_dir
	
	def coordinate(self):
		if not self.solved:
			raise ValueError(f"Figure not solved when coordinating.")

		for id in self._tris:
			tri = self._get_tri(id)
			point_ids = id.split(" ")
			points = tri.coordinate([self[point_id] for point_id in point_ids])
			for point_id, point in zip(point_ids, points):
				self._points[point_id] = point
			
		print(self._points)

		for id in self._tris:
			point_ids = id.split(" ")
			turtle.penup()
			turtle.goto(self[point_ids[-1]])
			turtle.write(point_ids[-1])
			turtle.pendown()
			for point_id in point_ids:
				turtle.goto(self[point_id])
				turtle.write(point_id)
		
		turtle.done()