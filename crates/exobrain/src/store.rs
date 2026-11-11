//! The candidates in the Nix store, and their registered size and age.
//!
//! The listing is one directory read plus one stat per `-source` entry:
//! a copy of a working tree carries its `.git`, and a source fetched from
//! a forge does not. The size and age come from the Nix database through
//! one batched `nix path-info --json`, never from a walk of the copy.

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use camino::Utf8PathBuf;
use serde::Deserialize;

use crate::error::Error;

/// The store that `EXOBRAIN_STORE_DIR` names, or `/nix/store`.
#[must_use]
pub fn store_dir() -> PathBuf {
    std::env::var_os("EXOBRAIN_STORE_DIR")
        .filter(|value| !value.is_empty())
        .map_or_else(|| PathBuf::from("/nix/store"), PathBuf::from)
}

/// The `nix` that `EXOBRAIN_NIX_BIN` names, or `nix` from `PATH`.
#[must_use]
pub fn nix_bin() -> PathBuf {
    std::env::var_os("EXOBRAIN_NIX_BIN")
        .filter(|value| !value.is_empty())
        .map_or_else(|| PathBuf::from("nix"), PathBuf::from)
}

/// What the `.git` of a store entry is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitKind {
    /// A `.git` file: the copy of a linked worktree.
    File,
    /// A `.git` folder: the copy of a main checkout or a clone.
    Dir,
}

/// A store entry that carries a `.git`.
#[derive(Debug, Clone)]
pub struct Candidate {
    /// The store path.
    pub path: Utf8PathBuf,
    /// What its `.git` is.
    pub git: GitKind,
}

/// List the `-source` entries of the store that carry a `.git`, sorted.
///
/// # Errors
///
/// [`Error::Io`] when the store cannot be listed.
pub fn candidates() -> Result<Vec<Candidate>, Error> {
    let dir = store_dir();
    let entries = std::fs::read_dir(&dir).map_err(|source| Error::Io {
        what: format!("list {}", dir.display()),
        source,
    })?;
    let mut found: Vec<Candidate> = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().ends_with("-source"))
        .filter_map(|entry| {
            let path = Utf8PathBuf::from_path_buf(entry.path()).ok()?;
            let meta = std::fs::symlink_metadata(path.join(".git")).ok()?;
            let git = if meta.is_file() {
                GitKind::File
            } else if meta.is_dir() {
                GitKind::Dir
            } else {
                return None;
            };
            Some(Candidate { path, git })
        })
        .collect();
    found.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(found)
}

/// The registered size and age of one store path.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathInfo {
    /// The NAR size in bytes.
    pub nar_size: u64,
    /// The registration time, in seconds since the epoch.
    pub registration_time: i64,
}

/// Query the Nix database for `paths` in one call. A path the collector
/// removed meanwhile comes back as `null` and is dropped.
///
/// # Errors
///
/// [`Error::Nix`] when `nix` does not run, fails, or prints no JSON.
pub fn path_info(paths: &[Utf8PathBuf]) -> Result<HashMap<Utf8PathBuf, PathInfo>, Error> {
    if paths.is_empty() {
        return Ok(HashMap::new());
    }
    let output = Command::new(nix_bin())
        .args(["path-info", "--json", "--json-format", "1"])
        .args(paths)
        .output()
        .map_err(|e| Error::Nix(e.to_string()))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Nix(stderr.lines().last().unwrap_or("").to_owned()));
    }
    let parsed: HashMap<Utf8PathBuf, Option<PathInfo>> =
        serde_json::from_slice(&output.stdout).map_err(|e| Error::Nix(e.to_string()))?;
    Ok(parsed
        .into_iter()
        .filter_map(|(path, info)| Some((path, info?)))
        .collect())
}
