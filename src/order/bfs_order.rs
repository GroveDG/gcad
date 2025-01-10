use std::collections::HashSet;

use itertools::Itertools;
use multimap::MultiMap;

use crate::constraints::{elements::Point, Constraint};

use super::PointIndex;

fn add_constraints<'a>(
    index: &'a PointIndex,
    points: &HashSet<&Point>,
    point: &Point,
    support: &mut MultiMap<&'a Point, &'a dyn Constraint>
) -> Vec<&'a Point> {
    // For all constraints containing this point...
    index.get_constraints(point).into_iter().map(|c| {
        // for all valid targets of this constraint...
        c.targets(&points).into_iter().filter_map(|t| {
            // Get current number of constraints on target.
            if support.get_vec(t)
                .map(|vs| {
                    vs.iter().any(|&v| {
                        v == c
                    })
                })
                .unwrap_or(false) {
                    return None
                }
            // Add constraint to target.
            support.insert(t, c);
            // If target is now discrete...
            if support.get_vec(t).map(|v| {v.len() != 2})
                .unwrap_or(true) { return None; }
            // return as known.
            Some(t)
        }).collect::<Vec<&Point>>()
    }).flatten().unique().collect()
}

pub fn compute_tree<'a>(
    root: &'a Point,
    orbiter: &'a Point,
    index: &'a PointIndex,
) -> (Vec<&'a Point>, HashSet<&'a Point>, MultiMap<&'a Point, &'a dyn Constraint>) {
    let mut support = MultiMap::new();
    let mut points: HashSet<&Point> = HashSet::from_iter([root]);
    add_constraints(index, &points, root, &mut support);
    points.insert(orbiter);
    let mut i = 1;
    let mut order = Vec::from_iter([root, orbiter]);
    // Supporting constraints.
    // Any constraints which can be applied to a point for a given set of
    // previous points.
    // If any point has 2 or more constraints it is assumed to be discrete.
    while i < order.len() {
        let point = order[i];
        // Mark as known.
        points.insert(point);
        let next = add_constraints(index, &points, point, &mut support);
        for p in next {
            // Add into order.
            order.push(p);
        }
        i += 1;
    }
    (order, points, support)
}

pub fn compute_forest(index: &PointIndex) -> Vec<Vec<(Point, Vec<&dyn Constraint>)>> {
    let mut forest: Vec<(Vec<&Point>, HashSet<&Point>, MultiMap<&Point, &dyn Constraint>)> = Vec::new();
    for [root, orbiter] in index
        // Iterate through points...
        .get_points()
        .into_iter()
        // as roots of potential trees.
        .map(|root| {
            // Given this point (root) as the only known point...
            let known_points = HashSet::from_iter([root]);
            index
                // iterate through the root's constraints...
                .get_constraints(root)
                .into_iter()
                // get the points which can be constrained...
                .map(move |c| c.targets(&known_points))
                // combine these sets of points...
                .flatten()
                // get only the unique points...
                .unique() 
                // and map them in pairs with the root.
                .map(move |orbiter| {
                    let mut ps = [root, orbiter];
                    // (sorted to detect when the orbiter is the root)
                    ps.sort();
                    ps
                })
        })
        // Get only unique [root, orbiter] pairs.
        .flatten()
        .unique()
    {
        // If this root pair is contained in any tree, skip it.
        if forest.iter().any(|(_, p, _)| {
            p.contains(root) && p.contains(orbiter)
        }) { continue }
        // Compute this pair's tree.
        let (order, points, support) = compute_tree(root, orbiter, &index);
        // Iterate through trees...
        forest = forest
            .into_iter()
            // and discard any contained by this new tree.
            .filter(|(_, p, _)| {
                !points.is_superset(p)
            })
            .collect();
        // Add this new tree.
        forest.push((order, points, support));
    }

    forest.into_iter().map(|(t, _, mut s)| {
        t.into_iter().map(|p| {
            (p.clone(), s.remove(p).unwrap_or_else(|| Vec::new()))
        }).collect::<Vec<(Point, Vec<&dyn Constraint>)>>()
    }).collect::<Vec<Vec<(Point, Vec<&dyn Constraint>)>>>()
}

pub fn bfs_order(index: &PointIndex) -> Vec<(Point, Vec<&dyn Constraint>)> {
    let mut o = Vec::new();
    for mut t in compute_forest(&index) {
        o.append(&mut t);
    }
    o
}
