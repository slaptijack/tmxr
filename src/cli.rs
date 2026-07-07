//! Command-line argument parsing, kept separate from execution.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tmxr", about = "Opinionated tmux workspace launcher")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check the local environment for tmxr's requirements.
    Doctor,
}
