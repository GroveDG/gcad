use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use clap_derive::ValueEnum;
use gsolve::document::{order_bfs, solve_brute, draw_svg, draw_terminal, Document};
use gsolve::util::print_heading;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Output {
    /// Prints figure.
    Terminal,
    /// Saves points as CSV.
    CSV,
    /// Saves figure as SVG.
    SVG,
}
#[derive(Debug, Clone, Default)]
pub struct DrawOptions {
    pub output: Option<Output>,
}

#[derive(Parser)]
struct CLIArgs {
    /// Path to GCAD file.
    file: PathBuf,
    /// Output for solved figure.
    #[arg(value_enum)]
    output: Option<Output>,
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), String> {
    let args = CLIArgs::parse();

    let contents = fs::read_to_string(args.file).map_err(|e| format!("{e}"))?;

    let mut index: Document = contents.parse()?;

    if args.verbose {
        print_heading("Constraints");
        for c in index.constraints() {
            println!("{:?}", c);
        }
    }

    let order = order_bfs(&mut index);

    if args.verbose {
        print_heading("Constraints by Point");
        for (id, cs) in order.iter().enumerate() {
            println!("{}:", index.get_point(&id));
            for &c in cs {
                println!(" {}", index.get_constraint(c));
            }
        }
    }

    let positions = solve_brute(&mut index, order)?;

    if args.verbose {
        print_heading("Positions");
        for (point, pos) in positions.iter() {
            println!("{}: {}", point, pos);
        }
    }
    if let Some(output) = args.output {
        match output {
            Output::CSV => {
                let mut csv = File::create("figure.csv").map_err(|e| format!("{e}"))?;
                for (point, pos) in positions.iter() {
                    csv.write(format!("{}, {}, {}\n", point, pos.x, pos.y).as_bytes())
                        .map_err(|e| e.to_string())?;
                }
            }
            Output::SVG => {
                draw_svg(positions, &index).map_err(|e| e.to_string())?;
            }
            Output::Terminal => {
                print_heading("Figure");
                draw_terminal(positions, &index);
            }
        }
    }

    Ok(())
}
