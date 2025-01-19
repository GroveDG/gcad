use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

use const_format::formatc;

use crate::{
    math::{
        geo::Geo,
        vector::Vector,
    },
    order::{PointID, PointIndex},
};

pub mod constraints;
pub mod elements;
// pub mod modifiers;

const POINT: &str = r"\s*\w+\s*";
const TWO_POINTS: &str = r"\s*(\w+)\s+(\w+)\s*";
const THREE_POINTS: &str = r"\s*(\w+)\s+(\w+)\s+(\w+)\s*";
const ANGLE_EXPR: &str = formatc!(r"\s*∠\s*{THREE_POINTS}\s*");

pub trait Constraint: Debug + Display {
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()>
    where
        Self: Sized;
    fn points(&self) -> &[PointID];
    fn points_mut(&mut self) -> &mut [PointID];
    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID>;
    // REMEMBER: map old point IDs to ordered point IDs
    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo>;
}

// Only works if refrencing the same instance exactly.
// This is fine because constraints are contained in
// the PointIndex so all equal references refer to the
// same address.
impl PartialEq for dyn Constraint + '_ {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(std::ptr::from_ref(self), std::ptr::from_ref(other))
    }
}
impl Eq for dyn Constraint + '_ {}
