
from typing import List, Set, Self, Tuple
from pyparsing import Opt, ParseResults, Word, alphas, one_of, DelimitedList, pyparsing_common, Combine
from pint import UnitRegistry
from abc import abstractmethod
from index import Index
import geo
from util import *
from math import sin, cos, isclose, floor

UREG = UnitRegistry()

QUANTITY = Combine(pyparsing_common.number + Opt(Word(alphas)))
def parse_quantity(s: str, loc: int, res: ParseResults):
    return UREG.parse_expression(res[0])
QUANTITY.set_parse_action(parse_quantity)

class BaseExpr():
    parser = None
    symbol = "�"

    def __init__(self):
        self.points: Tuple[str] = tuple()

    @abstractmethod
    def apply(self, ind: Index): pass

class Assignable(BaseExpr):
    def assign(self, value):
        self.measure = value

class Constraint(BaseExpr):
    def apply(self, ind: Index):
        ind.add_constraint(self)

    @abstractmethod
    def to_geo(self, pos, target): pass

# === Geometry ===

class Point(BaseExpr):
    parser = (
        Word(alphas)
    ).set_parse_action(Self)
    symbol = "∙"

    id: str

    def __init__(self, res: ParseResults) -> None:
        super().__init__()
        self.id = res[0]
    
    def __str__(self) -> str:
        return self.id
Point.parser.set_parse_action(Point)

class Distance(Assignable, Constraint):
    parser = (
        "|" +
        Point.parser("start") +
        Point.parser("end") +
        "|"
    )
    symbol = "|"

    def __init__(self, res: ParseResults) -> None:
        super().__init__()
        self.points = (res.start.id, res.end.id)

    @property
    def start(self): return self.points[0]
    @property
    def end(self): return self.points[1]
        
    def to_geo(self, pos, target):
        base_point = self.end if self.start == target else self.start
        center = pos[base_point]
        return geo.Circle(center, self.measure)
    
    def __str__(self):
        return f"|{' '.join(self.points)}| = {ff(self.measure)}"
    def __repr__(self):
        return self.__str__()
Distance.parser.add_parse_action(Distance)

class Angle(Assignable, Constraint):
    parser = (
        one_of(["<", "∠"]) +
        Point.parser("start") + 
        Point.parser("vertex") + 
        Point.parser("end")
    )
    symbol = "∠"

    def __init__(self, res: ParseResults) -> None:
        super().__init__()
        self.points = (res.start.id, res.vertex.id, res.end.id)

    @property
    def start(self): return self.points[0]
    @property
    def vertex(self): return self.points[1]
    @property
    def end(self): return self.points[2]

    def to_geo(self, pos, target):
        if target == self.vertex:
            # The space is a circle where the known points
            # create a chord. The circle circumscribes an
            # isosceles triangle. The space bellow the line
            # is not valid as the angles sum to over pi.
            # TODO: Replace with arc.
            # Inscribed angles which intercept the same arc
            # are equal.
            # This is the reverse of a circle-circle
            # intersection.
            # https://www.desmos.com/calculator/ecwcottxwy
            p0, p1 = pos[self.start], pos[self.end]
            l = (p1-p0).mag
            assert l != 0
            # Chord length solved for radius
            r = (l/2)/sin(self.measure)
            mid = (p0+p1)/2 # Midpoint
            a = r*cos(self.measure) # Apothem
            if isclose(a, 0, abs_tol=1e-9):
                return geo.Circle(mid, r)
            else:
                v = (p1-p0)/l # Direction
                v.x, v.y = -v.y, v.x # Perpendicular
                v *= a # Vector apothem
                return [
                    geo.Circle(mid+v, r),
                    geo.Circle(mid-v, r)
                ]
        else:
            center = pos[self.vertex]
            base_point = self.end if self.start == target else self.start
            base_pos = pos[base_point]
            base: geo.Vec = (base_pos - center).normalized()
            p, n = base.rot_both(self.measure)
            return [
                geo.Ray(center, p),
                geo.Ray(center, n)
            ]
    
    def __str__(self):
        return f"∠{' '.join(self.points)} = {ff(self.measure)}"
    def __repr__(self):
        return self.__str__()
Angle.parser.set_parse_action(Angle)

