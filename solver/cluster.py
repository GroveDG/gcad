from dataclasses import dataclass, field
from enum import Enum, auto
from typing import Self
import numpy as np
from points import Points
from copy import copy

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

	c2.points.map_to(c1.points)
	
	result = c1 | c2

	return result

def rule_2(c1: Cluster, c2: Cluster) -> Cluster:
	assert c1.type == ClusterType.Radial and c2.type == ClusterType.Radial

	c2.points.map_to(c1.points, do_scale=True)
	
	result = c1 | c2

	return result

def rule_3(c1: Cluster, c2: Cluster) -> Cluster:
	assert c1.type == ClusterType.Scalable and c2.type == ClusterType.Scalable

	c2.points.map_to(c1.points, do_scale=True)
	
	result = c1 | c2

	return result

def rule_4(c1: Cluster, c2: Cluster, c3: Cluster) -> Cluster:
	assert c1.type == ClusterType.Rigid and c2.type == ClusterType.Rigid and c3.type == ClusterType.Rigid

	# The index of the point corresponds to the cluster it is NOT in.
	# ex: shared_points[0] is not in c1
	shared_points = [
		c2.points.ids.intersection(c3.points.ids),
		c3.points.ids.intersection(c1.points.ids),
		c1.points.ids.intersection(c2.points.ids)
	]

	for i in range(3):
		assert len(shared_points[i]) == 1
		shared_points[i] = shared_points[i][0]

	result = copy(c1)
	result[shared_points[0]]
	

	return result