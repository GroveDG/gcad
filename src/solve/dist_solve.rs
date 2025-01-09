use std::collections::HashMap;

use crate::{constraints::{elements::Point, Constraint}, math::{geo::{choose, meet, Geo}, vector::Vector}};

fn solve_iter(
    order: &Vec<(Point, Vec<&dyn Constraint>)>,
    positions: &mut HashMap<Point, Vector>,
    i: usize
) -> Result<(), ()> {
    let Some((point, cs)) = order.get(i) else { return Ok(()) };
    let mut geo = vec![Geo::All];
    for &c in cs {
        let i = c.points().iter().position(|v| v==point).unwrap();
        let c_geo = c.to_geo(&positions, i);
        geo = meet(geo.as_slice(), c_geo.as_slice());
    }
    for g in geo {
        positions.insert(point.clone(), choose(g));
        if solve_iter(order, positions, i+1).is_ok() {
            return Ok(());
        }
    }
    return Err(());
}

pub fn dist_solve(order: Vec<(Point, Vec<&dyn Constraint>)>) -> Result<HashMap<Point, Vector>, String>{
    let mut positions = HashMap::new();
    if solve_iter(&order, &mut positions, 0).is_ok() {
        Ok(positions)
    } else {
        Err("solve failed".to_string())
    }
}