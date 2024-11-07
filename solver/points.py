import numpy as np
from dataclasses import dataclass, field
from typing import Self

@dataclass
class Points():
	_index: dict = field(default_factory=dict)
	_vecs: np.ndarray = field(default_factory=np.ndarray((0,3)))

	def from_dict(points: dict):
		index = {point: i for i, point in enumerate(points.keys())}
		vecs = np.concat(list(points.values()))
		return Points(index, vecs)
	
	def to_dict(self) -> dict:
		return {point: self._vec[i] for point, i in self._index.items()}
	
	def __getitem__(self, key) -> np.ndarray:
		i = self._index[key]
		return self._vecs[i]
		
	def __setitem__(self, key, value):
		if key in self._index:
			i = self._index[key]
			self._vec[i] = value
		else:
			self._index[key] = self._vec.shape[0]
			self._vec = self._vec.append(value, axis=0)
			
	def __or__(self, other: Self) -> Self:
		points = self.to_dict()
		points |= other.to_dict()
		
		return Points.from_dict(points)
			
	@property
	def ids(self):
		return set(self._index.keys())
	
	def move(self, p: str, neg=False):
		return np.ndarray([
			[1, 0, self[p, 0] * -1 if neg else 1]
			[0, 1, self[p, 1] * -1 if neg else 1]
			[0, 0, 1]
		])
	
	def rotate(self, targets: Self, origin: str, p: str):
		target_vec = targets[p] - targets[origin]
		source_vec = self[p] - self[origin]
	
		target_len = np.linalg.norm(target_vec)
		source_len = np.linalg.norm(source_vec)
	
		target_vec /= target_len
		source_vec /= source_len
	
		s = np.cross(source_vec, target_vec)
		c = np.dot(source_vec, target_vec)
		
		return np.ndarray([
			[c, -s, 0],
			[s, c, 0],
			[0, 0, 1]
		]), np.identity(3) * (target_len/source_len)
			
	def map_to(self, targets: Self, do_scale: bool=False):
		shared_points = self.ids.intersection(targets.ids)
		
		assert len(shared_points) == 2
		p0, p1 = shared_points
	
		prerot_translation = self.move(p0, neg=True)
		rotation, scale = self.rotate(targets, p0, p1)
		postrot_translation = targets.move(p0)
	
		transform = prerot_translation
		if do_scale: transform @= scale
		transform @= rotation
		transform @= postrot_translation
	
		self._vecs @= transform