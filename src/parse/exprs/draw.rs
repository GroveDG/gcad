use crate::{
    draw::PathCmd,
    parse::parsing::{literal, space, word},
};

pub fn parse_path(mut input: &str) -> Option<Vec<PathCmd>> {
    let mut points = Vec::new();
    let mut cmds = Vec::new();
    let mut term = true;
    loop {
        points.push(word(&mut input)?);
        if term {
            cmds.push(match points.len() {
                1 => PathCmd::Line(points[0].to_string()),
                2 => PathCmd::Quadratic(points[0].to_string(), points[1].to_string()),
                3 => PathCmd::Cubic(
                    points[0].to_string(),
                    points[1].to_string(),
                    points[2].to_string(),
                ),
                _ => return None,
            });
            points.clear();
        }
        term = if literal("→")(&mut input)
            .or(literal("->")(&mut input))
            .is_some()
        {
            true
        } else if literal("-")(&mut input).is_some() {
            false
        } else {
            break;
        };
        space(&mut input);
    }

    if !points.is_empty() {
        return None;
    }

    // Starting M (Move) command
    cmds[0] = match &cmds[0] {
        PathCmd::Line(p) => PathCmd::Move(p.clone()),
        _ => unreachable!(),
    };

    Some(cmds)
}
