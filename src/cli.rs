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
        /// 入力パス
        path: PathBuf,
    },
    /// stats <path>
    Stats {
        /// 入力パス
        path: PathBuf,
    },
}
