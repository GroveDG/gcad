import networkx as nx
from matplotlib import pyplot as plt
from util import reg_id, discard, get_other
from index import Index
from geo import *
from itertools import permutations, pairwise, chain
from collections import namedtuple, deque
import parsing

ConGraphs = namedtuple('ConGraphs', ['origin', 'orbiter', 'foreward', 'backward', 'checks'])

def solve_figure(ind: Index, root = None):
	if root == None: root = ind.points[0]

	pos = {p: All() for p in ind.points}

	graph = nx.DiGraph()

	# TODO: Figure out best pathing through tree.
	# - Build as many fixed points as possible.
	# - Use as many constraints to fix a point as possible.
	# https://en.wikipedia.org/wiki/Breadth-first_search#Pseudocode
	queue = deque()
	fixed = set([root])
	queue.append(root)
	able_cs = {}
	
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
				(loose,) = loose

				print(p, loose, c)

				if loose not in able_cs:
					able_cs[loose] = [c]
				else:
					able_cs[loose].append(c)
					# Assumption:
					#  2 constraints are always finite.
					# TODO: Rigorously determine finiteness
					if len(able_cs[loose]) == 2:
						graph.add_edge(
							p, loose,
							cs = able_cs[loose]
						)
						to_queue.add(loose)
			for q in to_queue:
				queue.append(q)
				fixed.add(q)
		# Fix a point arbitrarily if there are remaining points
		# that are still not finitely constrained.
		# TODO: Support isolated sub-figures. 
		# (Arbitrary selection on unconstrained points)
		# TODO: Design optimal selection method.
		continuums = set(able_cs.keys()).difference(fixed)
		if len(continuums) > 0:
			rnd_point = continuums.pop()
			able_cs[rnd_point].append("Arbitrary")
			graph.add_edge(
				p, rnd_point,
				cs = able_cs[rnd_point],
				arbitrary=True
			)
			queue.append(rnd_point)
			fixed.add(rnd_point)

	checks = set(ind._cs)
	used = [cs for cs in able_cs.values() if len(cs) >= 2]
	used = set(chain.from_iterable(used))
	checks.difference_update(used)
	del used
	checks = list(checks)

	print(f"origin: {root}")
	print(f"checks: {checks}")
	display(graph, checks)

	quit()

	pos[root] = Vec(0,0)
	
	return pos

def display(con_graph: nx.DiGraph, checks: list):
	graph = nx.Graph()
	edge_colors = []
	edge_labels = {}

	for (u, v, cs) in con_graph.edges.data("cs"):
		graph.add_edge(
			u, v,
			color = "black"
		)
		edge_labels[(u,v)] = "\n".join([str(c) for c in cs])

	for check in checks:
		# TODO: Accurately represent checks with
		# more than 2 points.
		u, v = check.points[0], check.points[1]
		if graph.has_edge(u, v): continue
		graph.add_edge(
			u, v,
			color = "red"
		)
		edge_labels[(u,v)] = str(check)
	
	edge_colors = [color for _,_,color in graph.edges.data("color")]

	pos = nx.spring_layout(graph)
	nx.draw_networkx(
		graph, pos,
		edge_color=edge_colors
	)
	nx.draw_networkx_edge_labels(
		graph, pos,
		edge_labels=edge_labels,
		font_size=9
	)
	plt.show()