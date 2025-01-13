use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use multimap::MultiMap;

use crate::constraints::{elements::Point, Constraint};

use super::PointIndex;

fn expand_tree<'a>(
    index: &'a PointIndex,
    points: &HashSet<&Point>,
    point: &Point,
    support: &mut MultiMap<&'a Point, &'a dyn Constraint>,
) -> Vec<&'a Point> {
    // For all constraints containing this point...
    index
        .get_constraints(point)
        .unwrap()
        .into_iter()
        .map(|c| {
            // for all valid targets of this constraint...
            c.targets(&points)
                .into_iter()
                .filter_map(|t| {
                    // See if constraint is already applied.
                    if support
                        .get_vec(t)
                        .map(|vs| vs.iter().any(|&v| v == c))
                        .unwrap_or(false)
                    {
                        return None;
                    }
                    // Add constraint to target.
                    support.insert(t, c);
                    // If target is now discrete...
                    if support.get_vec(t).map(|v| v.len() != 2).unwrap_or(true) {
                        return None;
                    }
                    // return as known.
                    Some(t)
                })
                .collect::<Vec<&Point>>()
        })
        .flatten()
        .unique()
        .collect()
}

fn compute_tree<'a>(
    root: &'a Point,
    orbiter: &'a Point,
    index: &'a PointIndex,
) -> (
    Vec<(&'a Point, Vec<&'a dyn Constraint>)>,
    HashSet<&'a Point>,
) {
    let mut support = MultiMap::new();
    let mut points: HashSet<&Point> = HashSet::from_iter([root]);
    expand_tree(index, &points, root, &mut support);
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
        for p in expand_tree(index, &points, point, &mut support) {
            // Add into order.
            order.push(p);
        }
        i += 1;
    }
    (
        order
            .into_iter()
            .map(|p| (p, support.remove(p).unwrap_or_default()))
            .collect(),
        points,
    )
}

fn root_pairs<'a>(index: &'a PointIndex) -> impl Iterator<Item = (&'a Point, &'a Point)> {
    let mut neighbors = HashMap::new();
    for p in index.get_points() {
        let known_points = HashSet::from_iter([p]);
        let targets = index
            .get_constraints(p)
            .unwrap()
            .into_iter()
            // Get valid targets given only this root.
            .map(|c| c.targets(&known_points))
            // Only unique targets.
            .flatten()
            .unique()
            // Exclude pairs already found reversed.
            .filter(|t| {
                neighbors
                    .get(t)
                    .is_none_or(|n: &Vec<&String>| !n.contains(&p))
            })
            .collect::<Vec<&Point>>();
        neighbors.insert(p, targets);
    }
    neighbors
        .into_iter()
        .map(|(p, targets)| [p].into_iter().cycle().zip(targets.into_iter()))
        .flatten()
}

fn compute_forest(index: &PointIndex) -> Vec<Vec<(&Point, Vec<&dyn Constraint>)>> {
    let mut forest: Vec<(
        Vec<(&Point, Vec<&dyn Constraint>)>, // order
        HashSet<&Point>,                     // contained
    )> = Vec::new();
    for (root, orbiter) in root_pairs(index) {
        // If this root pair is contained in any tree, skip it.
        if forest
            .iter()
            .any(|(_, p)| p.contains(root) && p.contains(orbiter))
        {
            continue;
        }
        // Compute this pair's tree.
        let (order, points) = compute_tree(root, orbiter, &index);
        // Iterate through trees...
        forest = forest
            .into_iter()
            // and discard any contained by this new tree.
            .filter(|(_, p)| !points.is_superset(p))
            .collect();
        // Add this new tree.
        forest.push((order, points));
    }

    forest.into_iter().map(|(t, _)| t).collect()
}

pub fn bfs_order(index: &PointIndex) -> Vec<(&Point, Vec<&dyn Constraint>)> {
    compute_forest(&index).concat()
}
