//! Environment diagnostics for tmxr, independent of the CLI layer.

use crate::tmux::{CommandRunner, TmuxStatus, check_tmux};

/// Runs tmxr's environment checks and returns either a success message or
/// an actionable error message.
pub fn run(runner: &dyn CommandRunner) -> Result<String, String> {
    match check_tmux(runner) {
        TmuxStatus::Available { version } => Ok(format!("tmux found: {version}")),
        TmuxStatus::NotFound => Err(
            "tmux not found in PATH. Install it, e.g. `brew install tmux` (macOS) or \
             `apt install tmux` (Debian/Ubuntu), then try again."
                .to_string(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::os::unix::process::ExitStatusExt;
    use std::process::{ExitStatus, Output};

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

    #[test]
    fn success_message_includes_tmux_version() {
        let runner = FakeRunner {
            result: Ok(Output {
                status: ExitStatus::from_raw(0),
                stdout: b"tmux 3.4\n".to_vec(),
                stderr: Vec::new(),
            }),
        };

        assert_eq!(run(&runner), Ok("tmux found: tmux 3.4".to_string()));
    }

    #[test]
    fn error_message_is_actionable_when_tmux_missing() {
        let runner = FakeRunner {
            result: Err(io::Error::new(io::ErrorKind::NotFound, "no such file")),
        };

        let err = run(&runner).unwrap_err();
        assert!(err.contains("tmux not found"));
        assert!(
            err.contains("Install"),
            "message should suggest a fix: {err}"
        );
    }
}
