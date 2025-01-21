use std::fs;
use std::fs::File;
use std::io::Write;

use argparse::ArgumentParser;
use gsolve::draw::draw;
use gsolve::order::bfs_order;
use gsolve::parse::parse_document;
use gsolve::solve::brute_solve;
use gsolve::util::print_header;
// use gsolve::solve::dist_solve;

fn main() -> Result<(), String> {
    let mut file_path = String::new();
    let mut verbose = true;
    let mut output = true;

    {
        let mut parser = ArgumentParser::new();

        parser
            .refer(&mut file_path)
            .add_argument("file_path", argparse::Store, "Path to gcad file")
            .required();

        parser
            .refer(&mut verbose)
            .add_option(
                &["-s", "--silent"],
                argparse::StoreFalse,
                "Hide result print outs",
            )
            .add_option(
                &["-v", "--verbose"],
                argparse::StoreTrue,
                "Print out results",
            );

        parser
            .refer(&mut output)
            .add_option(
                &["--no_out", "--output_none"],
                argparse::StoreFalse,
                "Do not output to a file",
            )
            .add_option(
                &["--csv_out", "--output_csv"],
                argparse::StoreTrue,
                "Output to CSV",
            );

        parser.parse_args_or_exit();
    }

    let contents = fs::read_to_string(file_path).map_err(|e| format!("{e}"))?;

    let mut index = parse_document(contents)?;

    if verbose {
        print_header("Constraints");
        for c in index.constraints() {
            println!("{}", c);
        }
    }

    let order = bfs_order(&mut index);

    if verbose {
        print_header("Constraints by Point");
        for (id, cs) in order.iter().enumerate() {
            println!("{}:", index.get_point(&id));
            for &c in cs {
                println!(" {}", index.get_constraint(c));
            }
        }
    }

    let positions = brute_solve(&mut index, order)?;

    if verbose {
        print_header("Positions");
        for (point, pos) in positions.iter() {
            if verbose {
                println!("{}: {}", point, pos);
            }
        }
    }
    if output {
        let mut csv = File::create("points.csv").map_err(|e| format!("{e}"))?;
        for (point, pos) in positions.iter() {
            csv.write(format!("{}, {}, {}\n", point, pos.x, pos.y).as_bytes())
                .map_err(|e| format!("{e}"))?;
        }
    }

    if verbose {
        print_header("Figure");
        draw(positions, &index);
    }

    Ok(())
}
