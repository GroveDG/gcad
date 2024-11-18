from dataclasses import dataclass
from typing import Self
import numpy as np
from math import sin, cos

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
        print(col, matrix)
        col = matrix @ col
        return col[:, 0]

@dataclass
class Line():
    o: Vec
    v: Vec

    def join(p0: Vec, p1: Vec) -> Self:
        return Line(p0, (p1 - p0).normalized())

class Ray(Line):
    def join(p0: Vec, p1: Vec) -> Self:
        return Ray(p0, (p1 - p0).normalized())

@dataclass
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
        case Vec(), Line(): # https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Vector_formulation
            p_o = obj_2.o - obj_1
            dot = p_o.dot(obj_2.v)
            if isinstance(obj_2, Line) or dot <= 0:
                diff = p_o - dot * obj_2.v
            else:
                diff = p_o
            return abs(diff)
        case Vec(), Circle():
            return abs(obj_1 - obj_2.o)
        
def meet(space_1, space_2):
    if not isinstance(space_1, list): space_1 = [space_1]
    if not isinstance(space_2, list): space_2 = [space_2]
    
    results = []

    for obj_1 in space_1:
        for obj_2 in space_2:
            if _key(obj_1) > _key(obj_2): obj_1, obj_2 = obj_2, obj_1

            match obj_1, obj_2:
                case Line(), Line(): # https://math.stackexchange.com/a/406895  https://en.wikipedia.org/wiki/Cramer's_rule
                    A = np.column_stack([obj_1.v, -obj_2.v])
                    det_A = np.linalg.det(A)
                    if det_A == 0: continue
                    col_b = (obj_2.o - obj_1.o)
                    A[:,0] = col_b
                    det_A0 = np.linalg.det(A)
                    a = det_A0 / det_A
                    if isinstance(obj_1, Ray) and a < 0: continue
                    if isinstance(obj_2, Ray):
                        A[:,0] = obj_1.v
                        A[:,1] = col_b
                        det_A1 = np.linalg.det(A)
                        b = det_A1 / det_A
                        if b < 0: continue
                    results.append(obj_1.o + obj_1.v*a)

    if len(results) == 1: results = results[0]
    
    return results