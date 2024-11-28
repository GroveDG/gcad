from .fig import Figure
from typing import Self

class Index():
    def __init__(self) -> None:
        self.edges = {}
        self.angles = {}

    @property
    def points(self) -> list:
        return list(
            self.edges.keys() |
            self.angles.keys()
        )

    def get_all(self, point: str) -> list:
        edges = self.edges[point]
        angles = self.angles[point]
        return [*edges, *angles]
    
    def add(self, id: tuple) -> None:
        index = self.edges if len(id) == 2 else self.angles
        for point in id:
            if point in index:
                index[point].append(id)
            else:
                index[point] = [id]
    
    def from_fig(fig: Figure) -> Self:
        index = Index()

        for edge in fig._edges.keys():
            index.add(edge)
        for angle in fig._angles.keys():
            index.add(angle)
        
        return index