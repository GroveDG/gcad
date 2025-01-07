use calc::parse_calc;
use elements::parse_element;

use crate::constraints::Constraint;

mod calc;
mod elements;

pub fn parse_document(doc: String) -> Result<Vec<Box<dyn Constraint>>, String> {
    let mut exprs = vec![];
    for line in doc.lines() {
        exprs.append(&mut parse_line(line)?);
    }
    Ok(exprs)
}

pub fn parse_line(line: &str) -> Result<Vec<Box<dyn Constraint>>, String> {
    if let Ok(parsed) = parse_equality(line) {
        return Ok(parsed)
    }
    Err(format!("failed to parse line: {line}"))
}

pub fn parse_equality(line: &str) -> Result<Vec<Box<dyn Constraint>>, String> {
    if !line.contains("=") {
        return Err("equality does not contain equal sign".to_string());
    }
    let mut value = None;
    let mut elements = vec![];
    for expr in line.split("=") {
        if let Ok(parsed) = parse_element(expr) {
            elements.push(parsed);
        }
        else if let Ok(parsed) = parse_calc(expr) {
            if value.is_some() {
                return Err("equality contains multiple values".to_string());
            }
            value = Some(parsed);
        } else {
            return Err(format!("invalid expression in equality {expr}"));
        }
    }
    if let Some(value) = value {
        Ok(elements.into_iter().map(|e| e.to_constraint(value)).collect())
    } else {
        Err("no value in equality".to_string())
    }
}