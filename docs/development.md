# Development

## Goals

tmxr should be:

- fast
- deterministic
- cross-platform
- easy to understand

## Build system

tmxr uses Cargo as the canonical build system.

Bazel is intentionally not required for this repository. Contributors should
be able to clone the repo and use standard Rust tooling without additional
project infrastructure.

Required checks:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets --all-features`
- `shellcheck` for shell scripts

Run `scripts/check.sh` to perform all of the above locally in one step.

## Principles

Prefer:

- explicit code
- small functions
- minimal dependencies
- stable interfaces

Avoid:

- hidden magic
- implicit behavior
- unnecessary configuration

## Testing

Every bug fix should include a regression test.

Tests should:

- run without network access
- avoid machine-specific assumptions
- be deterministic

## CLI

User-facing commands should:

- produce helpful error messages
- return meaningful exit codes
- support automation
- avoid breaking existing behavior
