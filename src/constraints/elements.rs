use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;
use nom::{
    character::complete::{char as nom_char, one_of, space1},
    combinator::all_consuming,
    sequence::{delimited, preceded},
};

use crate::{
    constraints::Constraint,
    math::{
        geo::{Geo, OneD},
        vector::{AboutEq, Number, Vector},
    },
    order::{PointID, PointIndex},
    parse::{ident, list_len, ws},
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
        let mut parser = all_consuming(ws(delimited(
            nom_char::<&str, ()>('|'),
            ws(list_len(space1, ident, 2)),
            nom_char::<&str, ()>('|'),
        )));
        let Ok((_, p)) = parser(s) else {
            return Err(());
        };
        Ok(Self {
            points: [index.get_or_insert(p[0]), index.get_or_insert(p[1])],
            dist: 0.0, // Placeholder value
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
        let mut parser = all_consuming(ws(preceded(
            one_of::<&str, &[char], ()>(&['<', '∠']),
            ws(list_len(space1, ident, 3)),
        )));
        let Ok((_, p)) = parser(s) else {
            return Err(());
        };
        Ok(Self {
            points: [
                index.get_or_insert(p[0]),
                index.get_or_insert(p[1]),
                index.get_or_insert(p[2]),
            ],
            measure: 0.0, // Placeholder value
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
