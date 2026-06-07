use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about = "compile/run/stats cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// run <path>
    Run { path: PathBuf },
    /// compile <path>
    Compile {
        path: PathBuf,
        #[arg(short, long, default_value = "out.ll")]
        output: PathBuf,
    },
    /// stats <path>
    Stats { path: PathBuf },
    /// json <path>
    JSON { path: PathBuf },
}
