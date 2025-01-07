use std::{collections::{HashMap, HashSet}, f64::NEG_INFINITY};

use itertools::Itertools;

use crate::{math::{geo::{line_from_points, Geo}, vector::Vector}};

use super::{elements::Point, Constraint};

#[derive(Debug)]
pub struct Collinear {
    pub points: Vec<String>
}

impl Constraint for Collinear {
    fn points(&self) -> &[Point] {
        self.points.as_slice()
    }

    fn targets(&self, known_points: &HashSet<Point>) -> &[String] {
        if self.points().iter().filter(|&p|{
            !known_points.contains(p)
        }).count() >= 2 { self.points.as_slice() } else { &[] }
    }

    fn to_geo(&self, pos: &HashMap<Point, Vector>, _target_ind: usize) -> Vec<Geo> {
        if let Some([p0, p1]) = self.points.iter().filter(|&p| {
            !pos.contains_key(p)
        }).next_array() {
            vec![
                line_from_points(pos[p0], pos[p1], NEG_INFINITY)
            ]
        } else {
            vec![]
        }
    }
}