use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

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
