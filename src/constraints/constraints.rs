use std::{
    collections::{HashMap, HashSet},
    f64::NEG_INFINITY,
    fmt::Display,
    str::FromStr,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::math::{
    geo::{line_from_points, Geo},
    vector::Vector,
};

use super::{elements::Point, Constraint};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Collinear {
    pub points: Vec<String>,
}

impl FromStr for Collinear {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\w+(?:-\w+)+$").unwrap();
        }
        if !RE.is_match(s) {
            return Err(());
        }
        let points = s.split("-").map(|s| {
            s.trim().to_string()
        }).collect();
        Ok(Self { points })
    }
}

impl Display for Collinear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.points.join(" - "))
    }
}

impl Constraint for Collinear {
    fn points(&self) -> &[Point] {
        self.points.as_slice()
    }

    fn targets(&self, known_points: &HashSet<&Point>) -> Vec<&String> {
        if self
            .points()
            .iter()
            .filter(|&p| !known_points.contains(p))
            .count()
            >= 2
        {
            self.points
                .iter()
                .filter(|&p| known_points.contains(p))
                .collect()
        } else {
            vec![]
        }
    }

    fn to_geo(&self, pos: &HashMap<Point, Vector>, _target_ind: usize) -> Vec<Geo> {
        if let Some([p0, p1]) = self
            .points
            .iter()
            .filter(|&p| !pos.contains_key(p))
            .next_array()
        {
            vec![line_from_points(pos[p0], pos[p1], NEG_INFINITY)]
        } else {
            vec![]
        }
    }
}
