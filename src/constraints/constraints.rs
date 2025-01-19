use std::{collections::HashSet, fmt::Display};

use const_format::formatc;
use itertools::Itertools;
use regex::Regex;

use crate::{
    constraints::{ANGLE_EXPR, POINT, TWO_POINTS},
    math::{
        geo::{line_from_points, Geo, OneD, TwoD},
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
        let s = s.replace("||", "∥");
        lazy_static::lazy_static! {
            static ref RE: Regex =
                Regex::new(formatc!(r"^{TWO_POINTS}(∥{TWO_POINTS})+$")).unwrap();
            static ref LINE: Regex = Regex::new(TWO_POINTS).unwrap();
        }
        if !RE.is_match(&s) {
            return Err(());
        }
        let points = s
            .split("∥")
            .map(|l| {
                LINE.captures(l)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|m| index.get_or_insert(m.unwrap().as_str()))
                    .collect()
            })
            .concat();
        Ok(Self { points })
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
        vec![Geo::One(OneD::Linear {
            o: pos[self.points[l_ind * 2 + other]],
            v,
            l: Number::NEG_INFINITY,
        })]
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
        let s = s.replace("_|_", "⟂");
        lazy_static::lazy_static! {
            static ref RE: Regex =
                Regex::new(formatc!(r"^{TWO_POINTS}(⟂{TWO_POINTS})+$")).unwrap();
            static ref LINE: Regex = Regex::new(TWO_POINTS).unwrap();
        }
        if !RE.is_match(&s) {
            return Err(());
        }
        let points = s
            .split("⟂")
            .map(|l| {
                LINE.captures(l)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|m| index.get_or_insert(m.unwrap().as_str()))
                    .collect()
            })
            .concat();
        Ok(Self { points })
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
        vec![Geo::One(OneD::Linear {
            o: pos[self.points[l_ind * 2 + other]],
            v: if (l_ind.abs_diff(ind)) % 2 == 0 {
                v
            } else {
                v.perp()
            },
            l: Number::NEG_INFINITY,
        })]
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
            static ref RE: Regex = Regex::new(formatc!(r"^{POINT}(-{POINT})+$")).unwrap();
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

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if self
            .points
            .iter()
            .filter(|&p| known_points.contains(p))
            .count()
            >= 2
        {
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
        vec![Geo::One(line_from_points(
            pos[p0],
            pos[p1],
            Number::NEG_INFINITY,
        ))]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Polarity {
    Pro,
    Anti,
}
impl Display for Polarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Polarity::Pro => "±",
                Polarity::Anti => "∓",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AnglePolarity {
    pub points: Vec<PointID>,
    pub polarities: Vec<Polarity>,
}

impl Display for AnglePolarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.points
                .chunks_exact(3)
                .zip_eq(self.polarities.iter())
                .map(|(a, pol)| format!("{} {} {} {}", pol, a[0], a[1], a[2]))
                .join(", ")
        )
    }
}

impl Constraint for AnglePolarity {
    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if self
            .points
            .chunks_exact(3)
            .filter(|&a| a.iter().all(|p| known_points.contains(p)))
            .next()
            .is_some()
        {
            self.points
                .chunks_exact(3)
                .filter_map(|a| {
                    a.iter()
                        .filter(|p| !known_points.contains(p))
                        .exactly_one()
                        .ok()
                })
                .copied()
                .collect()
        } else {
            Vec::new()
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        let Some((a, pol)) = self
            .points
            .chunks_exact(3)
            .zip(self.polarities.iter())
            .filter(|&(a, _)| a.iter().all(|&p| pos.len() > p))
            .next()
        else {
            return Vec::new();
        };
        let t_a_ind = t_ind / 3;
        let t_i = t_ind % 3;
        let t_a = &self.points[t_a_ind * 3..(t_a_ind + 1) * 3];
        let t_pol = &self.polarities[t_a_ind];

        let dir = (pos[a[1]] - pos[a[0]])
            .cross(pos[a[2]] - pos[a[1]])
            .signum();
        let t_dir = if pol == t_pol { dir } else { -dir };

        let o = pos[t_a[(t_i + 1) % 3]];
        let p = pos[t_a[(t_i + 2) % 3]];

        vec![Geo::Two(TwoD::Half {
            o,
            n: (p - o).unit().perp() * t_dir,
        })]
    }

    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()> {
        let s = s.replace("+/-", "±");
        let s = s.replace("-/+", "∓");
        let s = s.replace("<", "∠");
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(formatc!(r"^\s*[±∓]{ANGLE_EXPR}(,\s*[±∓]{ANGLE_EXPR})+$")).unwrap();
            static ref ANGLE: Regex = Regex::new(ANGLE_EXPR).unwrap();
        }
        if !RE.is_match(&s) {
            return Err(());
        }
        let (points, polarities): (Vec<[usize; 3]>, _) = s
            .split(",")
            .map(|s| {
                let captures = ANGLE.captures(s).unwrap();
                (
                    [
                        index.get_or_insert(&captures[1]),
                        index.get_or_insert(&captures[2]),
                        index.get_or_insert(&captures[3]),
                    ],
                    if s.contains("±") {
                        Polarity::Pro
                    } else {
                        Polarity::Anti
                    },
                )
            })
            .unzip();
        Ok(Self {
            points: points.into_flattened(),
            polarities,
        })
    }
}
