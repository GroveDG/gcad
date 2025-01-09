use std::fmt::Display;

use multimap::MultiMap;
use slab_tree::{NodeRef, Tree};

use crate::constraints::{elements::Point, Constraint};

mod bfs_order;
pub use bfs_order::bfs_order;

pub(crate) struct PointIndex {
    map: MultiMap<Point, usize>,
    constraints: Vec<Box<dyn Constraint>>
}

impl PointIndex {
    pub(crate) fn from_constraints(constraints: Vec<Box<dyn Constraint>>) -> Self{
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

    pub(crate) fn get_constraints(&self, point: &Point) -> Vec<&Box<dyn Constraint>> {
        self.map
            .get_vec(point)
            .unwrap()
            .into_iter()
            .map(|&i| &self.constraints[i])
            .collect()
    }

    pub(crate) fn get_points(&self) -> Vec<&Point> {
        self.map.keys().collect()
    }
}

pub(crate) fn print_tree<T: Display>(tree: Tree<T>) {
    if let Some(root) = tree.root() {
        print_node(root, 0);
    }
}

fn print_node<T: Display>(node: NodeRef<T>, i: usize) {
    println!(
        "{}{}{}",
        " ".repeat(i.saturating_sub(1)),
        if i == 0 {" "} else {"⮡ "},
        node.data()
    );
    for c in node.children() {
        print_node(c, i+1);
    }
}