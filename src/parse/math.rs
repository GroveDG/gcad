use std::{
    collections::VecDeque,
    f64::consts::{PI, TAU},
    ops::{Add, Div, Mul, Sub},
};

use super::{
    ParseErr::{self, *},
    QuantityType, literal, parse_distance, parse_orientation, space, take_while,
};
use gsolve::math::{Number, Vector};

#[derive(Debug, Clone)]
pub struct MathExpr {
    expr: Vec<Math>,
    pub(super) points: Vec<String>,
}
impl MathExpr {
    pub(super) fn func(&self) -> Result<Box<dyn Fn(&[Vector]) -> Number>, ParseErr> {
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
#[derive(Debug, Clone)]
pub enum Math {
    Operand(Operand),
    Operator(Op),
}
#[derive(Debug, Clone)]
pub enum Operand {
    Constant(Number),
    Quantity(QuantityType, Vec<String>),
}

// https://mathcenter.oxford.emory.edu/site/cs171/shuntingYardAlgorithm/
pub(super) fn parse_math(mut expr: &str) -> Result<MathExpr, ParseErr> {
    let mut output = Vec::new();
    let mut stack: Vec<Op> = Vec::new();

    'parsing: loop {
        while let Some(op) = parse_op(&mut expr) {
            if op != Op::LPn {
                println!("{}", expr);
                return Err(Invalid);
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
                    return Err(Extra);
                } else {
                    break 'parsing;
                }
            };
            if op == Op::RPn {
                loop {
                    let op = stack.pop().ok_or(No("("))?;
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
                return Err(Invalid);
            }
            _ => {
                let Some(&o) = stack.last() else {
                    stack.push(op);
                    continue;
                };
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
    println!("{:?}", output);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
