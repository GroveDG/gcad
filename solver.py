import networkx as nx
from matplotlib import pyplot as plt
from util import *
from index import Index
from geo import *
from itertools import chain
from collections import deque

ARBITRARY = "Arbitrary"

def solve_figure(ind: Index, root = None):
	if root == None: root = ind.points[0]

	graph = nx.DiGraph()

	# TODO: Figure out best pathing through tree.
	# - Build as many fixed points as possible.
	# - Use as many constraints to fix a point as possible.
	# https://en.wikipedia.org/wiki/Breadth-first_search#Pseudocode
	queue = deque()
	fixed = set([root])
	queue.append(root)
	support = {p: set() for p in ind.points}
	support[root].add(ARBITRARY)
	path = [root]
	
	while len(queue) > 0:
		while len(queue) > 0:
			p = queue.pop()
			to_queue = set()
			for c in ind.get_constraints(p):
				for loose in c.targets(fixed):
					if len(support[loose]) == 2: continue

					support[loose].add(c)

					# Assumption:
					#  2 constraints are always finite.
					# TODO: Rigorously determine finiteness
					if len(support[loose]) == 2:
						graph.add_edge(
							p, loose,
							cs = support[loose]
						)
						to_queue.add(loose)
			for q in to_queue:
				path.append(q)
				queue.append(q)
				fixed.add(q)
		# Fix a point arbitrarily if there are remaining points
		# that are still not finitely constrained.
		# TODO: Support isolated sub-figures. 
		# (Arbitrary selection on unconstrained points)
		# TODO: Design optimal selection method.
		continuums = set([p for p, v in support.items() if len(v)==1]).difference(fixed)
		if len(continuums) > 0:
			rnd_point = continuums.pop()
			support[rnd_point].add(ARBITRARY)
			graph.add_edge(
				p, rnd_point,
				cs = support[rnd_point],
				arbitrary=True
			)
			path.append(rnd_point)
			queue.append(rnd_point)
			fixed.add(rnd_point)
		elif len(set(ind.points).difference(fixed)) == 0:
			break
		else: 
			raise ValueError(f"Figure underspecified from root: {root}\nPath taken: {path}\nCurrent point: {p}")

	checks = set(ind._cs)
	used = [cs for cs in support.values() if len(cs) >= 2]
	used = set(chain.from_iterable(used))
	checks.difference_update(used)
	del used
	checks = list(checks)

	print(f"root: {root}")
	print(f"checks: {checks}")
	print(f"path: {path}")
	print(support)
	# display(graph, checks)

	pos = {}

	for check in checks:
		max_ind = max([path.index(p) for p in check.points])
		support[path[max_ind]].add(check)

	def explore(i):
		p = path[i]
		space = All()
		arbitrary = False
		for c in support[p]:
			if c == ARBITRARY:
				arbitrary = True
				continue
			g = c.to_geo(pos, p)
			space = meet(space, g)
		if arbitrary:
			space = choose(space)
		if isinstance(space, Vec):
			space = [space]
		if space == None:
			print(f"Backtrack from {p}")
			return
		assert isinstance(space, list)
		for s in space:
			assert isinstance(s, Vec)
			pos[p] = s
			if i == len(path)-1:
				return True
			elif explore(i+1):
				return True
		print(f"Backtrack from {p}")

	assert explore(0), "Over-constrained and unsolvable."
	
	return pos

def display(con_graph: nx.DiGraph, checks: list):
	edge_labels = {}

	for (u, v, cs) in con_graph.edges.data("cs"):
		edge_labels[(u,v)] = "\n".join([str(c) for c in cs])

	pos = tree_pos(con_graph)
	
	for check in checks:
		draw_check(check, pos)

	nx.draw_networkx(
		con_graph, pos
	)
	nx.draw_networkx_edge_labels(
		con_graph, pos,
		edge_labels=edge_labels,
		font_size=9
	)
	plt.show()

def draw_check(check, pos):
	po = [pos[p] for p in check.points]
	po.sort()
	po = np.stack(po)
	min_x, min_y = np.min(po, axis=0) - 0.5
	max_x, max_y = np.max(po, axis=0) + 0.5
	min_y -= 0.1*(min_x%3)
	max_y -= 0.1*(min_x%3)
	new_pos = [
		(min_x, min_y),
		(min_x, max_y),
		(max_x, max_y),
		(max_x, min_y),
	]
	po = np.stack(new_pos).T
	plt.fill(
		po[0], po[1],
		facecolor=(1,0,0,0.05),
		edgecolor=(1,0,0,0.5), linewidth=1
	)
	plt.annotate(
		str(check),
		(min_x, min_y),
		bbox = {
			"boxstyle": "round",
			"ec": (1.0, 0.0, 0.0, 0.5),
			"fc": (1.0, 1.0, 1.0)
		}
	)