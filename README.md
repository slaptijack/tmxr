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
