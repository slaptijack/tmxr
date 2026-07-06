# Contributing

Thanks for your interest in contributing to tmxr!

## Philosophy

tmxr is intentionally opinionated. The goal is not to support every possible
workflow, but to provide a fast, consistent developer experience.

When contributing, prioritize:

- simplicity over cleverness
- deterministic behavior
- reproducibility
- clear user-facing behavior
- comprehensive tests

If a feature adds complexity, it should provide significant user value.

## Development

Clone the repository and use the provided build commands.

All changes should:

- build successfully
- include tests when appropriate
- update documentation if behavior changes

Run `scripts/check.sh` before opening or updating a pull request to run the
same checks as CI.

## Pull Requests

Small pull requests are preferred.

Each PR should:

- solve a single problem
- explain the motivation
- describe the approach
- mention any tradeoffs

Behavioral changes should include examples in the documentation.

### Pull request titles

Pull request titles MUST use:

`<type>(<scope>): <summary>`

Where `<type>` is one of:

- `feat` — user-visible feature
- `fix` — bug fix
- `docs` — documentation-only change
- `refactor` — internal restructuring without behavior change
- `test` — test-only change
- `chore` — maintenance
- `build` — build system or dependency change
- `ci` — GitHub Actions or automation
- `perf` — performance improvement
- `revert` — revert a previous change

`<scope>` should be a short area name, such as:

- `cli`
- `tmux`
- `session`
- `config`
- `docs`
- `release`
- `ci`
- `tests`

The `<summary>` should be imperative, concise, lower-case unless using a proper noun, and should not end with punctuation.

Examples:

- `feat(cli): add session picker command`
- `fix(tmux): handle missing server socket`
- `docs(readme): document installation flow`
- `test(session): cover detached session reuse`
- `ci(github): add pull request checks`
- `chore(release): prepare initial version`

## Code Review

Reviews focus on:

- correctness
- maintainability
- CLI consistency
- test coverage
- long-term maintainability

## Architecture

Please avoid introducing abstractions before they are needed.

A small amount of duplication is often preferable to unnecessary
generalization.

Significant architectural decisions are recorded in
[`docs/adr/`](docs/adr/README.md). Check there before revisiting a settled
question, and propose new decisions there following the same process.
