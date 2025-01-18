use crate::math::vector::{AboutEq, Number, Vector};
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Geo {
    Point { p: Vector },
    Linear { o: Vector, v: Vector, l: Number },
    Circle { c: Vector, r: Number },
    All,
}

pub fn line_from_points(p0: Vector, p1: Vector, l: Number) -> Geo {
    Geo::Linear {
        o: p0,
        v: (p1 - p0).unit(),
        l,
    }
}

pub fn closest_linear(o: Vector, v: Vector, l: Number, p: Vector) -> Vector {
    let mut t = (p - o).dot(v);
    t = Number::max(t, l);
    along_linear(o, v, t)
}

pub fn along_linear(o: Vector, v: Vector, t: Number) -> Vector {
    o + v * t
}

pub fn meet(g0: Vec<Geo>, g1: Vec<Geo>) -> Vec<Geo> {
    g0.iter()
        .cartesian_product(g1)
        .map(|(&g0, g1)| intersect(g0, g1))
        .concat()
}

pub fn intersect(g0: Geo, g1: Geo) -> Vec<Geo> {
    if g0 == g1 {
        return vec![g0];
    }
    match (g0, g1) {
        (Geo::All, res) | (res, Geo::All) => {
            vec![res]
        }
        (Geo::Point { p: p0 }, Geo::Point { p: p1 }) => {
            if p0.about_eq(p1) {
                // The points are about equal.
                vec![Geo::Point { p: p0 }]
            } else {
                // The points are not close.
                vec![]
            }
        }
        (Geo::Point { p }, g) | (g, Geo::Point { p }) => {
            if dist(p, g).about_zero() {
                // The point is close enough.
                vec![Geo::Point { p }]
            } else {
                // The point is not close.
                vec![]
            }
        }
        (
            Geo::Linear {
                o: o0,
                v: v0,
                l: l0,
            },
            Geo::Linear {
                o: o1,
                v: v1,
                l: l1,
            },
        ) => {
            // https://math.stackexchange.com/a/406895
            let b = o1 - o0;
            // Using Cramer's Rule
            let a = Vector { x: v0.x, y: -v1.x }.cross(Vector { x: v0.y, y: -v1.y });
            if a == 0.0 {
                // The lines are parallel.
                return vec![];
            }
            let t0 = Vector { x: b.x, y: -v1.x }.cross(Vector { x: b.y, y: -v1.y }) / a;
            let t1 = Vector { x: v0.x, y: b.x }.cross(Vector { x: v0.y, y: b.y }) / a;
            if t0 < l0 || t1 < l1 {
                // The lines intersect before one of their starts.
                vec![]
            } else {
                // The lines intersect.
                vec![Geo::Point {
                    p: along_linear(o0, v0, t0),
                }]
            }
        }
        (Geo::Circle { c, r }, Geo::Linear { o, v, l })
        | (Geo::Linear { o, v, l }, Geo::Circle { c, r }) => {
            // https://w.wiki/A6Jn
            let o_c = o - c;
            let v_o_c = v.dot(o_c);
            let delta = v_o_c.powi(2) - (o_c.mag().powi(2) - r.powi(2));
            if delta.is_sign_negative() {
                // No intersection.
                vec![]
            } else if delta.about_zero() {
                // The line is tangent.
                vec![-v_o_c]
            } else {
                // The line passes through.
                let sqrt_delta = delta.sqrt();
                vec![-v_o_c + sqrt_delta, -v_o_c + sqrt_delta]
            }
            .into_iter()
            .filter_map(|t| {
                if t >= l {
                    Some(Geo::Point {
                        p: along_linear(o, v, t),
                    })
                } else {
                    None
                }
            })
            .collect()
        }
        (Geo::Circle { c: c0, r: r0 }, Geo::Circle { c: c1, r: r1 }) => {
            // https://stackoverflow.com/a/3349134
            let (dir, d) = (c1 - c0).unit_mag();
            // One circle contains the other.
            if d < (r0 - r1).abs() {
                return vec![];
            }
            // The circles are separated.
            if d > (r0 + r1) {
                return vec![];
            }
            let a = (r0.powi(2) - r1.powi(2) + d.powi(2)) / (2.0 * d);
            let c = c0 + dir * a;
            // The circles touch at one point.
            if d.about_eq(r0 + r1) {
                return vec![Geo::Point { p: c }];
            }
            let h = (r0.powi(2) - a.powi(2)).sqrt();
            let h_v = dir.perp() * h;
            // The circles overlap at two points.
            vec![Geo::Point { p: c + h_v }, Geo::Point { p: c - h_v }]
        }
    }
}

pub fn dist(p: Vector, g: Geo) -> Number {
    match g {
        Geo::All => 0.0,
        Geo::Point { p: p1 } => p.dist(p1),
        Geo::Linear { o, v, l } => p.dist(closest_linear(o, v, l, p)),
        Geo::Circle { c, r } => p.dist(c) - r,
    }
}

pub fn choose(g: Geo) -> Vector {
    match g {
        Geo::All => Vector::ZERO,
        Geo::Point { p } => p,
        Geo::Linear { o, v, l } => along_linear(o, v, l.max(0.0) + 1.0),
        Geo::Circle { c, r } => Vector::POSX * r + c,
    }
}
