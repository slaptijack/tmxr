#!/bin/bash
# Integration smoke test that exercises the real `tmxr` binary against a
# real tmux on PATH, rather than the fakes used by `cargo test`. See
# docs/testing.md for how this fits alongside the rest of the test suite.
#
# Not part of `cargo test`/`scripts/check.sh` on purpose: it needs a real
# tmux binary and spawns real tmux sessions, so it isn't hermetic in the
# sense the rest of the suite is required to be.
set -euo pipefail

if ! command -v tmux >/dev/null 2>&1; then
    echo "tmux-smoke-test: tmux not found on PATH" >&2
    exit 1
fi

tmux_version=$(tmux -V)
echo "==> using $tmux_version"
case "$tmux_version" in
    "tmux 3.0"*) ;;
    *) echo "==> warning: expected tmux 3.0.x, continuing anyway (local run?)" >&2 ;;
esac

root_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
tmxr_bin=${TMXR_BIN:-"$root_dir/target/debug/tmxr"}

if [[ ! -x "$tmxr_bin" ]]; then
    echo "==> building tmxr (set TMXR_BIN to reuse an existing binary)"
    (cd "$root_dir" && cargo build --all-features)
fi

work_dir=$(mktemp -d "${TMPDIR:-/tmp}/tmxr-smoke-start-XXXXXX")
config_dir=""
start_log=$(mktemp)
config_log=$(mktemp)
cleanup() {
    tmux kill-session -t "$(basename "$work_dir")" >/dev/null 2>&1 || true
    if [[ -n "$config_dir" ]]; then
        tmux kill-session -t "$(basename "$config_dir")" >/dev/null 2>&1 || true
    fi
    rm -rf "$work_dir" "$config_dir" "$start_log" "$config_log"
}
trap cleanup EXIT

echo "==> tmxr doctor"
doctor_output=$("$tmxr_bin" doctor)
echo "$doctor_output"
[[ "$doctor_output" == *"tmux found"* ]] || {
    echo "tmux-smoke-test: unexpected doctor output" >&2
    exit 1
}

session_name=$(basename "$work_dir")

echo "==> tmxr start (create-then-attach; attach is expected to fail without a tty)"
(cd "$work_dir" && "$tmxr_bin" start >"$start_log" 2>&1) || true

if ! tmux has-session -t "$session_name" 2>/dev/null; then
    echo "tmux-smoke-test: session '$session_name' was not created" >&2
    cat "$start_log" >&2 || true
    exit 1
fi
echo "==> session '$session_name' created successfully"

echo "==> post-create config (split/select-pane/send-keys)"
config_dir=$(mktemp -d "${TMPDIR:-/tmp}/tmxr-smoke-config-XXXXXX")
cat >"$config_dir/.tmxr.toml" <<'EOF'
[[commands]]
type = "split"
direction = "vertical"

[[commands]]
type = "select-pane"
index = 0

[[commands]]
type = "send-keys"
keys = ["echo tmxr-smoke-test", "Enter"]
EOF

config_session=$(basename "$config_dir")
(cd "$config_dir" && "$tmxr_bin" start >"$config_log" 2>&1) || true

if ! tmux has-session -t "$config_session" 2>/dev/null; then
    echo "tmux-smoke-test: session '$config_session' was not created" >&2
    cat "$config_log" >&2 || true
    exit 1
fi

pane_count=$(tmux list-panes -t "$config_session" | wc -l | tr -d ' ')
if [[ "$pane_count" -lt 2 ]]; then
    echo "tmux-smoke-test: expected split-window to produce 2+ panes, got $pane_count" >&2
    exit 1
fi
echo "==> post-create setup produced $pane_count panes as expected"

echo "==> tmux-smoke-test: all checks passed"
