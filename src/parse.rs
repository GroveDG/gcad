use std::{collections::HashMap, fmt::Display};

mod math;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ParseErr(pub(super) ParseErrType, pub(super) *const u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ParseErrType {
    Nothing,
    Wrong,
    No(&'static str),
    Invalid,
    Extra,
}
impl Default for ParseErrType {
    fn default() -> Self {
        Nothing
    }
}
use math::{parse_math, parse_vector, MathExpr};
use ParseErrType::*;
impl Display for ParseErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nothing => write!(f, "Nothing"),
            No(s) => write!(f, "No {s}"),
            Invalid => write!(f, "Invalid"),
            Extra => write!(f, "Extra"),
            Wrong => write!(f, "Wrong"),
        }
    }
}

use gsolve::{
    math::{Geo, Vector},
    order::Quantity,
    Order, PID,
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
            for dependency in statement.dependencies() {
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

        if !map.is_empty() {
            return Err(format!("Unused {:?} {:?}", fig.point_map, map));
        }

        Ok(fig)
    }
}

pub fn parse(document: &str) -> Result<Vec<Statement>, ParseErr> {
    let mut statements = Vec::new();
    for line in document.lines() {
        statements.append(&mut parse_line(line)?);
    }
    Ok(statements)
}

#[derive(Debug, Clone, Hash)]
pub enum StatementType {
    Origin(Vector),
    Quantity(QuantityType, MathExpr),
}
#[derive(Debug, Clone, Copy, Hash)]
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
#[derive(Debug, Clone, Hash)]
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
    fn dependencies(&self) -> Vec<String> {
        let mut points: Vec<String> = self.points[..self.points.len() - 1]
            .iter()
            .cloned()
            .collect();
        match &self.s_type {
            StatementType::Quantity(t, m) => points.extend(m.points.iter().cloned()),
            _ => {}
        }
        points
    }
    fn quantity(&self, point_map: &HashMap<String, PID>) -> Option<Quantity> {
        let mut points = self.points[..self.points.len() - 1]
            .iter()
            .map(|p| point_map.get(p).copied())
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
                            .map(|p| point_map.get(p).copied())
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
    if blank(line) {
        return Ok(Vec::new());
    }
    let mut err = ParseErr(Nothing, line.as_ptr());
    for parser in [parse_origin, parse_multi_expr] {
        let e = match (parser)(line) {
            Ok(s) => return Ok(s),
            Err(e) => e,
        };
        if err.0 == Nothing && err.1 == line.as_ptr() {
            err = e;
        }
    }
    Err(err)
}

fn parse_multi_expr(line: &str) -> Result<Vec<Statement>, ParseErr> {
    let exprs: Vec<_> = line.split("=").map(|e| e.trim_start()).collect();

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
            if !blank(expr) {
                errs.push(ParseErr(Extra, expr.as_ptr()));
                continue 'parsers;
            }
            statements.push(points);
        }
        let n = match if let Some(mut expr) = exprs.last().copied() {
            parse_math(expr).map_err(|err| match err.0 {
                Nothing => match (qt.parser())(&mut expr) {
                    Ok(_) => ParseErr(No("value"), expr.as_ptr()),
                    Err(e) => e,
                },
                _ => err,
            })
        } else {
            Err(ParseErr(Nothing, exprs.last().unwrap_or(&line).as_ptr()))
        } {
            Ok(n) => n,
            Err(e) => {
                errs.push(e);
                continue 'parsers;
            }
        };
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
        .reduce(|err, e| {
            if err.0 == Nothing && err.1 == line.as_ptr() {
                e
            } else {
                err
            }
        })
        .unwrap_or(ParseErr(Nothing, line.as_ptr()));
    Err(err)
}

fn parse_origin(mut expr: &str) -> Result<Vec<Statement>, ParseErr> {
    let p = wrap(word(&mut expr), Nothing)?;
    space(&mut expr);
    let v = if literal("=")(&mut expr).is_ok() {
        parse_vector(expr.trim_start())?
    } else {
        Vector::ZERO
    };
    Ok(vec![Statement {
        s_type: StatementType::Origin(v),
        points: vec![p.to_string()],
    }])
}

fn parse_distance(expr: &mut &str) -> Result<Vec<String>, ParseErr> {
    wrap(literal("|")(expr), Nothing)?;
    space(expr);
    let p0 = wrap(word(expr), No("point"))?;
    wrap(space(expr), No("space"))?;
    let p1 = wrap(word(expr), No("point"))?;
    space(expr);
    wrap(literal("|")(expr), No("|"))?;
    Ok(vec![p0.to_string(), p1.to_string()])
}

fn parse_orientation(expr: &mut &str) -> Result<Vec<String>, ParseErr> {
    wrap(literal("<")(expr), Nothing)?;
    space(expr);
    let p0 = wrap(word(expr), No("point"))?;
    wrap(space(expr), No("space"))?;
    let p1 = wrap(word(expr), No("point"))?;
    space(expr);
    wrap(literal(">")(expr), No(">"))?;
    Ok(vec![p0.to_string(), p1.to_string()])
}

const fn take_while<'a>(
    mut f: impl FnMut(char) -> bool,
    min: usize,
    max: usize,
) -> impl FnMut(&mut &'a str) -> Result<&'a str, *const u8> {
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
            return Err(input.as_ptr());
        }
        let result = &input[..split];
        *input = &input[split..];
        Ok(result)
    }
}

#[inline]
fn space<'a>(input: &mut &'a str) -> Result<&'a str, *const u8> {
    take_while(char::is_whitespace, 1, usize::MAX)(input)
}

#[inline]
fn word<'a>(input: &mut &'a str) -> Result<&'a str, *const u8> {
    take_while(char::is_alphabetic, 1, usize::MAX)(input)
}

#[inline]
const fn literal<'a>(pattern: &'a str) -> impl Fn(&mut &'a str) -> Result<&'a str, *const u8> {
    move |i: &mut &'a str| {
        *i = i.strip_prefix(pattern).ok_or(i.as_ptr())?;
        Ok(pattern)
    }
}

#[inline]
pub(super) fn wrap<T>(res: Result<T, *const u8>, t: ParseErrType) -> Result<T, ParseErr> {
    res.map_err(|p| ParseErr(t, p))
}

#[inline]
pub(super) fn blank(input: &str) -> bool {
    input.trim().is_empty()
}
