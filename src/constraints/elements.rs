use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

use crate::{
    constraints::Constraint,
    math::{
        geo::{Geo, OneD},
        AboutEq as _, Number, Vector,
    },
    order::{PointID, PointIndex},
    parsing::{literal, space, word},
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
    fn parse(mut input: &str, index: &mut PointIndex) -> Option<Self> {
        literal("|")(&mut input)?;
        space(&mut input);
        let a = word(&mut input)?;
        space(&mut input)?;
        let b = word(&mut input)?;
        space(&mut input);
        literal("|")(&mut input)?;

        let points = [index.get_or_insert(a), index.get_or_insert(b)];
        Some(Self {
            points,
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
    fn parse(mut input: &str, index: &mut PointIndex) -> Option<Self> {
        literal("∠")(&mut input).or(literal("<")(&mut input))?;
        space(&mut input);
        let a = word(&mut input)?;
        space(&mut input)?;
        let b = word(&mut input)?;
        space(&mut input)?;
        let c = word(&mut input)?;

        let points = [
            index.get_or_insert(a),
            index.get_or_insert(b),
            index.get_or_insert(c),
        ];
        Some(Self {
            points,
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
