use std::env;
use std::fs;

use ansi_term::Style;
use gsolve::draw::draw;
use gsolve::order::bfs_order;
use gsolve::parse::parse_document;
use gsolve::solve::brute_solve;
use gsolve::util::print_header;
// use gsolve::solve::dist_solve;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1)
    .ok_or("no file path specified")?;

    let contents = fs::read_to_string(file_path)
    .map_err(|e| {format!("{e}")})?;

    let mut index = parse_document(contents)?;

    print_header("Constraints");
    for c in index.constraints() {
        println!("{}", c);
    }

    let order = bfs_order(&mut index);

    print_header("Constraints by Point");
    for (id, cs) in order.iter().enumerate() {
        println!("{}:", index.get_point(&id));
        for &c in cs {
            println!(" {}", index.get_constraint(c));
        }
    }
    
    let positions = brute_solve(&mut index, order)?;

    print_header("Positions");
    for (point, pos) in positions.iter() {
        println!("{}: {}", point, pos);
    }
    
    print_header("Figure");
    draw(positions);

    Ok(())
}