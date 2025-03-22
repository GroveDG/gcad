use std::{collections::HashMap, fmt::Display};

mod math;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseErr {
    Nothing,
    No(&'static str),
    Invalid,
    Extra,
}
impl Default for ParseErr {
    fn default() -> Self {
        Nothing
    }
}
use ParseErr::*;
use math::{MathExpr, parse_math, parse_vector};
impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nothing => write!(f, "Nothing"),
            No(s) => write!(f, "No {s}"),
            Invalid => write!(f, "Invalid"),
            Extra => write!(f, "Extra"),
        }
    }
}

use gsolve::{
    Order, PID,
    math::{Geo, Vector},
    order::Quantity,
};
use multimap::MultiMap;

#[derive(Default)]
pub struct Figure {
    pub order: Order,
    pub point_map: HashMap<String, PID>,
}
impl Figure {
    fn add_recursive(
        &mut self,
        target: &String,
        tree: &MultiMap<String, String>,
        map: &mut MultiMap<String, Statement>,
    ) {
        // Return if the point is already added.
        let Some(statements) = map.get_vec(target) else {
            return;
        };
        // Returns if dependencies are unsatisfied.
        let Some(quantities) = statements
            .into_iter()
            .map(|s| s.quantity(&self.point_map))
            .collect()
        else {
            return;
        };
        // Add point and mark as known.
        let pid = self.order.add_point(quantities);
        self.point_map.insert(target.clone(), pid);
        map.remove(target); // Unfortunate drop.
        // Add all dependents.
        for next in tree.get_vec(target).map(|v| v.iter()).unwrap_or_default() {
            self.add_recursive(next, tree, map);
        }
    }
    pub fn from_statements(statements: Vec<Statement>) -> Result<Self, String> {
        let mut map = MultiMap::new();
        let mut roots = Vec::new();
        let mut tree = MultiMap::new();

        // Set up roots and mapping.
        // For each statement...
        for statement in statements {
            // Origins are roots.
            if let StatementType::Origin(_) = statement.s_type {
                roots.push(statement.target().clone());
            }
            // Link dependencies to the target.
            for dependency in statement.points[..statement.points.len()-1].iter().cloned() {
                tree.insert(dependency, statement.target().clone());
            }
            // Map the target to their statements.
            map.insert(statement.target().clone(), statement);
        }

        let mut fig = Figure::default();
        // Starting from each root...
        for root in roots {
            // Add points when their depencencies are satisfied.
            fig.add_recursive(&root, &tree, &mut map)
        }

        Ok(fig)
    }
}

pub fn parse(document: &str) -> Result<Figure, String> {
    let mut statements = Vec::new();
    for line in document.lines() {
        statements.append(&mut parse_line(line).map_err(|e| e.to_string())?);
    }
    Figure::from_statements(statements)
}

