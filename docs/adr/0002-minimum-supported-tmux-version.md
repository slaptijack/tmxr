# 0002. Minimum supported tmux version

## Status

Accepted

## Context

`tmxr doctor` (added in #16) confirms that `tmux` is present, but never
checked whether the installed version is new enough for tmxr to work
correctly. A user on a very old tmux would pass `doctor` and only hit a
confusing failure later. No minimum-version policy existed yet in this
repo.

## Decision

tmxr requires tmux 3.0 or newer. tmux 3.0 (released 2019) is a safe,
widely-available floor: it's present on current LTS distributions and via
Homebrew, without requiring an unusually recent install.

`tmxr doctor` parses the `tmux -V` output and fails with an actionable,
distinct error message when the detected version is older than 3.0 (see
`src/doctor.rs`, `MIN_TMUX_VERSION`).

## Consequences

- Users on tmux older than 3.0 get a clear error from `doctor` instead of
  a confusing failure elsewhere.
- The minimum version is a `(u32, u32)` constant in `src/doctor.rs`;
  raising or lowering it is a one-line change, but should be deliberate —
  raising it can break existing users, and lowering it needs its own
  justification. Either should update this ADR.
- CI verifies tmxr's real (non-faked) tmux invocations against this floor;
  see the "Integration smoke test" section of `docs/testing.md`.
