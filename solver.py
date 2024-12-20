import networkx as nx
from matplotlib import pyplot as plt
from util import reg_id, discard, get_other
from index import Index
from geo import *
from itertools import permutations, pairwise, combinations
import parsing

def solve_figure(ind: Index):
	pos = {p: All() for p in ind.points}
	
	origin = ind.points[0]
	base = ind.get_constraints(origin, parsing.Distance)[0]
	orbiter = get_other(base.points, origin)

	pos[origin] = Vec(0,0)
	pos[orbiter] = Vec(base.measure, 0)

	graph = nx.Graph()
	for c in ind._constraints:
		for edge in combinations(c.points, 2):
			graph.add_edge(*edge)
	
	nx.draw_networkx(graph)
	plt.show()

	print(f"origin: {origin}, orbiter: {orbiter}")
	quit()

	g = path_graph()
	cycles = []
	starts = [
		list(g.successors(origin)),
		list(g.successors(orbiter))
	]
	ends = [
		list(g.predecessors(origin)),
		list(g.predecessors(orbiter))
	]
	discard(starts[0], orbiter)
	discard(starts[1], origin)
	discard(ends[0], orbiter)
	discard(ends[1], origin)
	
	print(starts, ends)
	nx.draw_networkx(g)
	plt.show()

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