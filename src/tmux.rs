//! Detection of the `tmux` binary, independent of the CLI layer.

use std::io;
use std::process::{Command, Output};

/// Abstraction over running an external command, so tmux detection can be
/// tested without depending on a real `tmux` installation.
pub trait CommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> io::Result<Output>;
}

/// A `CommandRunner` that actually spawns subprocesses.
pub struct SystemCommandRunner;

impl CommandRunner for SystemCommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> io::Result<Output> {
        Command::new(program).args(args).output()
    }
}

/// The result of checking for a usable `tmux` binary.
#[derive(Debug, PartialEq, Eq)]
pub enum TmuxStatus {
    /// `tmux -V` ran successfully; `version` is its trimmed stdout.
    Available { version: String },
    /// `tmux` could not be found or did not run successfully.
    NotFound,
}

/// Checks whether `tmux` is available by invoking `tmux -V` through the
/// given `runner`.
pub fn check_tmux(runner: &dyn CommandRunner) -> TmuxStatus {
    match runner.run("tmux", &["-V"]) {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            TmuxStatus::Available { version }
        }
        Ok(_) | Err(_) => TmuxStatus::NotFound,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    struct FakeRunner {
        result: io::Result<Output>,
    }

    impl CommandRunner for FakeRunner {
        fn run(&self, _program: &str, _args: &[&str]) -> io::Result<Output> {
            match &self.result {
                Ok(output) => Ok(Output {
                    status: output.status,
                    stdout: output.stdout.clone(),
                    stderr: output.stderr.clone(),
                }),
                Err(e) => Err(io::Error::new(e.kind(), e.to_string())),
            }
        }
    }

    fn success_output(stdout: &str) -> Output {
        Output {
            status: ExitStatus::from_raw(0),
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
        }
    }

    fn failure_output() -> Output {
        Output {
            status: ExitStatus::from_raw(1 << 8),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }

    #[test]
    fn reports_available_when_tmux_runs_successfully() {
        let runner = FakeRunner {
            result: Ok(success_output("tmux 3.4\n")),
        };

        assert_eq!(
            check_tmux(&runner),
            TmuxStatus::Available {
                version: "tmux 3.4".to_string()
            }
        );
    }

    #[test]
    fn reports_not_found_when_spawn_fails() {
        let runner = FakeRunner {
            result: Err(io::Error::new(io::ErrorKind::NotFound, "no such file")),
        };

        assert_eq!(check_tmux(&runner), TmuxStatus::NotFound);
    }

    #[test]
    fn reports_not_found_when_command_exits_unsuccessfully() {
        let runner = FakeRunner {
            result: Ok(failure_output()),
        };

        assert_eq!(check_tmux(&runner), TmuxStatus::NotFound);
    }
}
