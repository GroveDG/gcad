use std::{collections::{HashMap, HashSet}, fmt::{Debug, Display}};

use crate::{
    constraints::elements::Point,
    math::{
        geo::Geo,
        vector::Vector,
    }
};

pub mod elements;
pub mod constraints;

pub trait Constraint: Debug + Display {
    fn points(&self) -> Vec<&Point>;
    fn targets(&self, known_points: &HashSet<&Point>) -> Vec<&Point>;
    fn to_geo(&self, pos: &HashMap<Point, Vector>, t_ind: usize) -> Vec<Geo>;
}

// Only works if refrencing the same instance exactly.
// This is fine because constraints are contained in
// the PointIndex so all equal references refer to the
// same address.
impl PartialEq for dyn Constraint + '_ {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(
            std::ptr::from_ref(self),
            std::ptr::from_ref(other)
        )
    }
}
impl Eq for dyn Constraint + '_ {}