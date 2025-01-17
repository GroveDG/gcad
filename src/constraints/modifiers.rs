use std::{collections::{HashMap, HashSet}, fmt::Display};

use itertools::Itertools;

use crate::math::{geo::{Dimension, Geo}, vector::Vector};

use super::{elements::Point, Constraint};


#[derive(Debug, PartialEq, Eq, Hash)]
enum Polarity {
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
struct Sign {
    pub angles: Vec<(Polarity, [Point; 3])>,
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.angles
                .iter()
                .map(|(pol, [p0, p1, p2])| format!("{} {} {} {}", pol, p0, p1, p2))
                .join(", ")
        )
    }
}

impl Constraint for Sign {
    fn dim(&self) -> Dimension {
        Dimension::Two
    }

    fn points(&self) -> Vec<&Point> {
        self.angles.iter().map(|(_, a)| a).flatten().collect()
    }

    fn targets(&self, known_points: &HashSet<&Point>) -> Vec<&Point> {
        if self
            .angles
            .iter()
            .filter(|(_, a)| a.iter().all(|p| known_points.contains(p)))
            .next()
            .is_some()
        {
            self.angles
                .iter()
                .filter_map(|(_, a)| {
                    a.iter()
                        .filter(|p| !known_points.contains(p))
                        .exactly_one()
                        .ok()
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    fn geo(&self, pos: &HashMap<Point, Vector>, t_ind: usize) -> Vec<Geo> {
        let Some((pol, [r, v, l])) = self
            .angles
            .iter()
            .filter(|(_, a)| a.iter().all(|p| pos.contains_key(p)))
            .next()
        else {
            return Vec::new();
        };
        let t_a_ind = t_ind / 3;
        let t_i = t_ind % 3;
        let (t_pol, t_a) = &self.angles[t_a_ind];

        let dir = (pos[v] - pos[r]).cross(pos[l] - pos[v]).signum();
        let t_dir = if pol == t_pol { dir } else { -dir };

        todo!()
    }
}