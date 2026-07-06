# Development

## Goals

tmxr should be:

- fast
- deterministic
- cross-platform
- easy to understand

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
