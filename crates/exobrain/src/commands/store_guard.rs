//! `exobrain store-guard`: find the copies of this checkout in the Nix
//! store, which a flake reference that does not read the git tree leaves
//! behind.
//!
//! Its daily mode also warns about a large build folder of the checkout,
//! and runs `prek cache gc` once a week per prek store.

use std::path::{Path, PathBuf};
use std::process::Command;

use camino::Utf8PathBuf;
use serde::Serialize;

use crate::cli::store_guard::StoreGuardArgs;
use crate::error::Error;
use crate::origin::{self, Attribution, Checkout};
use crate::stamp::{self, Acquired};
use crate::{output, store, switch};

/// One store copy of this checkout.
#[derive(Debug, Clone, Serialize)]
pub struct StoreCopy {
    /// The store path.
    pub path: Utf8PathBuf,
    /// The registered NAR size in bytes.
    pub nar_size: u64,
    /// The registration time, in seconds since the epoch.
    #[serde(serialize_with = "rfc3339")]
    pub registered: i64,
    /// Where the copy came from.
    pub origin: String,
}

/// A copy whose origin cannot be decided.
#[derive(Debug, Clone, Serialize)]
pub struct Unattributed {
    /// The store path.
    pub path: Utf8PathBuf,
    /// Why the origin is unknown.
    pub reason: String,
}

/// What one scan found.
#[derive(Debug, Clone, Serialize)]
pub struct Report {
    /// The canonical top level of the checkout.
    pub checkout: std::path::PathBuf,
    /// The copies of this checkout at or over the minimum size.
    pub copies: Vec<StoreCopy>,
    /// The copies whose origin is unknown.
    pub unattributed: Vec<Unattributed>,
    /// The sum of the copies' NAR sizes.
    pub total_nar_size: u64,
}

#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "serde's serialize_with passes the field by reference"
)]
fn rfc3339<S: serde::Serializer>(seconds: &i64, serializer: S) -> Result<S::Ok, S::Error> {
    match jiff::Timestamp::from_second(*seconds) {
        Ok(ts) => serializer.collect_str(&ts),
        Err(_) => serializer.serialize_i64(*seconds),
    }
}

/// Run the verb, returning the exit code.
///
/// # Errors
///
/// In operator mode, a folder outside git, an unreadable store, or a
/// failing `nix path-info`. `--daily` never fails.
pub fn run(args: &StoreGuardArgs) -> Result<u8, Error> {
    if args.daily {
        daily(args.min_size);
        return Ok(0);
    }
    let checkout = origin::checkout()?;
    let report = scan(&checkout, args.min_size)?;
    output::report(&report, args.json);
    Ok(u8::from(!report.copies.is_empty()))
}

/// The `.envrc` mode: switched off in CI or by the operator, at most one
/// scan a day per checkout, quiet on a clean store, and never a failure.
fn daily(min_size: u64) {
    if switch::switched_off() || switch::in_ci() {
        return;
    }
    let Ok(checkout) = origin::checkout() else {
        return;
    };
    let Some(dir) = stamp::state_dir() else {
        output::note("exobrain store-guard: neither XDG_STATE_HOME nor HOME is set; skipped");
        return;
    };
    let key = stamp::key(&checkout.root);
    let lock = match std::fs::create_dir_all(&dir)
        .map(|()| stamp::acquire(&dir.join(format!("{key}.lock"))))
    {
        Ok(Acquired::Held(lock)) => lock,
        Ok(Acquired::Contended) => return,
        Ok(Acquired::Unavailable(e)) | Err(e) => {
            output::note(&format!(
                "exobrain store-guard: state folder {} is not writable ({e}); skipped",
                dir.display()
            ));
            return;
        }
    };
    let stamp_path = dir.join(format!("{key}.stamp"));
    if stamp::stamped_today(&stamp_path) {
        return;
    }
    if let Err(e) = stamp::write_stamp(&stamp_path) {
        output::note(&format!(
            "exobrain store-guard: cannot write {} ({e}); skipped",
            stamp_path.display()
        ));
        return;
    }
    match scan(&checkout, min_size) {
        Ok(report) if !report.copies.is_empty() => output::daily_warning(&report),
        Ok(_) => {}
        Err(e) => output::note(&format!("exobrain store-guard: {e}")),
    }
    target_size(&checkout.root);
    prek_gc(&dir);
    drop(lock);
}

