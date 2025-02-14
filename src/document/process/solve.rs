use std::collections::HashMap;

use crate::{
    document::{Document, CID}, math::{
        geo::{choose, meet, Geo, TwoD},
        Vector,
    }, util::locate
};

fn iter_brute(
    order: &Vec<Vec<CID>>,
    index: &mut Document,
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
        .unwrap_or_else(|| vec![Geo::Two(TwoD::All)]);
    for g in geo {
        positions[i] = choose(g);
        if iter_brute(order, index, positions, i + 1).is_ok() {
            return Ok(());
        }
    }
    return Err(());
}

pub fn solve_brute(
    index: &mut Document,
    order: Vec<Vec<CID>>,
) -> Result<HashMap<String, Vector>, String> {
    let mut positions = vec![Vector::ZERO; order.len()];
    if iter_brute(&order, index, &mut positions, 0).is_ok() {
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
