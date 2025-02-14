use std::fmt::Display;

use itertools::Itertools;

use crate::{
    math::{
        geo::{line_from_points, Geo, OneD, TwoD},
        Number, Vector,
    },
    order::{PointID, PointIndex},
    parsing::{literal, space, word},
};

use super::{ConFlags, Constraint};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Parallel {
    pub points: Vec<PointID>,
}

impl Display for Parallel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.points
                .chunks_exact(2)
                .into_iter()
                .map(|l| format!("{} {}", l[0], l[1]))
                .join(" ∥ ")
        )
    }
}

impl Constraint for Parallel {
    fn parse(mut input: &str, index: &mut PointIndex) -> Option<Self> {
        let mut point_names = Vec::new();
        loop {
            point_names.push(word(&mut input)?);
            space(&mut input)?;
            point_names.push(word(&mut input)?);
            space(&mut input);
            if literal("∥")(&mut input)
                .or(literal("||")(&mut input))
                .is_none()
            {
                break;
            }
            space(&mut input);
        }
        if point_names.len() < 4 {
            return None;
        }
        let points = point_names
            .into_iter()
            .map(|p| index.get_or_insert(p))
            .collect();
        Some(Self { points })
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, contains: &dyn Fn(PointID) -> bool) -> Vec<PointID> {
        if contained_elements(&self.points, &contains, 2).count() >= 1 {
            targets(&self.points, &contains, 2)
        } else {
            vec![]
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        let binding = |p| p < pos.len();
        let (_, l) = contained_elements(&self.points, &binding, 2)
            .next()
            .unwrap();
        let v = (pos[l[1]] - pos[l[0]]).unit();
        let l_ind = t_ind / 2;
        let other = (t_ind + 1) % 2;
        vec![Geo::One(OneD::Linear {
            o: pos[self.points[l_ind * 2 + other]],
            v,
            l: Number::NEG_INFINITY,
        })]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Perpendicular {
    pub points: Vec<PointID>,
}

impl Display for Perpendicular {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.points
                .chunks_exact(2)
                .map(|l| format!("{} {}", l[0], l[1]))
                .join(" ⟂ ")
        )
    }
}

impl Constraint for Perpendicular {
    fn parse(mut input: &str, index: &mut PointIndex) -> Option<Self> {
        let mut point_names = Vec::new();
        loop {
            point_names.push(word(&mut input)?);
            space(&mut input)?;
            point_names.push(word(&mut input)?);
            space(&mut input);
            if literal("⟂")(&mut input)
                .or(literal("_|_")(&mut input))
                .is_none()
            {
                break;
            }
            space(&mut input);
        }
        if point_names.len() < 4 {
            return None;
        }
        let points = point_names
            .into_iter()
            .map(|p| index.get_or_insert(p))
            .collect();
        Some(Self { points })
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, contains: &dyn Fn(PointID) -> bool) -> Vec<PointID> {
        if contained_elements(&self.points, &contains, 2).count() >= 1 {
            targets(&self.points, &contains, 2)
        } else {
            vec![]
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        let binding = |p| p < pos.len();
        let (ind, l) = contained_elements(&self.points, &binding, 2)
            .next()
            .unwrap();
        let v = (pos[l[1]] - pos[l[0]]).unit();
        let l_ind = t_ind / 2;
        let other = (t_ind + 1) % 2;
        vec![Geo::One(OneD::Linear {
            o: pos[self.points[l_ind * 2 + other]],
            v: if (l_ind.abs_diff(ind)) % 2 == 0 {
                v
            } else {
                v.perp()
            },
            l: Number::NEG_INFINITY,
        })]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Collinear {
    pub points: Vec<PointID>,
}

impl Display for Collinear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.points.iter().join("-"))
    }
}

impl Constraint for Collinear {
    fn parse(mut input: &str, index: &mut PointIndex) -> Option<Self> {
        let mut point_names = Vec::new();
        loop {
            point_names.push(word(&mut input)?);
            space(&mut input);
            if literal("-")(&mut input).is_none() {
                break;
            }
            space(&mut input);
        }
        if point_names.len() < 3 {
            return None;
        }
        let points = point_names
            .into_iter()
            .map(|p| index.get_or_insert(p))
            .collect();
        Some(Self { points })
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, contains: &dyn Fn(PointID) -> bool) -> Vec<PointID> {
        if contained_elements(&self.points, &contains, 1).count() >= 2 {
            targets(&self.points, &contains, 1)
        } else {
            Vec::new()
        }
    }

