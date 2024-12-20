from typing import Dict, List

class Index():
    def __init__(self) -> None:
        self._constraints: list = []
        self._mapping: Dict[str, List[int]] = {}

    def add_constraint(self, constraint) -> None:
        c_ind = len(self._constraints)
        self._constraints.append(constraint)
        for point in constraint.points:
            self._map_point_to_constraint(point, c_ind)
    
    def _map_point_to_constraint(self, point, c_ind) -> None:
        if point in self._mapping:
            self._mapping[point].append(c_ind)
        else:
            self._mapping[point] = [c_ind]
    
    def get_constraints(self, point: str, t: type = object) -> list:
        constraints = [self._constraints[c_ind] for c_ind in self._mapping[point]]
        constraints = [c for c in constraints if isinstance(c, t)]
        return constraints

    @property
    def points(self) -> List[str]:
        return list(self._mapping.keys())