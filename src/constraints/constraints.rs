use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::all_consuming,
    sequence::{pair, terminated},
};

use crate::{
    math::{
        geo::{line_from_points, Geo, OneD, TwoD},
        Number, Vector,
    },
    order::{PointID, PointIndex},
    parse::{flag, ident, list_len, separated_listn, ws},
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
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()> {
        let mut parser = all_consuming(ws(separated_listn(
            alt((tag("∥"), tag("||"))),
            ws(list_len(space1, ident, 2)),
            2,
        )));
        let Ok((_, points)) = parser(s) else {
            return Err(());
        };
        Ok(Self {
            points: points
                .into_iter()
                .flatten()
                .map(|p| index.get_or_insert(p))
                .collect(),
        })
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if let Some(_) = self
            .points
            .chunks_exact(2)
            .into_iter()
            .filter(|l| known_points.contains(&l[0]) && known_points.contains(&l[1]))
            .next()
        {
            self.points
                .chunks_exact(2)
                .filter_map(|l| {
                    if !known_points.contains(&l[0]) && known_points.contains(&l[1]) {
                        Some(l[0])
                    } else if !known_points.contains(&l[1]) && known_points.contains(&l[0]) {
                        Some(l[1])
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        let l = self
            .points
            .chunks_exact(2)
            .filter(|&l| pos.len() > l[0] && pos.len() > l[1])
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
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()> {
        let mut parser = all_consuming(ws(separated_listn(
            alt((tag("⟂"), tag("_|_"))),
            ws(list_len(space1, ident, 2)),
            2,
        )));
        let Ok((_, points)) = parser(s) else {
            return Err(());
        };
        Ok(Self {
            points: points
                .into_iter()
                .flatten()
                .map(|p| index.get_or_insert(p))
                .collect(),
        })
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if let Some(_) = self
            .points
            .chunks_exact(2)
            .filter(|&l| known_points.contains(&l[0]) && known_points.contains(&l[1]))
            .next()
        {
            self.points
                .chunks_exact(2)
                .filter_map(|l| {
                    if !known_points.contains(&l[0]) && known_points.contains(&l[1]) {
                        Some(l[0])
                    } else if !known_points.contains(&l[1]) && known_points.contains(&l[0]) {
                        Some(l[1])
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        let (ind, l) = self
            .points
            .chunks_exact(2)
            .enumerate()
            .filter(|&(_, l)| pos.len() > l[0] && pos.len() > l[1])
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
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()> {
        let mut parser = all_consuming(ws(separated_listn(tag("-"), ws(ident), 3)));
        let Ok((_, points)) = parser(s) else {
            return Err(());
        };
        Ok(Self {
            points: points.into_iter().map(|p| index.get_or_insert(p)).collect(),
        })
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if self
            .points
            .iter()
            .filter(|&p| known_points.contains(p))
            .count()
            >= 2
        {
            self.points
                .iter()
                .copied()
                .filter(|p| !known_points.contains(p))
                .collect()
        } else {
            vec![]
        }
    }

    fn geo(&self, pos: &[Vector], _t_ind: usize) -> Vec<Geo> {
        let [&p0, &p1] = self
            .points
            .iter()
            .filter(|&&p| pos.len() > p)
            .next_array()
            .unwrap();
        vec![Geo::One(line_from_points(
            pos[p0],
            pos[p1],
            Number::NEG_INFINITY,
        ))]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
    fn parse(s: &str, index: &mut PointIndex) -> Result<Self, ()> {
        let mut parser = all_consuming(ws(separated_listn(
            tag(","),
            ws(pair(
                terminated(
                    ws(flag(
                        alt((tag("±"), tag("+/-"))),
                        alt((tag("∓"), tag("-/+"))),
                    )),
                    ws(alt((tag("<"), tag("∠")))),
                ),
                ws(list_len(space1, ident, 3)),
            )),
            2,
        )));
        let Ok((_, points)) = parser(s) else {
            return Err(());
        };
        let (polarities, points): (Vec<Polarity>, Vec<[PointID; 3]>) = points
            .into_iter()
            .map(|(pol, p)| {
                (
                    if pol { Polarity::Pro } else { Polarity::Anti },
                    [
                        index.get_or_insert(p[0]),
                        index.get_or_insert(p[1]),
                        index.get_or_insert(p[2]),
                    ],
                )
            })
            .unzip();
        Ok(Self {
            points: points.into_flattened(),
            polarities,
        })
    }

    fn points(&self) -> &[PointID] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut [PointID] {
        &mut self.points
    }

    fn targets(&self, known_points: &HashSet<PointID>) -> Vec<PointID> {
        if self
            .points
            .chunks_exact(3)
            .filter(|&a| a.iter().all(|p| known_points.contains(p)))
            .next()
            .is_some()
        {
            self.points
                .chunks_exact(3)
                .filter_map(|a| {
                    a.iter()
                        .filter(|p| !known_points.contains(p))
                        .exactly_one()
                        .ok()
                })
                .copied()
                .collect()
        } else {
            Vec::new()
        }
    }

    fn geo(&self, pos: &[Vector], t_ind: usize) -> Vec<Geo> {
        let Some((a, pol)) = self
            .points
            .chunks_exact(3)
            .zip(self.polarities.iter())
            .filter(|&(a, _)| a.iter().all(|&p| pos.len() > p))
            .next()
        else {
            return Vec::new();
        };
        let t_a_ind = t_ind / 3;
        let t_i = t_ind % 3;
        let t_a = &self.points[t_a_ind * 3..(t_a_ind + 1) * 3];
        let t_pol = &self.polarities[t_a_ind];

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
