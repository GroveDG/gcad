use std::{collections::HashMap, str::FromStr};

use gsolve::{
    constraints::{Constraint, Element, Polarity},
    math::{Number, Vector},
    order_bfs, solve_brute, PID,
};

use crate::{parsing::*, GCADFigure};

impl FromStr for GCADFigure {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fig = GCADFigure::default();
        let mut comment = false;
        for mut line in s.lines() {
            line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with('\"') {
                comment = true;
            }
            if line.ends_with('\"') {
                comment = false;
                continue;
            }
            if comment {
                continue;
            }
            if !parse_line(line, &mut fig) {
                return Err(format!("failed to parse line: {line}"));
            }
        }
        Ok(fig)
    }
}

impl GCADFigure {
    pub fn get_or_insert_id(&mut self, point: &str) -> PID {
        if let Some(id) = self.get_id(point) {
            *id
        } else {
            let id = self.fig.new_point();
            self.points.insert(point.to_owned(), id);
            id
        }
    }
    pub fn get_id(&self, point: &str) -> Option<&PID> {
        self.points.get_by_left(point)
    }
    pub fn get_name(&self, id: &PID) -> Option<&String> {
        self.points.get_by_right(id)
    }
    pub fn add_constraint(&mut self, constraint: Constraint, points: Vec<&str>) {
        let points = points
            .into_iter()
            .map(|p| self.get_or_insert_id(p))
            .collect();
        self.fig.add_constraint(constraint, points);
    }
    pub fn add_path(&mut self, path: Vec<PathCmd>) {
        self.paths.push(path);
    }
    pub(crate) fn paths<'a>(&'a self) -> std::slice::Iter<'a, Vec<PathCmd>> {
        self.paths.iter()
    }
    pub fn solve(&mut self) -> Result<HashMap<String, Vector>, String> {
        println!("{:#?}", self.points);
        let order = order_bfs(&mut self.fig);
        let positions = solve_brute(order)?;
        let mut pos_map = HashMap::new();
        for (i, pos) in positions.into_iter().enumerate() {
            let Some(id) = self.points.get_by_right(&i) else {
                return Err("Improper point mapping.".to_owned())
            };
            pos_map.insert(id.clone(), pos);
        }
        Ok(pos_map)
    }
}

pub fn parse_line(line: &str, fig: &mut GCADFigure) -> bool {
    if let Some((points, constraint)) = parse_constraint(line) {
        fig.add_constraint(constraint, points);
        return true;
    }
    if let Some(constraints) = parse_equality(line) {
        for (points, constraint) in constraints {
            fig.add_constraint(constraint, points);
        }
        return true;
    }
    if let Some(path) = parse_path(line) {
        fig.add_path(path);
        return true;
    }
    false
}

pub fn parse_constraint<'a>(input: &'a str) -> Option<(Vec<&'a str>, Constraint)> {
    const PARSERS: &[fn(&str) -> Option<(Vec<&str>, Constraint)>] = &[
        parse_parallel,
        parse_perpendicular,
        parse_collinear,
        parse_chirality,
    ];
    for parser in PARSERS {
        if let Some(result) = parser(input) {
            return Some(result);
        }
    }
    None
}

pub fn parse_equality<'a>(line: &'a str) -> Option<Vec<(Vec<&'a str>, Constraint)>> {
    const PARSERS: &[fn(&str) -> Option<(Vec<&str>, Element)>] = &[parse_distance, parse_angle];
    if !line.contains("=") {
        return None;
    }
    let mut value = None;
    let mut elements: Vec<(Vec<&str>, Element)> = Vec::new();
    'exprs: for expr in line.split("=") {
        let expr = expr.trim();
        for parser in PARSERS {
            if let Some(element) = parser(expr) {
                elements.push(element);
                continue 'exprs;
            }
        }
        if let Ok(parsed) = expr.parse::<Number>() {
            if value.is_some() {
                return None;
            }
            value = Some(parsed);
            continue;
        }
        return None;
    }
    let Some(value) = value else {
        return None;
    };
    Some(
        elements
            .into_iter()
            .map(|(points, element)| -> (Vec<&'a str>, Constraint) {
                (points, Constraint::Element(value, element))
            })
            .collect(),
    )
}

