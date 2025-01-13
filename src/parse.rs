use crate::{
    constraints::{
        constraints::Collinear, elements::{Angle, Distance}, Constraint
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
    let mut elements: Vec<Element> = vec![];
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
        .collect::<Vec<Box<dyn Constraint>>>())
}
