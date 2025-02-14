pub mod exprs;
mod parsing;

use crate::{
    math::Number,
    order::PointIndex,
    parse::exprs::{
        constraints::{AnglePolarity, Collinear, Parallel, Perpendicular},
        draw::parse_path,
        elements::{Angle, Distance},
        Constraint,
    },
};

pub fn parse_document(doc: String) -> Result<PointIndex, String> {
    let mut index = PointIndex::default();
    for line in doc.lines() {
        if parse_line(line, &mut index).is_err() {
            return Err(format!("failed to parse line: {line}"));
        }
    }
    Ok(index)
}

pub fn parse_line(mut line: &str, index: &mut PointIndex) -> Result<(), ()> {
    line = line.trim();
    if line.is_empty() {
        return Ok(());
    }
    if let Ok(cs) = parse_constraint(line, index) {
        for c in cs {
            index.add_constraint(c);
        }
        return Ok(());
    }
    if let Some(path) = parse_path(line) {
        index.add_path(path);
        return Ok(());
    }
    if line.starts_with("\"") && line.ends_with("\"") {
        return Ok(());
    }
    Err(())
}

pub fn parse_constraint(
    line: &str,
    index: &mut PointIndex,
) -> Result<Vec<Box<dyn Constraint>>, String> {
    if let Some(parsed) = Parallel::parse(line, index) {
        return Ok(vec![Box::new(parsed)]);
    }
    if let Some(parsed) = Perpendicular::parse(line, index) {
        return Ok(vec![Box::new(parsed)]);
    }
    if let Some(parsed) = Collinear::parse(line, index) {
        return Ok(vec![Box::new(parsed)]);
    }
    if let Some(parsed) = AnglePolarity::parse(line, index) {
        return Ok(vec![Box::new(parsed)]);
    }
    if let Ok(parsed) = parse_equality(line, index) {
        return Ok(parsed);
    }
    Err(format!("failed to parse constraint {line}"))
}

pub fn parse_equality(
    line: &str,
    index: &mut PointIndex,
) -> Result<Vec<Box<dyn Constraint>>, String> {
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
        let expr = expr.trim();
        if let Some(parsed) = Distance::parse(expr, index) {
            elements.push(Element::D(parsed));
            continue;
        }
        if let Some(parsed) = Angle::parse(expr, index) {
            elements.push(Element::A(parsed));
            continue;
        }
        if let Ok(parsed) = expr.parse::<Number>() {
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
