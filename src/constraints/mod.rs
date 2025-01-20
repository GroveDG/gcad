use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

use const_format::formatc;
use flags::ConFlags;

use crate::{
    math::{geo::Geo, vector::Vector},
    order::{PointID, PointIndex},
};

pub mod constraints;
pub mod elements;
pub mod flags;

const POINT: &str = r"\s*\w+\s*";
const TWO_POINTS: &str = r"\s*(\w+)\s+(\w+)\s*";
const THREE_POINTS: &str = r"\s*(\w+)\s+(\w+)\s+(\w+)\s*";
const ANGLE_EXPR: &str = formatc!(r"\s*∠\s*{THREE_POINTS}\s*");

pub trait Constraint: Debug + Display {
    /// Determine if `s` is parseable as `Self`.
    ///
    /// If it is, `get_or_insert` all points to get their indicies
    /// and return `Self`.
    /// 
    /// Otherwise, return `Err(())` *before* inserting points.
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()>
    where
        Self: Sized;

    /// Return a slice iterating over all points refrenced by this
    /// constraint (including duplicates).
    ///
    /// This allows the `PointIndex` to map points to constraints
    /// without iterating over every constraint for every point.
    fn points(&self) -> &[PointID];

    /// Return a mut slice iterating over all points refrenced by
    /// this constraint (including duplicates).
    ///
    /// This allows the `PointID`s to be replaced with ordered IDs
    /// before solving. (See `geo`)
    fn points_mut(&mut self) -> &mut [PointID];

    /// Returns the valid targets of the constraint given a set of known
    /// points.
    ///
    /// Already known points are invalid targets.
    ///
    /// An empty `Vec` should be returned if the constraint cannot be
    /// applied.
    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID>;

    /// Returns the geometry representing the constraint's possibility
    /// space.
    ///
    /// Point IDs have been replaced with their index in the ordering.
    /// A point is known if its ID is an index in the `pos` slice.
    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo>;

    /// Characterize this constraint with flags from `ConFlags`.
    fn flags(&self) -> ConFlags {
        ConFlags::default()
    }
}
