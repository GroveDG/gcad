import networkx as nx
from itertools import pairwise
from matplotlib import pyplot as plt
from .space import Vector, Rays, Circle
from util import regularize_id
from .fig import Figure
from .index import Index
	
def solve_figure(fig: Figure):
    path_graph = nx.MultiGraph()
    index: Index = Index.from_fig(fig)

    all_points = set()
    for edge in fig._edges.keys():
        for point in edge:
            all_points.add(point)
            path_graph.add_edge(point, edge)
    for angle in fig._angles.keys():
        edges = (
            regularize_id(angle[0:2]),
            regularize_id(angle[1:3])
            )
        path_graph.add_edge(*edges)
    all_points = list(all_points)

    nx.draw_networkx(path_graph)
    plt.show()

    rem_points = all_points.copy()
    
    origin = rem_points.pop()
    base = index.edges[origin][0]
    orbiter = base[1] if base[0] == origin else base[0]
    rem_points.remove(orbiter)

    pos = {
        origin: Vector(0,0),
        orbiter: Vector(fig[base], 0)
    }
    
    def mark_solved(node):
        edges = list(path_graph.edges(node))
        path_graph.remove_edges_from(edges)
        path_graph.remove_node(node)
        new_edges = []
        for edge in edges:
            if "Solved" in edge: continue
            edge = list(edge)
            edge.remove(node)
            edge.append("Solved")
            new_edges.append(edge)
        path_graph.add_edges_from(new_edges)
        if isinstance(node, str):
            path_graph.add_edge(node, "Solved")

    mark_solved(origin)
    mark_solved(base)
    mark_solved(orbiter)

    print(rem_points)

    def con_to_space(con: tuple, target):
        measure = fig[con]
        match len(con):
            case 2:
                center = list(con)
                center.remove(target)
                center = center[0]
                return Circle(pos[center], measure)
            case 3:
                i = con.index(target)
                i = 0 if i == 2 else 2
                return Rays(
                    pos[con[1]],
                    (pos[con[i]]-pos[con[1]]).normalize(),
                    measure
                    )
                    

    while len(rem_points) > 0:
        cons = set()
        for point in rem_points:
            for edge in index.edges[point]:
                points = list(edge)
                points.remove(point)
                if points[0] not in rem_points:
                    cons.add(edge)
            for angle in index.angles[point]:
                points = list(angle)
                points.remove(point)
                if points[0] not in rem_points and points[1] not in rem_points:
                    cons.add(angle)
        
        for con in cons:
            for p in con:
                if p in rem_points: continue
                point = p
                break
            if point in pos:
                pos[point] = con_to_space(con)
            else:
                pos[point] = pos[point].intersect(con_to_space(con))
            pos[point]
        break


    print(pos)

    nx.draw_networkx(path_graph)
    plt.show()

    # nx.dijkstra_path(edge_graph, origin, )