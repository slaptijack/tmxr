//! Shared test doubles for anything that shells out via `CommandRunner`.
//! Only compiled for tests.

use std::cell::RefCell;
use std::io;
use std::os::unix::process::ExitStatusExt;
use std::process::{ExitStatus, Output};

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
