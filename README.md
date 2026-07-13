# tmxr
Opinionated tmux workspace launcher.

## Requirements

- tmux 3.0 or newer

## Shell completions

`tmxr` can generate shell completion scripts via `tmxr completions <shell>`.

```sh
# zsh
tmxr completions zsh > "${fpath[1]}/_tmxr"

# bash
tmxr completions bash > /etc/bash_completion.d/tmxr

# fish
tmxr completions fish > ~/.config/fish/completions/tmxr.fish
```

Restart your shell (or re-source your completion files) after installing.

## Per-project session setup

Drop a `.tmxr.toml` file in a project directory to have `tmxr` set up
panes and run commands the first time it creates a session there.
`tmxr` looks for `.tmxr.toml` starting in the current directory and
walking upward (checking `$HOME` last, if the directory is under it),
the same way tools like `.editorconfig` discover their config — so a
file at a project's root also applies to its subdirectories. Setup only
runs when a session is newly created, never when re-attaching to one
that already exists.

`commands` is an ordered list of steps, applied in sequence:

```toml
# .tmxr.toml
[[commands]]
type = "split"
direction = "vertical"   # "vertical" | "horizontal"
size = 15                # optional; omit for an even split

[[commands]]
type = "select-pane"
index = 1

[[commands]]
type = "send-keys"
keys = ["htop", "Enter"]

[[commands]]
type = "select-pane"
index = 0

[[commands]]
type = "split"
direction = "horizontal"
```

This example splits the initial pane into a 15-line pane at the bottom,
runs `htop` there, then splits the remaining top pane in two.

Note that tmux renumbers pane indices by their position in the layout
after each split, so a `select-pane` index refers to whatever pane
currently holds that position — reference an index right after the
split that creates it, before any later split can shift it, the same
way the example above does.
