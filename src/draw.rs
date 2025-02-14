use std::{collections::HashMap, fs::File, io::Write};

use clap_derive::ValueEnum;
use rsille::Canvas;

use crate::{
    math::{bounding_box, Number, Vector},
    order::PointIndex,
};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Output {
    /// Prints figure.
    Terminal,
    /// Saves points as CSV.
    CSV,
    /// Saves figure as SVG.
    SVG,
}
#[derive(Debug, Clone, Default)]
pub struct DrawOptions {
    pub output: Option<Output>,
    pub paths: Vec<Vec<PathCmd>>,
}

pub fn draw_terminal(mut positions: HashMap<String, Vector>, index: &PointIndex) {
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

pub fn draw_svg(mut positions: HashMap<String, Vector>, index: &PointIndex) -> std::io::Result<()> {
    let (min, max) = bounding_box(positions.values());
    let size = max - min;
    let (size_x, size_y) = size.into();

    for pos in positions.values_mut() {
        *pos -= min;
    }

    let mut svg = File::create("figure.svg")?;

    svg.write(
        format!(r#"<svg width="{size_x}" height="{size_y}" xmlns="http://www.w3.org/2000/svg">"#)
            .as_bytes(),
    )?;

    for path in index.paths() {
        let mut d = String::new();
        let mut first = "";
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
                    if !(i == path.len() - 1 && first == p) {
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
            if i == path.len() - 1 && first == end {
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
    Move(String),
    Line(String),
    Quadratic(String, String),
    Cubic(String, String, String),
}
