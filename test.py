import argparse
from pathlib import Path
from geo import parse_expr, read_file
from colorama import Fore, Back, Style
from shutil import get_terminal_size
from textwrap import wrap
from itertools import zip_longest
from solve import solve

def solve_file(filepath):
	exprs = read_file(filepath)
	exprs = [parse_expr(expr) for expr in exprs]
	solve(exprs)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="GCAD Test Suite",
        description="The GCAD test suite for debugging, testing, and validating GCAD files."
    )
    parser.add_argument(
        "command",
        help="The command to run. One of: validate."
    )
    parser.add_argument(
        "filepath",
        help="The path to the .gcad file.",
        type=Path
    )

    args = parser.parse_args()

    match args.command:
        case "validate":
            exprs = read_file(args.filepath)
            for expr in exprs:
                try:
                    right = parse_expr(expr, verbose=True)
                    success = True
                except Exception as e:
                    right = e
                    success = False
                width = get_terminal_size()[0]
                l_width = min(width//2 - 2, len(str(expr)))
                r_width = width - l_width - 4
                left = wrap(str(expr), l_width)
                right = wrap(str(right), r_width)
                for (left_line, right_line) in zip_longest(left, right, fillvalue=""):
                    print(
                        (Fore.GREEN if success else Fore.LIGHTRED_EX) +
                        f"{left_line : <{l_width}}" +
                        "    " + 
                        f"{right_line : >{r_width}}" +
                        Fore.RESET
                    )
        case "solve":
            solve_file(args.filepath)