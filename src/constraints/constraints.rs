use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

use crate::math::{
    geo::{line_from_points, Geo},
    vector::{Number, Vector},
};

use super::{elements::Point, Constraint};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Parallel {
    pub lines: Vec<[Point; 2]>,
}

impl Display for Parallel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.lines
                .iter()
                .map(|[p0, p1]| format!("{} {}", p0, p1))
                .join(" ∥ ")
        )
    }
}

impl Constraint for Parallel {
    fn points(&self) -> Vec<&Point> {
        self.lines.iter().flatten().collect()
    }

    fn targets(&self, known_points: &HashSet<&Point>) -> Vec<&String> {
        if let Some(_) = self
            .lines
            .iter()
            .filter(|&l| known_points.contains(&l[0]) && known_points.contains(&l[1]))
            .next()
        {
            self.lines
                .iter()
                .filter_map(|l| {
                    if !known_points.contains(&l[0]) && known_points.contains(&l[1]) {
                        Some(&l[0])
                    } else if !known_points.contains(&l[1]) && known_points.contains(&l[0]) {
                        Some(&l[1])
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn to_geo(&self, pos: &HashMap<Point, Vector>, t_ind: usize) -> Vec<Geo> {
        let l = self
            .lines
            .iter()
            .filter(|&l| pos.contains_key(&l[0]) && pos.contains_key(&l[1]))
            .next()
            .unwrap();
        let v = (pos[&l[1]] - pos[&l[0]]).unit();
        let l_ind = t_ind / 2;
        let other = (t_ind + 1) % 2;
        vec![Geo::Linear {
            o: pos[&self.lines[l_ind][other]],
            v,
            l: Number::NEG_INFINITY,
        }]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Perpendicular {
    pub lines: Vec<[Point; 2]>,
}

impl Display for Perpendicular {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.lines
                .iter()
                .map(|[p0, p1]| format!("{} {}", p0, p1))
                .join(" ⟂ ")
        )
    }
}

impl Constraint for Perpendicular {
    fn points(&self) -> Vec<&Point> {
        self.lines.iter().flatten().collect()
    }

    fn targets(&self, known_points: &HashSet<&Point>) -> Vec<&String> {
        if let Some(_) = self
            .lines
            .iter()
            .filter(|&l| known_points.contains(&l[0]) && known_points.contains(&l[1]))
            .next()
        {
            self.lines
                .iter()
                .filter_map(|l| {
                    if !known_points.contains(&l[0]) && known_points.contains(&l[1]) {
                        Some(&l[0])
                    } else if !known_points.contains(&l[1]) && known_points.contains(&l[0]) {
                        Some(&l[1])
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn to_geo(&self, pos: &HashMap<Point, Vector>, t_ind: usize) -> Vec<Geo> {
        let (ind, l) = self
            .lines
            .iter()
            .enumerate()
            .filter(|&(_, l)| pos.contains_key(&l[0]) && pos.contains_key(&l[1]))
            .next()
            .unwrap();
        let v = (pos[&l[1]] - pos[&l[0]]).unit();
        let l_ind = t_ind / 2;
        let other = (t_ind + 1) % 2;
        vec![Geo::Linear {
            o: pos[&self.lines[l_ind][other]],
            v: if (l_ind.abs_diff(ind)) % 2 == 0 { v } else { v.perp() },
            l: Number::NEG_INFINITY,
        }]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Collinear {
    pub points: Vec<Point>,
}

impl Display for Collinear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.points.join("-"))
    }
}

impl Constraint for Collinear {
    fn points(&self) -> Vec<&Point> {
        self.points.iter().collect()
    }

    fn targets(&self, known_points: &HashSet<&Point>) -> Vec<&String> {
        if self
            .points()
            .iter()
            .filter(|&p| known_points.contains(p))
            .count()
            >= 2
        {
            self.points
                .iter()
                .filter(|&p| !known_points.contains(p))
                .collect()
        } else {
            vec![]
        }
    }

    fn to_geo(&self, pos: &HashMap<Point, Vector>, _t_ind: usize) -> Vec<Geo> {
        let [p0, p1] = self
            .points
            .iter()
            .filter(|&p| pos.contains_key(p))
            .next_array()
            .unwrap();
        vec![line_from_points(pos[p0], pos[p1], Number::NEG_INFINITY)]
    }
}
