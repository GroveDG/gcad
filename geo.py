
from dataclasses import dataclass
from typing import List, Set, Self
from fig import Figure
from pyparsing import Opt, ParseResults, Word, alphas, one_of, DelimitedList, pyparsing_common, Combine
from pint import UnitRegistry
from abc import abstractmethod

UREG = UnitRegistry()

QUANTITY = Combine(pyparsing_common.number + Opt(Word(alphas)))
def parse_quantity(s: str, loc: int, res: ParseResults):
    return UREG.parse_expression(res[0])
QUANTITY.set_parse_action(parse_quantity)

class BaseExpr():
    parser = None
    symbol = "�"

    @abstractmethod
    def apply(self, graph: Figure): pass

class ElementExpr(BaseExpr):
    @abstractmethod
    def assign(self, graph: Figure, value): pass

# === Geometry ===

class Point(BaseExpr):
    parser = (
        Word(alphas)
    ).set_parse_action(Self)
    symbol = "∙"

    id: str

    def __init__(self, res: ParseResults) -> None:
        self.id = res[0]
    
    def __str__(self) -> str:
        return self.id

    # def apply(self, graph: nx.TriGraph):
    #     graph.add_node(self.id)f"{
Point.parser.set_parse_action(Point)

class Line(ElementExpr):
    parser = (
        Opt("<")("start_term") + "-" +
        Point.parser("start") + Point.parser("end") +
        "-" + Opt(">")("end_term")
    )
    symbol = "⟷"

    start: Point
    end: Point
    start_term: bool
    end_term: bool

    def __init__(self, res: ParseResults) -> None:
        self.start = res.start
        self.end = res.end
        self.start_term = hasattr(res, "start_term")
        self.end_term = hasattr(res, "end_term")

    # def apply(self, graph: nx.TriGraph):
    #     graph.add_edge(self.start.id, self.end.id)
Line.parser.set_parse_action(Line)

class Angle(ElementExpr):
    parser = (
        one_of(["<", "∠"]) +
        Point.parser("start") + 
        Point.parser("vertex") + 
        Point.parser("end")
    )
    symbol = "∠"
    
    start: Point
    vertex: Point
    end: Point

    def __init__(self, res: ParseResults) -> None:
        self.start = res.start
        self.vertex = res.vertex
        self.end = res.end
    
    @property
    def points(self):
        return [self.start, self.vertex, self.end]
    
    def apply(self, graph: Figure):
        graph._add_tri(self.points)
    
    def assign(self, graph: Figure, value):
        graph[*self.points] = value
Angle.parser.set_parse_action(Angle)

class Collinear(BaseExpr):
    parser = DelimitedList(
        Point.parser("points*"), "-", min=3
    )
    symbol = "⋮"

    points: Set[Point]

    def __init__(self, res: ParseResults) -> None:
        self.points = res.points
Collinear.parser.set_parse_action(Collinear)

class Parallel(BaseExpr):
    parser = DelimitedList(
        Line.parser("lines*"), one_of("||", "∥"), min=2
    )
    symbol = "∥"

    lines: Set[Line]

    def __init__(self, res: ParseResults) -> None:
        self.lines = res.lines
Parallel.parser.add_parse_action(Parallel)

class Perpendicular(BaseExpr):
    parser = DelimitedList(
        Line.parser("lines*"), one_of("_|_", "⊥"), min=2
    )
    symbol = "⊥"

    lines: List[Line]

    def __init__(self, res: ParseResults) -> None:
        self.lines = res.lines
Perpendicular.parser.add_parse_action(Perpendicular)

class Distance(ElementExpr):
    parser = (
        "|" +
        Point.parser("start") +
        Point.parser("end") +
        "|"
    )
    symbol = "|"

    start: Point
    end: Point

    def __init__(self, res: ParseResults) -> None:
        self.start = res.start
        self.end = res.end
    
    @property
    def points(self):
        return [self.start, self.end]
        
    def assign(self, graph: Figure, value):
        graph[*self.points] = value
Distance.parser.add_parse_action(Distance)

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
    
    def apply(self, graph: Figure):
        exprs = list(self.exprs)
        for expr in exprs:
            if isinstance(expr, ElementExpr):
                expr.apply(graph)
            else:
                quantity = expr
        for expr in exprs:
            if isinstance(expr, ElementExpr):
                expr.assign(graph, quantity)
Equality.parser.add_parse_action(Equality)

EXPRESSIONS: List[BaseExpr] = [
    Point,
    Line,
    Angle,
    Collinear,
    Parallel,
    Perpendicular,
    Distance,
    Equality
]

# ========== PARSING ========== #

from typing import List
from geo import EXPRESSIONS
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