/// The limit of a build folder: 10 GB, or the size that
/// `EXOBRAIN_TARGET_LIMIT` names.
const TARGET_LIMIT: u64 = 10_000_000_000;

/// The daily build-folder check: one line when the build folder of the
/// checkout holds more exclusive bytes than the limit, silent without a
/// `Cargo.toml`, without `cargo` on `PATH`, or before the first build, and
/// never a failure. It deletes nothing.
fn target_size(root: &Path) {
    let limit = match target_limit() {
        Ok(limit) => limit,
        Err(e) => {
            output::note(&format!("exobrain store-guard: EXOBRAIN_TARGET_LIMIT: {e}"));
            return;
        }
    };
    let folder = match build_folder(root) {
        Ok(Some(folder)) if folder.is_dir() => folder,
        Ok(_) => return,
        Err(e) => {
            output::note(&format!("exobrain store-guard: cargo metadata: {e}"));
            return;
        }
    };
    match extents::exclusive_bytes(&folder) {
        Ok(bytes) if bytes > limit => output::note(&format!(
            "exobrain store-guard: build folder {} holds {} of its own data; run `cargo clean --target-dir {}`",
            folder.display(),
            decimal_size(bytes),
            shell_word(&folder.display().to_string())
        )),
        Ok(_) => {}
        Err(e) => output::note(&format!(
            "exobrain store-guard: cannot measure {} ({e})",
            folder.display()
        )),
    }
}

/// The limit that `EXOBRAIN_TARGET_LIMIT` names, or [`TARGET_LIMIT`].
fn target_limit() -> Result<u64, String> {
    match std::env::var("EXOBRAIN_TARGET_LIMIT") {
        Ok(text) if !text.is_empty() => crate::cli::store_guard::parse_size(&text),
        Ok(_) | Err(std::env::VarError::NotPresent) => Ok(TARGET_LIMIT),
        Err(std::env::VarError::NotUnicode(_)) => Err("not a size".to_owned()),
    }
}

/// The `target_directory` that `cargo metadata` reports for the checkout at
/// `root`, which honors `CARGO_TARGET_DIR` and `build.target-dir`, or
/// `None` without a `Cargo.toml` at `root` or without `cargo` on `PATH`.
fn build_folder(root: &Path) -> Result<Option<PathBuf>, String> {
    if !root.join("Cargo.toml").is_file() {
        return Ok(None);
    }
    let output = match Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .current_dir(root)
        .output()
    {
        Ok(output) => output,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.to_string()),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
            .map_or_else(|| output.status.to_string(), str::to_owned));
    }
    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    metadata
        .get("target_directory")
        .and_then(serde_json::Value::as_str)
        .map(|dir| Some(PathBuf::from(dir)))
        .ok_or_else(|| "no target_directory in its output".to_owned())
}

/// The weekly `prek cache gc`: silent without `prek` on `PATH`, at most one
/// run every [`stamp::GC_INTERVAL_DAYS`] days and one at a time per prek
/// store, one line with the freed size, and never a failure.
fn prek_gc(dir: &Path) {
    let store = match prek_store() {
        Ok(Some(store)) => store,
        Ok(None) => return,
        Err(e) => {
            output::note(&format!("exobrain store-guard: prek cache dir: {e}"));
            return;
        }
    };
    let key = stamp::store_key(&store);
    let _lock = match stamp::acquire(&dir.join(format!("{key}.lock"))) {
        Acquired::Held(lock) => lock,
        Acquired::Contended => return,
        Acquired::Unavailable(e) => {
            output::note(&format!(
                "exobrain store-guard: state folder {} is not writable ({e}); prek cache gc skipped",
                dir.display()
            ));
            return;
        }
    };
    let stamp_path = dir.join(format!("{key}.stamp"));
    if !stamp::due_weekly(&stamp_path) {
        return;
    }
    if let Err(e) = stamp::write_stamp(&stamp_path) {
        output::note(&format!(
            "exobrain store-guard: cannot write {} ({e}); prek cache gc skipped",
            stamp_path.display()
        ));
        return;
    }
    match prek(&["cache", "gc"]) {
        Ok(Some(stdout)) => {
            if let Some(freed) = freed_size(&stdout) {
                output::note(&format!(
                    "exobrain store-guard: prek cache gc freed {freed} in {}",
                    store.display()
                ));
            }
        }
        Ok(None) => {}
        Err(e) => output::note(&format!("exobrain store-guard: prek cache gc: {e}")),
    }
}

