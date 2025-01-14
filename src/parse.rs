use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

use crate::{
    constraints::{
        constraints::{Collinear, Parallel},
        elements::{Angle, Distance},
        Constraint,
    },
    math::vector::Number,
};

pub fn parse_document(doc: String) -> Result<Vec<Box<dyn Constraint>>, String> {
    let mut exprs = vec![];
    for line in doc.lines() {
        exprs.append(&mut parse_line(line)?);
    }
    Ok(exprs)
}

pub fn parse_line(line: &str) -> Result<Vec<Box<dyn Constraint>>, String> {
    if let Ok(parsed) = parse_equality(line) {
        return Ok(parsed);
    }
    if let Ok(parsed) = parse_constraint(line) {
        return Ok(vec![parsed]);
    }
    Err(format!("failed to parse line: {line}"))
}

pub fn parse_constraint(line: &str) -> Result<Box<dyn Constraint>, String> {
    if let Ok(parsed) = line.parse::<Collinear>() {
        return Ok(Box::new(parsed));
    }
    Err(format!("failed to parse constraint {line}"))
}

pub fn parse_equality(line: &str) -> Result<Vec<Box<dyn Constraint>>, String> {
    enum Element {
        D(Distance),
        A(Angle),
    }
    if !line.contains("=") {
        return Err("equality does not contain equals sign".to_string());
    }
    let mut value = None;
    let mut elements: Vec<Element> = Vec::new();
    for expr in line.split("=") {
        if let Ok(parsed) = expr.parse::<Distance>() {
            elements.push(Element::D(parsed));
            continue;
        }
        if let Ok(parsed) = expr.parse::<Angle>() {
            elements.push(Element::A(parsed));
            continue;
        }
        if let Ok(parsed) = expr.trim().parse::<Number>() {
            if value.is_some() {
                return Err("multiple values in equality".to_string());
            }
            value = Some(parsed);
            continue;
        }
        return Err(format!("unparsed expr in equality: {expr}"));
    }
    let Some(value) = value else {
        return Err("no value in equality".to_string());
    };
    Ok(elements
        .into_iter()
        .map(|e| -> Box<dyn Constraint> {
            match e {
                Element::D(mut d) => {
                    d.dist = value;
                    Box::new(d)
                }
                Element::A(mut a) => {
                    a.measure = value;
                    Box::new(a)
                }
            }
        })
        .collect())
}

impl FromStr for Distance {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s*\|\s*(\w+)\s*(\w+)\s*\|\s*$").unwrap();
        }
        let captures = RE.captures(s).ok_or(())?;
        Ok(Self {
            points: [captures[1].to_string(), captures[2].to_string()],
            dist: 0.0,
        })
    }
}

impl FromStr for Angle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s*[<∠]\s*(\w+)\s*(\w+)\s*(\w+)\s*$").unwrap();
        }
        let captures = RE.captures(s).ok_or(())?;
        Ok(Self {
            points: [
                captures[1].to_string(),
                captures[2].to_string(),
                captures[3].to_string(),
            ],
            measure: 0.0,
        })
    }
}

impl FromStr for Collinear {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\w+(?:\s*-\s*\w+)+$").unwrap();
        }
        if !RE.is_match(s) {
            return Err(());
        }
        let points = s.split("-").map(|s| s.trim().to_string()).collect();
        Ok(Self { points })
    }
}

impl FromStr for Parallel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(\w+\s*\w+)(?:\s*(∥|(\|\|))\s*(\w+\s*\w+))+$").unwrap();
            static ref SPLIT: Regex = Regex::new(r"∥|(\|\|)").unwrap();
            static ref LINE: Regex = Regex::new(r"\s*(\w+)\s*(\w+)\s*").unwrap();
        }
        if !RE.is_match(s) {
            return Err(());
        }
        let lines = SPLIT
            .split(s)
            .map(|l| {
                LINE.captures(l)
                    .unwrap()
                    .iter()
                    .map(|m| m.unwrap().as_str().to_string())
                    .next_array()
                    .unwrap()
            })
            .collect();
        Ok(Self { lines })
    }
}
