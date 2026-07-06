# 0001. Use Cargo as the build system

## Status

Accepted

## Context

tmxr is a cross-platform Rust CLI. It needs a canonical build system that
contributors can rely on for building, testing, formatting, and linting.
Some Rust projects adopt a secondary build system such as Bazel on top of
Cargo, typically to unify builds across polyglot monorepos or to get more
granular, cacheable build graphs at large scale.

tmxr is a single-crate Rust project distributed as a CLI. It has none of
the polyglot or monorepo-scale concerns that motivate adopting Bazel, and
requiring one would add setup and maintenance overhead disproportionate to
the project's size.

## Decision

Use Cargo as the sole, canonical build system for tmxr. Bazel (or any
other build system) is intentionally not required.

## Consequences

- Contributors only need standard Rust tooling (`cargo`, `rustc`) to clone
  the repo and build, test, or lint it — no additional project
  infrastructure to install or learn.
- CI relies on standard Cargo-based checks: `cargo fmt --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cargo test --all-targets --all-features` (see `docs/development.md`).
- If tmxr ever grows into a polyglot or multi-crate monorepo with build
  performance or caching needs Cargo can't meet, this decision would need
  to be revisited via a new ADR rather than assumed to still hold.
