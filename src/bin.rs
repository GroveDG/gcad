use std::env;
use std::fs;

use draw::render;
use draw::SvgRenderer;
use gsolve::draw::draw;
use gsolve::math::vector::bounding_box;
use gsolve::order::bfs_order;
use gsolve::order::PointIndex;
use gsolve::parse::parse_document;
use gsolve::solve::dist_solve::dist_solve;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1)
    .ok_or("no file path specified")?;

    let contents = fs::read_to_string(file_path)
    .map_err(|e| {format!("{e}")})?;

    let doc = parse_document(contents)?;

    for c in doc.iter() {
        println!("{}", c);
    }

    let index = PointIndex::from_constraints(doc);
    let order = bfs_order(&index);

    let positions = dist_solve(order)?;

    for (point, pos) in positions.iter() {
        println!("{}: {}", point, pos);
    }

    let canvas = draw(positions, 96.0);

    render::save(
        &canvas,
        "figure.svg",
        SvgRenderer::new()
    ).or(Err("render failed".to_string()))?;

    Ok(())
}