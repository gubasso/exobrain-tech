//! The daily stamp and the per-checkout lock of `store-guard --daily`.
//!
//! Both live under `${XDG_STATE_HOME:-$HOME/.local/state}/exobrain/store-guard/`,
//! keyed by the checkout's base name and a digest of its canonical path.
//! The lock follows release-kit's guard: an `O_EXCL` create holding the
//! owner's pid, taken over when procfs shows the owner gone or, where
//! procfs cannot judge, after a grace period.

use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::Duration;

use sha2::{Digest as _, Sha256};

/// A lock whose owner procfs cannot judge is taken over after this long.
const LOCK_GRACE: Duration = Duration::from_secs(15 * 60);

/// The folder that holds every stamp and lock, where a root is set.
#[must_use]
pub fn state_dir() -> Option<PathBuf> {
    let root = std::env::var_os("XDG_STATE_HOME")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .filter(|value| !value.is_empty())
                .map(|home| PathBuf::from(home).join(".local/state"))
        })?;
    Some(root.join("exobrain").join("store-guard"))
}

/// The key of one checkout: its base name and the first 16 hex digits of
/// the SHA-256 of its canonical path.
#[must_use]
pub fn key(checkout: &Path) -> String {
    let digest = Sha256::digest(checkout.as_os_str().as_encoded_bytes());
    let hex = digest.iter().take(8).fold(String::new(), |mut hex, byte| {
        let _ = write!(hex, "{byte:02x}");
        hex
    });
    let base = checkout.file_name().map_or_else(
        || "root".to_owned(),
        |name| name.to_string_lossy().into_owned(),
    );
    format!("{base}-{hex}")
}

/// Today, as the local calendar date `YYYY-MM-DD`.
#[must_use]
pub fn today() -> String {
    jiff::Zoned::now().date().to_string()
}

/// Whether the stamp at `path` holds today's date.
#[must_use]
pub fn stamped_today(path: &Path) -> bool {
    fs::read_to_string(path).is_ok_and(|text| text.trim() == today())
}

/// Write today's date at `path` through a temporary file and a rename,
/// so a reader sees the old stamp or the new one, never a torn one.
///
/// # Errors
///
/// Any I/O failure; on failure the temporary file is removed.
pub fn write_stamp(path: &Path) -> std::io::Result<()> {
    let name = path
        .file_name()
        .ok_or_else(|| std::io::Error::other("the stamp path has no file name"))?;
    let mut tmp_name = std::ffi::OsString::from(format!(".{}.tmp.", std::process::id()));
    tmp_name.push(name);
    let tmp = path.with_file_name(tmp_name);
    let written = fs::File::create(&tmp)
        .and_then(|mut file| {
            file.write_all(format!("{}\n", today()).as_bytes())
                .and_then(|()| file.sync_all())
        })
        .and_then(|()| fs::rename(&tmp, path));
    if written.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    written
}

/// The held lock; removed on drop.
#[derive(Debug)]
pub struct Lock(PathBuf);

impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

/// What acquisition found.
#[derive(Debug)]
pub enum Acquired {
    /// This run holds the checkout.
    Held(Lock),
    /// Another live run holds it: normal, and skipped in silence.
    Contended,
    /// The lock cannot be taken at all.
    Unavailable(std::io::Error),
}

/// Take the lock at `path` through `O_EXCL`. A lock whose owner is
/// provably gone, or older than the grace period where procfs cannot
/// judge, is removed and the take retried once.
#[must_use]
pub fn acquire(path: &Path) -> Acquired {
    match take(path) {
        Ok(lock) => Acquired::Held(lock),
        Err(source) if source.kind() == std::io::ErrorKind::AlreadyExists => {
            let pid = fs::read_to_string(path).ok().and_then(|text| {
                text.lines()
                    .find_map(|line| line.strip_prefix("pid=")?.trim().parse::<u64>().ok())
            });
            if !owner_gone(pid, path) {
                return Acquired::Contended;
            }
            match fs::remove_file(path) {
                Ok(()) => {}
                Err(source) if source.kind() == std::io::ErrorKind::NotFound => {}
                Err(source) => return Acquired::Unavailable(source),
            }
            match take(path) {
                Ok(lock) => Acquired::Held(lock),
                Err(source) if source.kind() == std::io::ErrorKind::AlreadyExists => {
                    Acquired::Contended
                }
                Err(source) => Acquired::Unavailable(source),
            }
        }
        Err(source) => Acquired::Unavailable(source),
    }
}

/// One exclusive create, holding the owner's process id.
fn take(path: &Path) -> std::io::Result<Lock> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    file.write_all(format!("pid={}\n", std::process::id()).as_bytes())?;
    Ok(Lock(path.to_path_buf()))
}

/// Whether the lock's owner is gone: judged by procfs where it can, else
/// by the lock's age against the grace period.
fn owner_gone(pid: Option<u64>, lock: &Path) -> bool {
    if let Some(pid) = pid
        && Path::new("/proc/self").is_dir()
    {
        match Path::new(&format!("/proc/{pid}")).try_exists() {
            Ok(true) => return false,
            Ok(false) => return true,
            Err(_) => {}
        }
    }
    fs::metadata(lock)
        .and_then(|meta| meta.modified())
        .ok()
        .and_then(|modified| modified.elapsed().ok())
        .is_some_and(|age| age > LOCK_GRACE)
}

#[cfg(test)]
mod tests {
    use super::{Acquired, acquire, key};

    #[test]
    fn the_lock_is_exclusive_and_released_on_drop() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("k.lock");
        let Acquired::Held(held) = acquire(&path) else {
            panic!("the first take holds");
        };
        assert!(matches!(acquire(&path), Acquired::Contended));
        drop(held);
        assert!(!path.exists(), "the lock is removed on drop");
    }

    #[test]
    fn a_lock_of_a_dead_owner_is_taken_over() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("k.lock");
        std::fs::write(&path, format!("pid={}\n", u32::MAX)).unwrap();
        assert!(matches!(acquire(&path), Acquired::Held(_)));
    }

    #[test]
    fn the_key_is_the_base_name_and_sixteen_hex_digits() {
        let key = key(std::path::Path::new("/srv/repo"));
        let (base, digest) = key.rsplit_once('-').unwrap();
        assert_eq!(base, "repo");
        assert_eq!(digest.len(), 16);
    }
}