# class Line(BaseExpr):
#     parser = (
#         Opt("<")("start_term") + "-" +
#         Point.parser("start") + Point.parser("end") +
#         "-" + Opt(">")("end_term")
#     )
#     symbol = "⟷"

#     points: Tuple[Point]
#     terms: Tuple[bool]

#     def __init__(self, res: ParseResults) -> None:
#         self.points = (res.start, res.end)
#         self.terms = (
#             hasattr(res, "start_term"),
#             hasattr(res, "end_term")
#         )
# Line.parser.set_parse_action(Line)

# class Collinear(BaseExpr):
#     parser = DelimitedList(
#         Point.parser("points*"), "-", min=3
#     )
#     symbol = "⋮"

#     points: Set[Point]

#     def __init__(self, res: ParseResults) -> None:
#         self.points = res.points
# Collinear.parser.set_parse_action(Collinear)

class Parallel(Constraint):
    parser = DelimitedList(
        Point.parser + Point.parser, one_of("||", "∥"), min=2
    )
    symbol = "∥"

    def __init__(self, res: ParseResults) -> None:
        self.points = [p.id for p in res]
    
    def to_geo(self, pos, target):
        t_ind = self.points.index(target)
        o_ind = 2*floor(t_ind/2)+(t_ind%2+1)%2
        o_p = self.points[o_ind]
        for i in range(int(len(self.points)/2)):
            p0, p1 = self.points[2*i:2*(i+1)]
            if p0 not in pos or p1 not in pos: continue
            v = (pos[p1]-pos[p0]).normalized()
            print(p0, p1)
            break
        print(pos[o_p], v)
        return geo.Line(pos[o_p], v)
Parallel.parser.add_parse_action(Parallel)

# class Perpendicular(BaseExpr):
#     parser = DelimitedList(
#         Line.parser("lines*"), one_of("_|_", "⊥"), min=2
#     )
#     symbol = "⊥"

#     lines: List[Line]

#     def __init__(self, res: ParseResults) -> None:
#         self.lines = res.lines
# Perpendicular.parser.add_parse_action(Perpendicular)

class Equality(BaseExpr):
    parser = DelimitedList(
        (Angle.parser | QUANTITY | Distance.parser)("exprs*"), delim="=", min=2
    )
    symbol = "="

    exprs: list

    def __init__(self, res: ParseResults) -> None:
        self.exprs = []
        for expr in res.exprs:
            self.exprs.append(expr[0])
    
    def apply(self, ind: Index):
        exprs = list(self.exprs)
        quantity = [expr for expr in exprs if not isinstance(expr, Assignable)]
        assert len(quantity) <= 1, "Assignents must only have one quantity."
        quantity = quantity[0]
        for expr in exprs:
            if isinstance(expr, Assignable):
                expr.assign(quantity)
            if isinstance(expr, Constraint):
                expr.apply(ind)
Equality.parser.add_parse_action(Equality)

EXPRESSIONS: List[BaseExpr] = [
    Point,
    # Line,
    Angle,
    # Collinear,
    Parallel,
    # Perpendicular,
    Distance,
    Equality
]

# ========== PARSING ========== #

from typing import List
from colorama import Fore, Style

class ParseException(Exception):
    def __init__(self, expr: str) -> None:
        self.expr = expr
        super().__init__(
            f"Unparsed expression: {expr}"
        )

def parse_expr(expr: str, verbose=False):
    def diagnostic_str():
        return (
                Style.DIM +
                Fore.RED + 
                " ".join([expr.symbol for expr in failed]) + 
                Fore.RESET +
                Style.RESET_ALL +
                Fore.GREEN +
                (f" {EXPR.symbol} " if EXPR else "") +
                Fore.RESET + 
                Style.DIM +
                Fore.LIGHTBLACK_EX +
                " ".join([expr.symbol for expr in reversed(untried)])
                + Fore.RESET
                + Style.RESET_ALL
            )
    untried = EXPRESSIONS.copy()
    untried.reverse()
    failed = []
    while len(untried) > 0:
        EXPR = untried.pop()
        try:
            value = EXPR.parser.parse_string(expr, parse_all=True)[0]
            if verbose: print(diagnostic_str())
            return value
        except Exception as e:
            failed.append(EXPR)
            continue
    EXPR = None
    if verbose: print(diagnostic_str())
    raise ParseException(expr)