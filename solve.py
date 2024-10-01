from geo import *
from geo import BaseExpr
from typing import List
from matplotlib import pyplot as plt
import turtle
from time import sleep
from graph import TriGraph

def solve(exprs: List[BaseExpr]):
    graph = TriGraph()
    solved_tris = []
    iter_diff = True
    for expr in exprs:
        expr.apply(graph)
    print(graph._points)
    print(graph._edges)
    print(graph._angles)
    print(graph._tris)
    graph.solve()

    for id in graph._tris:
        tri = graph._get_tri(id)
        tri.draw()
        
    # points = {}
    # for tri_id, tri in graph.nodes(data="tri", default=None):
    #     point_ids = tri_id.split(" ")
    #     contained_points = []
    #     for point_id in point_ids:
    #         if point_id in points:
    #             contained_points.append(point_id)
    #     match len(contained_points):
    #         case 0:
    #             point_pos = tri.draw(names=point_ids)
    #         case 1:
    #             start = contained_points[0]
    #             index = point_ids.index(start)
    #             point = points[start]
    #             turtle.teleport(point[0], point[1])
    #             point_pos = tri.draw(start=index, names=point_ids)
    #         case 2:
    #             start = contained_points[0]
    #             end = contained_points[1]
    #             start_index = point_ids.index(start)
    #             end_index = point_ids.index(end)

    #             if start_index > end_index:
    #                 start_index, end_index = end_index, start_index
    #                 start, end = end, start

    #             start_point = points[start]
    #             end_point = points[end]
    #             turtle.penup()
    #             turtle.setpos(start_point[0], start_point[1])
    #             turtle.pendown()
    #             turtle.setheading(turtle.towards(end_point))
    #             point_pos = tri.draw(start=start_index, names=point_ids)
    #     for point, point_id in zip(point_pos, point_ids):
    #         points[point_id] = point
    
    # print(points)
    
    turtle.done()