use std::{collections::{HashMap, HashSet}, fmt::Debug};

use crate::{
    elements::Point,
    geo::Geo,
    vector::Vector,
};

pub trait Constraint: Debug {
    fn points(&self) -> Vec<Point>;
    fn targets(&self, known_points: &HashSet<Point>) -> Vec<Point>;
    fn to_geo(&self, pos: &HashMap<Point, Vector>, target_ind: usize) -> Vec<Geo>;
}
