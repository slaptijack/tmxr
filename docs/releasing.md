# Releasing

Releases should be:

- reproducible
- tagged
- documented

Each release should include:

- version
- release notes
- significant user-visible changes

Breaking changes should be clearly documented.

## Versioning

`Cargo.toml`'s `version` is bumped by [release-plz](https://release-plz.dev/),
not by hand in feature or fix PRs. release-plz derives the bump and changelog
from conventional-commit PR types (see `CONTRIBUTING.md`) and commits the
result as its own dedicated release PR/commit.

Bump mapping (pre-1.0, so `0.x` semver semantics apply — there is no `major`
bump until 1.0):

| PR type                                              | Bump  |
| ----------------------------------------------------- | ----- |
| `feat`                                                 | minor |
| `fix`, `perf`                                          | patch |
| breaking change (pre-1.0 stand-in for a `major` bump)  | minor |
| `docs`, `refactor`, `test`, `chore`, `build`, `ci`, `revert` | none  |

Feature and fix PRs must not modify `version` in `Cargo.toml`. release-plz
owns that field exclusively.

CI wiring to run release-plz automatically is a separate, not-yet-implemented
follow-up. Until it lands, this section documents the intended policy.
