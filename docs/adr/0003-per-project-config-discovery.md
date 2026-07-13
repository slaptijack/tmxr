# 0003. Per-project config for post-create session setup

## Status

Accepted

## Context

`bin/t`, the legacy launcher `tmxr` supersedes, hardcodes a fixed
post-create layout for its one session: split a pane and run `htop` in
it. `tmxr`'s own session workflow (`src/session.rs`, added in #33) only
ever creates a bare session — no panes, no post-create commands.

Issue #36 asks for a way to express `bin/t`'s behavior (and layouts like
it) as configuration instead of hardcoding it, scoped per-project rather
than globally. tmux's own `session-created` hook in `~/.tmux.conf`
already covers a single global default layout for every session, tmxr-
created or not — that case doesn't need a tmxr feature. What it doesn't
cover is a different layout per project directory, which is only
reachable in tmux.conf via `if-shell` chains keyed on
`#{pane_current_path}` — effectively hand-rolling a config system inside
tmux's own scripting. This ADR is about closing that per-project gap.

## Decision

- **Format**: a TOML file named `.tmxr.toml`, parsed with the `toml` and
  `serde` crates (new dependencies — the project otherwise favors
  minimal dependencies, but hand-rolling a config parser for a stable,
  widely-used format isn't a good tradeoff here).
- **Schema**: an ordered list of tagged commands (`split`, `select-pane`,
  `send-keys`), not a declarative pane tree. This is general enough to
  express `bin/t`'s layout and keeps the mapping to tmux CLI calls
  direct — see `src/config.rs`'s `Command` enum and
  `apply_post_create_setup`.
- **Discovery**: walk upward from the target directory toward the
  filesystem root, checking for `.tmxr.toml` at each level, the same
  pattern used by `.editorconfig`/`.git`/workspace-root discovery. The
  search stops after checking `$HOME` (inclusive), or at the filesystem
  root if the target directory is outside `$HOME` or `$HOME` can't be
  determined. See `src/config.rs`'s `discover_config`.
- **No global fallback config** (e.g. no `~/.config/tmxr/config.toml`).
  tmux's own `session-created` hook already covers the global case, so
  this is deliberately out of scope for now.
- **Creation-only**: post-create setup runs only when `ensure_session`
  reports it created a new session (`SessionOutcome::Created`), never on
  re-attach to an existing one. See `src/session.rs`.
- **Error handling**: a missing `.tmxr.toml` is not an error — silent,
  no behavior change from today. A `.tmxr.toml` that exists but fails to
  read or parse is a hard error: `tmxr` aborts before creating or
  attaching to any session, with a message naming the file. A tmux
  command failing during setup application is also a hard error,
  surfaced without attempting to kill or roll back the (possibly
  partially set up) session.

## Consequences

- New dependencies: `toml`, `serde` (with the `derive` feature). This is
  a deliberate exception to the project's general "minimal dependencies"
  preference, justified by TOML being the idiomatic, well-supported
  choice for Rust config files versus a hand-rolled parser.
- Discovery is purely lexical: symlinked project directories are not
  resolved/canonicalized before walking upward or comparing against
  `$HOME`. This mirrors `std::env::current_dir()`'s own behavior and
  keeps discovery simple; it's a known limitation, not a bug to fix
  here.
- Config loading currently happens unconditionally in `main.rs`'s
  `start_session()`, before `ensure_session` knows whether it will
  create or reuse a session. This means a broken `.tmxr.toml` blocks
  *re-attaching* to an already-running session in that directory, not
  just creating a new one — even though the config would be a no-op on
  reuse anyway. This was accepted for v1 as the simpler, more
  predictable behavior (config errors always surface at the same well-
  defined point). If this proves disruptive in practice, a follow-up
  could defer loading the config until `ensure_session` confirms
  creation is actually happening.
- No global fallback config is supported. Users who want a single
  default layout for every session should use tmux's own
  `session-created` hook; a global tmxr-specific fallback is explicitly
  deferred, not forgotten, should a need for one emerge later.
