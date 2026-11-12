//! The host side of `host-check`: the `PATH` lookup, the version probe,
//! and the version cache.
//!
//! The cache keys a version on the real path of the binary, its inode,
//! size, and modification time, and the argument that printed it. An
//! upgrade in place changes the inode or the modification time, and a Nix
//! upgrade changes the path, so a hit never serves the version of an
//! older binary. Every cache failure is silent: the probe then runs.

use std::ffi::OsStr;
use std::fmt::Write as _;
use std::os::unix::fs::{MetadataExt as _, PermissionsExt as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use semver::Version;
use sha2::{Digest as _, Sha256};

/// The first executable file named `name` in the folders of `path_var`.
#[must_use]
pub fn which(name: &str, path_var: &OsStr) -> Option<PathBuf> {
    std::env::split_paths(path_var)
        .filter(|dir| !dir.as_os_str().is_empty())
        .map(|dir| dir.join(name))
        .find(|candidate| {
            std::fs::metadata(candidate)
                .is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
        })
}

/// The first version in `output`: two or three dot-separated numbers that
/// no digit or dot precedes. A two-part version gains
/// a `.0`, and a fourth part or a suffix is dropped.
#[must_use]
pub fn first_version(output: &str) -> Option<Version> {
    let bytes = output.as_bytes();
    let mut start = 0;
    while start < bytes.len() {
        let boundary = start == 0 || {
            let before = bytes[start - 1];
            !before.is_ascii_digit() && before != b'.'
        };
        if boundary
            && bytes[start].is_ascii_digit()
            && let Some(version) = version_at(&output[start..])
        {
            return Some(version);
        }
        start += 1;
    }
    None
}

fn version_at(text: &str) -> Option<Version> {
    let mut parts = Vec::new();
    let mut rest = text;
    while parts.len() < 3 {
        let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
        if digits == 0 {
            break;
        }
        parts.push(rest[..digits].parse::<u64>().ok()?);
        rest = &rest[digits..];
        match rest.strip_prefix('.') {
            Some(after) if after.starts_with(|c: char| c.is_ascii_digit()) => rest = after,
            _ => break,
        }
    }
    match parts[..] {
        [major, minor] => Some(Version::new(major, minor, 0)),
        [major, minor, patch] => Some(Version::new(major, minor, patch)),
        _ => None,
    }
}

/// Run `binary version_cmd` and read the first version of its stdout,
/// then of its stderr.
#[must_use]
pub fn probe(binary: &Path, version_cmd: &str) -> Option<Version> {
    let output = Command::new(binary)
        .arg(version_cmd)
        .stdin(Stdio::null())
        .output()
        .ok()?;
    first_version(&String::from_utf8_lossy(&output.stdout))
        .or_else(|| first_version(&String::from_utf8_lossy(&output.stderr)))
}

/// The cache folder: `${XDG_CACHE_HOME:-$HOME/.cache}/exobrain/host-check`.
/// A relative `XDG_CACHE_HOME` counts as unset, as the XDG specification
/// says.
#[must_use]
pub fn cache_dir() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .filter(|p| p.is_absolute())
                .map(|home| home.join(".cache"))
        })?;
    Some(base.join("exobrain").join("host-check"))
}

/// The cache entry of `binary` under `dir`, or `None` when the binary
/// cannot be resolved.
fn entry(dir: &Path, binary: &Path, version_cmd: &str) -> Option<PathBuf> {
    let real = std::fs::canonicalize(binary).ok()?;
    let meta = std::fs::metadata(&real).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(real.as_os_str().as_encoded_bytes());
    hasher.update([0]);
    hasher.update(
        format!(
            "{}:{}:{}:{}:{}",
            meta.dev(),
            meta.ino(),
            meta.size(),
            meta.mtime(),
            meta.mtime_nsec()
        )
        .as_bytes(),
    );
    hasher.update([0]);
    hasher.update(version_cmd.as_bytes());
    let mut name = String::with_capacity(64);
    for byte in hasher.finalize() {
        let _ = write!(name, "{byte:02x}");
    }
    Some(dir.join(name))
}

/// The version of `binary`: from the cache under `dir` when an entry
/// matches, else from a probe, which then fills the cache.
#[must_use]
pub fn version(dir: Option<&Path>, binary: &Path, version_cmd: &str) -> Option<Version> {
    let entry = dir.and_then(|dir| entry(dir, binary, version_cmd));
    if let Some(cached) = entry
        .as_deref()
        .and_then(|e| std::fs::read_to_string(e).ok())
        .and_then(|text| Version::parse(text.trim()).ok())
    {
        return Some(cached);
    }
    let found = probe(binary, version_cmd)?;
    if let Some(entry) = entry {
        store(&entry, &found);
    }
    Some(found)
}

/// Write one entry atomically: a temporary file beside it, then a rename.
fn store(entry: &Path, version: &Version) {
    let Some(dir) = entry.parent() else { return };
    if std::fs::create_dir_all(dir).is_err() {
        return;
    }
    let tmp = entry.with_extension(format!("tmp{}", std::process::id()));
    if std::fs::write(&tmp, format!("{version}\n")).is_ok() && std::fs::rename(&tmp, entry).is_err()
    {
        let _ = std::fs::remove_file(&tmp);
    }
}

#[cfg(test)]
mod tests {
    use super::first_version;
    use semver::Version;

    fn v(text: &str) -> Option<String> {
        first_version(text).map(|v| v.to_string())
    }

    #[test]
    fn reports_every_failure_reads_the_first_version() {
        assert_eq!(v("git version 2.51.0\n"), Some("2.51.0".into()));
        assert_eq!(
            v("gh version 2.57.0 (2024-09-16)\nhttps://x/v2.57.0"),
            Some("2.57.0".into())
        );
        assert_eq!(v("Python 3.12"), Some("3.12.0".into()));
        assert_eq!(v("tool v1.2.3-rc1"), Some("1.2.3".into()));
        assert_eq!(v("x 1.2.3.4"), Some("1.2.3".into()));
        assert_eq!(v("prek 0.5.7 (abc 2026-01-01)"), Some("0.5.7".into()));
        assert_eq!(v("build 7 then 1.0"), Some("1.0.0".into()));
        assert_eq!(v("no version here"), None);
        assert_eq!(first_version("08.1"), Some(Version::new(8, 1, 0)));
    }
}
