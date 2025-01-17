use std::
    collections::{HashMap, HashSet}
;

use itertools::Itertools;

use crate::{
    constraints::{elements::Point, Constraint},
    math::{
        geo::{choose, dist, meet, Geo},
        vector::{Number, Vector},
    },
};

fn solve_iter(
    order: &Vec<(Point, Vec<&dyn Constraint>)>,
    geo_order: &Vec<(&Point, Vec<(&dyn Constraint, &Point)>)>,
    positions: &mut HashMap<Point, Vector>,
    geo: &mut HashMap<&Point, Vec<Vec<Geo>>>,
    i: usize,
) -> Result<(), ()> {
    let Some((point, cs)) = geo_order.get(i) else {
        return Ok(());
    };
    let future_geos = order
        .split_at(i)
        .1
        .iter()
        .map(|(p, _)| {
            geo.get(p)
                .unwrap()
                .iter()
                .cloned()
                .reduce(|g0, g1| meet(g0, g1))
                .unwrap()
        })
        .collect::<Vec<Vec<Geo>>>();
    let mut gs = geo
        .get(point)
        .unwrap()
        .iter()
        .cloned()
        .reduce(meet)
        .unwrap();
    if gs.is_empty() {
        return Err(());
    }
    gs = gs
        .iter()
        .map(|g| {
            (
                g,
                future_geos
                    .iter()
                    .map(|g_f| {
                        g_f.iter()
                            .map(|f| dist(choose(*g), *f))
                            .reduce(Number::min)
                            .unwrap()
                    })
                    .rev()
                    .reduce(|d, d_i| {
                        // if a prev point's dist is 0, then it takes priority
                        d_i * (1.0 + d)
                    })
                    .unwrap(),
            )
        })
        .sorted_by(|(_, w0), (_, w1)| w0.total_cmp(w1))
        .map(|(g, _)| *g)
        .collect::<Vec<Geo>>();
    for g in gs {
        positions.insert((*point).clone(), choose(g));
        for (c, t) in cs.iter() {
            let i = c.points().iter().position(|v| v == point).unwrap();
            let c_g = c.geo(&positions, i);
            geo.get_mut(*t).unwrap().push(c_g);
        }
        if solve_iter(order, geo_order, positions, geo, i + 1).is_ok() {
            return Ok(());
        }
        for (_, t) in cs {
            geo.get_mut(*t).unwrap().pop();
        }
    }
    return Err(());
}

pub fn dist_solve(
    order: Vec<(Point, Vec<&dyn Constraint>)>,
) -> Result<HashMap<Point, Vector>, String> {
    let geo_order = compute_geo_order(&order);
    let mut positions = HashMap::new();
    let mut geo = HashMap::new();
    println!("{:?}", geo_order);
    for (p, _) in order.iter() {
        geo.insert(p, vec![vec![Geo::All]]);
    }
    if solve_iter(&order, &geo_order, &mut positions, &mut geo, 0).is_ok() {
        Ok(positions)
    } else {
        Err("solve failed".to_string())
    }
}

pub fn compute_geo_order<'a>(
    order: &'a Vec<(Point, Vec<&'a dyn Constraint>)>,
) -> Vec<(&'a Point, Vec<(&'a dyn Constraint, &'a Point)>)> {
    let mut geo_order = vec![];
    let mut known = HashSet::from_iter(order.iter().map(|(p, _)| p));
    let mut queue = Vec::new();
    for (p, cs) in order.iter().rev() {
        known.remove(p);
        for &c in cs {
            queue.push((c, p));
        }
        let supported;
        (queue, supported) = queue
            .iter()
            .partition(|(c, t)| c.targets(&known).contains(&t));
        geo_order.push((p, supported));
    }
    geo_order.reverse();
    geo_order
}