    fn geo(&self, pos: &[Vector], _t_ind: usize) -> Vec<Geo> {
        let binding = |p| p < pos.len();
        let mut contained = contained_elements(&self.points, &binding, 1);
        let (_, p0) = contained.next().unwrap();
        let (_, p1) = contained.next().unwrap();
        vec![Geo::One(line_from_points(
            pos[p0[0]],
            pos[p1[0]],
            Number::NEG_INFINITY,
        ))]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    Pro,
    Anti,
}
impl Display for Polarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Polarity::Pro => "±",
                Polarity::Anti => "∓",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AnglePolarity {
    pub points: Vec<PointID>,
    pub polarities: Vec<Polarity>,
}

impl Display for AnglePolarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.points
                .chunks_exact(3)
                .zip_eq(self.polarities.iter())
                .map(|(a, pol)| format!("{} {} {} {}", pol, a[0], a[1], a[2]))
                .join(", ")
        )
    }
}

impl Constraint for AnglePolarity {
    fn parse(mut input: &str, index: &mut PointIndex) -> Option<Self> {
        let mut polarities = Vec::new();
        let mut point_names = Vec::new();
        loop {
            polarities.push(
                if literal("±")(&mut input)
                    .or(literal("+/-")(&mut input))
                    .is_some()
                {
                    Polarity::Pro
                } else if literal("∓")(&mut input)
                    .or(literal("-/+")(&mut input))
                    .is_some()
                {
                    Polarity::Anti
                } else {
                    return None;
                },
            );
            space(&mut input);
            literal("∠")(&mut input).or(literal("<")(&mut input))?;
            space(&mut input);
            point_names.push(word(&mut input)?);
            space(&mut input)?;
            point_names.push(word(&mut input)?);
            space(&mut input)?;
            point_names.push(word(&mut input)?);
            space(&mut input);
            if literal(",")(&mut input).is_none() {
                break;
            }
            space(&mut input);
        }
        if polarities.len() < 2 {
            return None;
        }
        let points = point_names
            .into_iter()
            .map(|p| index.get_or_insert(p))
            .collect();
        Some(Self { points, polarities })
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, contains: &dyn Fn(PointID) -> bool) -> Vec<PointID> {
        if contained_elements(&self.points, &contains, 3).count() >= 1 {
            targets(&self.points, &contains, 3)
        } else {
            Vec::new()
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        let binding = |p| p < pos.len();
        let (ind, a) = contained_elements(&self.points, &binding, 3)
            .next()
            .unwrap();
        let pol = self.polarities[ind];

        let t_a_ind = t_ind / 3;
        let t_i = t_ind % 3;
        let t_a = &self.points[t_a_ind * 3..(t_a_ind + 1) * 3];
        let t_pol = self.polarities[t_a_ind];

        let dir = (pos[a[1]] - pos[a[0]])
            .cross(pos[a[2]] - pos[a[1]])
            .signum();
        let t_dir = if pol == t_pol { dir } else { -dir };

        let o = pos[t_a[(t_i + 1) % 3]];
        let p = pos[t_a[(t_i + 2) % 3]];

        vec![Geo::Two(TwoD::Half {
            o,
            n: (p - o).unit().perp() * t_dir,
        })]
    }

    fn flags(&self) -> ConFlags {
        ConFlags::empty()
    }
}

pub(super) fn contained_elements<'a>(
    points: &'a [PointID],
    contains: &'a dyn Fn(PointID) -> bool,
    n: usize,
) -> impl Iterator<Item = (usize, &'a [PointID])> {
    points
        .chunks_exact(n)
        .enumerate()
        .filter(move |(_, element)| element.iter().all(|p| contains(*p)))
}

pub(super) fn partial_elements<'a>(
    points: &'a [PointID],
    contains: &'a dyn Fn(PointID) -> bool,
    n: usize,
) -> impl Iterator<Item = (usize, &'a [PointID])> {
    points.chunks_exact(n).filter_map(move |element| {
        let idx = element
            .iter()
            .positions(|p| !contains(*p))
            .exactly_one()
            .ok()?;
        Some((idx, element))
    })
}

pub(super) fn targets<'a>(
    points: &'a [PointID],
    contains: &'a dyn Fn(PointID) -> bool,
    n: usize,
) -> Vec<PointID> {
    partial_elements(points, contains, n)
        .map(|(idx, points)| points[idx])
        .collect()
}
