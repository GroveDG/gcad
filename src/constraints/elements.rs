use std::{collections::HashSet, fmt::Display};

use const_format::formatc;
use itertools::Itertools;
use regex::Regex;

use crate::{
    constraints::{Constraint, ANGLE_EXPR, TWO_POINTS},
    math::{
        geo::{Geo, OneD},
        vector::{AboutEq, Number, Vector},
    },
    order::{PointID, PointIndex},
};

pub type Point = String;

#[derive(Debug)]
pub struct Distance {
    pub points: [PointID; 2],
    pub dist: Number,
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{} {}| = {}", self.points[0], self.points[1], self.dist)
    }
}

impl Constraint for Distance {
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(formatc!(r"^\s*\|{TWO_POINTS}\|\s*$")).unwrap();
        }
        let captures = RE.captures(s).ok_or(())?;
        Ok(Self {
            points: [
                index.get_or_insert(&captures[1]),
                index.get_or_insert(&captures[2]),
            ],
            dist: 0.0,
        })
    }

    fn points(&self) -> &[PointID] {
        self.points.as_slice()
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        self.points.as_mut_slice()
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if let Ok(&t) = self
            .points
            .iter()
            .filter(|&p| !known_points.contains(p))
            .exactly_one()
        {
            vec![t]
        } else {
            Vec::new()
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        let i: usize = if t_ind == 1 { 0 } else { 1 };
        vec![Geo::One(OneD::Circle {
            c: pos[self.points[i]],
            r: self.dist,
        })]
    }
}

#[derive(Debug)]
pub struct Angle {
    pub points: [PointID; 3],
    pub measure: Number,
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "∠ {} {} {} = {}",
            self.points[0], self.points[1], self.points[2], self.measure
        )
    }
}

impl Constraint for Angle {
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(format!(r"^\s*{ANGLE_EXPR}\s*$").as_str()).unwrap();
        }
        let s = s.replace("<", "∠");
        let captures = RE.captures(&s).ok_or(())?;
        Ok(Self {
            points: [
                index.get_or_insert(&captures[1]),
                index.get_or_insert(&captures[2]),
                index.get_or_insert(&captures[3]),
            ],
            measure: 0.0,
        })
    }

    fn points(&self) -> &[PointID] {
        self.points.as_slice()
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        self.points.as_mut_slice()
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if let Ok(&t) = self
            .points
            .iter()
            .filter(|&p| !known_points.contains(p))
            .exactly_one()
        {
            vec![t]
        } else {
            Vec::new()
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        if t_ind == 1 {
            let s = pos[self.points[0]];
            let e = pos[self.points[2]];
            let (v, d) = (e - s).unit_mag();
            debug_assert_ne!(d, 0.0);
            let r = d / 2.0 / self.measure.sin();
            let mid = (s + e) / 2.0;
            let a = r * self.measure.cos();
            if a.about_zero() {
                vec![Geo::One(OneD::Circle { c: mid, r })]
            } else {
                let v_a = v.perp() * a;
                vec![
                    Geo::One(OneD::Circle { c: mid + v_a, r }),
                    Geo::One(OneD::Circle { c: mid - v_a, r }),
                ]
            }
        } else {
            let i = if t_ind == 2 { 0 } else { 2 };
            let o = pos[self.points[1]];
            let b = pos[self.points[i]];
            let b_v = (b - o).unit();
            vec![
                Geo::One(OneD::Linear {
                    o,
                    v: b_v.rot(self.measure),
                    l: 0.0,
                }),
                Geo::One(OneD::Linear {
                    o,
                    v: b_v.rot(-self.measure),
                    l: 0.0,
                }),
            ]
        }
    }
}
