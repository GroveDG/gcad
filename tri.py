from dataclasses import dataclass, field
from math import sin, cos, tan, asin, acos, atan2, pi as PI, tau as TAU, sqrt, degrees as to_degrees
from argparse import ArgumentParser
from turtle import *
from util import ff
from collections import deque
from typing import List, Tuple

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
    
    def __str__(self) -> str:
        return f"Triangle( Angles: {self.angles}   Edges: {self.edges} )"
    
    def coordinate(self, points=[Vec2D(0,0),None,None], reverse=False) -> List[Vec2D]:
        if reverse: points.reverse()

        if not any(points):
            points[0] = Vec2D(0,0)

        angles = deque(self.angles)
        if not reverse: angles.rotate(1)
        if reverse: angles.reverse()

        edges = deque(self.edges)
        if reverse: edges.rotate(1)
        if reverse: edges.reverse()

        start = 0
        for i, point in enumerate(points):
            if point != None:
                start = i
                break

        seq = deque(zip(angles, edges))
        seq.rotate(-start)
        seq = list(seq)

        points = deque(points)
        points.rotate(-start)

        total_angle = 0
        if points[0] and points[1]:
            v = points[1] - points[0]
            total_angle = to_degrees(atan2(v[1], v[0]))

        print(points)
        for i, (angle, edge) in enumerate(seq[0:2]):
            point = Vec2D(edge, 0)
            point = point.rotate(total_angle)
            total_angle += to_degrees(PI - angle)
            point += points[i]
            points[i+1] = point
        print(points)
        
        points.rotate(start)
        if reverse: points.reverse()
        points = list(points)
        
        return points

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