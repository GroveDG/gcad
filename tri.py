from dataclasses import dataclass, field
from math import sin, cos, tan, asin, acos, atan, pi as PI, sqrt
from argparse import ArgumentParser
from turtle import *
from util import ff
from collections import deque
from typing import List

@dataclass
class Triangle:
    angles: List[float] = field(default_factory=lambda: [None, None, None])
    edges: List[float] = field(default_factory=lambda: [None, None, None])

    def solve(self):
        self.angles, self.edges = solve_tri(self.angles, self.edges)

    @property
    def solved(self):
        return (
            all([edge != None for edge in self.edges]) and
            all([angle != None for angle in self.angles])
        )
    
    def draw(self, start=0, names=["", "", ""]):
        radians()
        edges = deque(self.edges.copy())
        angles = deque(self.angles.copy())

        # if self.ccw:
        #     sides.reverse()
        #     angles.reverse()

        edges.rotate(1)
        angles.rotate(-1)

        seq = deque(zip(edges, angles, names))

        seq.rotate(2*start)

        points = []

        for edge, angle, name in seq:
            #write(f"{f"{name}=" if name != "" else ""}{ff(angle)}", align="center")
            points.append(position())
            forward(edge/2)
            write(f"{ff(edge)}", align="center")
            forward(edge/2)
            ext_angle = PI-angle # Convert interior to exterior angle
            # if self.ccw: right(ext_angle)
            left(ext_angle)
        
        return points

    def __str__(self) -> str:
        return f"Triangle( Angles: {self.angles}   Edges: {self.edges} )"

def solve_tri(angles: List[float], edges: List[float]):
    # Angle Sum
    if angles.count(None) == 1:
        n = angles.index(None)
        angles[n] = PI - (angles[n-1] + angles[n-2])

    # Law of Cosines
    for n in range(3):
        if not edges[n-1] or not edges[n-2]:
            continue
        if angles[n] and not edges[n]:
            edges[n] = law_of_cosines_side(edges[n-1], edges[n-2], angles[n])
        if edges[n] and not angles[n]:
            angles[n] = law_of_cosines_angle(edges[n], edges[n-1], edges[n-2])

    # Law of Sines
    sines_value = None
    for n in range(3):
        if angles[n] and edges[n]:
            sines_value = edges[n]/sin(angles[n])
            break
        
    if sines_value:
        for n in range(3):
            if edges[n] and not angles[n]:
                angles[n] = asin(edges[n] / sines_value)

    # Angle Sum
    if angles.count(None) == 1:
        n = angles.index(None)
        angles[n] = PI - (angles[n-1] + angles[n-2])

    # Law of Sines
    for n in range(3):
        if angles[n] and not edges[n]:
            edges[n] = sin(angles[n]) * sines_value

    if all(angles) and all(edges):
        return angles, edges
    else:
        raise ValueError(f"""
            Calculation Error: Underspecified triangle.
            Angles: {angles}
            Edges: {edges}
        """)

def law_of_cosines_side(b, c, A):
    return sqrt(b**2 + c**2 - 2*b*c*cos(A))

def law_of_cosines_angle(a, b, c):
    return acos(
        (b**2 + c**2 - a**2)
        /(2*b*c)
    )

if __name__ == "__main__":
    parser = ArgumentParser(
        prog="Triangle Solver",
        description="Solves a triangle given sides and angles."
    )
    
    parser.add_argument("--a", required=False, type=float)
    parser.add_argument("--b", required=False, type=float)
    parser.add_argument("--c", required=False, type=float)
    parser.add_argument("--A", required=False, type=float)
    parser.add_argument("--B", required=False, type=float)
    parser.add_argument("--C", required=False, type=float)

    args = parser.parse_args()

    tri = Triangle(
        a=args.a,
        b=args.b,
        c=args.c,
        A=args.A,
        B=args.B,
        C=args.C,
    )
    
    tri.solve()
    tri.draw()

    done()