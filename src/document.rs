pub mod exprs;
mod parse;
mod parsing;

use std::collections::HashMap;

use exprs::Constraint;
pub use exprs::{
    constraints::{AnglePolarity, Collinear, Parallel, Perpendicular},
    elements::{Angle, Distance},
    draw::PathCmd,
};
pub use parse::parse_document;
use bimap::BiHashMap;

use crate::draw::DrawOptions;

pub type PointID = usize;
pub type CID = usize;

#[derive(Debug, Default)]
pub struct Document {
    id2p: BiHashMap<PointID, String>,
    id2c: HashMap<PointID, Vec<CID>>,
    constraints: Vec<Box<dyn Constraint>>,
    pub draw: DrawOptions,
}

impl Document {
    pub fn get_or_insert(&mut self, p: &str) -> PointID {
        self.id2p.get_by_right(p).copied().unwrap_or_else(|| {
            let id = self.id2p.len();
            self.id2c.insert(id, Vec::new());
            self.id2p.insert(id, p.to_owned());
            id
        })
    }

    pub fn add_constraint(&mut self, c: Box<dyn Constraint>) {
        let cid = self.constraints.len();
        for id in c.points() {
            self.id2c.get_mut(id).unwrap().push(cid);
        }
        self.constraints.push(c);
    }

    pub fn add_path(&mut self, path: Vec<PathCmd>) {
        self.draw.paths.push(path);
    }

    pub fn paths(&self) -> &[Vec<PathCmd>] {
        &self.draw.paths
    }

    pub fn constraints(&self) -> &[Box<dyn Constraint>] {
        &self.constraints
    }

    pub fn get_constraint(&self, cid: CID) -> &dyn Constraint {
        self.constraints[cid].as_ref()
    }

    pub fn get_cids(&self, point: &PointID) -> &Vec<CID> {
        &self.id2c[point]
    }

    pub fn ids(&self) -> impl Iterator<Item = &PointID> {
        self.id2p.left_values()
    }

    pub fn get_point(&self, id: &PointID) -> &String {
        self.id2p.get_by_left(id).unwrap()
    }

    pub fn map_ids(&mut self, mapping: &HashMap<PointID, usize>) {
        for c in self.constraints.iter_mut() {
            for p in c.as_mut().points_mut() {
                *p = mapping[p];
            }
        }
        let mut id2c = std::mem::take(&mut self.id2c);
        let mut id2p = std::mem::take(&mut self.id2p);
        for (p, q) in mapping {
            let v = id2c.remove(p).unwrap();
            self.id2c.insert(*q, v);
            let (_, r) = id2p.remove_by_left(p).unwrap();
            self.id2p.insert(*q, r);
        }
    }
}