/// The store that `prek cache dir` resolves, or `None` without `prek` on
/// `PATH`.
fn prek_store() -> Result<Option<PathBuf>, String> {
    Ok(prek(&["cache", "dir"])?
        .map(|stdout| PathBuf::from(stdout.trim()))
        .filter(|store| !store.as_os_str().is_empty()))
}

/// Run `prek` with `args` and return its stdout, or `None` when `prek` is
/// not on `PATH`.
fn prek(args: &[&str]) -> Result<Option<String>, String> {
    let output = match Command::new("prek")
        .args(args)
        .env("PREK_COLOR", "never")
        .output()
    {
        Ok(output) => output,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.to_string()),
    };
    if output.status.success() {
        return Ok(Some(String::from_utf8_lossy(&output.stdout).into_owned()));
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let reason = stderr
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map_or_else(|| output.status.to_string(), str::to_owned);
    Err(reason)
}

/// The freed size of a `prek cache gc` report: the last parenthesized
/// value of its `Removed ...` line, or `None` when it removed nothing.
fn freed_size(stdout: &str) -> Option<&str> {
    let line = stdout
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("Removed "))?;
    let open = line.rfind('(')?;
    let close = line[open..].find(')')? + open;
    Some(&line[open + 1..close]).filter(|size| !size.is_empty())
}

/// List, attribute, and measure the store copies of `checkout`.
///
/// # Errors
///
/// An unreadable store or a failing `nix path-info`.
pub fn scan(checkout: &Checkout, min_size: u64) -> Result<Report, Error> {
    let mut ours = Vec::new();
    let mut unattributed = Vec::new();
    for candidate in store::candidates()? {
        match origin::attribute(&candidate, checkout) {
            Attribution::Ours(origin) => ours.push((candidate.path, origin)),
            Attribution::Unknown(reason) => unattributed.push(Unattributed {
                path: candidate.path,
                reason,
            }),
            Attribution::Foreign => {}
        }
    }
    let paths: Vec<Utf8PathBuf> = ours.iter().map(|(path, _)| path.clone()).collect();
    let info = store::path_info(&paths)?;
    let copies: Vec<StoreCopy> = ours
        .into_iter()
        .filter_map(|(path, origin)| {
            let found = info.get(&path)?;
            (found.nar_size >= min_size).then_some(StoreCopy {
                nar_size: found.nar_size,
                registered: found.registration_time,
                origin,
                path,
            })
        })
        .collect();
    let total_nar_size = copies.iter().map(|copy| copy.nar_size).sum();
    Ok(Report {
        checkout: checkout.root.clone(),
        copies,
        unattributed,
        total_nar_size,
    })
}

/// A size in decimal gigabytes with one decimal, or in megabytes below
/// one gigabyte.
fn decimal_size(bytes: u64) -> String {
    if bytes < 1_000_000_000 {
        return output::megabytes(bytes);
    }
    let tenths = bytes.saturating_add(50_000_000) / 100_000_000;
    format!("{}.{} GB", tenths / 10, tenths % 10)
}

