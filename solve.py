from geo import *
from geo import BaseExpr
import networkx as nx
from typing import List
from matplotlib import pyplot as plt
import turtle
from time import sleep

def solve(exprs: List[BaseExpr]):
    graph = nx.Graph()
    solved_tris = []
    iter_diff = True
    while iter_diff:
        iter_diff = False
        for expr in exprs:
            expr.apply(graph)
        for tri_id, tri in graph.nodes(data="tri", default=None):
            if tri.solved:
                continue
            try:
                tri.solve()
                iter_diff = True
                solved_tris.append(tri)
            except Exception as e:
                print(e)
    
    for tri_id, tri in graph.nodes(data="tri", default=None):
        print(tri_id, tri)
    nx.draw_networkx(graph)
    plt.show()

    size = turtle.screensize()
    points = {}
    for tri_id, tri in graph.nodes(data="tri", default=None):
        point_ids = tri_id.split(" ")
        contained_points = []
        for point_id in point_ids:
            if point_id in points:
                contained_points.append(point_id)
        match len(contained_points):
            case 0:
                point_pos = tri.draw(names=point_ids)
            case 1:
                start = contained_points[0]
                index = point_ids.index(start)
                point = points[start]
                turtle.teleport(point[0], point[1])
                point_pos = tri.draw(start=index, names=point_ids)
            case 2:
                start = contained_points[0]
                end = contained_points[1]
                start_index = point_ids.index(start)
                end_index = point_ids.index(end)

                if start_index > end_index:
                    start_index, end_index = end_index, start_index
                    start, end = end, start

                start_point = points[start]
                end_point = points[end]
                turtle.teleport(start_point[0], start_point[1])
                turtle.setheading(turtle.towards(end_point))
                point_pos = tri.draw(start=start_index, names=point_ids)
        for point, point_id in zip(point_pos, point_ids):
            points[point_id] = point
    
    print(points)
    
    turtle.done()