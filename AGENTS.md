# AGENTS

This repository welcomes contributions from both humans and AI coding agents.

## Instruction precedence

When working in this repository, follow instructions in this order:

1. System / platform instructions
2. `AGENTS.md`
3. `CONTRIBUTING.md`
4. Relevant files under `docs/`
5. Existing code and tests

## Required reading

Before making any changes, read these documents in order:

1. `CONTRIBUTING.md`
2. `docs/architecture.md`
3. `docs/development.md`
4. `docs/testing.md`

When modifying release tooling or CI, also read:

- `docs/releasing.md`

These documents are considered part of the repository instructions.

## Goals

Agents should optimize for:

- correctness
- readability
- maintainability
- minimal changes

## Expectations

Prefer:

- fixing root causes
- updating tests
- updating documentation
- preserving existing behavior

Avoid:

- unrelated refactoring
- unnecessary dependency changes
- speculative improvements

## Pull Requests

Each PR should:

- explain what changed
- explain why
- describe testing performed

## Architecture

Follow the existing architecture before introducing new abstractions.

When in doubt, choose the simpler implementation.
