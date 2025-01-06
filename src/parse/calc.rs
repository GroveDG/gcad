use lazy_static::lazy_static;
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};

use crate::vector::Number;

#[derive(pest_derive::Parser)]
#[grammar = "parse/calc.pest"]
struct CalcParser;
lazy_static! {
    static ref CALC_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left))
            .op(Op::prefix(neg))

    };
}

pub fn parse_calc(expr: &str) -> Result<Number, String> {
    let mut pairs = CalcParser::parse(Rule::line, expr)
    .map_err(|e| {format!("{e}")})?;
    Ok(parse_expr(pairs.next().unwrap().into_inner()))
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Number {
    CALC_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::pi => std::f64::consts::PI,
            Rule::number => primary.as_str().parse::<Number>().unwrap(),
            Rule::quantity => parse_expr(primary.into_inner()),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::sub => lhs - rhs,
            Rule::mul => lhs * rhs,
            Rule::div => lhs / rhs,
            rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
        })
        .map_prefix(|op, rhs: f64| match op.as_rule() {
            Rule::neg => -rhs,
            _ => unreachable!(),
        })
        .parse(pairs)
}
