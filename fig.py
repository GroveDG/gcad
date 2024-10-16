import networkx as nx
from typing import Iterable
from matplotlib import pyplot as plt
from itertools import pairwise
from tri import solve_tri
from collections import deque
from math import pi as PI, tau as TAU, fmod, isclose
from point import Point, angle_btw_cartesian

def regularize_id(id) -> tuple[str]:
	if isinstance(id, str):
		id = id.split(" ")
	elif not isinstance(id, list) and isinstance(id, Iterable):
		id = list(id)
	if hasattr(id[0], "id"):
		id = [point.id for point in id]
	if len(id) == 3:
		vertex = id.pop(1)
		id.sort()
		id.insert(1, vertex)
	else:
		id.sort()
	return tuple(id) if len(id) > 1 else id[0]

class Figure():
	def __init__(self):
		self._points = {}
		self._dists = {}
		self._angles = {}
		self._tris = set()
	
	def __str__(self) -> str:
		return "\n".join(
			[
				str(self._points),
				str(self._dists),
				str(self._angles)
			]
		)
	
	def _add_point(self, id: str):
		if not id in self._points:
			self._points[id] = None
	
	def _add_dist(self, id: str):
		for point in id:
			self._add_point(point)

		if not id in self._dists:
			self._dists[id] = None

	def _add_angle(self, id):
		if not id in self._angles:
			self._angles[id] = None

		for edge in pairwise(id):
			self._add_dist(regularize_id(edge))

	def _add_tri(self, id):
		id = tuple(sorted(regularize_id(id)))

		if id not in self._tris:
			self._tris.add(id)
		
		points = deque(id)

		for _ in range(3):
			points.rotate(1)
			angle = regularize_id(points)
			self._add_angle(angle)

	def __getitem__(self, ids):
		ids = regularize_id(ids)

		match len(ids):
			case 1:
				return self._points[ids]
			case 2:
				return self._dists[ids]
			case 3:
				return self._angles[ids]
	
	def get(self, ids, default=None):
		ids = regularize_id(ids)

		match len(ids):
			case 1:
				if ids in self._points:
					return self._points[ids]
				else:
					return default
			case 2:
				if ids in self._dists:
					return self._dists[ids]
				else:
					return default
			case 3:
				if ids in self._angles:
					return self._angles[ids]
				else:
					return default
	
	def __setitem__(self, ids, value):
		ids = regularize_id(ids)

		match len(ids):
			case 1:
				self._add_point(ids)
				self._points[ids] = value
			case 2:
				self._add_dist(ids)
				self._dists[ids] = value
			case 3:
				self._add_angle(ids)
				self._angles[ids] = value

	def _solve_tri(self, tri):
		edge_ids = [regularize_id((tri[i-1], tri[i-2])) for i in range(3)]
		angle_ids = [regularize_id((tri[i-1], vertex, tri[i-2])) for i, vertex in enumerate(tri)]

		edges = [self.get(edge_id) for edge_id in edge_ids]
		angles = [self.get(angle_id) for angle_id in angle_ids]

		result = solve_tri(edges, angles)

		if result != None:
			edges, angles = result
		
		for edge_id, edge in zip(edge_ids, edges):
			self[edge_id] = edge
		
		for angle_id, angle in zip(angle_ids, angles):
			self[angle_id] = angle

	def _solve_tris(self):
		errs = [True]
		while any(errs):
			successes = 0
			errs.clear()

			for tri in self._tris:
				try:
					self._solve_tri(tri)
					successes += 1
				except AssertionError as e:
					e.add_note(f"Triangle: {" ".join(tri)}")
					errs.append(e)

			if successes == 0:
				raise ExceptionGroup(
					"Figure unsolved. Remaining triangles raised the following exceptions.",
					errs
				)
	
	# angle_graph shows if sections are connected
	# edge_graph shows how sections are connected
	#
	# take edge_graph remove all solved cycles
	# any remaining 3-cycles should be added as SSS tris
	def _graph(self):
		edge_graph = nx.Graph()
		angle_graph = nx.Graph()

		for point, value in self._points.items():
			edge_graph.add_node(point)

		for (p1, p2), value in self._dists.items():
			if value: edge_graph.add_edge(p1, p2)
			angle_graph.add_node((p1, p2))
		
		for (p1, v, p2), value in self._angles.items():
			angle_graph.add_edge(
				regularize_id((p1, v)),
				regularize_id((v, p2))
			)

		for i, points in enumerate(nx.connected_components(angle_graph)):
			sub_fig, pos = self._resolve_root(next(points.__iter__()), angle_graph)
			plt.figure(i)
			nx.draw_networkx(sub_fig, pos)
		plt.show()
	
	def _resolve_root(self, root: str, angle_graph: nx.Graph) -> tuple[nx.Graph, dict]:
		angle_tree = nx.dfs_tree(angle_graph, source=root)
		pos = {}

		pos[root[0]] = Point(0, 0)
		pos[root[1]] = Point(0, self[root], rel_to=pos[root[0]])
		edge_dirs = {root: 0}

		for prev_edge, next_edge in nx.bfs_edges(angle_tree, source=root):
			print(prev_edge, next_edge)
			rev = [False, False]
			if prev_edge[0] in next_edge:
				vertex, start = prev_edge
			else:
				vertex, start = reversed(prev_edge)
				rev[0] = True
			if next_edge[0] in prev_edge:
				vertex, end = next_edge
			else:
				vertex, end = reversed(next_edge)
				rev[1] = True

			prev_dir = edge_dirs[prev_edge]
			if rev[0]: prev_dir = fmod(prev_dir + PI, TAU)

			if end in pos:
				s = pos[start].simplified.cartesian
				e = pos[end].simplified.cartesian
				next_dir = fmod(angle_btw_cartesian(s, e), TAU)
				if rev[1]: next_dir = fmod(next_dir + PI, TAU)
				print(edge_dirs)
				print(s, e, next_dir, prev_dir, self[start, vertex, end])
				assert isclose(fmod(next_dir - prev_dir, TAU), self[start, vertex, end])
				edge_dirs[next_edge] = next_dir
			else:
				next_dir = prev_dir + self[start, vertex, end]
				if rev[1]: next_dir = fmod(next_dir + PI, TAU)
				edge_dirs[next_edge] = next_dir
				pos[end] = Point(next_dir, self[next_edge], rel_to=pos[vertex])
		
		points = set()
		for edge in angle_tree.nodes:
			points.add(edge[0])
			points.add(edge[1])

		for id, point in pos.items():
			pos[id] = point.simplified.cartesian
		
		graph = nx.Graph([edge for edge in self._dists.keys() if edge[0] in points and edge[1] in points])

		return graph, pos