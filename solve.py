from geo import BaseExpr
from typing import List
from fig import Figure
import argparse
from pathlib import Path
from util import read_file
from geo import parse_expr

def solve(exprs: List[BaseExpr]):
    figure = Figure()
    for expr in exprs:
        expr.apply(figure)
    figure._solve_tris()
    figure._graph()
    print(figure)

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