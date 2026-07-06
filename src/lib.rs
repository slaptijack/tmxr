//! Core library for tmxr, kept independent of the CLI entry point.

/// Returns the greeting `tmxr` prints on startup.
pub fn greeting() -> &'static str {
    "tmxr: opinionated tmux workspace launcher"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_is_not_empty() {
        assert!(!greeting().is_empty());
    }
}