#[derive(Debug, Clone)]
pub enum StatementType {
    Origin(Vector),
    Quantity(QuantityType, MathExpr),
}
#[derive(Debug, Clone, Copy)]
pub enum QuantityType {
    Distance,
    Orientation,
}
impl QuantityType {
    fn parser(&self) -> fn(&mut &str) -> Result<Vec<String>, ParseErr> {
        match self {
            Self::Distance => parse_distance,
            Self::Orientation => parse_orientation,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Statement {
    s_type: StatementType,
    points: Vec<String>,
}
impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.s_type {
            StatementType::Origin(v) => {
                let p = &self.points[0];
                write!(f, "{p} = {v}")
            }
            StatementType::Quantity(t, m) => {
                let p0 = &self.points[0];
                let p1 = &self.points[1];
                match t {
                    QuantityType::Distance => write!(f, "|{p0} {p1}| = {m:?}"),
                    QuantityType::Orientation => write!(f, "<{p0} {p1}> = {m:?}"),
                }
            }
        }
    }
}
impl Statement {
    fn quantity(&self, point_map: &HashMap<String, PID>) -> Option<Quantity> {
        let mut points = self
            .points[..self.points.len()-1]
            .iter()
            .map(|p| point_map.get(p).cloned())
            .collect::<Option<Vec<_>>>()?;
        Some(Quantity {
            func: match &self.s_type {
                StatementType::Origin(v) => {
                    let v = *v;
                    Box::new(move |_| vec![Geo::Point(v)])
                }
                StatementType::Quantity(t, m) => {
                    points.append(
                        &mut m
                            .points
                            .iter()
                            .map(|p| point_map.get(p).cloned())
                            .collect::<Option<Vec<_>>>()?,
                    );
                    let m_func = m.func().ok()?;
                    match t {
                        QuantityType::Distance => {
                            Box::new(move |pos| vec![Geo::Circle(pos[0], m_func(&pos[1..]))])
                        }
                        QuantityType::Orientation => Box::new(move |pos| {
                            vec![Geo::Ray(pos[0], Vector::from_angle(m_func(&pos[1..])))]
                        }),
                    }
                }
            },
            points,
        })
    }
    fn target(&self) -> &String {
        self.points.last().unwrap()
    }
}

fn parse_line(line: &str) -> Result<Vec<Statement>, ParseErr> {
    if line.is_empty() {
        return Ok(Vec::new());
    }
    let mut err = Nothing;
    for parser in [parse_origin, parse_multi_expr] {
        let e = match (parser)(line) {
            Ok(s) => return Ok(s),
            Err(e) => e,
        };
        if err == Nothing {
            err = e;
        }
    }
    Err(err)
}

fn parse_multi_expr(line: &str) -> Result<Vec<Statement>, ParseErr> {
    let exprs: Vec<_> = line.split("=").map(|e| e.trim()).collect();

    let mut errs = Vec::new();
    'parsers: for qt in [QuantityType::Distance, QuantityType::Orientation] {
        let mut statements = Vec::new();
        for mut expr in exprs[..exprs.len() - 1].iter().copied() {
            let points = match (qt.parser())(&mut expr) {
                Ok(c) => c,
                Err(e) => {
                    errs.push(e);
                    continue 'parsers;
                }
            };
            statements.push(points);
        }
        let n = if let Some(mut expr) = exprs.last().copied() {
            parse_math(expr).map_err(|err| match err {
                Nothing => match (qt.parser())(&mut expr) {
                    Ok(_) => No("value"),
                    Err(e) => e,
                },
                _ => err,
            })
        } else {
            Err(Nothing)
        }?;
        return Ok(statements
            .into_iter()
            .map(|points| Statement {
                s_type: StatementType::Quantity(qt, n.clone()),
                points,
            })
            .collect());
    }

    let err = errs
        .into_iter()
        .reduce(|err, e| if err == Nothing { e } else { err })
        .unwrap_or_default();
    Err(err)
}

fn parse_origin(mut expr: &str) -> Result<Vec<Statement>, ParseErr> {
    let p = word(&mut expr).ok_or(Nothing)?;
    space(&mut expr);
    let v = if literal("=")(&mut expr).is_some() {
        parse_vector(expr.trim())?
    } else {
        Vector::ZERO
    };
    Ok(vec![Statement {
        s_type: StatementType::Origin(v),
        points: vec![p.to_string()],
    }])
}

fn parse_distance(expr: &mut &str) -> Result<Vec<String>, ParseErr> {
    literal("|")(expr).ok_or(Nothing)?;
    space(expr);
    let p0 = word(expr).ok_or(No("point"))?;
    space(expr).ok_or(No("space"))?;
    let p1 = word(expr).ok_or(No("point"))?;
    space(expr);
    literal("|")(expr).ok_or(No("|"))?;
    Ok(vec![p0.to_string(), p1.to_string()])
}

fn parse_orientation(expr: &mut &str) -> Result<Vec<String>, ParseErr> {
    literal("<")(expr).ok_or(Nothing)?;
    space(expr);
    let p0 = word(expr).ok_or(No("point"))?;
    space(expr).ok_or(No("space"))?;
    let p1 = word(expr).ok_or(No("point"))?;
    space(expr);
    literal(">")(expr).ok_or(No(">"))?;
    Ok(vec![p0.to_string(), p1.to_string()])
}

const fn take_while<'a>(
    mut f: impl FnMut(char) -> bool,
    min: usize,
    max: usize,
) -> impl FnMut(&mut &'a str) -> Option<&'a str> {
    move |input: &mut &'a str| {
        let mut chars = input.char_indices();
        let split = loop {
            if let Some((i, c)) = chars.next() {
                if f(c) {
                    continue;
                }
                break i;
            } else {
                break input.len();
            }
        };
        if split < min || split >= max {
            return None;
        }
        let result = &input[..split];
        *input = &input[split..];
        Some(result)
    }
}

fn space<'a>(input: &mut &'a str) -> Option<&'a str> {
    take_while(char::is_whitespace, 1, usize::MAX)(input)
}

fn word<'a>(input: &mut &'a str) -> Option<&'a str> {
    take_while(char::is_alphabetic, 1, usize::MAX)(input)
}

const fn literal<'a>(pattern: &'a str) -> impl Fn(&mut &'a str) -> Option<&'a str> {
    move |i: &mut &'a str| {
        *i = i.strip_prefix(pattern)?;
        Some(pattern)
    }
}

const fn end(input: &str) -> Option<()> {
    if input.is_empty() { Some(()) } else { None }
}
