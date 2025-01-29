use std::fs;
use std::fs::File;
use std::io::Write;

use argparse::ArgumentParser;
use gsolve::draw::{self, draw_svg, draw_terminal};
use gsolve::order::bfs_order;
use gsolve::parse::parse_document;
use gsolve::solve::brute_solve;
use gsolve::util::print_header;
// use gsolve::solve::dist_solve;

fn main() -> Result<(), String> {
    let mut file_path = String::new();
    let mut verbose = true;
    let mut output: Option<draw::Output> = None;

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
                argparse::StoreConst(None),
                "Do not output to a file",
            )
            .add_option(
                &["--csv", "--csv_out", "--output_csv"],
                argparse::StoreConst(Some(draw::Output::CSV)),
                "Output to CSV",
            )
            .add_option(
                &["--svg", "--svg_out", "--output_svg"],
                argparse::StoreConst(Some(draw::Output::SVG)),
                "Output to SVG",
            )
            .add_option(
                &[
                    "--term", "--term_out", "--output_term",
                    "--terminal", "--terminal_out", "--output_terminal"
                ],
                argparse::StoreConst(Some(draw::Output::Terminal)),
                "Display in terminal",
            );

        parser.parse_args_or_exit();
    }

    let contents = fs::read_to_string(file_path).map_err(|e| format!("{e}"))?;

    let mut index = parse_document(contents)?;
    index.draw.output = output;

    if verbose {
        print_header("Constraints");
        for c in index.constraints() {
            println!("{:?}", c);
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
    if let Some(output) = index.draw.output {
        match output {
            draw::Output::CSV => {
                let mut csv = File::create("figure.csv").map_err(|e| format!("{e}"))?;
                for (point, pos) in positions.iter() {
                    csv.write(format!("{}, {}, {}\n", point, pos.x, pos.y).as_bytes())
                        .map_err(|e| e.to_string())?;
                }
            }
            draw::Output::SVG => {
                draw_svg(positions, &index).map_err(|e| e.to_string())?;
            }
            draw::Output::Terminal => {
                print_header("Figure");
                draw_terminal(positions, &index);
            }
        }
    }

    Ok(())
}
