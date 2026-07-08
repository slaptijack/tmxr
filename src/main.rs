use std::process::ExitCode;

use clap::Parser;
use tmxr::cli::{Cli, Commands};
use tmxr::session::SystemSessionAttacher;
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
        Some(Commands::Start) | None => start_session(),
        Some(Commands::Completions { shell }) => {
            tmxr::completions::write(shell, &mut std::io::stdout());
            ExitCode::SUCCESS
        }
    }
}

fn start_session() -> ExitCode {
    let dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("failed to determine the current directory: {e}");
            return ExitCode::FAILURE;
        }
    };

    match tmxr::session::run(&SystemCommandRunner, &SystemSessionAttacher, &dir) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
