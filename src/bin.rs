use std::env;
use std::fs;

use gsolve::parse::parse_document;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1)
    .ok_or("no file path specified")?;

    let contents = fs::read_to_string(file_path)
    .map_err(|e| {format!("{e}")})?;

    let doc = parse_document(contents)?;

    println!("{:?}", doc);

    Ok(())
}