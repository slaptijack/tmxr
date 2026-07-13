# Testing

Every new feature should include tests.

Tests should verify:

- expected behavior
- edge cases
- error handling
- regressions

Tests should be:

- deterministic
- hermetic when practical
- independent of developer machines

Avoid tests that depend on:

- timing
- network connectivity
- local configuration

## Integration smoke test

`scripts/tmux-smoke-test.sh` drives the real `tmxr` binary against a real
`tmux` on `PATH`, exercising the actual `CommandRunner`/`SessionAttacher`
implementations instead of the fakes used everywhere else. It's
intentionally kept outside `cargo test`/`scripts/check.sh`: it needs a real
tmux binary and spawns real tmux sessions, so it can't meet the hermeticity
rules above.

CI runs it in a separate workflow
(`.github/workflows/tmux-smoke.yml`) against tmux 3.0a, tmxr's documented
minimum supported version (see
`docs/adr/0002-minimum-supported-tmux-version.md`), built from source and
cached. This catches tmux invocations that only work on newer tmux before
they reach a user still on the 3.0 floor.

To run it locally, build tmxr and have any tmux 3.0+ on `PATH`, then run
`scripts/tmux-smoke-test.sh` (it warns, but doesn't fail, if your local tmux
isn't exactly 3.0.x — only CI's pinned build enforces the exact floor).
