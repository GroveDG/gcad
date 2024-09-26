from dataclasses import dataclass
from math import sin, cos, tan, asin, acos, atan, pi as PI, sqrt
from argparse import ArgumentParser
from turtle import *
from util import ff
from collections import deque

@dataclass
class Triangle:
    a: float|None = None
    b: float|None = None
    c: float|None = None
    A: float|None = None
    B: float|None = None
    C: float|None = None
    ccw: bool = False
    
    def solve(self):
        self.a, self.b, self.c, self.A, self.B, self.C = solve_tri(
            self.a,
            self.b,
            self.c,
            self.A,
            self.B,
            self.C
        )

    @property
    def solved(self):
        return (
            self.a != None and
            self.b != None and
            self.c != None and
            self.A != None and
            self.B != None and
            self.C != None
        )
    
    def draw(self, start=0, names=["", "", ""]):
        radians()
        sides = deque([self.a, self.b, self.c])
        angles = deque([self.A, self.B, self.C])

        if self.ccw:
            sides.reverse()
            angles.reverse()

        sides.rotate(1)
        angles.rotate(-1)

        seq = deque(zip(sides, angles, names))

        seq.rotate(2*start)

        points = []

        for side, angle, name in seq:
            #write(f"{f"{name}=" if name != "" else ""}{ff(angle)}", align="center")
            points.append(position())
            forward(side/2)
            write(f"{ff(side)}", align="center")
            forward(side/2)
            ext_angle = PI-angle # Convert interior to exterior angle
            if self.ccw: right(ext_angle)
            else: left(ext_angle)
        
        return points

    def __str__(self) -> str:
        return f"Triangle( a = {ff(self.a)}, b = {ff(self.b)}, c = {ff(self.c)}, A = {ff(self.A)}, B = {ff(self.B)}, C = {ff(self.C)} )"

def solve_tri(
        a, b, c,
        A, B, C
    ):
    # Angle Sum
    if A and B: C = PI - (A + B)
    if B and C: A = PI - (B + C)
    if C and A: B = PI - (C + A)

    sines_value = None
    if b and c:
        if A:
            a = law_of_cosines_side(b, c, A)
        elif a:
            A = law_of_cosines_angle(a, b, c)
    if c and a:
        if B:
            b = law_of_cosines_side(c, a, B)
        elif b:
            B = law_of_cosines_angle(b, c, a)
    if a and b:
        if C:
            c = law_of_cosines_side(a, b, C)
        elif c:
            C = law_of_cosines_angle(c, a, b)
    
    if A and a: sines_value = a/sin(A)
    if B and b: sines_value = b/sin(B)
    if C and c: sines_value = c/sin(C)

    if a and not A: A = asin(a / sines_value)
    if b and not B: B = asin(b / sines_value)
    if c and not C: C = asin(c / sines_value)

    # Angle Sum
    if A and B and not C: C = PI - (A + B)
    if B and C and not A: A = PI - (B + C)
    if C and A and not B: B = PI - (C + A)

    if A and not a: a = sin(A) * sines_value
    if B and not b: b = sin(B) * sines_value
    if C and not c: c = sin(C) * sines_value

    if a and b and c and A and B and C:
        return a, b, c, A, B, C
    else:
        raise ValueError(f"""
            Calculation Error: Underspecified triangle.
            a: {a}, b: {b}, c: {c}, A: {A}, B: {B}, C: {C}
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