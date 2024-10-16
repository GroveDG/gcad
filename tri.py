from math import pi as PI, sqrt, cos, sin, acos, asin, isclose
from typing import List
from util import cyclic_pairwise

def law_of_cosines(edges: List[float], angles: List[float], index: int, for_edge=True):
    a = angles[index] if for_edge else edges[index]
    b = edges[index-1]
    c = edges[index-2]

    if for_edge:
        edges[index] = sqrt(b**2 + c**2 - 2*b*c*cos(a))
    else:
        angles[index] = acos((a**2 - b**2 - c**2) / 2*b*c)

    return edges, angles

def law_of_sines(edges: List[float], angles: List[float], index: int, for_angle=True):
    for edge, angle in zip(edges, angles):
        if edge and angle:
            sines_ratio = sin(angle) / edge if for_angle else edge / sin(angle)

    if for_angle:
        angles[index] = asin(sines_ratio * edges[index])
    else:
        edges[index] = sines_ratio * sin(angles[index])
    
    return edges, angles

def solve_tri(edges: List[float], angles: List[float]):
    assert edges.count(None) < 3, "Triangle underspecified, at least 1 edge must be specified to solve."
    assert all([angle > 0 for angle in angles if angle]), "Negative angle."
    assert all([edge > 0 for edge in edges if edge]), "Negative edge."
    assert sum([angle for angle in angles if angle]) < PI, "Oversized angle."
    if all(angles): isclose(sum(angles), PI), "Triangle invalid, sum of angles is not PI radians."

    for _ in range(angles.count(None)):
        match angles.count(None):
            case 3:
                edges, angles = law_of_cosines(edges, angles, 0, for_edge=False)
            case 2:
                index = None
                for i, (edge, angle) in enumerate(zip(edges, angles)):
                    if angle != None and edge == None: index = i
                if index != None:
                    edges, angles = law_of_cosines(edges, angles, index, for_edge=True)

                index = 0
                for i, edge in enumerate(edges):
                    if edge < edges[index]: index = i
                edges, angles = law_of_sines(edges, angles, index, for_angle=True)
            case 1:
                index = angles.index(None)
                last_angle = PI - (angles[index-1] + angles[index-2])
                angles[index] = last_angle
    
    assert angles.count(None) == 0, "Triangle underspecified, too many unsolved angles."

    for _ in range(edges.count(None)):
        index = edges.index(None)
        match edges.count(None):
            case 2:
                edges, angles = law_of_sines(edges, angles, index, for_angle=False)
            case 1:
                edges, angles = law_of_cosines(edges, angles, index, for_edge=True)

    assert edges.count(None) == 0, "Triangle underspecified, unsolved edges."

    for i in range(3):
        assert edges[i] < edges[i-1] + edges[i-2], "Triangle invalid, edges are too long/short."

    return edges, angles