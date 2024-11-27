import networkx as nx
from matplotlib import pyplot as plt
from util import regularize_id, get_other
from .fig import Figure
from .index import Index
from .geo import *
	
def solve_figure(fig: Figure):
    path_graph = nx.MultiGraph()
    index: Index = Index.from_fig(fig)

    # Collect all points to mark as solved when needed.
    # Points are represented by graph nodes.
    all_points = set()
    # Edges are connected to their points.
    # Edges are represented by graph nodes.
    for edge in fig._edges.keys():
        for point in edge:
            all_points.add(point)
            path_graph.add_edge(point, edge)
    # Angles connect their edges.
    # Angles are represented by graph edges.
    for angle in fig._angles.keys():
        edges = (
            regularize_id(angle[0:2]),
            regularize_id(angle[1:3])
            )
        path_graph.add_edge(*edges)
    rem_points = list(all_points)

    pos = {p: All() for p in all_points}
    
    # Designate a point as the origin.
    origin = rem_points[0]
    pos[origin] = Vec(0,0)
    # Select an edge to be the baseline.
    base = index.edges[origin][0]
    # Assign the endpoint of the baseline a position.
    orbiter = base[1] if base[0] == origin else base[0]
    pos[orbiter] = Vec(fig[base], 0)
    
    # Merge node into "Solved" node for pathfinding and
    # collecting neighbors.
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
            rem_points.remove(node)

    # Mark all starting geometry as solved.
    mark_solved(origin)
    mark_solved(base)
    mark_solved(orbiter)

    # Convert a constraint into its possible solutions (possibility space)
    def con_to_space(con: tuple, target):
        measure = fig[con]
        if len(con) == 2: # Edge/Distance
            center = pos[get_other(con, target)]
            return Circle(center, measure)
        if len(con) == 3: # Angle
            center = pos[con[1]]
            base_point = pos[get_other(con[0:3:2], target)]
            base: Vec = (base_point - center).normalized()
            return [
                Ray(center, base.rotate(measure)),
                Ray(center, base.rotate(-measure))
            ]
                    

    while len(rem_points) > 0:
        # Get all constraints on neighboring points.
        constraints = set()
        for point in rem_points:
            for edge in index.edges[point]:
                if get_other(edge, point) not in rem_points:
                    constraints.add(edge)
            for angle in index.angles[point]:
                points = list(angle)
                points.remove(point)
                if all([p not in rem_points for p in points]):
                    constraints.add(angle)
        # Apply all collected constraints.
        for c in constraints:
            for p in c:
                if p not in rem_points: continue
                point = p
                break
            space = con_to_space(c, point)
            pos[point] = meet(pos[point], space)
        break

    print(pos)

    nx.draw_networkx(path_graph)
    plt.show()

    # nx.dijkstra_path(edge_graph, origin, )