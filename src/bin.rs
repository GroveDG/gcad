use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use gsolve::draw::{self, draw_svg, draw_terminal};
use gsolve::order::bfs_order;
use gsolve::parse::parse_document;
use gsolve::solve::brute_solve;
use gsolve::util::print_heading;

#[derive(Parser)]
struct CLIArgs {
    /// Path to GCAD file.
    file: PathBuf,
    /// Output for solved figure.
    #[arg(value_enum)]
    output: Option<draw::Output>,
    #[arg(short, long)]
    verbose: bool
}

fn main() -> Result<(), String> {
    let args = CLIArgs::parse();

    // {
    //     let mut parser = ArgumentParser::new();

    //     parser
    //         .refer(&mut file_path)
    //         .add_argument("file_path", argparse::Store, "Path to gcad file")
    //         .required();

    //     parser
    //         .refer(&mut verbose)
    //         .add_option(
    //             &["-s", "--silent"],
    //             argparse::StoreFalse,
    //             "Hide result print outs",
    //         )
    //         .add_option(
    //             &["-v", "--verbose"],
    //             argparse::StoreTrue,
    //             "Print out results",
    //         );

    //     parser
    //         .refer(&mut output)
    //         .add_option(
    //             &["--no_out", "--output_none"],
    //             argparse::StoreConst(None),
    //             "Do not output to a file",
    //         )
    //         .add_option(
    //             &["--csv", "--csv_out", "--output_csv"],
    //             argparse::StoreConst(Some(draw::Output::CSV)),
    //             "Output to CSV",
    //         )
    //         .add_option(
    //             &["--svg", "--svg_out", "--output_svg"],
    //             argparse::StoreConst(Some(draw::Output::SVG)),
    //             "Output to SVG",
    //         )
    //         .add_option(
    //             &[
    //                 "--term", "--term_out", "--output_term",
    //                 "--terminal", "--terminal_out", "--output_terminal"
    //             ],
    //             argparse::StoreConst(Some(draw::Output::Terminal)),
    //             "Display in terminal",
    //         );

    //     parser.parse_args_or_exit();
    // }

    let contents = fs::read_to_string(args.file).map_err(|e| format!("{e}"))?;

    let mut index = parse_document(contents)?;
    index.draw.output = args.output;

    if args.verbose {
        print_heading("Constraints");
        for c in index.constraints() {
            println!("{:?}", c);
        }
    }

    let order = bfs_order(&mut index);

    if args.verbose {
        print_heading("Constraints by Point");
        for (id, cs) in order.iter().enumerate() {
            println!("{}:", index.get_point(&id));
            for &c in cs {
                println!(" {}", index.get_constraint(c));
            }
        }
    }

    let positions = brute_solve(&mut index, order)?;

    if args.verbose {
        print_heading("Positions");
        for (point, pos) in positions.iter() {
            println!("{}: {}", point, pos);
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
                print_heading("Figure");
                draw_terminal(positions, &index);
            }
        }
    }

    Ok(())
}
