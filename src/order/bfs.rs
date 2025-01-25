use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use itertools::Itertools;

use crate::constraints::ConFlags;

use super::{PointID, PointIndex, CID};

type HashMapSet<K, V> = HashMap<K, HashSet<V>>;

fn expand_tree<'a>(
    index: &PointIndex,
    points: &HashSet<PointID>,
    point: PointID,
    support: &mut HashMap<PointID, Vec<CID>>,
) -> Vec<PointID> {
    let mut new_points = Vec::new();
    for &cid in index.get_cids(&point) {
        let c = index.get_constraint(cid);
        for t in c.targets(&points) {
            if !support.contains_key(&t) {
                support.insert(t, Vec::new());
            }
            let s_v = support.get_mut(&t).unwrap();
            // Skip if constraint is already applied.
            if s_v.contains(&cid) {
                continue;
            }
            // Add constraint to target.
            s_v.push(cid);
            // Skip if non-discretizing.
            if !c.flags().contains(ConFlags::DISCRETIZING) {
                continue;
            }
            // Push if just discretized.
            if s_v
                .iter()
                .filter(|&cid| {
                    index
                        .get_constraint(*cid)
                        .flags()
                        .contains(ConFlags::DISCRETIZING)
                })
                .count()
                != 2
            {
                continue;
            }
            new_points.push(t)
        }
    }
    new_points
}

fn compute_tree<'a>(
    root: PointID,
    orbiter: PointID,
    index: &PointIndex,
) -> (Vec<(PointID, Vec<CID>)>, HashSet<PointID>) {
    let mut support = HashMap::new();
    let mut points: HashSet<PointID> = HashSet::from_iter([root]);

    expand_tree(index, &points, root, &mut support);
    points.insert(orbiter);

    let mut i = 1;
    let mut order = Vec::from_iter([root, orbiter]);
    while i < order.len() {
        let point = order[i];
        // Mark as known.
        points.insert(point);
        // Add new points to queue/order.
        order.append(&mut expand_tree(index, &points, point, &mut support));
        i += 1;
    }
    (
        order
            .into_iter()
            .map(|p| (p, support.remove(&p).unwrap_or_default()))
            .collect(),
        points,
    )
}

fn root_pairs<'a>(index: &'a PointIndex) -> impl Iterator<Item = (PointID, PointID)> {
    let mut neighbors: HashMapSet<PointID, PointID> = HashMap::new();
    for p in index.ids() {
        let known_points = HashSet::from_iter([*p]);
        let n = HashSet::from_iter(
            index
                .get_cids(p)
                .iter()
                .map(|&cid| index.get_constraint(cid))
                .map(|c| c.targets(&known_points))
                .flatten()
                .unique()
                .filter(|t| neighbors.get(t).is_none_or(|t_n| !t_n.contains(&p))),
        );
        neighbors.insert(*p, n);
    }
    neighbors
        .into_iter()
        .map(|(p, targets)| repeat(p).zip(targets))
        .flatten()
}

fn compute_forest(index: &mut PointIndex) -> Vec<Vec<(PointID, Vec<CID>)>> {
    let mut forest: Vec<(
        Vec<(PointID, Vec<CID>)>, // order
        HashSet<PointID>,         // contained
    )> = Vec::new();

    for (root, orbiter) in root_pairs(index) {
        // If this root pair is contained in any tree, skip it.
        if forest
            .iter()
            .any(|(_, p)| p.contains(&root) && p.contains(&orbiter))
        {
            continue;
        }
        // Compute this pair's tree.
        let (order, points) = compute_tree(root, orbiter, &index);
        // Discard subtrees.
        forest.retain(|(_, p)| !points.is_superset(p));
        // Add this new tree.
        forest.push((order, points));
    }

    let (orders, _): (_, Vec<_>) = forest.into_iter().unzip();
    orders
}

pub fn bfs_order(index: &mut PointIndex) -> Vec<Vec<CID>> {
    let forest = compute_forest(index).into_iter().flatten().collect_vec();

    let mut mapping: HashMap<PointID, usize> = HashMap::new();
    let mut order: Vec<Vec<CID>> = Vec::new();
    for (id, mut cids) in forest {
        if mapping.contains_key(&id) {
            // incorrect: moves constraints backwards.
            order[mapping[&id]].append(&mut cids);
            panic!("multiple trees")
        } else {
            mapping.insert(id, order.len());
            
            // Move non-discretizing to back.
            let mut non: Vec<_>;
            (cids, non) = cids.into_iter().partition(|cid| {
                index
                    .get_constraint(*cid)
                    .flags()
                    .contains(ConFlags::DISCRETIZING)
            });
            cids.append(&mut non);

            order.push(cids);
        }
    }

    index.map_ids(&mapping);

    order
}
