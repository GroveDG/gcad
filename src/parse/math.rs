use std::{
    collections::VecDeque,
    f64::consts::{PI, TAU},
    ops::{Add, Div, Mul, Sub},
};

use super::{
    ParseErr::{self, *},
    literal, space, take_while,
};
use gsolve::math::{Number, Vector};

#[derive(Debug)]
enum Math {
    Operand(Operand),
    Operator(Op),
}
#[derive(Debug)]
enum Operand {
    Constant(Number),
}

// https://mathcenter.oxford.emory.edu/site/cs171/shuntingYardAlgorithm/
pub(super) fn parse_math(expr: &mut &str) -> Result<Number, ParseErr> {
    let mut output = Vec::new();
    let mut stack: Vec<Op> = Vec::new();
    while !expr.is_empty() {
        output.push(Math::Operand(Operand::Constant(parse_number(expr)?)));
        let Some(op) = parse_op(expr) else {
            break;
        };
        match op {
            Op::LPn => {
                stack.push(op);
            }
            Op::RPn => loop {
                let op = stack.pop().ok_or(No("("))?;
                if op == Op::LPn {
                    break;
                }
                output.push(Math::Operator(op));
            },
            _ => {
                let Some(&o) = stack.last() else {
                    stack.push(op);
                    continue;
                };
                if o == Op::LPn || op > o || (op == o && op.is_r_assoc()) {
                    stack.push(op);
                    continue;
                }
                while stack
                    .last()
                    .is_some_and(|&o| op < o || (op == o && !op.is_r_assoc()))
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
    Ok(compute_math(output)?)
}

pub(super) fn parse_vector(mut expr: &str) -> Result<Vector, ParseErr> {
    literal("(")(&mut expr).ok_or(Nothing)?;
    space(&mut expr);
    let x = parse_number(&mut expr).map_err(|e| match e {
        Nothing => No("number"),
        _ => Invalid,
    })?;
    space(&mut expr);
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

pub(super) fn parse_number(expr: &mut &str) -> Result<Number, ParseErr> {
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
    let mut n = literal("+")(expr)
        .or_else(|| literal("-")(expr))
        .unwrap_or_default()
        .to_string();
    n.push_str(take_while(|c| c.is_ascii_digit() || c == '.', 1, usize::MAX)(expr).ok_or(Nothing)?);
    n.parse().map_err(|_| Invalid)
}

fn compute_math(math: Vec<Math>) -> Result<Number, ParseErr> {
    let mut stack = VecDeque::new();
    println!("{:?}", math);
    for m in math {
        match m {
            Math::Operand(o) => stack.push_back(match o {
                Operand::Constant(n) => n,
            }),
            Math::Operator(op) => {
                let lhs = stack.pop_front().ok_or(Invalid)?;
                let rhs = stack.pop_front().ok_or(Invalid)?;
                stack.push_back((op.func().ok_or(Invalid)?)(lhs, rhs));
            }
        }
    }
    if stack.len() != 1 {
        return Err(Invalid);
    }
    Ok(stack[0])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
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
