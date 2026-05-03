use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about = "run/stats cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// run <path>
    Run {
        path: PathBuf,
        #[arg(long)]
        emit_llvm: Option<PathBuf>,
    },
    /// stats <path>
    Stats { path: PathBuf },
    /// json <path>
    JSON { path: PathBuf },
}
