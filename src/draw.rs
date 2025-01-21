use std::collections::HashMap;

use const_format::formatc;
use regex::Regex;
use rsille::Canvas;

use crate::{
    constraints::{elements::Point, POINT},
    math::vector::{bounding_box, Number, Vector},
    order::PointIndex,
};

pub fn draw(positions: HashMap<Point, Vector>, index: &PointIndex) {
    let (mut min, mut max) = bounding_box(positions.values());

    let mut size = max - min;
    min -= size * 0.125;
    max += size * 0.125;
    size *= 1.25;

    let mut canvas = Canvas::new();
    let t_size = term_size::dimensions().unwrap();
    let t_size = Vector {
        x: t_size.0 as Number * 2.0,
        y: (t_size.1 - 4) as Number * 4.0,
    };
    let scale = t_size / size;
    let scale = if t_size.y > scale.x * size.y {
        scale.x
    } else {
        scale.y
    };
    size *= scale;

    let tf = |v: Vector| -> Vector {
        (v - min) * scale
    };

    canvas.set_size(size.x, size.y);
    for (_, &pos) in positions.iter() {
        let pos = (pos - min) * scale;
        canvas.set(pos.x, pos.y);
    }
    for path in index.paths() {
        let mut pos = Vector::ZERO;
        for cmd in path {
            pos = match cmd {
                PathCmd::Move(p) => tf(positions[p]),
                PathCmd::Line(p) => {
                    let pos_1 = tf(positions[p]);
                    canvas.line(pos.into(), pos_1.into());
                    pos_1
                }
                PathCmd::Quadratic(_, _) => todo!(),
                PathCmd::Cubic(_, _, _) => todo!(),
            }
        }
    }
    canvas.print();
}

#[derive(Debug, Clone)]
pub enum PathCmd {
    Move(Point),
    Line(Point),
    Quadratic(Point, Point),
    Cubic(Point, Point, Point),
}

pub fn parse_path(mut s: &str) -> Result<Vec<PathCmd>, ()> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(formatc!(
            r"^{POINT}((?:-{POINT}){{0,2}}->{POINT})+$"
        )).unwrap();
        static ref MOVE: Regex = Regex::new(formatc!(r"^{POINT}")).unwrap();

    }

    if !RE.is_match(s) {
        return Err(());
    }

    let mut cmds = Vec::new();

    // Starting M (Move) command
    {
        let c = MOVE.captures(s).unwrap();
        cmds.push(PathCmd::Move(c[1].to_string()));
        s = &s[c.len()..]
    }

    let mut points = Vec::new();
    for p in s.split("-") {
        if !p.starts_with(">") {
            points.push(p.trim());
            continue;
        }
        points.push(p[1..].trim());
        cmds.push(match points.len() {
            0 => unreachable!(),
            1 => PathCmd::Line(points[0].to_string()),
            2 => PathCmd::Quadratic(points[0].to_string(), points[1].to_string()),
            3 => PathCmd::Cubic(
                points[0].to_string(),
                points[1].to_string(),
                points[2].to_string(),
            ),
            _ => panic!(),
        });
        points.clear();
    }

    // TODO: Add !points.is_empty() error.

    Ok(cmds)
}
