
from dataclasses import dataclass
from typing import List, Set, Self
from networkx import Graph
from pyparsing import Opt, ParseResults, Word, alphas, one_of, DelimitedList, pyparsing_common, Combine
from pint import UnitRegistry
import networkx as nx
from networkx import set_node_attributes
from abc import abstractmethod, ABCMeta
from tri import Triangle

UREG = UnitRegistry()

QUANTITY = Combine(pyparsing_common.number + Opt(Word(alphas)))
def parse_quantity(s: str, loc: int, res: ParseResults):
    return UREG.parse_expression(res[0])
QUANTITY.set_parse_action(parse_quantity)

class BaseExpr():
    parser = None
    symbol = "�"

    @abstractmethod
    def apply(self, graph: nx.Graph): pass

class ElementExpr(BaseExpr):
    @abstractmethod
    def assign(self, graph: nx.Graph, value): pass

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

    # def apply(self, graph: nx.Graph):
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

    # def apply(self, graph: nx.Graph):
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
        points = [self.start, self.vertex, self.end]
        points.sort(key=lambda point: point.id)
        return points

    @property
    def tri_id(self):
        return " ".join([point.id for point in self.points])
    
    @property
    def index(self):
        index = self.points.index(self.vertex)
        match index:
            case 0: return "A"
            case 1: return "B"
            case 2: return "C"
    
    def apply(self, graph: Graph):
        tri_id = self.tri_id
        if not graph.has_node(tri_id):
            tri = Triangle()
            graph.add_node(tri_id)
            graph.nodes[tri_id]["tri"] = tri
    
    def assign(self, graph: Graph, value):
        tri_id = self.tri_id
        tri = graph.nodes[tri_id]["tri"]
        setattr(tri, self.index, value)
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

    def in_tri(self, tri_id: str):
        return (
            self.start.id in tri_id.split(" ") and
            self.end.id in tri_id.split(" ")
        )
    
    def index(self, tri_id: str):
        points = tri_id.split(" ")
        start_index = points.index(self.start.id)
        end_index = points.index(self.end.id)
        index = [0, 1, 2]
        index.remove(start_index)
        index.remove(end_index)
        index = index[0]
        match index:
            case 0: return "a"
            case 1: return "b"
            case 2: return "c"

    def __init__(self, res: ParseResults) -> None:
        self.start = res.start
        self.end = res.end
    def assign(self, graph: Graph, value):
        for node in graph.nodes:
            if self.in_tri(node):
                tri = graph.nodes[node]["tri"]
                setattr(tri, self.index(node), value)
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
    
    def apply(self, graph: Graph):
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

from pathlib import Path
from typing import List
from geo import EXPRESSIONS
from colorama import Back, Fore, Style

class ParseException(Exception):
    def __init__(self, expr: str) -> None:
        self.expr = expr
        super().__init__(
            f"Unparsed expression: {expr}"
        )

def read_file(filepath: Path) -> List[str]:
    with open(filepath) as file:
        doc = file.read()
    doc = doc.replace("\n", ",")
    return doc.split(",")

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