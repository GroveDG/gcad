use std::collections::HashMap;

use crate::constraints::{elements::Point, Constraint};

mod bfs;
pub use bfs::bfs_order;

pub struct PointIndex {
    map: HashMap<Point, Vec<usize>>,
    constraints: Vec<Box<dyn Constraint>>,
}

impl PointIndex {
    pub fn from_constraints(constraints: Vec<Box<dyn Constraint>>) -> Self {
        let mut index = PointIndex {
            map: HashMap::new(),
            constraints,
        };
        for (i, c) in index.constraints.iter().enumerate() {
            for p in c.points() {
                Self::insert(&mut index.map, p, i);
            }
        }
        index
    }

    fn insert(map: &mut HashMap<Point, Vec<usize>>, p: &Point, i: usize) {
        if map.contains_key(p) {
            map.get_mut(p).unwrap().push(i);
        } else {
            map.insert(p.clone(), vec![i]);
        }
    }

    pub fn get_constraints(&self, point: &Point) -> Option<Vec<&dyn Constraint>> {
        self.map.get(point).map(|indices| {
            indices
                .into_iter()
                .map(|&i| self.constraints[i].as_ref())
                .collect()
        })
    }

    pub fn get_points(&self) -> Vec<&Point> {
        self.map.keys().collect()
    }
}
