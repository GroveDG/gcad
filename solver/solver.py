import networkx as nx
from matplotlib import pyplot as plt
from util import reg_id, get_other
from .fig import Figure
from .index import Index
from .geo import *
	
def solve_figure(fig: Figure):
    index: Index = Index.from_fig(fig)

    rem_points = index.points

    pos = {p: All() for p in rem_points}
    
    # Designate a point as the origin.
    origin = rem_points[0]
    pos[origin] = Vec(0,0)
    # Select an edge to be the baseline.
    base = index.edges[origin][0]
    # Assign the endpoint of the baseline a position.
    orbiter = base[1] if base[0] == origin else base[0]
    pos[orbiter] = Vec(fig[base], 0)

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
    
    # Merge node into "Solved" node for pathfinding and
    # collecting neighbors.
    def mark_solved(*points):
        for point in points:
            rem_points.remove(point)
        # Get all constraints on neighboring points.
        constraints = set()
        for point in points:
            for c in index.get_all(point):
                target = [p for p in c if p in rem_points]
                if len(target) == 1:
                    constraints.add((c, target[0]))
        # Apply all collected constraints.
        for c, target in constraints:
            space = con_to_space(c, target)
            pos[target] = meet(pos[target], space)

    # Mark all starting geometry as solved.
    mark_solved(origin, orbiter)

    def path_graph() -> nx.MultiDiGraph:
        graph = nx.MultiDiGraph()
        for point in rem_points:
            for angle in index.angles[point]:
                if not angle[1] == point: continue
                e1, e2 = angle[1::-1], angle[1:3]
                print(e1, e2)
                if reg_id(e1) in index.edges[point]:
                    graph.add_edge(*e1)
                if reg_id(e2) in index.edges[point]:
                    graph.add_edge(*e2)
        return graph
    
    while len(rem_points) > 0:
        continuums, finites = [], []
        for point in rem_points:
            if type(pos[point]) is All: continue
            if isinstance(pos[point], list) and all([type(p) is Vec for p in pos[point]]):
                finites.append(point)
            else:
                continuums.append(point)
        print(finites, continuums)
        graph = path_graph()
        nx.draw_networkx(graph)
        plt.show()
        break

    print(pos)

    # nx.dijkstra_path(edge_graph, origin, )