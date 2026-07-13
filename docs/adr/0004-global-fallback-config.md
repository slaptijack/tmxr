# 0004. Global fallback config

## Status

Accepted

## Context

[ADR 0003](0003-per-project-config-discovery.md) added per-project
`.tmxr.toml` discovery for post-create session setup, but deliberately left
out a global fallback (e.g. `~/.config/tmxr/config.toml`), reasoning that
tmux's own `session-created` hook already covers a single default layout
for every session. In practice, that hook fires for every tmux session,
not just ones `tmxr` creates, and can't reuse `tmxr`'s own command schema
(`split` / `select-pane` / `send-keys` in `src/config.rs`). Users who want
a default layout for projects that don't have their own `.tmxr.toml`, but
expressed in the same schema, have no way to do that.

## Decision

- Add a global fallback config at `$HOME/.config/tmxr/config.toml`,
  parsed with the same `Config` schema as per-project `.tmxr.toml`.
- **Precedence**: per-project `.tmxr.toml` discovery (walking upward from
  the target directory, as described in ADR 0003) always takes priority.
  The global fallback is only consulted when that search finds nothing.
- **Discovery**: `discover_global_config` in `src/config.rs` resolves
  `$HOME` and checks for the fixed path directly — no further upward
  search, since there's only one location to check. If `$HOME` can't be
  determined, no fallback is attempted.
- **Error handling**: unchanged from ADR 0003. A missing global config is
  not an error. A global config that exists but fails to read or parse is
  a hard error, reported the same way as a broken project-level config
  (naming the offending path).

## Consequences

- Users get a project-independent default layout expressed in `tmxr`'s own
  command schema, without needing a tmux `session-created` hook.
- `load_config` now has two failure-eligible paths (project and global)
  instead of one; both surface equivalent, path-qualified error messages,
  so this doesn't add meaningfully to the error-handling surface.
- This also lands alongside the fix to ADR 0003's other deferred
  limitation (config loading now happens only when a session is actually
  being created, not on re-attach) — see ADR 0003's updated Consequences
  section.
