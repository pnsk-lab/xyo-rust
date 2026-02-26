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
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

fn run(cli: Cli) -> Result<(), ()> {
    match cli.command {
        Command::Run { path } => {
            let project = handle_error(sb3::read_sb3(&path), "Load error")?;
            handle_error(project_parser(&project), "Parse error")?;
        }
        Command::Stats { path } => {
            let s = Instant::now();
            let project = handle_error(sb3::read_sb3(&path), "Load error")?;
            println!("File: {}", path.display());
            println!("Loading Time: {:?}", s.elapsed());
            println!("Block Number: {}", project.count_blocks());
            println!("Using Op Codes: {:?}", project.check_op_codes());
        }
        Command::JSON { path } => {
            let json = handle_error(sb3::read_json(&path), "Load error")?;
            println!("{}", json);
        }
    };
    Ok(())
}

fn handle_error<T, E>(result: Result<T, E>, prefix: &str) -> Result<T, ()>
where
    E: Error,
{
    result.map_err(|err| report_error(prefix, &err))
}

fn report_error(prefix: &str, err: &dyn Error) {
    eprintln!("{prefix}: {err}");
    let mut source = err.source();
    while let Some(cause) = source {
        eprintln!("  cause: {cause}");
        source = cause.source();
    }
}
