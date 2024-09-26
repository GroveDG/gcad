import networkx as nx
from tri import Triangle
from typing import List

def add_tri(graph: nx.Graph, tri_id) -> Triangle:
    tri = Triangle()
    graph.add_node(tri_id)
    graph.nodes[tri_id]["tri"] = tri
    
    connect_tri(graph, tri_id)
    
    return tri
   
def connect_tri(graph: nx.Graph, tri_id)
	for tri_i, data in graph.nodes(data=True):
		shared = relate_tris(graph, tri_id, tri_i)

class Relation():

	def __init__(self, tri_1: str, tri_2: str):
		points_1 = tri_id_to_points(tri_1)
		set_1 = set(points_1)
	
		points_2 = tri_id_to_points(tri_2)
		set_2 = set(points_2)
	
		shared = set.intersection(points_1, points_2)
	
		match len(shared):
			case 0:
				# no relation
			case 1:
				# shared point
			case 2:
				# shared edge
			case 3:
				# same triangle

def regularize_tri_id(tri_id: str) -> str:
	points = tri_id_to_points(tri_id)
	points.sort()
	return " ".join(points)
	
def tri_id_to_points(tri_id: str) -> List[str]:
	return tri_id.split(" ")

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
		self._targets = {}
		
	def add_tri(self, tri_id: str):
		tri_id = regularize_tri_id(tri_id)
		
		points = tri_id_to_points(tri_id)
		for point in points:
			self._points[point] = None
		
		for edge in pairwise(points):
			self._edges[edge] = None
		
		for angle in 