use crate::{
    constraints::{
        constraints::{Collinear, Parallel, Perpendicular, AnglePolarity},
        elements::{Angle, Distance},
        Constraint,
    },
    math::vector::Number, order::PointIndex,
};

pub fn parse_document(doc: String) -> Result<PointIndex, String> {
    let mut index = PointIndex::default();
    for line in doc.lines() {
        for c in parse_line(line, &mut index)? {
            index.add_constraint(c);
        }
    }
    Ok(index)
}

pub fn parse_line(mut line: &str, index: &mut PointIndex) -> Result<Vec<Box<dyn Constraint>>, String> {
    line = line.trim();
    if line.is_empty() {
        return Ok(Vec::new());
    }
    if let Ok(parsed) = parse_equality(line, index) {
        return Ok(parsed);
    }
    if let Ok(parsed) = parse_constraint(line, index) {
        return Ok(vec![parsed]);
    }
    Err(format!("failed to parse line: {line}"))
}

pub fn parse_constraint(line: &str, index: &mut PointIndex) -> Result<Box<dyn Constraint>, String> {
    if let Ok(parsed) = Parallel::parse(line, index) {
        return Ok(Box::new(parsed));
    }
    if let Ok(parsed) = Perpendicular::parse(line, index) {
        return Ok(Box::new(parsed));
    }
    if let Ok(parsed) = Collinear::parse(line, index) {
        return Ok(Box::new(parsed));
    }
    if let Ok(parsed) = AnglePolarity::parse(line, index) {
        return Ok(Box::new(parsed));
    }
    Err(format!("failed to parse constraint {line}"))
}

pub fn parse_equality(line: &str, index: &mut PointIndex) -> Result<Vec<Box<dyn Constraint>>, String> {
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
        if let Ok(parsed) = Distance::parse(expr, index) {
            elements.push(Element::D(parsed));
            continue;
        }
        if let Ok(parsed) = Angle::parse(expr, index) {
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