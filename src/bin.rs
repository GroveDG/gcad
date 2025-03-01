use std::env::args;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use bimap::BiMap;
use clap::Parser;
use clap_derive::ValueEnum;
use draw::{draw_svg, draw_terminal};
use gsolve::{Figure, PID};
use inquire::validator::Validation;
use inquire::{CustomUserError, Select, Text};
use parse::PathCmd;

mod draw;
mod parse;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Output {
    /// Prints figure.
    Terminal,
    /// Saves points as CSV.
    CSV,
    /// Saves figure as SVG.
    SVG,
}
impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Output::Terminal => "Terminal",
            Output::CSV => "CSV",
            Output::SVG => "SVG",
        })
    }
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

#[derive(Debug, Clone, Default)]
pub struct GCADFigure {
    pub fig: Figure,
    points: BiMap<String, PID>,
    paths: Vec<Vec<PathCmd>>,
}

fn validate_file(input: &str) -> Result<Validation, CustomUserError> {
    if Path::new(input).is_file() {
        Ok(Validation::Valid)
    } else {
        Ok(inquire::validator::Validation::Invalid(
            "File does not exist.".into(),
        ))
    }
}

fn main() -> Result<(), String> {
    let args = if args().len() == 1 {
        let file = Text::new("File:")
            .with_validator(validate_file)
            .prompt()
            .map_err(|e| e.to_string())?
            .into();
        let output = Select::new("Output:", vec![Output::SVG, Output::Terminal, Output::CSV])
            .prompt_skippable()
            .map_err(|e| e.to_string())?;
        CLIArgs {
            file,
            output,
            verbose: false,
        }
    } else {
        CLIArgs::parse()
    };

    let contents = fs::read_to_string(args.file).map_err(|e| format!("{e}"))?;

    let mut fig: GCADFigure = contents.parse()?;
    let positions = fig.solve()?;

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
                draw_svg(positions, &fig).map_err(|e| e.to_string())?;
            }
            Output::Terminal => {
                print_heading("Figure");
                draw_terminal(positions, &fig);
            }
        }
    }

    Ok(())
}

pub fn print_heading(s: &str) {
    let style = { ansi_term::Style::new().underline() };
    println!(
        "\n\n{}\n",
        style.paint(
            [
                s,
                " ".repeat(term_size::dimensions().unwrap().0 - s.len())
                    .as_str(),
            ]
            .concat()
        )
    );
}