/// `word` as one shell word: unchanged when it holds only safe characters,
/// else in single quotes.
pub(crate) fn shell_word(word: &str) -> String {
    let safe = |c: char| c.is_ascii_alphanumeric() || "/._-+=:@,%".contains(c);
    if !word.is_empty() && word.chars().all(safe) {
        return word.to_owned();
    }
    format!("'{}'", word.replace('\'', r"'\''"))
}

/// The exclusive bytes of a folder: the bytes that deleting it would free.
///
/// Each regular file counts the `unique` bytes that `kache_fs` measures,
/// the extents that the `FIEMAP` ioctl does not flag as shared, so a
/// copy-on-write clone of a kache entry counts only the blocks it changed.
/// Where the filesystem has no `FIEMAP`, a file counts its allocated bytes.
/// A file with several hard links counts once, and only when every link
/// lies inside the folder. Two files of the folder that share an extent
/// both count it as shared, so the total can read a little low.
mod extents {
    use std::collections::HashMap;
    use std::io;
    use std::path::{Path, PathBuf};

    /// The links seen inside the folder, the link count, and the bytes of
    /// one inode with several hard links.
    type Linked = (u64, u64, u64);

    /// The exclusive bytes of `folder`, which it walks without following
    /// a symbolic link. A file that disappears during the walk counts as
    /// zero, because a running build deletes files.
    pub fn exclusive_bytes(folder: &Path) -> io::Result<u64> {
        let mut total = 0_u64;
        let mut linked: HashMap<(u64, u64), Linked> = HashMap::new();
        let mut stack: Vec<PathBuf> = vec![folder.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let entries = match std::fs::read_dir(&dir) {
                Ok(entries) => entries,
                Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                Err(e) => return Err(with_path(&e, &dir)),
            };
            for entry in entries {
                let entry = entry.map_err(|e| with_path(&e, &dir))?;
                let path = entry.path();
                let kind = match entry.file_type() {
                    Ok(kind) => kind,
                    Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                    Err(e) => return Err(with_path(&e, &path)),
                };
                if kind.is_dir() {
                    stack.push(path);
                    continue;
                }
                if !kind.is_file() {
                    continue;
                }
                let sizing = match kache_fs::measure_file(&path) {
                    Ok(sizing) => sizing,
                    Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                    Err(e) => return Err(with_path(&e, &path)),
                };
                let bytes = sizing.unique.unwrap_or(sizing.allocated);
                if sizing.nlink > 1 {
                    linked
                        .entry((sizing.inode.dev, sizing.inode.ino))
                        .and_modify(|seen| seen.0 += 1)
                        .or_insert((1, sizing.nlink, bytes));
                } else {
                    total = total.saturating_add(bytes);
                }
            }
        }
        Ok(linked
            .values()
            .filter(|(seen, nlink, _)| seen >= nlink)
            .fold(total, |sum, (_, _, bytes)| sum.saturating_add(*bytes)))
    }

    fn with_path(e: &io::Error, path: &Path) -> io::Error {
        io::Error::new(e.kind(), format!("{}: {e}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::{decimal_size, freed_size, shell_word};

    #[test]
    fn a_size_reads_in_gigabytes_from_one_gigabyte() {
        assert_eq!(decimal_size(12_345_678_901), "12.3 GB");
        assert_eq!(decimal_size(1_000_000_000), "1.0 GB");
        assert_eq!(decimal_size(999_000_000), "999.0 MB");
    }

    #[test]
    fn a_path_quotes_as_one_shell_word() {
        assert_eq!(shell_word("/srv/build/target"), "/srv/build/target");
        assert_eq!(shell_word("/tmp/my target"), "'/tmp/my target'");
        assert_eq!(shell_word("/tmp/it's"), r"'/tmp/it'\''s'");
    }

    #[test]
    fn the_freed_size_comes_from_the_removed_line() {
        assert_eq!(
            freed_size("Removed 2 repos, 3 hook envs (123.4MiB)\n"),
            Some("123.4MiB")
        );
        assert_eq!(freed_size("Nothing to clean\n"), None);
        assert_eq!(freed_size(""), None);
        assert_eq!(freed_size("Removed 1 repo\n"), None);
    }
}
