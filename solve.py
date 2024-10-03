from geo import *
from geo import BaseExpr
from typing import List
from matplotlib import pyplot as plt
import turtle
from time import sleep
from graph import TriGraph

def solve(exprs: List[BaseExpr]):
    graph = TriGraph()
    for expr in exprs:
        expr.apply(graph)
    graph.solve()
    graph.connect()
    graph.coordinate()