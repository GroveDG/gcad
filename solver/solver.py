import networkx as nx
from matplotlib import pyplot as plt
from util import reg_id, get_other
from .fig import Figure
from .index import Index
from .geo import *
from itertools import permutations, pairwise
	
def solve_figure(fig: Figure):
	index: Index = Index.from_fig(fig)

	pos = {p: All() for p in index.points}

	def rem_points(pos:dict=pos):
		return [p for p,v in pos.items() if not isinstance(v, Vec)]
	
	# Select an edge to be the baseline.
	base = list(index.edges)[0]
	# Designate a point as the origin.
	origin = base[0]
	pos[origin] = Vec(0,0)
	# Assign the endpoint of the baseline a position.
	orbiter = base[1]
	pos[orbiter] = Vec(fig[base], 0)

	# Convert a constraint into its possible solutions (possibility space)
	def con_to_space(target, *cons, pos=pos) -> list:
		spaces = []
		for con in cons:
			measure = fig[con]
			if len(con) == 2: # Edge/Distance
				center = pos[get_other(con, target)]
				spaces.append(
					Circle(center, measure)
				)
			if len(con) == 3: # Angle
				if target == con[1]: continue
				center = pos[con[1]]
				base_point = pos[get_other(con[0:3:2], target)]
				base: Vec = (base_point - center).normalized()
				spaces.append([
					Ray(center, base.rotate(measure)),
					Ray(center, base.rotate(-measure))
				])
		return spaces
	
	def mark_solved(*points, pos=pos):
		# Apply all neighboring constraints.
		constraints = index.one_of(*rem_points(pos)) & index.any_of(*points)
		for con in constraints:
			for p in con:
				if not isinstance(pos[p], Vec):
					target = p
					break
			pos[target] = meet(pos[target], *con_to_space(target, con, pos=pos))

	# Mark all starting geometry as solved.
	mark_solved(origin, orbiter)

	def path_graph() -> nx.MultiDiGraph:
		graph = nx.MultiDiGraph()
		for point in rem_points(pos):
			for angle in index.p2a[point]:
				if not angle[1] == point: continue
				e1, e2 = angle[1::-1], angle[1:]
				if reg_id(e1) in index.p2e[point]:
					graph.add_edge(*e1)
				if reg_id(e2) in index.p2e[point]:
					graph.add_edge(*e2)
		return graph
	
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
			d = [(p, dist(p, path_pos[end])) for p in path_pos[prev]]
			d = sorted(d, key=lambda x: x[1])

			path_pos[prev] = d[0][0]
			mark_solved(prev, pos=path_pos)

			assert is_finite(path_pos[next])
		pos = path_pos

	return pos