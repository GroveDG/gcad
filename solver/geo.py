from dataclasses import dataclass
from typing import Self
import numpy as np
from math import sin, cos, sqrt, isclose
from itertools import product
from util import ff

ABS_TOL = 1e-8

class All():
    def __repr__(self) -> str:
        return "Any"

class Vec(np.ndarray):
    def __new__(cls, x, y):
        obj = np.asarray([x,y]).view(cls)
        return obj

    @property
    def x(self) -> float:
        return self[0]
    
    @property
    def y(self) -> float:
        return self[1]
    
    @property
    def mag(self) -> float:
        return np.linalg.norm(self)
    
    def normalized(self) -> Self:
        return self / self.mag
    
    def polar(angle, r=1) -> Self:
        return r*Vec(cos(angle), sin(angle))
    
    def rotate(self, angle) -> Self:
        c = cos(angle)
        s = sin(angle)
        rotation = np.asarray([
            [c, -s],
            [s, c]
        ])
        return self.transform(rotation)
    
    def transform(self, matrix) -> Self:
        col = self.reshape(-1, 1)
        col = matrix @ col
        return col[:, 0]
    
    def __repr__(self) -> str:
        return f"({ff(self.x)}, {ff(self.y)})"

@dataclass
class Line():
    o: Vec
    v: Vec

    @classmethod
    def join(cls, p0: Vec, p1: Vec) -> Self:
        return cls(p0, (p1 - p0).normalized())
    
    def closest(self, p: Vec) -> Vec:
        d = (p - self.o).dot(self.v)
        return self.along(self.clamp(d))
    
    def along(self,  d: float) -> Vec:
        if not self.bound(d): raise ValueError("Distance along Line/Ray out of bounds.")
        return self.o + self.v*d
    
    def bound(self, d: float) -> bool:
        return True
    
    def clamp(self, d: float) -> float:
        if self.bound(d): return d
        if d < 0: return 0
class Ray(Line):
    def bound(self, d: float) -> bool:
        return d >= 0

@dataclass
class Circle():
    o: Vec
    r: float

def _key(obj) -> int:
    match obj:
        case All(): return -1
        case Vec(): return 0
        case Ray(): return 1
        case Line(): return 2
        case Circle(): return 3

def dist(space_1, space_2) -> float:
    if not isinstance(space_1, list): space_1 = [space_1]
    if not isinstance(space_2, list): space_2 = [space_2]
    
    results = []

    for objs in product(space_1, space_2):
        results.append(_dist_pair_approx(*objs))
        
    return min(results)

def _dist_pair_approx(obj_1, obj_2):
    if _key(obj_1) > _key(obj_2): obj_1, obj_2 = obj_2, obj_1
    match obj_1, obj_2:
        case Vec(), Vec():
            diff = (obj_2 - obj_1)
            return diff.x + diff.y
        case Vec(), Line():
            return dist(obj_1, obj_2.closest(obj_1))
        case Vec(), Circle():
            return abs(dist(obj_1, obj_2.o) - obj_2.r)
        case _:
            raise NotImplementedError(f"Distance from {obj_1.__class__.__name__} to {obj_2.__class__.__name__} is not implemented.")

def _dist_pair(obj_1, obj_2):
    if _key(obj_1) > _key(obj_2): obj_1, obj_2 = obj_2, obj_1
    
    match obj_1, obj_2:
        case Vec(), Vec():
            return (obj_2 - obj_1).mag
        case Vec(), Line():
            return dist(obj_1, obj_2.closest(obj_1))
        case Vec(), Circle():
            return abs((obj_2.o - obj_1).mag - obj_2.r)
        case _:
            raise NotImplementedError(f"Distance from {obj_1.__class__.__name__} to {obj_2.__class__.__name__} is not implemented.")

def meet(*spaces):
    spaces = list(spaces)
    result = spaces.pop()
    for space in spaces:
        result = _meet_pair(result, space)
    return result

def _meet_pair(space_1, space_2):
    if not isinstance(space_1, list): space_1 = [space_1]
    if not isinstance(space_2, list): space_2 = [space_2]
    
    results = []

    for objs in product(space_1, space_2):
        result = _intersect(*objs)
        if result is not None:
            if isinstance(result, list):
                results = [*results, *result]
            else:
                results.append(result)

    if len(results) == 1: results = results[0]
    
    return results

def _intersect(obj_1, obj_2):
    if _key(obj_1) > _key(obj_2): obj_1, obj_2 = obj_2, obj_1

    match obj_1, obj_2:
        case All(), _:
            return obj_2
        case Vec(), _:
            d = dist(obj_1, obj_2)
            if isclose(d, 0, abs_tol=ABS_TOL):
                return obj_1
        case Line(), Line():
            return _line_line(obj_1, obj_2)
        case Line(), Circle():
            return _line_circle(obj_1, obj_2)
        case Circle(), Circle():
            return _circle_circle(obj_1, obj_2)
        case _:
            raise NotImplementedError(f"Meeting of {obj_1.__class__.__name__} and {obj_2.__class__.__name__} is not implemented.")
        
def _line_line(l1: Line, l2: Line): # https://math.stackexchange.com/a/406895
    a = np.column_stack([l1.v, -l2.v])
    b = l2.o - l1.o
    try:
        x = np.linalg.solve(a, b)
    except np.linalg.LinAlgError:
        return # Parallel Lines
    if not l1.bound(x[0]): return
    if not l2.bound(x[1]): return
    return l1.along(x[0])

def _line_circle(l: Line, c: Circle): # https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
    diff: Vec = l.o - c.o
    dot = l.v.dot(diff)
    delta = dot**2 - (diff.mag**2-c.r**2)
    if isclose(delta, 0, abs_tol=sqrt(ABS_TOL)):
        if l.bound(-dot): return l.along(-dot)
    elif delta < 0: return
    else:
        delta = sqrt(delta)
        bounds = (l.bound(-dot + delta), l.bound(-dot - delta))
        if all(bounds):
            return [
                l.along(-dot + delta),
                l.along(-dot - delta)
            ]
        elif any(bounds):
            return l.along(-dot + delta) if bounds[0] else l.along(-dot - delta)

def _circle_circle(c1: Circle, c2: Circle): # https://stackoverflow.com/a/3349134
    d = dist(c1.o, c2.o)
    if d > c1.r + c2.r: return
    a = (c1.r**2 - c2.r**2 + d**2)/(2*d)
    dir: Vec = (c2.o-c1.o)/d
    center = c1.o + a*dir
    if d == c1.r + c2.r:
        return center
    h = sqrt(c1.r**2 - a**2)
    h_v = h*Vec(dir.y,-dir.x)
    return [center+h_v, center-h_v]

def is_finite(space):
    if isinstance(space, Vec): return True
    if isinstance(space, list):
        if len(space) == 0: raise ValueError("Figure overconstrained.")
        return all([isinstance(p, Vec) for p in space])
    return False