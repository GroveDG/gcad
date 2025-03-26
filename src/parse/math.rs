use std::{
    collections::VecDeque,
    f64::consts::{E, PI, TAU},
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
};

use super::{
    literal, parse_distance, parse_orientation, space, take_while, wrap, ParseErr,
    ParseErrType::{self, *},
    QuantityType,
};
use gsolve::math::{Number, Vector};

#[derive(Debug, Clone, Hash)]
pub struct MathExpr {
    expr: Vec<Math>,
    pub(super) points: Vec<String>,
}
impl MathExpr {
    pub(super) fn func(&self) -> Result<Box<dyn Fn(&[Vector]) -> Number>, ParseErrType> {
        let mut stack: VecDeque<Box<dyn Fn(&[Vector]) -> f64>> = VecDeque::new();

        for m in &self.expr {
            match m {
                Math::Operand(o) => stack.push_front(match o {
                    Operand::Constant(n) => {
                        let n = *n;
                        Box::new(move |_| n)
                    }
                    Operand::Quantity(t, _) => match t {
                        QuantityType::Distance => Box::new(|pos| -> Number { pos[0].dist(pos[1]) }),
                        QuantityType::Orientation => {
                            Box::new(|pos| -> Number { (pos[1] - pos[0]).angle() })
                        }
                    },
                }),
                Math::Operator(op) => {
                    let rhs = stack.pop_front().ok_or(Invalid)?;
                    let lhs = stack.pop_front().ok_or(Invalid)?;
                    let op_func = op.func().ok_or(Invalid)?;
                    stack.push_front(Box::new(move |pos| (op_func)(lhs(pos), rhs(pos))));
                }
            }
        }
        if stack.len() != 1 {
            return Err(Invalid);
        }
        Ok(stack.pop_front().unwrap())
    }
}
#[derive(Debug, Clone, Hash)]
pub enum Math {
    Operand(Operand),
    Operator(Op),
}
#[derive(Debug, Clone)]
pub enum Operand {
    Constant(Number),
    Quantity(QuantityType, Vec<String>),
}
impl Hash for Operand {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Operand::Constant(n) => n.to_be_bytes().hash(state),
            Operand::Quantity(quantity_type, points) => {
                quantity_type.hash(state);
                points.hash(state);
            }
        }
    }
}

// https://mathcenter.oxford.emory.edu/site/cs171/shuntingYardAlgorithm/
pub(super) fn parse_math(mut expr: &str) -> Result<MathExpr, ParseErr> {
    let mut output = Vec::new();
    let mut stack: Vec<Op> = Vec::new();

    'parsing: loop {
        while let Some(op) = parse_op(&mut expr) {
            if op != Op::LPn {
                return Err(ParseErr(Invalid, expr.as_ptr()));
            }
            stack.push(op);
            space(&mut expr);
        }
        output.push(Math::Operand(
            parse_distance(&mut expr)
                .map(|p| Operand::Quantity(QuantityType::Distance, p))
                .or_else(|_| {
                    parse_orientation(&mut expr)
                        .map(|p| Operand::Quantity(QuantityType::Orientation, p))
                })
                .or_else(|_| parse_number(&mut expr).map(|n| Operand::Constant(n)))?,
        ));
        space(&mut expr);
        let op = loop {
            let Some(op) = parse_op(&mut expr) else {
                if !expr.is_empty() {
                    return Err(ParseErr(Extra, expr.as_ptr()));
                } else {
                    break 'parsing;
                }
            };
            if op == Op::RPn {
                loop {
                    let op = stack.pop().ok_or(ParseErr(No("("), expr.as_ptr()))?;
                    if op == Op::LPn {
                        break;
                    }
                    output.push(Math::Operator(op));
                }
            } else {
                break op;
            }
        };
        space(&mut expr);
        match op {
            Op::LPn | Op::RPn => {
                return Err(ParseErr(Invalid, expr.as_ptr()));
            }
            _ => {
                while stack
                    .last()
                    .is_some_and(|&o| o != Op::LPn && (op < o || (op == o && !op.is_r_assoc())))
                {
                    output.push(Math::Operator(stack.pop().unwrap()));
                }
                stack.push(op);
            }
        }
    }
    for op in stack.into_iter().rev() {
        output.push(Math::Operator(op));
    }
    let points = output
        .iter()
        .filter_map(|m| match m {
            Math::Operand(operand) => match operand {
                Operand::Quantity(_, points) => Some(points),
                _ => None,
            },
            _ => None,
        })
        .flatten()
        .cloned()
        .collect();
    Ok(MathExpr {
        expr: output,
        points,
    })
}

