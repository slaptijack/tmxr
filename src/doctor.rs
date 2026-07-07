//! Environment diagnostics for tmxr, independent of the CLI layer.

use crate::tmux::{CommandRunner, TmuxStatus, check_tmux, parse_tmux_version};

/// Minimum tmux version (major, minor) that tmxr supports.
const MIN_TMUX_VERSION: (u32, u32) = (3, 0);

/// Runs tmxr's environment checks and returns either a success message or
/// an actionable error message.
pub fn run(runner: &dyn CommandRunner) -> Result<String, String> {
    match check_tmux(runner) {
        TmuxStatus::Available { version } => match parse_tmux_version(&version) {
            Some(parsed) if parsed >= MIN_TMUX_VERSION => Ok(format!("tmux found: {version}")),
            Some((major, minor)) => Err(format!(
                "tmux {major}.{minor} is too old; tmxr requires tmux {}.{} or newer. \
                 Upgrade tmux, e.g. `brew upgrade tmux` (macOS) or your distro's package \
                 manager, then try again.",
                MIN_TMUX_VERSION.0, MIN_TMUX_VERSION.1
            )),
            None => Err(format!(
                "could not parse tmux version from \"{version}\"; tmxr requires tmux \
                 {}.{} or newer.",
                MIN_TMUX_VERSION.0, MIN_TMUX_VERSION.1
            )),
        },
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

    fn version_output(stdout: &str) -> Output {
        Output {
            status: ExitStatus::from_raw(0),
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
        }
    }

    #[test]
    fn success_message_when_version_at_minimum() {
        let runner = FakeRunner {
            result: Ok(version_output("tmux 3.0\n")),
        };

        assert_eq!(run(&runner), Ok("tmux found: tmux 3.0".to_string()));
    }

    #[test]
    fn success_message_when_version_at_minimum_with_patch_letter() {
        let runner = FakeRunner {
            result: Ok(version_output("tmux 3.0a\n")),
        };

        assert_eq!(run(&runner), Ok("tmux found: tmux 3.0a".to_string()));
    }

    #[test]
    fn error_message_when_version_below_minimum() {
        let runner = FakeRunner {
            result: Ok(version_output("tmux 2.9\n")),
        };

        let err = run(&runner).unwrap_err();
        assert!(err.contains("too old"), "message should say too old: {err}");
        assert!(
            err.contains("3.0"),
            "message should name the minimum: {err}"
        );
        assert!(
            !err.contains("not found"),
            "too-old message should be distinct from not-found: {err}"
        );
    }

    #[test]
    fn error_message_when_version_unparseable() {
        let runner = FakeRunner {
            result: Ok(version_output("tmux devel\n")),
        };

        let err = run(&runner).unwrap_err();
        assert!(
            err.contains("could not parse"),
            "message should say the version couldn't be parsed: {err}"
        );
    }
}
