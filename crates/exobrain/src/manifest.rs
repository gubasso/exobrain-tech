//! `host-requirements.toml`: where it is, and its typed parse.
//!
//! The parse is the validation of the format. It rejects an unknown key,
//! an unknown `schema`, a tool name that is not a plain command name, a
//! `version` comparator with no operator, and a requirement that Cargo's
//! grammar does not accept. It collects every problem before it fails, so
//! one run names them all.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use semver::VersionReq;
use serde::Deserialize;

/// The name of the file.
pub const FILE_NAME: &str = "host-requirements.toml";

/// The one schema this binary reads.
pub const SCHEMA: i64 = 1;

/// The argument that prints a version when a tool names none.
pub const DEFAULT_VERSION_CMD: &str = "--version";

/// The operators a comparator must start with.
const OPERATORS: [&str; 7] = [">=", "<=", ">", "<", "=", "~", "^"];

/// What the file requires of one tool's version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Requirement {
    /// No `version`: only presence counts.
    Presence,
    /// `version = "*"`: the tool owns its version, and only presence counts.
    OwnedByTool,
    /// A requirement the checker matches.
    Matches {
        /// The text as the file writes it, for messages.
        text: String,
        /// The parsed requirement.
        req: VersionReq,
    },
}

/// One tool of the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tool {
    /// The command name, looked up on `PATH`.
    pub name: String,
    /// What the version must match.
    pub requirement: Requirement,
    /// The argument that prints the version.
    pub version_cmd: String,
}

