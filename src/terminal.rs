//! Detection of the controlling terminal's size, independent of the CLI
//! and session layers.

use std::io::IsTerminal;

/// Abstraction over querying the controlling terminal's size, so session
/// creation can be tested without depending on a real TTY.
pub trait TerminalSizeProvider {
    /// Returns `(columns, rows)`, or `None` if there's no controlling
    /// terminal to size from (e.g. non-interactive invocation).
    fn size(&self) -> Option<(u16, u16)>;
}

/// A `TerminalSizeProvider` that queries the real controlling terminal.
pub struct SystemTerminalSize;

impl TerminalSizeProvider for SystemTerminalSize {
    #[cfg(unix)]
    fn size(&self) -> Option<(u16, u16)> {
        use std::os::unix::io::AsRawFd;

        let stdout = std::io::stdout();
        if !stdout.is_terminal() {
            return None;
        }

        // SAFETY: `winsize` is zero-initialized and `TIOCGWINSZ` fills it
        // in place; the fd is valid for the lifetime of `stdout`.
        let winsize = unsafe {
            let mut winsize: libc::winsize = std::mem::zeroed();
            if libc::ioctl(stdout.as_raw_fd(), libc::TIOCGWINSZ, &mut winsize) != 0 {
                return None;
            }
            winsize
        };

        if winsize.ws_col == 0 || winsize.ws_row == 0 {
            return None;
        }

        Some((winsize.ws_col, winsize.ws_row))
    }

    #[cfg(not(unix))]
    fn size(&self) -> Option<(u16, u16)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_terminal_size_is_none_without_a_tty() {
        // Test runners never have a TTY on stdout, so this is deterministic.
        assert_eq!(SystemTerminalSize.size(), None);
    }
}
