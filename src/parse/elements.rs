use pest::Parser;

use crate::{constraints::Constraint, elements::{Angle, Distance, Point}, vector::Number};

#[derive(pest_derive::Parser)]
#[grammar = "parse/elements.pest"]
struct ElemParser;

#[derive(Debug)]
pub enum Element {
    Distance(Point, Point),
    Angle(Point, Point, Point),
}
impl Element {
    pub(crate) fn to_constraint(self, value: Number) -> Box<dyn Constraint> {
        match self {
            Element::Distance(p0, p1) => {
                Box::new(Distance {
                    points: [p0, p1],
                    dist: value
                })
            }
            Element::Angle(p0, p1, p2) => {
                Box::new(Angle{
                    points: [p0, p1, p2],
                    measure: value
                })
            },
        }
    }
}

pub fn parse_element(expr: &str) -> Result<Element, String> {
    let parsed = ElemParser::parse(Rule::element, expr)
        .map_err(|e| format!("{e}"))?
        .next()
        .ok_or("no element")?
        .into_inner()
        .next()
        .ok_or("no element")?;
    let rule = parsed.as_rule();
    let points: Vec<&str> = parsed
        .into_inner()
        .map(|x| x.as_str())
        .collect();
    let elem = match rule {
        Rule::dist => {
            debug_assert_eq!(points.len(), 2);
            Element::Distance(
                points[0].to_string(),
                points[1].to_string()
            )
        }
        Rule::angle => {
            debug_assert_eq!(points.len(), 3);
            Element::Angle(
                points[0].to_string(),
                points[1].to_string(),
                points[2].to_string()
            )
        }
        _ => return Err(
            format!("expected element found {rule:?}")
        ),
    };
    Ok(elem)
}
