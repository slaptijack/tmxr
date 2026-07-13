//! Shared test doubles for anything that shells out via `CommandRunner`, or
//! reads config via `ConfigLocator`. Only compiled for tests.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Output};

use crate::config::ConfigLocator;
use crate::tmux::CommandRunner;

pub fn success_output() -> Output {
    Output {
        status: ExitStatus::from_raw(0),
        stdout: Vec::new(),
        stderr: Vec::new(),
    }
}

pub fn failure_output(stderr: &str) -> Output {
    Output {
        status: ExitStatus::from_raw(1 << 8),
        stdout: Vec::new(),
        stderr: stderr.as_bytes().to_vec(),
    }
}

/// A `CommandRunner` that plays back a scripted queue of responses and
/// records the args it was called with, so multi-call workflows can be
/// tested.
pub struct ScriptedRunner {
    responses: RefCell<Vec<io::Result<Output>>>,
    calls: RefCell<Vec<Vec<String>>>,
}

impl ScriptedRunner {
    pub fn new(responses: Vec<io::Result<Output>>) -> Self {
        Self {
            responses: RefCell::new(responses),
            calls: RefCell::new(Vec::new()),
        }
    }

    pub fn call_count(&self) -> usize {
        self.calls.borrow().len()
    }

    pub fn calls(&self) -> Vec<Vec<String>> {
        self.calls.borrow().clone()
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

/// An in-memory `ConfigLocator` for hermetic discovery/parsing tests,
/// avoiding touching the real filesystem or `$HOME`.
pub struct FakeConfigLocator {
    files: RefCell<HashMap<PathBuf, String>>,
    home: Option<PathBuf>,
}

impl FakeConfigLocator {
    pub fn new(home: Option<&str>) -> Self {
        Self {
            files: RefCell::new(HashMap::new()),
            home: home.map(PathBuf::from),
        }
    }

    pub fn with_file(self, path: &str, contents: &str) -> Self {
        self.files
            .borrow_mut()
            .insert(PathBuf::from(path), contents.to_string());
        self
    }
}

impl ConfigLocator for FakeConfigLocator {
    fn exists(&self, path: &Path) -> bool {
        self.files.borrow().contains_key(path)
    }

    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .borrow()
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no such file"))
    }

    fn home_dir(&self) -> Option<PathBuf> {
        self.home.clone()
    }
}
