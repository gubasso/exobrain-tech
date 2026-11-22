//! The release cache of `exobrain outdated`: the latest tag of each GitHub
//! repository, read through the host `gh` once a week and shared by every
//! checkout of the host.
//!
//! The cache of `<owner>/<repo>` lives at
//! `${XDG_STATE_HOME:-$HOME/.local/state}/exobrain/releases/<owner>/<repo>.json`,
//! in lower case, with its lock beside it. A read is due [`SUCCESS_TTL`]
//! seconds after the last success, or [`FAILURE_TTL`] seconds after a
//! failed attempt. Every `gh` call goes through [`gh_api`], which ends the
//! child after [`TIMEOUT`].

use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::commands::hooks::write_atomic;
use crate::{probe, stamp};

/// The seconds after a success when the next read is due: 7 days.
pub const SUCCESS_TTL: i64 = 7 * 86_400;

/// The seconds after a failed attempt when the next read is due: 1 hour.
pub const FAILURE_TTL: i64 = 3_600;

/// The time after which a `gh` call ends.
pub const TIMEOUT: Duration = Duration::from_secs(10);

/// The tags of one page, as `flake-edit` asks for them.
const PER_PAGE: usize = 100;

/// The last page that a read walks, as `flake-edit` bounds it.
const MAX_PAGES: u32 = 20;

/// How often a running `gh` is polled.
const POLL: Duration = Duration::from_millis(20);

/// One GitHub repository, in lower case.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Repo {
    /// The owner.
    pub owner: String,
    /// The repository.
    pub name: String,
}

impl Repo {
    /// The repository `owner/name`, or `None` where either part is not a
    /// plain GitHub name that can serve as a file name.
    #[must_use]
    pub fn new(owner: &str, name: &str) -> Option<Self> {
        let plain = |part: &str| {
            !part.is_empty()
                && !part.starts_with('.')
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        };
        (plain(owner) && plain(name)).then(|| Self {
            owner: owner.to_ascii_lowercase(),
            name: name.to_ascii_lowercase(),
        })
    }

    /// The slug `owner/name`.
    #[must_use]
    pub fn slug(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

/// The content of one cache file.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// The time of the last attempt, in seconds since the epoch.
    pub attempted: Option<i64>,
    /// The time of the last success, in seconds since the epoch.
    pub succeeded: Option<i64>,
    /// The latest tag of the last success.
    pub latest: Option<String>,
    /// The error of the last attempt, `None` after a success.
    pub error: Option<String>,
}

impl Entry {
    /// Whether a read is due at `now`.
    #[must_use]
    pub fn due(&self, now: i64) -> bool {
        let fresh =
            |then: Option<i64>, ttl: i64| then.is_some_and(|then| (0..ttl).contains(&(now - then)));
        if self.error.is_some() {
            !fresh(self.attempted, FAILURE_TTL)
        } else {
            !fresh(self.succeeded, SUCCESS_TTL)
        }
    }
}

/// What one lookup found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookup {
    /// The latest tag that the cache holds after the lookup.
    pub latest: Option<String>,
    /// The error of a read that this lookup ran and that failed.
    pub failed: Option<String>,
}

/// The folder of every cache file, where a state root is set.
#[must_use]
pub fn cache_dir() -> Option<PathBuf> {
    Some(stamp::state_dir()?.parent()?.join("releases"))
}

/// The cache file of `repo` under `dir`.
#[must_use]
pub fn cache_file(dir: &Path, repo: &Repo) -> PathBuf {
    dir.join(&repo.owner).join(format!("{}.json", repo.name))
}

/// Read the cache file at `path`. A missing or unreadable file reads as
/// an empty entry, which is due.
#[must_use]
pub fn read_entry(path: &Path) -> Entry {
    std::fs::read(path)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

/// The latest tag of each repository of `repos`, read through `gh` where
/// the cache is due or `refresh` is set, else from the cache. The due
/// reads run in parallel. The result follows the order of `repos`.
#[must_use]
pub fn latest(repos: &[Repo], refresh: bool) -> Vec<Lookup> {
    let dir = cache_dir();
    let now = jiff::Timestamp::now().as_second();
    std::thread::scope(|scope| {
        // Every read starts before the first join, so the reads overlap.
        #[allow(
            clippy::needless_collect,
            reason = "the collect starts every read before a join"
        )]
        let handles: Vec<_> = repos
            .iter()
            .map(|repo| {
                let dir = dir.as_deref();
                scope.spawn(move || lookup(dir, repo, refresh, now))
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| {
                handle.join().unwrap_or_else(|_| Lookup {
                    latest: None,
                    failed: Some("the read panicked".to_owned()),
                })
            })
            .collect()
    })
}

