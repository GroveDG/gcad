use std::{
    collections::{HashMap, HashSet},
    f64::consts::{PI, TAU},
    fmt::Display,
    hash::RandomState,
    mem::take,
};

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
    math::{Geo, Number, Vector},
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
                roots.push(statement.target.clone());
            }
            // Link dependencies to the target.
            for dependency in statement.points.iter().cloned() {
                tree.insert(dependency, statement.target.clone());
            }
            // Map the target to their statements.
            map.insert(statement.target.clone(), statement);
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

#[derive(Debug, Clone, Copy)]
pub enum StatementType {
    Origin(Vector),
    Distance(Number),
    Orientation(Number),
}
#[derive(Debug, Clone)]
pub struct Statement {
    target: String,
    s_type: StatementType,
    points: Vec<String>,
}
impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.s_type {
            StatementType::Origin(v) => {
                let p = &self.points[0];
                write!(f, "{p} = {v}")
            }
            StatementType::Distance(n) => {
                let p0 = &self.points[0];
                let p1 = &self.points[1];
                write!(f, "|{p0} {p1}| = {n}")
            }
            StatementType::Orientation(n) => {
                let p0 = &self.points[0];
                let p1 = &self.points[1];
                write!(f, "<{p0} {p1}> = {n}")
            }
        }
    }
}
impl Statement {
    fn quantity(&self, point_map: &HashMap<String, PID>) -> Option<Quantity> {
        Some(Quantity {
            func: match self.s_type {
                StatementType::Origin(v) => Box::new(move |_| vec![Geo::Point(v)]),
                StatementType::Distance(n) => Box::new(move |pos| vec![Geo::Circle(pos[0], n)]),
                StatementType::Orientation(n) => {
                    Box::new(move |pos| vec![Geo::Ray(pos[0], Vector::from_angle(n))])
                }
            },
            points: self
                .points
                .iter()
                .map(|p| point_map.get(p).cloned())
                .collect::<Option<Vec<_>>>()?,
        })
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
    let parsers: [(
        fn(&str) -> Result<(String, Vec<String>), ParseErr>,
        Box<dyn Fn(&str) -> Result<StatementType, ParseErr>>,
    ); 2] = [
        (
            parse_distance,
            Box::new(|mut v| Ok(StatementType::Distance(parse_number(&mut v)?))),
        ),
        (
            parse_orientation,
            Box::new(|mut v| Ok(StatementType::Orientation(parse_number(&mut v)?))),
        ),
    ];
    'parsers: for (expr_parser, value_parser) in parsers {
        let mut statements = Vec::new();
        for expr in exprs[..exprs.len() - 1].iter() {
            let parts = match (expr_parser)(expr) {
                Ok(c) => c,
                Err(e) => {
                    errs.push(e);
                    continue 'parsers;
                }
            };
            statements.push(parts);
        }
        let s_type = if let Some(expr) = exprs.last() {
            (value_parser)(expr).map_err(|err| match err {
                Nothing => match (expr_parser)(expr) {
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
            .map(|(target, points)| Statement {
                target,
                s_type,
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

// fn parse_expr(expr: &str, value: Number) -> Result<Statement, ParseErr> {
//     let mut err = Nothing;
//         let e = match (parser)(expr, value) {
//             Ok(s) => return Ok(s),
//             Err(e) => e,
//         };
//         if err == Nothing {
//             err = e;
//         }
//     }
//     Err(err)
// }

fn parse_origin(mut expr: &str) -> Result<Vec<Statement>, ParseErr> {
    let p = word(&mut expr).ok_or(Nothing)?;
    space(&mut expr);
    let v = if literal("=")(&mut expr).is_some() {
        parse_vector(expr.trim())?
    } else {
        Vector::ZERO
    };
    Ok(vec![Statement {
        target: p.to_string(),
        s_type: StatementType::Origin(v),
        points: vec![],
    }])
}

fn parse_distance(mut expr: &str) -> Result<(String, Vec<String>), ParseErr> {
    literal("|")(&mut expr).ok_or(Nothing)?;
    space(&mut expr);
    let p0 = word(&mut expr).ok_or(No("point"))?;
    space(&mut expr).ok_or(No("space"))?;
    let p1 = word(&mut expr).ok_or(No("point"))?;
    space(&mut expr);
    literal("|")(&mut expr).ok_or(No("|"))?;
    end(expr).ok_or(Extra)?;
    Ok((p1.to_string(), vec![p0.to_string()]))
}

fn parse_orientation(mut expr: &str) -> Result<(String, Vec<String>), ParseErr> {
    literal("<")(&mut expr).ok_or(Nothing)?;
    space(&mut expr);
    let p0 = word(&mut expr).ok_or(No("point"))?;
    space(&mut expr).ok_or(No("space"))?;
    let p1 = word(&mut expr).ok_or(No("point"))?;
    space(&mut expr);
    literal(">")(&mut expr).ok_or(No(">"))?;
    end(expr).ok_or(Extra)?;
    Ok((p1.to_string(), vec![p0.to_string()]))
}

fn parse_vector(mut expr: &str) -> Result<Vector, ParseErr> {
    literal("(")(&mut expr).ok_or(Nothing)?;
    space(&mut expr);
    let x = parse_number(&mut expr).map_err(|e| match e {
        Nothing => No("number"),
        _ => Invalid,
    })?;
    space(&mut expr);
    println!("{} {}", expr, x);
    literal(",")(&mut expr).ok_or(No(","))?;
    space(&mut expr);
    let y = parse_number(&mut expr).map_err(|e| match e {
        Nothing => No("number"),
        _ => Invalid,
    })?;
    space(&mut expr);
    literal(")")(&mut expr).ok_or(No(")"))?;
    Ok(Vector { x, y })
}

fn parse_number(expr: &mut &str) -> Result<Number, ParseErr> {
    if literal("PI")(expr).is_some()
        || literal("Pi")(expr).is_some()
        || literal("pi")(expr).is_some()
        || literal("π")(expr).is_some()
    {
        return Ok(PI);
    }
    if literal("TAU")(expr).is_some()
        || literal("Tau")(expr).is_some()
        || literal("tau")(expr).is_some()
        || literal("τ")(expr).is_some()
    {
        return Ok(TAU);
    }
    let n = take_while(
        |c| c.is_ascii_digit() || c == '+' || c == '-' || c == '.',
        1,
        usize::MAX,
    )(expr)
    .ok_or(Nothing)?;
    n.parse().map_err(|_| Invalid)
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
