use std::process::ExitCode;

use clap::Parser;
use tmxr::cli::{Cli, Commands};
use tmxr::tmux::SystemCommandRunner;

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Doctor) => match tmxr::doctor::run(&SystemCommandRunner) {
            Ok(message) => {
                println!("{message}");
                ExitCode::SUCCESS
            }
            Err(message) => {
                eprintln!("{message}");
                ExitCode::FAILURE
            }
        },
        None => {
            println!("{}", tmxr::greeting());
            ExitCode::SUCCESS
        }
    }
}
