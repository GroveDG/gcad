use std::{
    collections::HashSet,
    fmt::Display,
};

use itertools::Itertools;
use regex::Regex;

use crate::{
    math::{
        geo::{line_from_points, Dimension, Geo},
        vector::{Number, Vector},
    },
    order::{PointID, PointIndex},
};

use super::Constraint;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Parallel {
    pub points: Vec<PointID>,
}

impl Display for Parallel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.points
                .chunks_exact(2)
                .into_iter()
                .map(|l| format!("{} {}", l[0], l[1]))
                .join(" ∥ ")
        )
    }
}

impl Constraint for Parallel {
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()> {
        lazy_static::lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(\w+\s*\w+)(?:\s*(∥|(\|\|))\s*(\w+\s*\w+))+$").unwrap();
            static ref SPLIT: Regex = Regex::new(r"∥|(\|\|)").unwrap();
            static ref LINE: Regex = Regex::new(r"\s*(\w+)\s*(\w+)\s*").unwrap();
        }
        if !RE.is_match(s) {
            return Err(());
        }
        let lines = SPLIT
            .split(s)
            .map(|l| {
                LINE.captures(l)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|m| index.get_or_insert(m.unwrap().as_str()))
                    .collect()
            })
            .concat();
        Ok(Self { points: lines })
    }

    fn dim(&self) -> Dimension {
        Dimension::One
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if let Some(_) = self
            .points
            .chunks_exact(2)
            .into_iter()
            .filter(|l| known_points.contains(&l[0]) && known_points.contains(&l[1]))
            .next()
        {
            self.points
                .chunks_exact(2)
                .filter_map(|l| {
                    if !known_points.contains(&l[0]) && known_points.contains(&l[1]) {
                        Some(l[0])
                    } else if !known_points.contains(&l[1]) && known_points.contains(&l[0]) {
                        Some(l[1])
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        let l = self
            .points
            .chunks_exact(2)
            .filter(|&l| pos.len() > l[0] && pos.len() > l[1])
            .next()
            .unwrap();
        let v = (pos[l[1]] - pos[l[0]]).unit();
        let l_ind = t_ind / 2;
        let other = (t_ind + 1) % 2;
        vec![Geo::Linear {
            o: pos[self.points[l_ind + other]],
            v,
            l: Number::NEG_INFINITY,
        }]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Perpendicular {
    pub points: Vec<PointID>,
}

impl Display for Perpendicular {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.points
                .chunks_exact(2)
                .map(|l| format!("{} {}", l[0], l[1]))
                .join(" ⟂ ")
        )
    }
}

impl Constraint for Perpendicular {
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()> {
        lazy_static::lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(\w+\s*\w+)(?:\s*(⟂|(_\|_))\s*(\w+\s*\w+))+$").unwrap();
            static ref SPLIT: Regex = Regex::new(r"⟂|(_\|_)").unwrap();
            static ref LINE: Regex = Regex::new(r"\s*(\w+)\s*(\w+)\s*").unwrap();
        }
        if !RE.is_match(s) {
            return Err(());
        }
        let lines = SPLIT
            .split(s)
            .map(|l| {
                LINE.captures(l)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|m| index.get_or_insert(m.unwrap().as_str()))
                    .collect()
            })
            .concat();
        Ok(Self { points: lines })
    }

    fn dim(&self) -> Dimension {
        Dimension::One
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if let Some(_) = self
            .points
            .chunks_exact(2)
            .filter(|&l| known_points.contains(&l[0]) && known_points.contains(&l[1]))
            .next()
        {
            self.points
                .chunks_exact(2)
                .filter_map(|l| {
                    if !known_points.contains(&l[0]) && known_points.contains(&l[1]) {
                        Some(l[0])
                    } else if !known_points.contains(&l[1]) && known_points.contains(&l[0]) {
                        Some(l[1])
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        let (ind, l) = self
            .points
            .chunks_exact(2)
            .enumerate()
            .filter(|&(_, l)| pos.len() > l[0] && pos.len() > l[1])
            .next()
            .unwrap();
        let v = (pos[l[1]] - pos[l[0]]).unit();
        let l_ind = t_ind / 2;
        let other = (t_ind + 1) % 2;
        vec![Geo::Linear {
            o: pos[self.points[l_ind + other]],
            v: if (l_ind.abs_diff(ind)) % 2 == 0 {
                v
            } else {
                v.perp()
            },
            l: Number::NEG_INFINITY,
        }]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Collinear {
    pub points: Vec<PointID>,
}

impl Display for Collinear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.points.iter().join("-"))
    }
}

impl Constraint for Collinear {
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"^\w+(?:\s*-\s*\w+)+$").unwrap();
        }
        if !RE.is_match(s) {
            return Err(());
        }
        let points = s
            .split("-")
            .map(|s| index.get_or_insert(s.trim()))
            .collect();
        Ok(Self { points })
    }

    fn dim(&self) -> Dimension {
        Dimension::One
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if self.points.iter().filter(|&p| known_points.contains(p)).count() >= 2 {
            self.points
                .iter()
                .copied()
                .filter(|p| !known_points.contains(p))
                .collect()
        } else {
            vec![]
        }
    }

    fn geo(&self, pos: &[Vector], _t_ind: usize) -> Vec<Geo> {
        let [&p0, &p1] = self
            .points
            .iter()
            .filter(|&&p| pos.len() > p)
            .next_array()
            .unwrap();
        vec![line_from_points(pos[p0], pos[p1], Number::NEG_INFINITY)]
    }
}
