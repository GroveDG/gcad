from typing import Iterable
from util import reg_id

class Figure():
	def __init__(self):
		self._points = {}
		self._edges = {}
		self._angles = {}

	def __getitem__(self, ids):
		ids = reg_id(ids)

		match len(ids):
			case 1:
				return self._points[ids]
			case 2:
				return self._edges[ids]
			case 3:
				return self._angles[ids]
	
	def get(self, ids, default=None):
		ids = reg_id(ids)

		match len(ids):
			case 1:
				if ids in self._points:
					return self._points.get(ids, default)
			case 2:
				if ids in self._edges:
					return self._edges.get(ids, default)
			case 3:
				if ids in self._angles:
					return self._angles.get(ids, default)
	
	def __setitem__(self, ids, value):
		ids = reg_id(ids)

		match len(ids):
			case 1:
				self._points[ids] = value
			case 2:
				self._edges[ids] = value
			case 3:
				self._angles[ids] = value