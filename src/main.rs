mod cli;
mod parser;
mod sb3;
mod types;

use clap::Parser;
use std::process::ExitCode;
use std::{error::Error, time::Instant};

use crate::cli::{Cli, Command};
use crate::parser::parser::project_parser;

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Run { path } => {
            let path = path.to_str().unwrap();
            let project = match sb3::read_sb3(path) {
                Ok(project) => project,
                Err(err) => {
                    eprintln!("Load error: {err}");
                    let mut source = err.source();
                    while let Some(cause) = source {
                        eprintln!("  cause: {cause}");
                        source = cause.source();
                    }
                    return ExitCode::FAILURE;
                }
            };
            project_parser(project);
        }
        Command::Stats { path } => {
            let path = path.to_str().unwrap();
            let s = Instant::now();
            let project = match sb3::read_sb3(path) {
                Ok(project) => project,
                Err(err) => {
                    eprintln!("Load error: {err}");
                    let mut source = err.source();
                    while let Some(cause) = source {
                        eprintln!("  cause: {cause}");
                        source = cause.source();
                    }
                    return ExitCode::FAILURE;
                }
            };
            println!("File: {}", path);
            println!("Loading Time: {:?}", s.elapsed());
            println!("Block Number: {}", project.count_blocks());
            println!("Using Op Codes: {:?}", project.check_op_codes());
        }
        Command::JSON { path } => {
            let path = path.to_str().unwrap();
            let json = match sb3::read_json(path) {
                Ok(project) => project,
                Err(err) => {
                    eprintln!("Load error: {err}");
                    let mut source = err.source();
                    while let Some(cause) = source {
                        eprintln!("  cause: {cause}");
                        source = cause.source();
                    }
                    return ExitCode::FAILURE;
                }
            };
            println!("{}", json);
        }
    };
    ExitCode::SUCCESS
}
