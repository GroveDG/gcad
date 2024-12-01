from parsing import BaseExpr
from typing import List
from solver.fig import Figure
import argparse
from pathlib import Path
from util import read_file
from parsing import parse_expr
from solver import solve_figure
import numpy as np

np.set_printoptions(precision=3, suppress=True)

def solve(exprs: List[BaseExpr]):
    figure = Figure()
    for expr in exprs:
        expr.apply(figure)
    pos = solve_figure(figure)
    print(pos)

def solve_file(filepath):
	exprs = read_file(filepath)
	exprs = [parse_expr(expr) for expr in exprs]
	solve(exprs)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="GCAD Solver",
        description="Solver for GCAD files."
    )

    parser.add_argument(
        "filepath",
        help="The path to the .gcad file.",
        type=Path
    )

    args = parser.parse_args()

    solve_file(args.filepath)