/// The lookup of one repository.
fn lookup(dir: Option<&Path>, repo: &Repo, refresh: bool, now: i64) -> Lookup {
    let Some(dir) = dir else {
        return match read_latest(repo) {
            Ok(tag) => Lookup {
                latest: tag,
                failed: None,
            },
            Err(error) => Lookup {
                latest: None,
                failed: Some(error),
            },
        };
    };
    let path = cache_file(dir, repo);
    let entry = read_entry(&path);
    if !refresh && !entry.due(now) {
        return Lookup {
            latest: entry.latest,
            failed: None,
        };
    }
    let Some(folder) = path.parent() else {
        return Lookup {
            latest: entry.latest,
            failed: None,
        };
    };
    if std::fs::create_dir_all(folder).is_err() {
        return Lookup {
            latest: entry.latest,
            failed: None,
        };
    }
    let _lock = match stamp::acquire(&path.with_extension("lock")) {
        stamp::Acquired::Held(lock) => lock,
        stamp::Acquired::Contended | stamp::Acquired::Unavailable(_) => {
            return Lookup {
                latest: entry.latest,
                failed: None,
            };
        }
    };
    // Another run can finish its read between the first look and the lock.
    let entry = read_entry(&path);
    if !refresh && !entry.due(now) {
        return Lookup {
            latest: entry.latest,
            failed: None,
        };
    }
    let (next, failed) = match read_latest(repo) {
        Ok(tag) => (
            Entry {
                attempted: Some(now),
                succeeded: Some(now),
                latest: tag,
                error: None,
            },
            None,
        ),
        Err(error) => (
            Entry {
                attempted: Some(now),
                succeeded: entry.succeeded,
                latest: entry.latest,
                error: Some(error.clone()),
            },
            Some(error),
        ),
    };
    if let Ok(bytes) = serde_json::to_vec(&next) {
        let _ = write_atomic(&path, &bytes);
    }
    Lookup {
        latest: next.latest,
        failed,
    }
}

/// Read the tags of `repo` as `flake-edit` 0.4.0 reads them, and return
/// the highest. All pages of one read share one [`TIMEOUT`].
fn read_latest(repo: &Repo) -> Result<Option<String>, String> {
    let gh = gh()?;
    let deadline = Instant::now() + TIMEOUT;
    let slug = repo.slug();
    let mut names = Vec::new();
    for page in 1..=MAX_PAGES {
        let text = gh_api(
            &gh,
            &[
                &format!("repos/{slug}/tags?per_page={PER_PAGE}&page={page}"),
                "--jq",
                ".[].name",
            ],
            deadline,
        )?;
        let found: Vec<String> = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToOwned::to_owned)
            .collect();
        let full = found.len() >= PER_PAGE;
        let any = found.iter().any(|name| tag_version(name).is_some());
        names.extend(found);
        if !full || any {
            break;
        }
    }
    Ok(highest(&names))
}

/// The tag of `names` with the highest version by semantic version
/// precedence, where several tie, the last of them, as `max_by` picks.
#[must_use]
pub fn highest(names: &[String]) -> Option<String> {
    names
        .iter()
        .filter_map(|name| tag_version(name).map(|version| (version, name)))
        .max_by(|(a, _), (b, _)| a.cmp_precedence(b))
        .map(|(_, name)| name.clone())
}

/// The version of a tag as `flake-edit` 0.4.0 reads it: drop a
/// `refs/tags/` prefix and every character before the first digit, pad
/// the core to three parts, and parse.
#[must_use]
pub fn tag_version(tag: &str) -> Option<Version> {
    let tag = tag.strip_prefix("refs/tags/").unwrap_or(tag);
    let digit = tag.find(|c: char| c.is_ascii_digit())?;
    let tag = &tag[digit..];
    let (core, suffix) = tag
        .find(['-', '+'])
        .map_or((tag, ""), |idx| (&tag[..idx], &tag[idx..]));
    let core = match core.matches('.').count() {
        0 => format!("{core}.0.0"),
        1 => format!("{core}.0"),
        _ => core.to_owned(),
    };
    Version::parse(&format!("{core}{suffix}")).ok()
}

/// Whether `latest` is a greater version than `pinned`.
#[must_use]
pub fn newer(latest: &str, pinned: &str) -> bool {
    match (tag_version(latest), tag_version(pinned)) {
        (Some(latest), Some(pinned)) => latest.cmp_precedence(&pinned) == Ordering::Greater,
        _ => false,
    }
}

