import numpy as np
from dataclasses import dataclass, field
from typing import Self

@dataclass
class Points():
	_index: dict = field(default_factory=dict)
	_vecs: np.ndarray = field(default_factory=np.ndarray((0,3)))
	
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
			
	def __or__(self, other) -> Self:
			points = {}
			
			points = {point: self._vec[i] for point, i in self.items()}
			points |= {point: self._vec[i] for point, i in other.items()}
			
			index = {point: i for i, point in enumerate(points.keys())}
			vecs = np.concat(list(points.values()))
			
			return Points(index, vecs)
			
	@property
	def ids(self):
			return set(self._index.keys())
			
	def map_to(self, targets: Self, do_scale: bool=False):
		shared_points = self.ids.intersection(targets.ids)
		
		assert len(shares_points) == 2
		p0, p1 = shared_points
	
		target_vec = targets[p1] - targets[p0]
		source_vec = self[p1] - self[p0]
	
		target_len = np.linalg.norm(target_vec)
		source_len = np.linalg.norm(source_vec)
	
		target_vec =/ target_len
		source_vec =/ source_len
	
		s = np.cross(source_vec, target_vec)
		c = np.dot(source_vec, target_vec)
	
		prerot_translation = np.ndarray([
			[1, 0, -source[p0, 0]]
			[0, 1, -source[p0, 1]]
			[0, 0, 1]
		])
		if do_scale:
			scale = np.identity(3) * (target_len/source_len)
		rotation = np.ndarray([
			[c, -s, 0],
			[s, c, 0],
			[0, 0, 1]
		])
		postrot_translation = np.ndarray([
			[1, 0, self[p0, 0]]
			[0, 1, self[p0, 1]]
			[0, 0, 1]
		])
	
		transform = prerot_translation
		if do_scale:
			transform @= scale
		transform @= rotation
		transform @= postrot_translation
	
		self._vecs @= transform