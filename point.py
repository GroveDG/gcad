from dataclasses import dataclass
from typing import Self
from functools import cached_property
from tri import law_of_cosines
from math import sqrt, cos, sin, atan2

@dataclass
class Point():
    t: float
    r: float
    rel_to: Self|None = None

    @cached_property
    def simplified(self) -> Self:
        if self.rel_to == None:
            return self
        rel: Self = self.rel_to.simplified
        d_t = self.t-rel.t
        r = sqrt(rel.r**2 + self.r**2 - 2*rel.r*self.r*cos(d_t))
        t = rel.t + atan2(self.r*sin(d_t), rel.r+self.r*cos(d_t))
        return Point(t, r)
    
    @cached_property
    def cartesian(self) -> tuple[float, float]:
        return (
            self.r*cos(self.t),
            self.r*sin(self.t)
        )
    
def angle_btw_cartesian(p1, p2) -> float:
    return atan2(p2[1]-p1[1], p2[0]-p1[0])