use std::{collections::{HashMap, HashSet}, fmt::Debug};

use crate::{
    constraints::elements::Point,
    geo::Geo,
    vector::Vector,
};

pub mod elements;
pub mod constraints;

pub trait Constraint: Debug {
    fn points(&self) -> &[Point];
    fn targets(&self, known_points: &HashSet<Point>) -> &[Point];
    fn to_geo(&self, pos: &HashMap<Point, Vector>, target_ind: usize) -> Vec<Geo>;
}
