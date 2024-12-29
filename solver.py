import networkx as nx
from matplotlib import pyplot as plt
from util import reg_id, discard, get_other
from index import Index
from geo import *
from itertools import permutations, pairwise, chain
from collections import namedtuple
import parsing

ConGraphs = namedtuple('ConGraphs', ['origin', 'orbiter', 'foreward', 'backward', 'checks'])

def solve_figure(ind: Index):
	pos = {p: All() for p in ind.points}
	
	origin = ind.points[0]
	base = ind.get_constraints(origin, parsing.Distance)[0]
	orbiter = get_other(base.points, origin)

	pos[origin] = Vec(0,0)
	pos[orbiter] = Vec(base.measure, 0)

	con_graph = create_con_graph(ind, origin, orbiter)
	display_con_graph(con_graph)

	print(f"origin: {origin}, orbiter: {orbiter}")
	quit()
	
	while len(rem_points(pos)) > 0:
		graph = path_graph()

		# Get all neighbors.
		# Two categories:
		# - Continuums are continuous spaces
		#   such as curves or lines.
		# - Finites are finite sets of
		#   possible points.
		continuums, finites = [], []
		for point in rem_points(pos):
			if type(pos[point]) is All: continue
			if is_finite(pos[point]):
				finites.append(point)
			else:
				continuums.append(point)
		
		# Find two connected solvable points
		# Either 2 finites or 1 finite and 1
		# continuum.
		match [len(finites), len(continuums)]:
			case [0, 0]:
				pass # escape?
			case [0, _]:
				raise ValueError("Figure underconstrained.")
			case [1, _]:
				a = finites[0]
				for b in continuums:
					try:
						path = nx.dijkstra_path(graph, a, b)
						break
					except: continue
			case [_, _]:
				for a, b in permutations(finites, 2):
					try:
						path = nx.dijkstra_path(graph, a, b)
						break
					except: continue
		if path is None:
			raise ValueError("Figure underconstrained.")

		start, end = path[0], path[-1]
		path_pos = pos.copy()
		for prev, next in pairwise(path):
			d = [(p, dist_approx(p, path_pos[end])) for p in path_pos[prev]]
			d = sorted(d, key=lambda x: x[1])

			path_pos[prev] = d[0][0]
			mark_solved(prev, pos=path_pos)

			assert is_finite(path_pos[next])
		pos = path_pos

	return pos

def create_con_graph(ind: Index, origin: str, orbiter: str):
	def start_graph(*points):
		"""First point is used as starting point."""
		graph = nx.DiGraph()
		able_constraints = {}
		def graph_point(fixed_points: set, point: str, able_constraints: dict):
			next_points = set()
			for c in ind.get_constraints(point):
				loose_points = set(c.points).difference(fixed_points)
				if len(loose_points) != 1: continue
				(loose_point,) = loose_points
				if loose_point not in able_constraints:
					able_constraints[loose_point] = [c]
				else:
					able_constraints[loose_point].append(c)
					# Replace with rigorous finite check
					# maybe use DOF analysis?
					if len(able_constraints[loose_point]) == 2:
						next_points.add(loose_point)

			for p in next_points:
				fixed_points.add(loose_point)
				graph.add_edge(point, p)
				graph_point(fixed_points, p, able_constraints)

			return fixed_points, able_constraints
		graph_point(set(points), points[0], able_constraints)
		return graph, able_constraints

	def get_unused_constraints(existing_constraints: set, able_constraints: dict):
		used_constraints = [cs for cs in able_constraints.values() if len(cs) >= 2]
		used_constraints = set(chain.from_iterable(used_constraints))
		return existing_constraints.difference(used_constraints)

	checks = set(ind._constraints)
	foreward, able_constraints = start_graph(origin, orbiter)
	print(able_constraints)
	checks = get_unused_constraints(checks, able_constraints)
	backward, able_constraints = start_graph(orbiter, origin)
	print(able_constraints)
	checks = get_unused_constraints(checks, able_constraints)
	print(checks)

	return ConGraphs(
		origin,
		orbiter,
		foreward,
		backward,
		checks
	)

def display_con_graph(con_graph: ConGraphs):
	graph = nx.MultiGraph()
	edge_colors = []

	for (u, v) in con_graph.foreward.edges:
		graph.add_edge(u, v)
		edge_colors.append("green")
	for (u, v) in con_graph.backward.edges:
		graph.add_edge(u, v)
		edge_colors.append("blue")

	for check in con_graph.checks:
		graph.add_edge(check.points[0], check.points[1])
		edge_colors.append("red")

	# nx.draw_networkx(graph, edge_color=edge_colors)
	print(list(con_graph.foreward.edges))
	print(list(con_graph.backward.edges))
	print(con_graph.checks)
	plt.show()