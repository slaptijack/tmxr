//! Shell completion script generation, kept separate from CLI parsing.

use std::io::Write;

use clap::CommandFactory;
use clap_complete::{Shell, generate};

use crate::cli::Cli;

pub fn write(shell: Shell, out: &mut impl Write) {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, out);
}

#[cfg(test)]
mod tests {
    use clap::ValueEnum;

    use super::*;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn generates_non_empty_script_for_every_shell() {
        for shell in Shell::value_variants() {
            let mut out = Vec::new();
            write(*shell, &mut out);
            assert!(!out.is_empty(), "expected output for shell {shell:?}");
        }
    }
}
