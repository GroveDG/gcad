use crate::{
    constraints::{
        constraints::{AnglePolarity, Collinear, Parallel, Perpendicular},
        elements::{Angle, Distance},
        Constraint,
    },
    draw::parse_path,
    math::Number,
    order::PointIndex,
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
    if let Ok(path) = parse_path(line) {
        index.add_path(path);
        return Ok(());
    }
    if line.trim_start().starts_with("\"") && line.trim_end().ends_with("\"") {
        return Ok(());
    }
    Err(())
}

pub fn parse_constraint(
    line: &str,
    index: &mut PointIndex,
) -> Result<Vec<Box<dyn Constraint>>, String> {
    if let Ok(parsed) = Parallel::parse(line, index) {
        return Ok(vec![Box::new(parsed)]);
    }
    if let Ok(parsed) = Perpendicular::parse(line, index) {
        return Ok(vec![Box::new(parsed)]);
    }
    if let Ok(parsed) = Collinear::parse(line, index) {
        return Ok(vec![Box::new(parsed)]);
    }
    if let Ok(parsed) = AnglePolarity::parse(line, index) {
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

use nom::{
    bytes::complete::take_while1, character::complete::space0, combinator::verify,
    error::ParseError, multi::separated_list0, sequence::delimited, IResult, Parser,
};

pub fn ws<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(space0, inner, space0)
}

pub fn ident(s: &str) -> IResult<&str, &str, ()> {
    take_while1(char::is_alphabetic)(s)
}

pub fn list_len<I, O, O2, E, F, G>(
    sep: G,
    f: F,
    len: usize,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + nom::InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    verify(separated_list0(sep, f), move |l: &Vec<O>| l.len() == len)
}

pub fn separated_listn<I, O, O2, E, F, G>(
    sep: G,
    f: F,
    len: usize,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + nom::InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    verify(separated_list0(sep, f), move |l: &Vec<O>| l.len() >= len)
}

pub fn flag<I, O, O2, E, F, G>(mut f: F, mut g: G) -> impl FnMut(I) -> IResult<I, bool, E>
where
    I: Clone,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    move |s| {
        if let Ok((i, _)) = f.parse(s.clone()) {
            return IResult::Ok((i, true));
        }
        g.parse(s).map(|(i, _)| (i, false))
    }
}

pub fn opt_flag<I, O, E, F>(mut f: F) -> impl FnMut(I) -> IResult<I, bool, E>
where
    I: Clone,
    F: Parser<I, O, E>,
    E: ParseError<I>,
{
    move |s| {
        IResult::Ok(if let Ok((i, _)) = f.parse(s.clone()) {
            (i, true)
        } else {
            (s, false)
        })
    }
}
