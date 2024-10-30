from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Self
import numpy as np

class ClusterType(Enum):
    Rigid = auto()
    Scalable = auto()
    Radial = auto()

@dataclass
class Cluster():
    type: ClusterType
    points: set = field(default_factory=set)
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

    shared_points = c1.points.intersection(c2.points)
    assert len(shared_points) == 2
    
    result = c1 | c2

    return result

def rule_2(c1: Cluster, c2: Cluster) -> Cluster:
    assert c1.type == ClusterType.Radial and c2.type == ClusterType.Radial

    shared_points = c1.points.intersection(c2.points)
    assert len(shared_points) == 2
    
    result = c1 | c2

    return result

def rule_3(c1: Cluster, c2: Cluster) -> Cluster:
    assert c1.type == ClusterType.Scalable and c2.type == ClusterType.Scalable

    shared_points = c1.points.intersection(c2.points)
    assert len(shared_points) == 2
    
    result = c1 | c2

    return result