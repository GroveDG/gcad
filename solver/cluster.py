from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Self
import numpy as np
from points import Points

class ClusterType(Enum):
    Rigid = auto()
    Scalable = auto()
    Radial = auto()

@dataclass
class Cluster():
    type: ClusterType
    points: Points = field(default_factory=Points)
    dists: dict = field(default_factory=dict)
    angles: dict = field(default_factory=dict)

    def __or__(self: Self, other: Self) -> Self:
        if not c_type:
            c_type = self.type
        
        return Cluster(
            c_type,
            self.points | other.points,
            self.dists | other.dists,
            self.angles | other.angles
            )

def rule_1(c1: Cluster, c2: Cluster) -> Cluster:
    assert c1.type == ClusterType.Rigid and c2.type == ClusterType.Rigid

	c1.points.map_to(c2.points)
    
    result = c1 | c2

    return result

def rule_2(c1: Cluster, c2: Cluster) -> Cluster:
    assert c1.type == ClusterType.Radial and c2.type == ClusterType.Radial

    c1.points.map_to(c2.points, do_scale=True)
    
    result = c1 | c2

    return result

def rule_3(c1: Cluster, c2: Cluster) -> Cluster:
    assert c1.type == ClusterType.Scalable and c2.type == ClusterType.Scalable

    c1.points.map_to(c2.points, do_scale=True)
    
    result = c1 | c2

    return result