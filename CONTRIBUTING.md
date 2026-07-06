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

## Pull Requests

Small pull requests are preferred.

Each PR should:

- solve a single problem
- explain the motivation
- describe the approach
- mention any tradeoffs

Behavioral changes should include examples in the documentation.

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
