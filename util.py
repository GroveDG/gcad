from pathlib import Path
from typing import List, Iterable
import networkx as nx

def ff(n):
	return f"{n:.3f}".rstrip('0').rstrip('.')

def discard(l: list, value):
	try:
		l.remove(value)
	except:
		pass

def ff(n):
	return f"{n:.3f}".rstrip('0').rstrip('.')

def cyclic_pairwise(iterable):
	iterator = iter(iterable)
	first = next(iterator, None)
	a = first

	for b in iterator:
		yield a, b
		a = b
	
	yield a, first

def reg_id(id) -> tuple[str]:
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

def get_other(ids, remove):
	assert len(ids) == 2
	ids = list(ids)
	ids.remove(remove)
	return ids[0]

def read_file(filepath: Path) -> List[str]:
	with open(filepath) as file:
		doc = file.read()
	doc = doc.replace("\n", ",")
	return doc.split(",")

def tree_pos(graph: nx.DiGraph) -> dict:
	pos = {}
	roots = [n for n,d in graph.in_degree() if d==0] 
	for i, layer in enumerate(nx.bfs_layers(graph, roots)):
		for j, n in enumerate(layer):
			pos[n] = (i, j)
	return pos