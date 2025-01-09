use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use multimap::MultiMap;
use slab_tree::{Tree, TreeBuilder};

use crate::constraints::{elements::Point, Constraint};

use super::{print_tree, PointIndex};

pub fn compute_tree<'a>(
    root: &'a Point,
    orbiter: &'a Point,
    index: &'a PointIndex,
) -> (Tree<&'a Point>, HashSet<&'a Point>) {
    let mut points: HashSet<&Point> = HashSet::from_iter([root, orbiter]);
    let mut tree = TreeBuilder::new().with_root(root).build();
    let mut queue = VecDeque::from_iter([
        (root, tree.root_id().unwrap()),
        (orbiter, tree.root_mut().unwrap().append(orbiter).node_id())
    ]);
    // Supporting constraints.
    // Any constraints which can be applied to a point for a given set of
    // previous points.
    // If any point has 2 or more constraints it is assumed to be discrete.
    let mut support = MultiMap::new();
    while !queue.is_empty() {
        let (p, node_id) = queue.pop_front().unwrap();
        for c in index.get_constraints(p) {
            for k in c.targets(&points) {
                // TODO
                // Replace with a rigorous discreteness test.
                let l = support.get_vec(k)
                    .map(|v| v.len())
                    .unwrap_or(0);
                debug_assert!(l <= 2);
                if l == 2 { continue }
                support.insert(k, c);
                if l+1 < 2 { continue }
                points.insert(k);
                queue.push_back((k, node_id));
                tree.get_mut(node_id).unwrap().append(k);
            }
        }
    }
    println!("{:?}", support);
    (tree, points)
}

pub fn compute_forest(index: &PointIndex) -> Vec<Tree<&Point>> {
    let mut forest: Vec<(Tree<&Point>, HashSet<&Point>)> = Vec::new();
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
        if forest.iter().any(|(_, p)| {
            p.contains(root) && p.contains(orbiter)
        }) { continue }
        // Compute this pair's tree.
        let (tree, points) = compute_tree(root, orbiter, &index);
        // Iterate through trees...
        forest = forest
            .into_iter()
            // and discard any contained by this new tree.
            .filter(|(_, p)| {
                !points.is_superset(p)
            })
            .collect();
        // Add this new tree.
        forest.push((tree, points));
    }
    forest.into_iter().map(|(t, _)| {
        t
    }).collect()
}

pub fn bfs_order(constraints: Vec<Box<dyn Constraint>>) {
    let index = PointIndex::from_constraints(constraints);
    let forest: Vec<Tree<&String>> = compute_forest(&index);
    for tree in forest {
        print_tree(tree);
    }
}
