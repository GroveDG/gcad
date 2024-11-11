from typing import Self
from dataclasses import dataclass
from math import sqrt

@dataclass
class Vector():
    x: float
    y: float

    def __add__(self, other) -> Self:
        if isinstance(other, Vector):
            return Vector(
                self.x + other.x,
                self.y + other.y
                )
        return Vector(
            self.x + other,
            self.y + other
            )
    
    def __sub__(self, other) -> Self:
        if isinstance(other, Vector):
            return Vector(
                self.x - other.x,
                self.y - other.y
                )
        return Vector(
            self.x - other,
            self.y - other
            )
    
    def __mul__(self, other) -> Self:
        if isinstance(other, Vector):
            return Vector(
                self.x * other.x,
                self.y * other.y
                )
        return Vector(
            self.x * other,
            self.y * other
            )
    
    def __div__(self, other) -> Self:
        if isinstance(other, Vector):
            return Vector(
                self.x / other.x,
                self.y / other.y
                )
        return Vector(
            self.x / other,
            self.y / other
            )
    
    def __len__(self) -> float:
        return sqrt(self.x**2 + self.y**2)
    
    def normalize(self) -> Self:
        return self / len(self)
    
    def dot(self, other: Self) -> Self:
        return self.x * other.y + self.y * other.x
    
    def dist(self, other) -> float:
        match other:
            case Vector():
                return len(other - self)
            case Rays():
                # https://www.geometrictools.com/Documentation/DistancePointLine.pdf
                return other.v.dot(self - other.o) / other.v.dot(other.v)
            case Circle():
                return len(other.o - self) - other.r

@dataclass
class Rays():
    o: Vector
    base: Vector
    angle: float

@dataclass
class Circle():
    o: Vector
    r: float

    # https://math.stackexchange.com/a/1033561
    def intersect(self, other: Self):
        d = len(other - self)

        assert d <= self.r + other.r

        l = (self.r**2 - other.r**2 + d**2)/(2*d)
        h = sqrt(self.r**2 - l**2)

        l_d = l/d
        h_d = h/d

        results = set(
            Vector(
                l_d*(other.o.x-self.o.x) + h_d*(other.o.y-self.o.y) + self.o.x,
                l_d*(other.o.y-self.o.y) - h_d*(other.o.x-self.o.x) + self.o.y,
            )
        )

        if h > 0:
            results.add(Vector(
                l_d*(other.o.x-self.o.x) - h_d*(other.o.y-self.o.y) + self.o.x,
                l_d*(other.o.y-self.o.y) + h_d*(other.o.x-self.o.x) + self.o.y,
            ))

        return results
    