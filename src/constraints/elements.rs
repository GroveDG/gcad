use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

use crate::{
    constraints::Constraint,
    math::{
        geo::Geo,
        vector::{AboutEq, Number, Vector},
    },
};

pub type Point = String;

#[derive(Debug)]
pub struct Distance {
    pub points: [Point; 2],
    pub dist: Number,
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "| {} {} | = {}",
            self.points[0], self.points[1], self.dist
        )
    }
}

impl Constraint for Distance {
    fn points(&self) -> Vec<&Point> {
        return self.points.iter().collect();
    }

    fn targets(&self, known_points: &HashSet<&Point>) -> Vec<&Point> {
        if let Ok(t) = self
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

    fn to_geo(&self, pos: &HashMap<Point, Vector>, target_ind: usize) -> Vec<Geo> {
        let i: usize = if target_ind == 1 { 0 } else { 1 };
        vec![Geo::Circle {
            c: pos[&self.points[i]],
            r: self.dist,
        }]
    }
}

#[derive(Debug)]
pub struct Angle {
    pub points: [Point; 3],
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
    fn points(&self) -> Vec<&Point> {
        return self.points.iter().collect();
    }

    fn targets(&self, known_points: &HashSet<&Point>) -> Vec<&Point> {
        if let Ok(t) = self
            .points()
            .iter()
            .filter(|&p| !known_points.contains(p))
            .exactly_one()
        {
            vec![t]
        } else {
            Vec::new()
        }
    }

    fn to_geo(&self, pos: &HashMap<Point, Vector>, target_ind: usize) -> Vec<Geo> {
        if target_ind == 1 {
            let s = pos[&self.points[0]];
            let e = pos[&self.points[2]];
            let (v, d) = (e - s).unit_mag();
            debug_assert_ne!(d, 0.0);
            let r = d / 2.0 / self.measure.sin();
            let mid = (s + e) / 2.0;
            let a = r * self.measure.cos();
            if a.about_zero() {
                vec![Geo::Circle { c: mid, r }]
            } else {
                let v_a = v.perp() * a;
                vec![
                    Geo::Circle { c: mid + v_a, r },
                    Geo::Circle { c: mid - v_a, r },
                ]
            }
        } else {
            let i = if target_ind == 2 { 0 } else { 2 };
            let o = pos[&self.points[1]];
            let b = pos[&self.points[i]];
            let b_v = (b - o).unit();
            vec![
                Geo::Linear {
                    o,
                    v: b_v.rot(self.measure),
                    l: 0.0,
                },
                Geo::Linear {
                    o,
                    v: b_v.rot(-self.measure),
                    l: 0.0,
                },
            ]
        }
    }
}
