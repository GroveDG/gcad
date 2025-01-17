use std::collections::HashMap;

use crate::{
    constraints::elements::Point,
    math::{
        geo::{choose, meet, Geo},
        vector::Vector,
    },
    order::{PointIndex, CID},
    util::locate,
};

fn solve_iter(
    order: &Vec<Vec<CID>>,
    index: &mut PointIndex,
    positions: &mut Vec<Vector>,
    i: usize,
) -> Result<(), ()> {
    if order.len() <= i {
        return Ok(());
    };
    let geo = order[i]
        .iter()
        .map(|&cid| {
            let c = index.get_constraint(cid);
            let t_ind = locate(c.points(), &i).unwrap();
            c.geo(&positions[..i], t_ind)
        })
        .reduce(meet)
        .unwrap_or_else(|| vec![Geo::All]);
    for g in geo {
        positions[i] = choose(g);
        if solve_iter(order, index, positions, i + 1).is_ok() {
            return Ok(());
        }
    }
    return Err(());
}

pub fn brute_solve(
    index: &mut PointIndex,
    order: Vec<Vec<CID>>,
) -> Result<HashMap<Point, Vector>, String> {
    let mut positions = vec![Vector::ZERO; order.len()];
    if solve_iter(&order, index, &mut positions, 0).is_ok() {
        Ok(HashMap::from_iter(
            positions
                .into_iter()
                .enumerate()
                .map(|(id, v)| (index.get_point(&id).clone(), v)),
        ))
    } else {
        Err("solve failed".to_string())
    }
}
