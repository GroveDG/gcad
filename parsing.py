
from typing import List, Set, Self, Tuple
from pyparsing import Opt, ParseResults, Word, alphas, one_of, DelimitedList, pyparsing_common, Combine
from pint import UnitRegistry
from abc import abstractmethod
from index import Index
import geo
from util import *

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
            raise ValueError("Angle cannot constrain vertex.")
        center = pos[self.vertex]
        base_point = self.end if self.start == target else self.start
        base_pos = pos[base_point]
        base: geo.Vec = (base_pos - center).normalized()
        return [
            geo.Ray(center, base.rotate(self.measure)),
            geo.Ray(center, base.rotate(-self.measure))
        ]
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

# class Parallel(BaseExpr):
#     parser = DelimitedList(
#         Line.parser("lines*"), one_of("||", "∥"), min=2
#     )
#     symbol = "∥"

#     lines: Set[Line]

#     def __init__(self, res: ParseResults) -> None:
#         self.lines = res.lines
# Parallel.parser.add_parse_action(Parallel)

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
    # Parallel,
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