//! `exobrain host-check`: check the host tools that
//! `host-requirements.toml` lists, and report every failure before it
//! exits. No check picks another tool or falls back.

use std::ffi::OsString;

use semver::Version;

use crate::cli::host_check::HostCheckArgs;
use crate::error::{Error, USAGE};
use crate::manifest::{self, FILE_NAME, Manifest, Requirement, Tool};
use crate::{output, probe};

/// The exit code of a missing tool or a version that misses its
/// requirement.
pub const FAILED: u8 = 1;

/// Find, parse, and check. The outcome is an exit code.
///
/// # Errors
///
/// The working folder or the file cannot be read.
pub fn run(args: &HostCheckArgs) -> Result<u8, Error> {
    let cwd = std::env::current_dir().map_err(|source| Error::Io {
        what: "the working folder".to_string(),
        source,
    })?;
    let Some(path) = manifest::find(&cwd) else {
        output::note(&format!(
            "exobrain host-check: no {FILE_NAME} in {} or a parent folder",
            cwd.display()
        ));
        return Ok(USAGE);
    };
    let text = std::fs::read_to_string(&path).map_err(|source| Error::Io {
        what: path.display().to_string(),
        source,
    })?;
    let manifest = match manifest::parse(&path, &text) {
        Ok(manifest) => manifest,
        Err(problems) => {
            notes(&problems);
            return Ok(USAGE);
        }
    };
    if args.parse_only {
        return Ok(0);
    }
    let tools = match selected(&manifest, &args.tools) {
        Ok(tools) => tools,
        Err(lines) => {
            notes(&lines);
            return Ok(USAGE);
        }
    };
    let path_var = std::env::var_os("PATH").unwrap_or_default();
    let cache = probe::cache_dir();
    let failures: Vec<String> = tools
        .iter()
        .filter_map(|tool| check(tool, &path_var, cache.as_deref()))
        .collect();
    notes(&failures);
    Ok(if failures.is_empty() { 0 } else { FAILED })
}

/// The tools a call names, or every tool when it names none.
///
/// # Errors
///
/// One line for each name the file does not list.
pub fn selected<'m>(
    manifest: &'m Manifest,
    names: &[String],
) -> Result<Vec<&'m Tool>, Vec<String>> {
    if names.is_empty() {
        return Ok(manifest.tools.values().collect());
    }
    let mut tools = Vec::new();
    let mut unknown = Vec::new();
    for name in names {
        match manifest.tools.get(name) {
            Some(tool) => {
                if !tools.iter().any(|t: &&Tool| t.name == tool.name) {
                    tools.push(tool);
                }
            }
            None => unknown.push(format!(
                "host tool {name} is not listed in {}",
                manifest.path.display()
            )),
        }
    }
    if unknown.is_empty() {
        Ok(tools)
    } else {
        Err(unknown)
    }
}

/// The failure line of one tool, or `None` when it passes.
fn check(tool: &Tool, path_var: &OsString, cache: Option<&std::path::Path>) -> Option<String> {
    let name = &tool.name;
    let Some(binary) = probe::which(name, path_var) else {
        return Some(format!("host tool {name} is missing"));
    };
    let Requirement::Matches { text, req } = &tool.requirement else {
        return None;
    };
    match probe::version(cache, &binary, &tool.version_cmd) {
        Some(found) if req.matches(&found) => None,
        Some(found) => Some(mismatch(name, &found, text)),
        None => Some(format!(
            "host tool {name} printed no version for `{name} {}` ({FILE_NAME})",
            tool.version_cmd
        )),
    }
}

fn mismatch(name: &str, found: &Version, requirement: &str) -> String {
    format!("host tool {name} {found} does not match {requirement} ({FILE_NAME})")
}

fn notes(lines: &[String]) {
    for line in lines {
        output::note(line);
    }
}
