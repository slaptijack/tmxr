//! Per-project post-create session setup, discovered from `.tmxr.toml`
//! files, independent of the CLI and session layers.

use std::io;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::tmux::CommandRunner;

/// The filename `discover_config` looks for in each directory it checks.
pub const CONFIG_FILE_NAME: &str = ".tmxr.toml";

/// A parsed `.tmxr.toml`: an ordered list of post-create setup commands.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub commands: Vec<Command>,
}

/// A single post-create setup step, applied in order after a session is
/// newly created.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Command {
    Split {
        direction: SplitDirection,
        #[serde(default)]
        size: Option<u32>,
    },
    SelectPane {
        index: u32,
    },
    SendKeys {
        keys: Vec<String>,
    },
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SplitDirection {
    Vertical,
    Horizontal,
}

/// Applies `config`'s commands, in order, to the given tmux `session` via
/// `runner`. Stops at the first command that fails.
pub fn apply_post_create_setup(
    runner: &dyn CommandRunner,
    session: &str,
    config: &Config,
) -> Result<(), String> {
    for command in &config.commands {
        let args = match command {
            Command::Split { direction, size } => {
                let flag = match direction {
                    SplitDirection::Vertical => "-v",
                    SplitDirection::Horizontal => "-h",
                };
                let mut args = vec!["split-window".to_string(), flag.to_string()];
                if let Some(size) = size {
                    args.push("-l".to_string());
                    args.push(size.to_string());
                }
                args.push("-t".to_string());
                args.push(session.to_string());
                args
            }
            Command::SelectPane { index } => vec![
                "select-pane".to_string(),
                "-t".to_string(),
                format!("{session}.{index}"),
            ],
            Command::SendKeys { keys } => {
                let mut args = vec![
                    "send-keys".to_string(),
                    "-t".to_string(),
                    session.to_string(),
                ];
                args.extend(keys.iter().cloned());
                args
            }
        };
        run(runner, &args)?;
    }
    Ok(())
}

