from dataclasses import dataclass
from typing import Self
import numpy as np

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
    
    def __abs__(self) -> float:
        return np.linalg.norm(self)
    
    def normalized(self) -> Self:
        return self / abs(self)

@dataclass
class Line():
    o: Vec
    v: Vec

    def join(p0: Vec, p1: Vec) -> Self:
        return Line(p0, (p1 - p0).normalized())

class Ray(Line):
    def join(p0: Vec, p1: Vec) -> Self:
        return Ray(p0, (p1 - p0).normalized())

class Circle():
    o: Vec
    r: float

def _key(obj) -> int:
    match obj:
        case Vec(): return 0
        case Ray(): return 1
        case Line(): return 2
        case Circle(): return 3

def dist(obj_1, obj_2) -> float:
    if _key(obj_1) > _key(obj_2): obj_1, obj_2 = obj_2, obj_1

    match obj_1, obj_2:
        case Vec(), Vec():
            return len(obj_1 - obj_2)
        case Vec(), Ray(): # https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Vector_formulation
            p_o = obj_2.o - obj_1
            dot = p_o.dot(obj_2.v)
            diff = p_o
            if dot <= 0: diff -= dot * obj_2.v
            return abs(diff)
        case Vec(), Line(): # https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Vector_formulation
            p_o = obj_2.o - obj_1
            return abs(p_o - p_o.dot(obj_2.v) * obj_2.v)
        case Vec(), Circle():
            return abs(obj_1 - obj_2.o)
        
def meet(obj_1, obj_2):
    if _key(obj_1) > _key(obj_2): obj_1, obj_2 = obj_2, obj_1

    match obj_1, obj_2:
        case Line(), Line(): # https://math.stackexchange.com/a/406895  https://en.wikipedia.org/wiki/Cramer's_rule
            A = np.column_stack([obj_1.v, -obj_2.v])
            det_A = np.linalg.det(A)
            if det_A == 0: return
            b = (obj_2.o - obj_1.o)
            A[:,0] = b
            det_A0 = np.linalg.det(A)
            a = det_A0 / det_A
            return obj_1.o + obj_1.v*a