/// The host `gh`.
fn gh() -> Result<PathBuf, String> {
    let path = std::env::var_os("PATH").unwrap_or_default();
    probe::which("gh", &path).ok_or_else(|| "gh is not on PATH".to_owned())
}

/// Run `gh api` with `args` and return its stdout. The call ends the
/// process group of `gh` at `deadline`.
///
/// # Errors
///
/// The first line of the stderr of a failed `gh`, or the reason the call
/// did not finish.
pub fn gh_api(gh: &Path, args: &[&str], deadline: Instant) -> Result<String, String> {
    use std::os::unix::process::CommandExt as _;
    let mut child = Command::new(gh)
        .arg("api")
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()
        .map_err(|e| format!("cannot run gh: {e}"))?;
    let stdout = child.stdout.take().map(drain);
    let stderr = child.stderr.take().map(drain);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if Instant::now() < deadline => std::thread::sleep(POLL),
            Ok(None) | Err(_) => break None,
        }
    };
    let Some(status) = status else {
        if let Some(pid) = rustix::process::Pid::from_raw(child.id().cast_signed()) {
            let _ = rustix::process::kill_process_group(pid, rustix::process::Signal::KILL);
        }
        let _ = child.kill();
        let _ = child.wait();
        return Err(format!("gh timed out after {} seconds", TIMEOUT.as_secs()));
    };
    let stdout = stdout.and_then(|h| h.join().ok()).unwrap_or_default();
    let stderr = stderr.and_then(|h| h.join().ok()).unwrap_or_default();
    if status.success() {
        return String::from_utf8(stdout).map_err(|_| "gh printed no UTF-8".to_owned());
    }
    let stderr = String::from_utf8_lossy(&stderr);
    let cause = stderr.lines().map(str::trim).find(|line| !line.is_empty());
    Err(cause.map_or_else(|| format!("gh api failed with {status}"), ToOwned::to_owned))
}

/// Read all of `pipe` on a thread of its own, so a full pipe never stops
/// the child.
fn drain(mut pipe: impl std::io::Read + Send + 'static) -> std::thread::JoinHandle<Vec<u8>> {
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = pipe.read_to_end(&mut bytes);
        bytes
    })
}

#[cfg(test)]
mod tests {
    use super::{Entry, FAILURE_TTL, Repo, SUCCESS_TTL, highest, newer, tag_version};

    #[test]
    fn outdated_tags_read_like_flake_edit() {
        assert_eq!(tag_version("v1.2.3").unwrap().to_string(), "1.2.3");
        assert_eq!(tag_version("refs/tags/v1.2").unwrap().to_string(), "1.2.0");
        assert_eq!(tag_version("release-24").unwrap().to_string(), "24.0.0");
        assert_eq!(
            tag_version("v2.0.0-beta.1").unwrap().to_string(),
            "2.0.0-beta.1"
        );
        assert_eq!(tag_version("nightly"), None);
        let names = ["v1.5.0", "v2.0.0-beta.1", "latest"].map(str::to_owned);
        assert_eq!(highest(&names).as_deref(), Some("v2.0.0-beta.1"));
        assert!(newer("v0.10.1", "v0.9.8"));
        assert!(!newer("v0.9.8", "v0.10.1"));
        assert!(!newer("v0.10.1", "0.10.1"));
    }

    #[test]
    fn outdated_cache_is_due_on_its_cadence() {
        let now = 10 * SUCCESS_TTL;
        assert!(Entry::default().due(now));
        let ok = |age| Entry {
            attempted: Some(now - age),
            succeeded: Some(now - age),
            latest: Some("v1.0.0".to_owned()),
            error: None,
        };
        assert!(!ok(0).due(now));
        assert!(!ok(SUCCESS_TTL - 1).due(now));
        assert!(ok(SUCCESS_TTL).due(now));
        assert!(ok(-1).due(now), "a time in the future is due");
        let failed = |age| Entry {
            attempted: Some(now - age),
            error: Some("x".to_owned()),
            ..ok(SUCCESS_TTL / 2)
        };
        assert!(!failed(FAILURE_TTL - 1).due(now));
        assert!(failed(FAILURE_TTL).due(now));
    }

    #[test]
    fn outdated_repo_names_stay_plain() {
        assert_eq!(
            Repo::new("Gubasso", "Release-Kit").unwrap().slug(),
            "gubasso/release-kit"
        );
        assert_eq!(Repo::new("..", "x"), None);
        assert_eq!(Repo::new("a/b", "x"), None);
        assert_eq!(Repo::new("", "x"), None);
    }
}
