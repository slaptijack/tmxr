//! Attach-or-create workflow for tmux sessions, independent of the CLI layer.

use std::io;
use std::path::Path;
use std::process::Command;

use crate::tmux::CommandRunner;

/// Derives a stable tmux session name from a directory path.
///
/// Uses the directory's final component, falling back to `"tmxr"` when one
/// isn't available (e.g. the root path). `.` and `:` are replaced with `_`
/// since tmux treats them specially in target names. Deeper naming rules
/// (collisions, sanitizing other characters, project detection) are left
/// to follow-up work.
pub fn derive_session_name(dir: &Path) -> String {
    let raw = dir
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "tmxr".to_string());

    raw.chars()
        .map(|c| if c == '.' || c == ':' { '_' } else { c })
        .collect()
}

/// Whether a tmux session named `name` currently exists.
pub fn session_exists(runner: &dyn CommandRunner, name: &str) -> Result<bool, String> {
    match runner.run("tmux", &["has-session", "-t", name]) {
        Ok(output) => Ok(output.status.success()),
        Err(e) => Err(format!("failed to run tmux: {e}")),
    }
}

/// Creates a detached tmux session named `name` rooted at `dir`.
pub fn create_session(runner: &dyn CommandRunner, name: &str, dir: &Path) -> Result<(), String> {
    let dir_str = dir.to_string_lossy();
    match runner.run("tmux", &["new-session", "-d", "-s", name, "-c", &dir_str]) {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(format!(
            "tmux failed to create session '{name}': {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )),
        Err(e) => Err(format!("failed to run tmux: {e}")),
    }
}

/// Derives the session name for `dir`, creating the session if it doesn't
/// already exist. Returns the session name either way.
pub fn ensure_session(runner: &dyn CommandRunner, dir: &Path) -> Result<String, String> {
    let name = derive_session_name(dir);

    if !session_exists(runner, &name)? {
        create_session(runner, &name, dir)?;
    }

    Ok(name)
}

/// Abstraction over attaching to a tmux session, so the workflow can be
/// tested without taking over the real terminal.
///
/// A successful attach replaces the current process and never returns, so
/// this only returns when the attempt fails.
pub trait SessionAttacher {
    fn attach(&self, name: &str) -> io::Error;
}

/// A `SessionAttacher` that execs the real `tmux attach-session`.
pub struct SystemSessionAttacher;

impl SessionAttacher for SystemSessionAttacher {
    #[cfg(unix)]
    fn attach(&self, name: &str) -> io::Error {
        use std::os::unix::process::CommandExt;
        Command::new("tmux")
            .args(["attach-session", "-t", name])
            .exec()
    }

    #[cfg(not(unix))]
    fn attach(&self, name: &str) -> io::Error {
        match Command::new("tmux")
            .args(["attach-session", "-t", name])
            .status()
        {
            Ok(_) => io::Error::other("tmux attach-session exited"),
            Err(e) => e,
        }
    }
}

/// Runs the full attach-or-create workflow for `dir`: ensures a session
/// exists, then attaches to it. Only returns on failure.
pub fn run(
    runner: &dyn CommandRunner,
    attacher: &dyn SessionAttacher,
    dir: &Path,
) -> Result<(), String> {
    let name = ensure_session(runner, dir)?;
    Err(format!(
        "failed to attach to session '{name}': {}",
        attacher.attach(&name)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::os::unix::process::ExitStatusExt;
    use std::process::{ExitStatus, Output};

    fn success_output() -> Output {
        Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }

    fn failure_output(stderr: &str) -> Output {
        Output {
            status: ExitStatus::from_raw(1 << 8),
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }

    #[test]
    fn derive_session_name_uses_final_path_component() {
        assert_eq!(derive_session_name(Path::new("/home/user/tmxr")), "tmxr");
    }

    #[test]
    fn derive_session_name_replaces_special_characters() {
        assert_eq!(
            derive_session_name(Path::new("/home/user/my.project:v2")),
            "my_project_v2"
        );
    }

    #[test]
    fn derive_session_name_falls_back_when_no_file_name() {
        assert_eq!(derive_session_name(Path::new("/")), "tmxr");
    }

    /// A `CommandRunner` that plays back a scripted queue of responses and
    /// records the args it was called with, so multi-call workflows like
    /// `ensure_session` can be tested.
    struct ScriptedRunner {
        responses: RefCell<Vec<io::Result<Output>>>,
        calls: RefCell<Vec<Vec<String>>>,
    }

    impl ScriptedRunner {
        fn new(responses: Vec<io::Result<Output>>) -> Self {
            Self {
                responses: RefCell::new(responses),
                calls: RefCell::new(Vec::new()),
            }
        }

        fn call_count(&self) -> usize {
            self.calls.borrow().len()
        }
    }

    impl CommandRunner for ScriptedRunner {
        fn run(&self, _program: &str, args: &[&str]) -> io::Result<Output> {
            self.calls
                .borrow_mut()
                .push(args.iter().map(|s| s.to_string()).collect());

            let mut responses = self.responses.borrow_mut();
            if responses.is_empty() {
                panic!("ScriptedRunner called more times than it has scripted responses");
            }
            match responses.remove(0) {
                Ok(output) => Ok(output),
                Err(e) => Err(io::Error::new(e.kind(), e.to_string())),
            }
        }
    }

    #[test]
    fn session_exists_true_when_has_session_succeeds() {
        let runner = ScriptedRunner::new(vec![Ok(success_output())]);
        assert_eq!(session_exists(&runner, "tmxr"), Ok(true));
    }

    #[test]
    fn session_exists_false_when_has_session_fails() {
        let runner = ScriptedRunner::new(vec![Ok(failure_output(""))]);
        assert_eq!(session_exists(&runner, "tmxr"), Ok(false));
    }

    #[test]
    fn session_exists_errors_when_tmux_cannot_run() {
        let runner = ScriptedRunner::new(vec![Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no such file",
        ))]);
        assert!(session_exists(&runner, "tmxr").is_err());
    }

    #[test]
    fn create_session_ok_when_new_session_succeeds() {
        let runner = ScriptedRunner::new(vec![Ok(success_output())]);
        assert_eq!(create_session(&runner, "tmxr", Path::new("/tmp")), Ok(()));
    }

    #[test]
    fn create_session_surfaces_stderr_on_failure() {
        let runner = ScriptedRunner::new(vec![Ok(failure_output("duplicate session"))]);
        let err = create_session(&runner, "tmxr", Path::new("/tmp")).unwrap_err();
        assert!(err.contains("duplicate session"));
        assert!(err.contains("tmxr"));
    }

    #[test]
    fn create_session_errors_when_tmux_cannot_run() {
        let runner = ScriptedRunner::new(vec![Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no such file",
        ))]);
        assert!(create_session(&runner, "tmxr", Path::new("/tmp")).is_err());
    }

    #[test]
    fn ensure_session_reuses_existing_session_without_creating() {
        let runner = ScriptedRunner::new(vec![Ok(success_output())]);
        let name = ensure_session(&runner, Path::new("/home/user/tmxr")).unwrap();
        assert_eq!(name, "tmxr");
        assert_eq!(runner.call_count(), 1, "should not call new-session");
    }

    #[test]
    fn ensure_session_creates_when_missing() {
        let runner = ScriptedRunner::new(vec![Ok(failure_output("")), Ok(success_output())]);
        let name = ensure_session(&runner, Path::new("/home/user/tmxr")).unwrap();
        assert_eq!(name, "tmxr");
        let calls = runner.calls.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0][0], "has-session");
        assert_eq!(calls[1][0], "new-session");
    }

    #[test]
    fn ensure_session_propagates_tmux_failure() {
        let runner = ScriptedRunner::new(vec![Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no such file",
        ))]);
        assert!(ensure_session(&runner, Path::new("/home/user/tmxr")).is_err());
    }

    struct FakeAttacher {
        attached_to: RefCell<Option<String>>,
    }

    impl FakeAttacher {
        fn new() -> Self {
            Self {
                attached_to: RefCell::new(None),
            }
        }
    }

    impl SessionAttacher for FakeAttacher {
        fn attach(&self, name: &str) -> io::Error {
            *self.attached_to.borrow_mut() = Some(name.to_string());
            io::Error::other("fake attach failure")
        }
    }

    #[test]
    fn run_attaches_to_existing_session() {
        let runner = ScriptedRunner::new(vec![Ok(success_output())]);
        let attacher = FakeAttacher::new();

        let err = run(&runner, &attacher, Path::new("/home/user/tmxr")).unwrap_err();

        assert_eq!(*attacher.attached_to.borrow(), Some("tmxr".to_string()));
        assert!(err.contains("tmxr"));
    }

    #[test]
    fn run_creates_then_attaches_when_session_missing() {
        let runner = ScriptedRunner::new(vec![Ok(failure_output("")), Ok(success_output())]);
        let attacher = FakeAttacher::new();

        run(&runner, &attacher, Path::new("/home/user/tmxr")).unwrap_err();

        assert_eq!(*attacher.attached_to.borrow(), Some("tmxr".to_string()));
    }

    #[test]
    fn run_short_circuits_before_attaching_on_tmux_failure() {
        let runner = ScriptedRunner::new(vec![Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no such file",
        ))]);
        let attacher = FakeAttacher::new();

        let err = run(&runner, &attacher, Path::new("/home/user/tmxr")).unwrap_err();

        assert!(attacher.attached_to.borrow().is_none());
        assert!(err.contains("failed to run tmux"));
    }
}