/// The parsed file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// Where the file is.
    pub path: PathBuf,
    /// Every tool, sorted by name.
    pub tools: BTreeMap<String, Tool>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFile {
    schema: i64,
    #[serde(default)]
    tools: BTreeMap<String, toml::Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTool {
    version: Option<String>,
    version_cmd: Option<String>,
}

/// The first `host-requirements.toml` in `start` or in one of its parent
/// folders.
#[must_use]
pub fn find(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .map(|dir| dir.join(FILE_NAME))
        .find(|candidate| candidate.is_file())
}

/// Parse the text of the file at `path`.
///
/// # Errors
///
/// Every problem of the file, one line each, each naming the file, the
/// tool where there is one, and the reason.
pub fn parse(path: &Path, text: &str) -> Result<Manifest, Vec<String>> {
    let shown = path.display();
    let raw: RawFile =
        toml::from_str(text).map_err(|e| vec![format!("{shown}: {}", one_line(&e, text))])?;
    let mut problems = Vec::new();
    if raw.schema != SCHEMA {
        problems.push(format!(
            "{shown}: unknown schema {}; this exobrain reads schema {SCHEMA}",
            raw.schema
        ));
    }
    let mut tools = BTreeMap::new();
    for (name, value) in raw.tools {
        match tool(&name, value) {
            Ok(tool) => {
                tools.insert(name, tool);
            }
            Err(reasons) => problems.extend(
                reasons
                    .into_iter()
                    .map(|reason| format!("{shown}: tool {name}: {reason}")),
            ),
        }
    }
    if problems.is_empty() {
        Ok(Manifest {
            path: path.to_path_buf(),
            tools,
        })
    } else {
        Err(problems)
    }
}

/// A TOML error on one line: its message, then its line.
fn one_line(error: &toml::de::Error, text: &str) -> String {
    let message = error.message().trim_end().trim_end_matches('.');
    error.span().map_or_else(
        || message.to_string(),
        |span| {
            let line = text
                .get(..span.start)
                .map_or(0, |before| before.matches('\n').count())
                + 1;
            format!("line {line}: {message}")
        },
    )
}

fn tool(name: &str, value: toml::Value) -> Result<Tool, Vec<String>> {
    let mut reasons = Vec::new();
    if name.is_empty() || name.contains(['/', '\0']) || name.trim() != name {
        reasons.push("the name is not a plain command name".to_string());
    }
    let raw = match RawTool::deserialize(value) {
        Ok(raw) => raw,
        Err(e) => {
            reasons.push(e.message().trim_end_matches('.').to_string());
            return Err(reasons);
        }
    };
    let version_cmd = raw
        .version_cmd
        .unwrap_or_else(|| DEFAULT_VERSION_CMD.to_string());
    if version_cmd.is_empty() {
        reasons.push("version_cmd is empty".to_string());
    }
    let requirement = raw
        .version
        .map_or(Ok(Requirement::Presence), |text| requirement(&text))
        .unwrap_or_else(|reason| {
            reasons.push(reason);
            Requirement::Presence
        });
    if reasons.is_empty() {
        Ok(Tool {
            name: name.to_string(),
            requirement,
            version_cmd,
        })
    } else {
        Err(reasons)
    }
}

/// Parse a `version` value: `"*"` alone, or comparators that each start
/// with an explicit operator.
///
/// # Errors
///
/// The reason the value is not an accepted requirement.
pub fn requirement(text: &str) -> Result<Requirement, String> {
    if text.trim() == "*" {
        return Ok(Requirement::OwnedByTool);
    }
    for comparator in text.split(',').map(str::trim) {
        if comparator.is_empty() {
            return Err(format!("version \"{text}\" has an empty comparator"));
        }
        if !OPERATORS.iter().any(|op| comparator.starts_with(op)) {
            return Err(format!(
                "version \"{text}\": comparator \"{comparator}\" has no operator; {}",
                "write one of >=, >, <=, <, =, ~, ^, or the whole value \"*\""
            ));
        }
    }
    VersionReq::parse(text)
        .map(|req| Requirement::Matches {
            text: text.to_string(),
            req,
        })
        .map_err(|e| format!("version \"{text}\" is not a Cargo requirement: {e}"))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, reason = "a test aborts on a broken fixture")]

    use super::{Requirement, parse, requirement};
    use std::path::Path;

    fn problems(text: &str) -> Vec<String> {
        parse(Path::new("host-requirements.toml"), text).unwrap_err()
    }

    #[test]
    fn validates_the_file_accepts_the_plan_example() {
        let text = "schema = 1\n[tools.git]\nversion = \">=2.36.0\"\n\
                    [tools.gh]\nversion = \">=2.57.0\"\n[tools.prek]\nversion = \"*\"\n";
        let manifest = parse(Path::new("host-requirements.toml"), text).unwrap();
        assert_eq!(manifest.tools.len(), 3);
        assert_eq!(manifest.tools["prek"].requirement, Requirement::OwnedByTool);
        assert_eq!(manifest.tools["git"].version_cmd, "--version");
    }

    #[test]
    fn validates_the_file_rejects_each_malformed_value() {
        assert!(requirement(">=0.5.7, <0.6").is_ok());
        assert!(requirement("~1.2").is_ok());
        assert!(requirement("2.57.0").unwrap_err().contains("no operator"));
        assert!(requirement("2.*").unwrap_err().contains("no operator"));
        assert!(requirement("2.x").unwrap_err().contains("no operator"));
        assert!(
            requirement(">=1.0, 2.0")
                .unwrap_err()
                .contains("\"2.0\" has no operator")
        );
        assert!(requirement(">=banana").unwrap_err().contains("not a Cargo"));
        assert!(requirement(">=1.0,").unwrap_err().contains("empty"));
    }

    #[test]
    fn validates_the_file_names_the_tool_and_collects_every_problem() {
        let got = problems(
            "schema = 2\n[tools.gh]\nversion = \"2.57.0\"\n[tools.git]\nflavor = \">=2\"\n",
        );
        assert_eq!(got.len(), 3, "{got:?}");
        assert!(got[0].contains("unknown schema 2"));
        assert!(got[1].contains("tool gh:") && got[1].contains("no operator"));
        assert!(got[2].contains("tool git:") && got[2].contains("flavor"));
    }

    #[test]
    fn validates_the_file_rejects_an_unknown_top_level_key() {
        let got = problems("schema = 1\nscope = \"global\"\n");
        assert!(got[0].contains("scope"), "{got:?}");
        assert!(problems("[tools.git]\n")[0].contains("schema"));
    }
}
