from parsing import BaseExpr, parse_expr
from typing import List
import argparse
from pathlib import Path
from util import read_file
from solver import solve_figure
import numpy as np
from time import time
import cProfile
import pstats
from index import Index

np.set_printoptions(precision=3, suppress=True)

def solve(exprs: List[BaseExpr], profile=False):
    ind = Index()
    for expr in exprs:
        expr.apply(ind)
    if profile:
        cProfile.runctx(
            'solve_figure(ind)',
            globals=globals(),
            filename="profile",
            locals={"ind": ind}
        )
        with open("profile.txt", mode='w') as file:
            stats = pstats.Stats("profile", stream=file)
            stats = stats.strip_dirs()
            stats = stats.sort_stats("cumulative")
            stats.print_stats()
    else:
        start = time()
        pos = solve_figure(ind)
        end = time()
        print(end-start)
        print(pos)

def solve_file(filepath, **kwargs):
	exprs = read_file(filepath)
	exprs = [parse_expr(expr) for expr in exprs]
	solve(exprs, **kwargs)

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

    parser.add_argument(
        '-p', '--profile',
        help="Uses cProfile to profile solver.",
        action="store_true"
    )

    args = parser.parse_args()

    solve_file(**vars(args))