fn parse_distance<'a>(mut input: &'a str) -> Option<(Vec<&'a str>, Element)> {
    literal("|")(&mut input)?;
    space(&mut input);
    let a = word(&mut input)?;
    space(&mut input)?;
    let b = word(&mut input)?;
    space(&mut input);
    literal("|")(&mut input)?;

    Some((vec![a, b], Element::Distance))
}
fn parse_angle<'a>(mut input: &'a str) -> Option<(Vec<&'a str>, Element)> {
    literal("∠")(&mut input).or(literal("<")(&mut input))?;
    space(&mut input);
    let a = word(&mut input)?;
    space(&mut input)?;
    let b = word(&mut input)?;
    space(&mut input)?;
    let c = word(&mut input)?;

    Some((vec![a, b, c], Element::Angle))
}
fn parse_parallel<'a>(mut input: &'a str) -> Option<(Vec<&'a str>, Constraint)> {
    let mut points = Vec::new();
    loop {
        points.push(word(&mut input)?);
        space(&mut input)?;
        points.push(word(&mut input)?);
        space(&mut input);
        if literal("∥")(&mut input)
            .or(literal("||")(&mut input))
            .is_none()
        {
            break;
        }
        space(&mut input);
    }
    if points.len() < 4 {
        return None;
    }
    Some((points, Constraint::Parallel))
}
fn parse_perpendicular<'a>(mut input: &'a str) -> Option<(Vec<&'a str>, Constraint)> {
    let mut points = Vec::new();
    loop {
        points.push(word(&mut input)?);
        space(&mut input)?;
        points.push(word(&mut input)?);
        space(&mut input);
        if literal("⟂")(&mut input)
            .or(literal("_|_")(&mut input))
            .is_none()
        {
            break;
        }
        space(&mut input);
    }
    if points.len() < 4 {
        return None;
    }
    Some((points, Constraint::Parallel))
}
fn parse_collinear<'a>(mut input: &'a str) -> Option<(Vec<&'a str>, Constraint)> {
    let mut points = Vec::new();
    loop {
        points.push(word(&mut input)?);
        space(&mut input);
        if literal("-")(&mut input).is_none() {
            break;
        }
        space(&mut input);
    }
    if points.len() < 3 {
        return None;
    }
    Some((points, Constraint::Collinear))
}
fn parse_chirality<'a>(mut input: &'a str) -> Option<(Vec<&'a str>, Constraint)> {
    let mut polarities = Vec::new();
    let mut points = Vec::new();
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
        points.push(word(&mut input)?);
        space(&mut input)?;
        points.push(word(&mut input)?);
        space(&mut input)?;
        points.push(word(&mut input)?);
        space(&mut input);
        if literal(",")(&mut input).is_none() {
            break;
        }
        space(&mut input);
    }
    if polarities.len() < 2 {
        return None;
    }
    Some((points, Constraint::Chirality(polarities)))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathCmd {
    Move(String),
    Line(String),
    Quadratic(String, String),
    Cubic(String, String, String),
}

pub fn parse_path(mut input: &str) -> Option<Vec<PathCmd>> {
    let mut points = Vec::new();
    let mut cmds = Vec::new();
    let mut term = true;
    loop {
        points.push(word(&mut input)?);
        if term {
            cmds.push(match points.len() {
                1 => PathCmd::Line(points[0].to_string()),
                2 => PathCmd::Quadratic(points[0].to_string(), points[1].to_string()),
                3 => PathCmd::Cubic(
                    points[0].to_string(),
                    points[1].to_string(),
                    points[2].to_string(),
                ),
                _ => return None,
            });
            points.clear();
        }
        term = if literal("→")(&mut input)
            .or(literal("->")(&mut input))
            .is_some()
        {
            true
        } else if literal("-")(&mut input).is_some() {
            false
        } else {
            break;
        };
        space(&mut input);
    }

    if !points.is_empty() {
        return None;
    }

    // Starting M (Move) command
    cmds[0] = match &cmds[0] {
        PathCmd::Line(p) => PathCmd::Move(p.clone()),
        _ => unreachable!(),
    };

    Some(cmds)
}
