use std::{collections::HashMap, fs::File, io::Write};

use const_format::formatc;
use regex::Regex;
use rsille::Canvas;

use crate::{
    constraints::{elements::Point, POINT},
    math::vector::{bounding_box, Number, Vector},
    order::PointIndex,
};

#[derive(Debug, Clone, Copy)]
pub enum Output {
    None,
    CSV,
    Terminal,
    SVG,
}

pub fn draw_terminal(mut positions: HashMap<Point, Vector>, index: &PointIndex) {
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

    for pos in positions.values_mut() {
        *pos -= min;
        *pos *= scale;
    }

    canvas.set_size(size.x, size.y);
    for (_, &pos) in positions.iter() {
        canvas.set(pos.x, pos.y);
    }
    for path in index.paths() {
        let mut pos = Vector::ZERO;
        for cmd in path {
            pos = match cmd {
                PathCmd::Move(p) => positions[p],
                PathCmd::Line(p) => {
                    let pos_0 = positions[p];
                    canvas.line(pos.into(), pos_0.into());
                    pos_0
                }
                PathCmd::Quadratic(_, _) => unimplemented!(),
                PathCmd::Cubic(_, _, _) => unimplemented!(),
            }
        }
    }
    canvas.print();
}

pub fn draw_svg(mut positions: HashMap<Point, Vector>, index: &PointIndex) -> std::io::Result<()> {
    let (min, max) = bounding_box(positions.values());
    let size = max - min;
    let (size_x, size_y) = size.into();

    for pos in positions.values_mut() {
        *pos -= min;
    }

    let mut svg = File::create("figure.svg")?;

    svg.write(
        format!(r#"<svg width="{size_x}" height="{size_y}" xmlns="http://www.w3.org/2000/svg">"#)
            .as_bytes()
    )?;

    for path in index.paths() {
        let mut d = String::new();
        let mut first = &String::new();
        for (i, cmd) in path.iter().enumerate() {
            let end = match cmd {
                PathCmd::Move(p) => {
                    let (x_0, y_0) = positions[p].into();
                    d.push_str(&format!("M {x_0} {y_0} "));
                    first = p;
                    p
                }
                PathCmd::Line(p) => {
                    let (x_0, y_0) = positions[p].into();
                    if !(i == path.len()-1 && first == p) {
                        d.push_str(&format!("L {x_0} {y_0} "));
                    }
                    p
                }
                PathCmd::Quadratic(p0, p1) => {
                    let (x_0, y_0) = positions[p0].into();
                    let (x_1, y_1) = positions[p1].into();
                    d.push_str(&format!("Q {x_0} {y_0}, {x_1} {y_1} "));
                    p1
                }
                PathCmd::Cubic(p0, p1, p2) => {
                    let (x_0, y_0) = positions[p0].into();
                    let (x_1, y_1) = positions[p1].into();
                    let (x_2, y_2) = positions[p2].into();
                    d.push_str(&format!("C {x_0} {y_0}, {x_1} {y_1}, {x_2} {y_2} "));
                    p2
                }
            };
            if i == path.len()-1 && first == end {
                d.push_str("Z");
            }
        }
        svg.write("\n\t".as_bytes())?;
        svg.write(format!(r#"<path d="{d}" fill="black"/>"#).as_bytes())?;
    }
    svg.write("\n</svg>".as_bytes())?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathCmd {
    Move(Point),
    Line(Point),
    Quadratic(Point, Point),
    Cubic(Point, Point, Point),
}

pub fn parse_path(s: &str) -> Result<Vec<PathCmd>, ()> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(formatc!(
            r"^{POINT}((-{POINT}){{0,2}}(->){POINT})+$"
        )).unwrap();
        static ref MOVE: Regex = Regex::new(formatc!(r"^{POINT}")).unwrap();

    }

    let mut s = s.replace("→", "->");

    if !RE.is_match(&s) {
        return Err(());
    }

    let mut cmds = Vec::new();

    // Starting M (Move) command
    {
        let c = MOVE.captures(&s).unwrap();
        cmds.push(PathCmd::Move(c[1].to_string()));
        s = s.split_off(c.len())
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
