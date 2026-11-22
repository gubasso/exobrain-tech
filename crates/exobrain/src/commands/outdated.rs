//! `exobrain outdated`: report what is behind, change nothing, and exit 0.
//!
//! The age check reads each flake lock offline. The `lastModified` of the
//! nixpkgs node of a lock is the clock of the whole lock, and a pin over
//! [`LIMIT_DAYS`] days old prints one line on stderr. The check reads no
//! network, no cache, and no state file, calls no `nix`, and writes nothing.

use std::path::{Component, Path, PathBuf};

use serde_json::Value;

use crate::cli::outdated::{DEFAULT_LOCK, OutdatedArgs};
use crate::commands::store_guard::shell_word;
use crate::error::Error;
use crate::output;

/// The age limit of a nixpkgs pin, in whole days. A pin of exactly this
/// age stays silent.
pub const LIMIT_DAYS: i64 = 30;

/// The seconds of one day.
const DAY: i64 = 86_400;

/// The depth at which a follows path counts as a cycle.
const MAX_FOLLOWS: usize = 32;

/// Run the command. It exits 0 in every case.
///
/// # Errors
///
/// None: every problem of a lock prints one line instead.
pub fn run(args: &OutdatedArgs) -> Result<u8, Error> {
    let now = jiff::Timestamp::now();
    let (locks, named) = if args.locks.is_empty() {
        (vec![PathBuf::from(DEFAULT_LOCK)], false)
    } else {
        (args.locks.clone(), true)
    };
    for lock in &locks {
        if let Some(line) = age_line(lock, named, now) {
            output::note(&line);
        }
    }
    Ok(0)
}

/// The line of the age check for the lock at `path`, judged at `now`.
///
/// `None` means silence. `named` tells whether the call named the lock: a
/// missing lock that the call did not name means the project has no flake.
/// The check reads the one file at `path` and nothing else.
#[must_use]
pub fn age_line(path: &Path, named: bool, now: jiff::Timestamp) -> Option<String> {
    let shown = path.display();
    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return named.then(|| {
                format!("{shown}: no such file, so the age of the nixpkgs pin is unknown.")
            });
        }
        Err(e) => return Some(unreadable(path, &e.to_string())),
    };
    let lock: Value = match serde_json::from_slice(&bytes) {
        Ok(lock) => lock,
        Err(e) => return Some(unreadable(path, &e.to_string())),
    };
    let clock = match clock(&lock) {
        Ok(clock) => clock,
        Err(Problem::NoRoot) => return Some(unreadable(path, "no root node")),
        Err(Problem::NoNixpkgs) => {
            return Some(format!(
                "{shown}: the root of the lock has no nixpkgs input, so the lock has no clock."
            ));
        }
    };
    let Some(last_modified) = clock
        .get("locked")
        .and_then(|locked| locked.get("lastModified"))
        .and_then(Value::as_i64)
    else {
        return Some(format!(
            "{shown}: the nixpkgs pin carries no lastModified, so its age is unknown."
        ));
    };
    let days = age_days(last_modified, now.as_second());
    (days > LIMIT_DAYS).then(|| {
        let flake = shell_word(&folder_of(path));
        format!(
            "{shown}: nixpkgs pin is {days} days old (limit {LIMIT_DAYS}). Move it with nix flake update --flake {flake} when ready."
        )
    })
}

/// The whole days from `last_modified` to `now`, rounded down. A time in
/// the future counts as age 0.
const fn age_days(last_modified: i64, now: i64) -> i64 {
    let seconds = now.saturating_sub(last_modified);
    if seconds <= 0 { 0 } else { seconds / DAY }
}

/// Why a lock gives no clock node.
enum Problem {
    /// The lock names no root node, or the root node is missing.
    NoRoot,
    /// The root has no `nixpkgs` input, or its follows path breaks.
    NoNixpkgs,
}

/// The node that the `nixpkgs` input of the root resolves to.
fn clock(lock: &Value) -> Result<&Value, Problem> {
    let nodes = lock.get("nodes").ok_or(Problem::NoRoot)?;
    let root_name = lock.get("root").and_then(Value::as_str).unwrap_or("root");
    let root = nodes.get(root_name).ok_or(Problem::NoRoot)?;
    let input = input_of(root, "nixpkgs").ok_or(Problem::NoNixpkgs)?;
    let name = resolve(nodes, root, input, 0).ok_or(Problem::NoNixpkgs)?;
    nodes.get(name).ok_or(Problem::NoNixpkgs)
}

/// The value of the input `name` of `node`.
fn input_of<'a>(node: &'a Value, name: &str) -> Option<&'a Value> {
    node.get("inputs")?.get(name)
}

/// The node name that an input value resolves to: the value itself when
/// it is a node name, or the end of its follows path, which starts at the
/// root node.
fn resolve<'a>(
    nodes: &'a Value,
    root: &'a Value,
    input: &'a Value,
    depth: usize,
) -> Option<&'a str> {
    if let Some(name) = input.as_str() {
        return Some(name);
    }
    if depth >= MAX_FOLLOWS {
        return None;
    }
    let mut node = root;
    let mut name = None;
    for step in input.as_array()? {
        let value = input_of(node, step.as_str()?)?;
        let next = resolve(nodes, root, value, depth + 1)?;
        node = nodes.get(next)?;
        name = Some(next);
    }
    name
}

/// The folder of the lock at `path` as a flake reference: `.` for a lock
/// in the working folder, and `./<folder>` for a relative folder.
fn folder_of(path: &Path) -> String {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    if parent.as_os_str().is_empty() {
        return ".".to_owned();
    }
    let anchored = matches!(
        parent.components().next(),
        Some(Component::RootDir | Component::Prefix(_) | Component::CurDir | Component::ParentDir)
    );
    if anchored {
        parent.display().to_string()
    } else {
        format!("./{}", parent.display())
    }
}

/// The line for a lock that does not read.
fn unreadable(path: &Path, reason: &str) -> String {
    format!(
        "{}: cannot read the lock ({reason}), so the age of the nixpkgs pin is unknown.",
        path.display()
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{DAY, age_days, folder_of};

    #[test]
    fn outdated_age_rounds_down_and_floors_at_zero() {
        assert_eq!(age_days(0, 30 * DAY + DAY - 1), 30);
        assert_eq!(age_days(0, 31 * DAY), 31);
        assert_eq!(age_days(DAY, 0), 0);
    }

    #[test]
    fn outdated_folder_reads_as_a_flake_reference() {
        assert_eq!(folder_of(Path::new("flake.lock")), ".");
        assert_eq!(folder_of(Path::new("nix/flake.lock")), "./nix");
        assert_eq!(folder_of(Path::new("./nix/flake.lock")), "./nix");
        assert_eq!(folder_of(Path::new("../up/flake.lock")), "../up");
        assert_eq!(folder_of(Path::new("/srv/x/flake.lock")), "/srv/x");
    }
}
