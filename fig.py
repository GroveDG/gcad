from typing import Iterable
from itertools import pairwise

def regularize_id(id) -> tuple[str]:
	if isinstance(id, str):
		id = id.split(" ")
	elif not isinstance(id, list) and isinstance(id, Iterable):
		id = list(id)
	if hasattr(id[0], "id"):
		id = [point.id for point in id]
	if len(id) == 3:
		vertex = id.pop(1)
		id.sort()
		id.insert(1, vertex)
	else:
		id.sort()
	return tuple(id) if len(id) > 1 else id[0]

class Figure():
	def __init__(self):
		self._points = {}
		self._edges = {}
		self._angles = {}
	
	def __str__(self) -> str:
		return "\n".join(
			[
				str(self._points),
				str(self._edges),
				str(self._angles)
			]
		)
	
	def _add_point(self, id: str):
		if not id in self._points:
			self._points[id] = None
	
	def _add_dist(self, id: str):
		for point in id:
			self._add_point(point)

		if not id in self._edges:
			self._edges[id] = None

	def _add_angle(self, id):
		if not id in self._angles:
			self._angles[id] = None

		for edge in pairwise(id):
			self._add_dist(regularize_id(edge))

	def __getitem__(self, ids):
		ids = regularize_id(ids)

		match len(ids):
			case 1:
				return self._points[ids]
			case 2:
				return self._edges[ids]
			case 3:
				return self._angles[ids]
	
	def get(self, ids, default=None):
		ids = regularize_id(ids)

		match len(ids):
			case 1:
				if ids in self._points:
					return self._points[ids]
				else:
					return default
			case 2:
				if ids in self._edges:
					return self._edges[ids]
				else:
					return default
			case 3:
				if ids in self._angles:
					return self._angles[ids]
				else:
					return default
	
	def __setitem__(self, ids, value):
		ids = regularize_id(ids)

		match len(ids):
			case 1:
				self._add_point(ids)
				self._points[ids] = value
			case 2:
				self._add_dist(ids)
				self._edges[ids] = value
			case 3:
				self._add_angle(ids)
				self._angles[ids] = value