use multimap::MultiMap;

use crate::constraints::{elements::Point, Constraint};

mod bfs_order;
pub use bfs_order::bfs_order;

pub struct PointIndex {
    map: MultiMap<Point, usize>,
    constraints: Vec<Box<dyn Constraint>>
}

impl PointIndex {
    pub fn from_constraints(constraints: Vec<Box<dyn Constraint>>) -> Self{
        let mut index = PointIndex {
            map: MultiMap::new(),
            constraints
        };
        for (i, c) in index.constraints
            .iter()
            .enumerate() {
            for p in c.points() {
                index.map.insert(p.clone(), i);
            }
        }
        index
    }

    pub(crate) fn get_constraints(&self, point: &Point) -> Vec<&dyn Constraint> {
        self.map
            .get_vec(point)
            .unwrap()
            .into_iter()
            .map(|&i| self.constraints[i].as_ref())
            .collect()
    }

    pub(crate) fn get_points(&self) -> Vec<&Point> {
        self.map.keys().collect()
    }
}