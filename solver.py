import networkx as nx
from matplotlib import pyplot as plt
from util import *
from index import Index
from geo import *
from itertools import permutations, pairwise, chain
from collections import namedtuple, deque
import parsing

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
	support = {p: [] for p in ind.points}
	support[root].append(ARBITRARY)
	path = [root]
	
	while len(queue) > 0:
		while len(queue) > 0:
			p = queue.pop()
			to_queue = set()
			for c in ind.get_constraints(p):
				# Assumption:
				#  A constraint must have all points
				#  except for one to be applied.
				# TODO: Allow constraints to specify
				# required points.
				loose = set(c.points).difference(fixed)
				if len(loose) != 1: continue
				loose, = loose

				support[loose].append(c)

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
		continuums = set(support.keys()).difference(fixed)
		if len(continuums) > 0:
			rnd_point = continuums.pop()
			support[rnd_point].append(ARBITRARY)
			graph.add_edge(
				p, rnd_point,
				cs = support[rnd_point],
				arbitrary=True
			)
			path.append(rnd_point)
			queue.append(rnd_point)
			fixed.add(rnd_point)

	checks = set(ind._cs)
	used = [cs for cs in support.values() if len(cs) >= 2]
	used = set(chain.from_iterable(used))
	checks.difference_update(used)
	del used
	checks = list(checks)

	print(f"root: {root}")
	print(f"checks: {checks}")
	# display(graph, checks)

	pos = {}
	ind_in_finite = [0]*len(path)

	for check in checks:
		max_ind = max([path.index(p) for p in check.points])
		support[path[max_ind]].append(check)

	for i, p in enumerate(path):
		space = All()
		for c in support[p]:
			if c == ARBITRARY:
				space = choose(space)
			else:
				space = meet(space, c.to_geo(pos, p))
		if isinstance(space, list):
			space = space[ind_in_finite[i]]
		assert isinstance(space, Vec)
		pos[p] = space

	print(pos)

	quit()
	
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