from dataclasses import dataclass
from typing import Self
import numpy as np
from math import sin, cos, sqrt
from itertools import product
from util import ff

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
        if d < 0: return self.o
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

    for obj_1, obj_2 in product(space_1, space_2):
        if _key(obj_1) > _key(obj_2): obj_1, obj_2 = obj_2, obj_1
        
        match obj_1, obj_2:
            case Vec(), Vec():
                results.append((obj_2 - obj_1).mag)
            case Vec(), Line():
                results.append(dist(obj_1, obj_2.closest(obj_1)))
            case Vec(), Circle():
                results.append(abs((obj_2.o - obj_1).mag - obj_2.r))
            case _:
                raise NotImplementedError(f"Distance from {obj_1.__class__.__name__} to {obj_2.__class__.__name__} is not implemented.")
        
    return min(results)

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

    for obj_1, obj_2 in product(space_1, space_2):
        if _key(obj_1) > _key(obj_2): obj_1, obj_2 = obj_2, obj_1

        match obj_1, obj_2:
            case All(), _:
                results.append(obj_2)
            case Line(), Line(): # https://math.stackexchange.com/a/406895
                a = np.column_stack([obj_1.v, -obj_2.v])
                b = obj_2.o - obj_1.o
                try:
                    x = np.linalg.solve(a, b)
                except np.linalg.LinAlgError:
                    continue # Parallel Lines
                if not obj_1.bound(x[0]): continue
                if not obj_2.bound(x[1]): continue
                results.append(obj_1.along(x[0]))
            case Line(), Circle(): # https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
                diff = obj_2.o - obj_1.o
                dot = obj_1.v.dot(diff)
                delta = dot**2 - (diff.mag**2-obj_2.r**2)
                if delta < 0: continue
                elif delta == 0:
                    if obj_1.bound(-dot):
                        results.append(obj_1.along(-dot))
                else:
                    delta = sqrt(delta)
                    if obj_1.bound(-dot + delta):
                        results.append(obj_1.along(-dot + delta))
                    if obj_1.bound(-dot - delta):
                        results.append(obj_1.along(-dot - delta))
            case _:
                raise NotImplementedError(f"Meeting of {obj_1.__class__.__name__} and {obj_2.__class__.__name__} is not implemented.")
            

    if len(results) == 1: results = results[0]
    
    return results

def is_finite(space):
    if isinstance(space, Vec): return True
    if isinstance(space, list):
        return all([isinstance(p, Vec) for p in space])
    return False