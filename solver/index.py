from .fig import Figure
from typing import Self

class Index():
    def __init__(self) -> None:
        self.p2e = dict()
        self.p2a = dict()
        self.edges = set()
        self.angles = set()

    @property
    def points(self) -> list:
        return list(
            self.p2e.keys() |
            self.p2a.keys()
        )
    
    def _get_all(self, *points):
        return [
            self.p2a[point] | self.p2e[point]
            for point in points
        ]
    
    def all_of(self, *points) -> set:
        values = self._get_all(*points)
        result = values.pop()
        for v in values:
            result &= v
        return result
    
    def any_of(self, *points) -> set:
        values = self._get_all(*points)
        result = values.pop()
        for v in values:
            result |= v
        return result
    
    def one_of(self, *points) -> set:
        values = self._get_all(*points)
        result = values.pop()
        for v in values:
            result ^= v
        return result
    
    def add(self, id: tuple) -> None:
        if len(id) == 2:
            index = self.p2e
            self.edges.add(id)
        else:
            index = self.p2a
            self.angles.add(id)
        for point in id:
            if point in index:
                index[point].add(id)
            else:
                index[point] = {id}
    
    def from_fig(fig: Figure) -> Self:
        index = Index()

        for edge in fig._edges.keys():
            index.add(edge)
        for angle in fig._angles.keys():
            index.add(angle)
        
        return index