fn run(runner: &dyn CommandRunner, args: &[String]) -> Result<(), String> {
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    match runner.run("tmux", &arg_refs) {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(format!(
            "tmux failed to run '{}' during post-create setup: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        )),
        Err(e) => Err(format!("failed to run tmux: {e}")),
    }
}

/// Abstraction over filesystem access needed to locate and read a
/// `.tmxr.toml`, so discovery can be tested without touching the real
/// filesystem or `$HOME`.
pub trait ConfigLocator {
    fn exists(&self, path: &Path) -> bool;
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
    fn home_dir(&self) -> Option<PathBuf>;
}

/// A `ConfigLocator` that reads the real filesystem and environment.
pub struct SystemConfigLocator;

impl ConfigLocator for SystemConfigLocator {
    fn exists(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }

    fn home_dir(&self) -> Option<PathBuf> {
        std::env::var_os("HOME").map(PathBuf::from)
    }
}

/// Walks upward from `dir` looking for `.tmxr.toml`, stopping after
/// checking `$HOME` (inclusive) if `dir` is under `$HOME`, or after
/// reaching the filesystem root otherwise. Returns the path to the
/// nearest config file found, or `None` if none exists within the
/// search boundary.
pub fn discover_config(locator: &dyn ConfigLocator, dir: &Path) -> Option<PathBuf> {
    let home = locator.home_dir();
    let mut current = dir;
    loop {
        let candidate = current.join(CONFIG_FILE_NAME);
        if locator.exists(&candidate) {
            return Some(candidate);
        }
        if home.as_deref() == Some(current) {
            return None;
        }
        current = current.parent()?;
    }
}

/// Discovers and parses the nearest `.tmxr.toml` for `dir`, if any.
/// Returns `Ok(None)` when no config file is found. Returns `Err` when a
/// config file is found but fails to read or parse.
pub fn load_config(locator: &dyn ConfigLocator, dir: &Path) -> Result<Option<Config>, String> {
    let Some(path) = discover_config(locator, dir) else {
        return Ok(None);
    };

    let contents = locator
        .read_to_string(&path)
        .map_err(|e| format!("failed to read {}: {e}", path.display()))?;

    let config: Config = toml::from_str(&contents)
        .map_err(|e| format!("failed to parse {}: {e}", path.display()))?;

    Ok(Some(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{ScriptedRunner, failure_output, success_output};
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[test]
    fn apply_post_create_setup_issues_no_calls_when_empty() {
        let runner = ScriptedRunner::new(vec![]);
        let config = Config { commands: vec![] };
        apply_post_create_setup(&runner, "tmxr", &config).unwrap();
        assert_eq!(runner.call_count(), 0);
    }

    #[test]
    fn apply_post_create_setup_maps_split_with_size() {
        let runner = ScriptedRunner::new(vec![Ok(success_output())]);
        let config = Config {
            commands: vec![Command::Split {
                direction: SplitDirection::Vertical,
                size: Some(15),
            }],
        };
        apply_post_create_setup(&runner, "tmxr", &config).unwrap();
        assert_eq!(
            runner.calls()[0],
            vec!["split-window", "-v", "-l", "15", "-t", "tmxr"]
        );
    }

    #[test]
    fn apply_post_create_setup_maps_split_without_size() {
        let runner = ScriptedRunner::new(vec![Ok(success_output())]);
        let config = Config {
            commands: vec![Command::Split {
                direction: SplitDirection::Horizontal,
                size: None,
            }],
        };
        apply_post_create_setup(&runner, "tmxr", &config).unwrap();
        assert_eq!(runner.calls()[0], vec!["split-window", "-h", "-t", "tmxr"]);
    }

    #[test]
    fn apply_post_create_setup_maps_select_pane() {
        let runner = ScriptedRunner::new(vec![Ok(success_output())]);
        let config = Config {
            commands: vec![Command::SelectPane { index: 1 }],
        };
        apply_post_create_setup(&runner, "tmxr", &config).unwrap();
        assert_eq!(runner.calls()[0], vec!["select-pane", "-t", "tmxr.1"]);
    }

    #[test]
    fn apply_post_create_setup_maps_send_keys() {
        let runner = ScriptedRunner::new(vec![Ok(success_output())]);
        let config = Config {
            commands: vec![Command::SendKeys {
                keys: vec!["htop".to_string(), "Enter".to_string()],
            }],
        };
        apply_post_create_setup(&runner, "tmxr", &config).unwrap();
        assert_eq!(
            runner.calls()[0],
            vec!["send-keys", "-t", "tmxr", "htop", "Enter"]
        );
    }

    #[test]
    fn apply_post_create_setup_reproduces_bin_t_sequence() {
        let runner = ScriptedRunner::new(vec![
            Ok(success_output()),
            Ok(success_output()),
            Ok(success_output()),
            Ok(success_output()),
            Ok(success_output()),
        ]);
        let config = Config {
            commands: vec![
                Command::Split {
                    direction: SplitDirection::Vertical,
                    size: Some(15),
                },
                Command::SelectPane { index: 1 },
                Command::SendKeys {
                    keys: vec!["htop".to_string(), "Enter".to_string()],
                },
                Command::SelectPane { index: 0 },
                Command::Split {
                    direction: SplitDirection::Horizontal,
                    size: None,
                },
            ],
        };
        apply_post_create_setup(&runner, "tmxr", &config).unwrap();
        let calls = runner.calls();
        assert_eq!(calls.len(), 5);
        assert_eq!(calls[0][0], "split-window");
        assert_eq!(calls[1][0], "select-pane");
        assert_eq!(calls[2][0], "send-keys");
        assert_eq!(calls[3][0], "select-pane");
        assert_eq!(calls[4][0], "split-window");
    }

    #[test]
    fn apply_post_create_setup_short_circuits_on_first_failure() {
        let runner = ScriptedRunner::new(vec![Ok(failure_output("boom"))]);
        let config = Config {
            commands: vec![
                Command::SelectPane { index: 1 },
                Command::SelectPane { index: 0 },
            ],
        };
        let err = apply_post_create_setup(&runner, "tmxr", &config).unwrap_err();
        assert!(err.contains("boom"));
        assert_eq!(runner.call_count(), 1, "should not run later commands");
    }

    /// An in-memory `ConfigLocator` for hermetic discovery/parsing tests.
    struct FakeConfigLocator {
        files: RefCell<HashMap<PathBuf, String>>,
        home: Option<PathBuf>,
    }

    impl FakeConfigLocator {
        fn new(home: Option<&str>) -> Self {
            Self {
                files: RefCell::new(HashMap::new()),
                home: home.map(PathBuf::from),
            }
        }

        fn with_file(self, path: &str, contents: &str) -> Self {
            self.files
                .borrow_mut()
                .insert(PathBuf::from(path), contents.to_string());
            self
        }
    }

    impl ConfigLocator for FakeConfigLocator {
        fn exists(&self, path: &Path) -> bool {
            self.files.borrow().contains_key(path)
        }

        fn read_to_string(&self, path: &Path) -> io::Result<String> {
            self.files
                .borrow()
                .get(path)
                .cloned()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no such file"))
        }

        fn home_dir(&self) -> Option<PathBuf> {
            self.home.clone()
        }
    }

    #[test]
    fn discover_config_finds_file_in_target_dir() {
        let locator = FakeConfigLocator::new(Some("/home/user"))
            .with_file("/home/user/project/.tmxr.toml", "");
        assert_eq!(
            discover_config(&locator, Path::new("/home/user/project")),
            Some(PathBuf::from("/home/user/project/.tmxr.toml"))
        );
    }

    #[test]
    fn discover_config_walks_up_to_ancestor() {
        let locator = FakeConfigLocator::new(Some("/home/user"))
            .with_file("/home/user/project/.tmxr.toml", "");
        assert_eq!(
            discover_config(&locator, Path::new("/home/user/project/src/nested")),
            Some(PathBuf::from("/home/user/project/.tmxr.toml"))
        );
    }

    #[test]
    fn discover_config_stops_at_home_inclusive() {
        let locator =
            FakeConfigLocator::new(Some("/home/user")).with_file("/home/user/.tmxr.toml", "");
        assert_eq!(
            discover_config(&locator, Path::new("/home/user/project")),
            Some(PathBuf::from("/home/user/.tmxr.toml"))
        );
    }

    #[test]
    fn discover_config_does_not_search_above_home() {
        let locator = FakeConfigLocator::new(Some("/home/user")).with_file("/.tmxr.toml", "");
        assert_eq!(
            discover_config(&locator, Path::new("/home/user/project")),
            None
        );
    }

    #[test]
    fn discover_config_returns_none_when_not_found_within_home_boundary() {
        let locator = FakeConfigLocator::new(Some("/home/user"));
        assert_eq!(
            discover_config(&locator, Path::new("/home/user/project")),
            None
        );
    }

    #[test]
    fn discover_config_walks_to_root_when_outside_home() {
        let locator = FakeConfigLocator::new(Some("/home/user")).with_file("/srv/.tmxr.toml", "");
        assert_eq!(
            discover_config(&locator, Path::new("/srv/project")),
            Some(PathBuf::from("/srv/.tmxr.toml"))
        );
    }

    #[test]
    fn discover_config_walks_to_root_when_home_unknown() {
        let locator = FakeConfigLocator::new(None).with_file("/.tmxr.toml", "");
        assert_eq!(
            discover_config(&locator, Path::new("/home/user/project")),
            Some(PathBuf::from("/.tmxr.toml"))
        );
    }

    #[test]
    fn load_config_returns_none_when_not_found() {
        let locator = FakeConfigLocator::new(Some("/home/user"));
        assert_eq!(
            load_config(&locator, Path::new("/home/user/project")),
            Ok(None)
        );
    }

    #[test]
    fn load_config_parses_valid_toml() {
        let toml = r#"
[[commands]]
type = "split"
direction = "vertical"
size = 15

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
"#;
        let locator = FakeConfigLocator::new(Some("/home/user"))
            .with_file("/home/user/project/.tmxr.toml", toml);
        let config = load_config(&locator, Path::new("/home/user/project"))
            .unwrap()
            .unwrap();
        assert_eq!(
            config,
            Config {
                commands: vec![
                    Command::Split {
                        direction: SplitDirection::Vertical,
                        size: Some(15),
                    },
                    Command::SelectPane { index: 1 },
                    Command::SendKeys {
                        keys: vec!["htop".to_string(), "Enter".to_string()],
                    },
                    Command::SelectPane { index: 0 },
                    Command::Split {
                        direction: SplitDirection::Horizontal,
                        size: None,
                    },
                ],
            }
        );
    }

    #[test]
    fn load_config_reports_path_on_malformed_toml() {
        let locator = FakeConfigLocator::new(Some("/home/user"))
            .with_file("/home/user/project/.tmxr.toml", "not valid toml [[[");
        let err = load_config(&locator, Path::new("/home/user/project")).unwrap_err();
        assert!(err.contains("/home/user/project/.tmxr.toml"));
    }

    #[test]
    fn load_config_reports_path_on_unreadable_file() {
        struct UnreadableLocator;
        impl ConfigLocator for UnreadableLocator {
            fn exists(&self, _path: &Path) -> bool {
                true
            }
            fn read_to_string(&self, _path: &Path) -> io::Result<String> {
                Err(io::Error::new(io::ErrorKind::PermissionDenied, "denied"))
            }
            fn home_dir(&self) -> Option<PathBuf> {
                Some(PathBuf::from("/home/user"))
            }
        }
        let err = load_config(&UnreadableLocator, Path::new("/home/user/project")).unwrap_err();
        assert!(err.contains("/home/user/project/.tmxr.toml"));
    }

    #[test]
    fn load_config_rejects_unknown_field() {
        let locator = FakeConfigLocator::new(Some("/home/user"))
            .with_file("/home/user/project/.tmxr.toml", "unknown_field = true\n");
        assert!(load_config(&locator, Path::new("/home/user/project")).is_err());
    }

    #[test]
    fn load_config_rejects_unknown_command_type() {
        let locator = FakeConfigLocator::new(Some("/home/user")).with_file(
            "/home/user/project/.tmxr.toml",
            "[[commands]]\ntype = \"bogus\"\n",
        );
        assert!(load_config(&locator, Path::new("/home/user/project")).is_err());
    }
}
