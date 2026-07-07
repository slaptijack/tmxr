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

The `release-plz` GitHub Actions workflow
(`.github/workflows/release-plz.yml`) runs this automatically on every push
to `main`, via two jobs:

- `release-plz-pr` opens or updates a release PR with the computed version
  bump and changelog.
- `release-plz-release` creates the git tag and GitHub Release once that
  release PR is merged. On ordinary feature/fix pushes, where the version
  hasn't changed, this job is a no-op.

`release-plz-release` also runs `cargo publish` as part of creating the
GitHub Release, authenticated via the `CARGO_REGISTRY_TOKEN` repository
secret. `release-plz-pr` only opens/updates the release PR and does not
publish anything.

### crates.io token

`CARGO_REGISTRY_TOKEN` is a crates.io API token scoped to publish the
`tmxr` crate, stored as a GitHub Actions repository secret. To rotate it:

1. Generate a new token on crates.io (Account Settings → API Tokens).
2. Update the `CARGO_REGISTRY_TOKEN` repository secret in GitHub with the
   new value.
3. Revoke the old token on crates.io.