pub(super) fn parse_vector(mut expr: &str) -> Result<Vector, ParseErr> {
    wrap(literal("(")(&mut expr), Nothing)?;
    space(&mut expr);
    let x = parse_number(&mut expr).map_err(|e| match e.0 {
        Nothing => ParseErr(No("number"), e.1),
        _ => ParseErr(Invalid, e.1),
    })?;
    space(&mut expr);
    wrap(literal(",")(&mut expr), No(","))?;
    space(&mut expr);
    let y = parse_number(&mut expr).map_err(|e| match e.0 {
        Nothing => ParseErr(No("number"), e.1),
        _ => ParseErr(Invalid, e.1),
    })?;
    space(&mut expr);
    wrap(literal(")")(&mut expr), No(")"))?;
    Ok(Vector { x, y })
}

pub(super) fn parse_number(expr: &mut &str) -> Result<Number, ParseErr> {
    if literal("PI")(expr).is_ok()
        || literal("Pi")(expr).is_ok()
        || literal("pi")(expr).is_ok()
        || literal("π")(expr).is_ok()
    {
        return Ok(PI);
    }
    if literal("TAU")(expr).is_ok()
        || literal("Tau")(expr).is_ok()
        || literal("tau")(expr).is_ok()
        || literal("τ")(expr).is_ok()
    {
        return Ok(TAU);
    }
    if literal("E")(expr).is_ok() || literal("e")(expr).is_ok() {
        return Ok(E);
    }
    let mut n = literal("+")(expr)
        .or_else(|_| literal("-")(expr))
        .unwrap_or_default()
        .to_string();
    n.push_str(wrap(
        take_while(|c| c.is_ascii_digit() || c == '.', 1, usize::MAX)(expr),
        Nothing,
    )?);
    n.parse().map_err(|_| ParseErr(Invalid, expr.as_ptr()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    LPn,
    RPn,
}
impl Op {
    fn func(&self) -> Option<fn(Number, Number) -> Number> {
        match self {
            Op::Add => Some(Add::add),
            Op::Sub => Some(Sub::sub),
            Op::Mul => Some(Mul::mul),
            Op::Div => Some(Div::div),
            Op::Pow => Some(Number::powf),
            _ => None,
        }
    }
    fn precedence(&self) -> Option<u8> {
        match self {
            Op::Add | Op::Sub => Some(0),
            Op::Mul | Op::Div => Some(1),
            Op::Pow => Some(2),
            Op::LPn | Op::RPn => None,
        }
    }
    fn is_r_assoc(&self) -> bool {
        match self {
            Op::Pow => true,
            _ => false,
        }
    }
}
impl PartialOrd for Op {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.precedence().partial_cmp(&other.precedence())
    }
}
fn parse_op(expr: &mut &str) -> Option<Op> {
    let mut chars = expr.chars();
    let op = match chars.next().unwrap_or_default() {
        '+' => Op::Add,
        '-' => Op::Sub,
        '*' | '×' => Op::Mul,
        '/' | '÷' => Op::Div,
        '^' => Op::Pow,
        '(' => Op::LPn,
        ')' => Op::RPn,
        _ => return None,
    };
    *expr = chars.as_str();
    Some(op)
}
