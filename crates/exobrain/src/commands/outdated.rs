//! `exobrain outdated`: report what is behind, change nothing, and exit 0.
//!
//! The age check reads each flake lock offline. The `lastModified` of the
//! nixpkgs node of a lock is the clock of the whole lock, and a pin over
//! [`LIMIT_DAYS`] days old prints one line on stderr. The age check reads
//! no network, no cache, and no state file, calls no `nix`, and writes
//! nothing.
//!
//! The release check judges each root input pinned at a version tag of a
//! GitHub repository against the latest tag of that repository, which
//! [`crate::release_cache`] reads through `gh` once a week.
//!
//! The CLI check and the hook-set check run once per call, on the
//! `.pre-commit-config.yaml` of the current folder. They judge the
//! newest tag of the exobrain-tech repository, which the same release
//! cache reads, against the running binary and against the revisions
//! that the project pins. The set files of a tag are read once, and kept
//! in the set cache of `exobrain hooks`.

use std::path::{Component, Path, PathBuf};
use std::time::Instant;

use serde_json::Value;

use crate::cli::outdated::{DEFAULT_LOCK, OutdatedArgs};
use crate::commands::hooks::{self, Project};
use crate::commands::store_guard::shell_word;
use crate::error::Error;
use crate::output;
use crate::release_cache::{self, Repo};

/// The age limit of a nixpkgs pin, in whole days. A pin of exactly this
/// age stays silent.
pub const LIMIT_DAYS: i64 = 30;

/// The seconds of one day.
const DAY: i64 = 86_400;

/// The depth at which a follows path counts as a cycle.
const MAX_FOLLOWS: usize = 32;

/// The prek config that the hook-set check reads.
const CONFIG: &str = ".pre-commit-config.yaml";

/// The file in the set folder of a tag that holds the time of the last
/// failed read of its sets.
const SETS_FAILED: &str = ".failed";

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
    let loaded: Vec<(PathBuf, Loaded)> = locks
        .into_iter()
        .map(|path| {
            let lock = load(&path);
            (path, lock)
        })
        .collect();
    let pins: Vec<Vec<TagInput>> = loaded
        .iter()
        .map(|(_, lock)| match lock {
            Loaded::Lock(lock) => tag_inputs(lock),
            Loaded::Missing | Loaded::Unreadable(_) => Vec::new(),
        })
        .collect();
    let config = hooks::read_config(Path::new(CONFIG));
    let tech = config
        .as_ref()
        .and_then(hooks::tech_slug)
        .or_else(hooks::own_slug)
        .and_then(|slug| repo_of(&slug));
    let project = config.and_then(Project::of);
    let mut repos: Vec<Repo> = pins.iter().flatten().map(|pin| pin.repo.clone()).collect();
    repos.extend(tech.clone());
    repos.sort();
    repos.dedup();
    let lookups = release_cache::latest(&repos, args.refresh);
    for ((path, lock), pins) in loaded.iter().zip(&pins) {
        if let Some(line) = judge_age(path, named, lock, now) {
            output::note(&line);
        }
        for pin in pins {
            let latest = repos
                .binary_search(&pin.repo)
                .ok()
                .and_then(|at| lookups[at].latest.as_deref());
            if let Some(latest) = latest
                && release_cache::newer(latest, &pin.tag)
            {
                output::note(&release_line(path, pin, latest));
            }
        }
    }
    let latest = tech
        .and_then(|tech| repos.binary_search(&tech).ok())
        .and_then(|at| lookups[at].latest.as_deref());
    let mut failed_sets = None;
    if let Some(latest) = latest {
        if let Some(line) = hooks::cli_line(latest) {
            output::note(&line);
        }
        if let Some(project) = &project {
            let (lines, failed) = hook_lines(project, latest, args.refresh);
            for line in lines {
                output::note(&line);
            }
            failed_sets = failed;
        }
    }
    for (repo, lookup) in repos.iter().zip(&lookups) {
        if let Some(error) = &lookup.failed {
            output::note(&format!(
                "cannot read the latest release of {} ({error}); the next read runs in 1 hour, or now with --refresh",
                repo.slug()
            ));
        }
    }
    if let Some(line) = failed_sets {
        output::note(&line);
    }
    Ok(0)
}

/// The repository of the slug `owner/name`.
fn repo_of(slug: &str) -> Option<Repo> {
    let (owner, name) = slug.split_once('/')?;
    Repo::new(owner, name)
}

/// The lines of the hook-set check of `project` against the newest tag
/// `latest`, and the line of a failed read of its sets.
///
/// The sets come from the set cache. Where it lacks one, the check reads
/// the sets through `gh` within one bounded call time, unless a read of
/// them failed within the hour and `refresh` is unset.
fn hook_lines(project: &Project, latest: &str, refresh: bool) -> (Vec<String>, Option<String>) {
    if !hooks::behind(project, latest) {
        return (Vec::new(), None);
    }
    let dir = hooks::sets_dir(&project.slug, latest);
    let mut failed = None;
    let sets = match dir
        .as_deref()
        .and_then(|dir| hooks::cached_sets(dir, &project.sets))
    {
        Some(sets) => Some(sets),
        None if !refresh && dir.as_deref().is_some_and(failed_recently) => None,
        None => {
            let deadline = Instant::now() + release_cache::TIMEOUT;
            match hooks::load_sets(
                &project.slug,
                latest,
                &project.sets,
                dir.as_deref(),
                deadline,
            ) {
                Ok(sets) => {
                    if let Some(dir) = &dir {
                        let _ = std::fs::remove_file(dir.join(SETS_FAILED));
                    }
                    Some(sets)
                }
                Err(error) => {
                    if let Some(dir) = &dir {
                        let stamp = format!("{}\n", jiff::Timestamp::now().as_second());
                        let _ = std::fs::create_dir_all(dir).and_then(|()| {
                            hooks::write_atomic(&dir.join(SETS_FAILED), stamp.as_bytes())
                        });
                    }
                    failed = Some(format!(
                        "cannot read the hook sets of {} ({error}); the next read runs in 1 hour, or now with --refresh",
                        project.slug
                    ));
                    None
                }
            }
        }
    };
    (
        hooks::project_lines(project, latest, sets.as_deref()),
        failed,
    )
}

/// Whether the set folder `dir` records a failed read within the hour.
fn failed_recently(dir: &Path) -> bool {
    let now = jiff::Timestamp::now().as_second();
    std::fs::read_to_string(dir.join(SETS_FAILED))
        .ok()
        .and_then(|text| text.trim().parse::<i64>().ok())
        .is_some_and(|then| (0..release_cache::FAILURE_TTL).contains(&(now - then)))
}

/// A lock as the command found it.
enum Loaded {
    /// The path does not exist.
    Missing,
    /// The file does not read or does not parse, with the reason.
    Unreadable(String),
    /// The parsed lock.
    Lock(Value),
}

/// Read and parse the lock at `path`.
fn load(path: &Path) -> Loaded {
    match std::fs::read(path) {
        Ok(bytes) => match serde_json::from_slice(&bytes) {
            Ok(lock) => Loaded::Lock(lock),
            Err(e) => Loaded::Unreadable(e.to_string()),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Loaded::Missing,
        Err(e) => Loaded::Unreadable(e.to_string()),
    }
}

/// One root input pinned at a version tag of a GitHub repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagInput {
    /// The name of the input in the root node.
    pub input: String,
    /// The repository of the input.
    pub repo: Repo,
    /// The tag that the input pins.
    pub tag: String,
}

/// Every root input of `lock` that names a node, not a follows path, whose
/// `original` is a GitHub reference at a version tag.
#[must_use]
pub fn tag_inputs(lock: &Value) -> Vec<TagInput> {
    let Some(nodes) = lock.get("nodes") else {
        return Vec::new();
    };
    let Some(inputs) = lock
        .get("root")
        .and_then(Value::as_str)
        .and_then(|root| nodes.get(root))
        .and_then(|root| root.get("inputs"))
        .and_then(Value::as_object)
    else {
        return Vec::new();
    };
    inputs
        .iter()
        .filter_map(|(input, value)| {
            let original = nodes.get(value.as_str()?)?.get("original")?;
            if original.get("type")?.as_str()? != "github" {
                return None;
            }
            let tag = original.get("ref")?.as_str()?;
            if !version_tag(tag) {
                return None;
            }
            let repo = Repo::new(
                original.get("owner")?.as_str()?,
                original.get("repo")?.as_str()?,
            )?;
            Some(TagInput {
                input: input.clone(),
                repo,
                tag: tag.to_owned(),
            })
        })
        .collect()
}

/// Whether `tag` reads `vMAJOR.MINOR.PATCH` or `MAJOR.MINOR.PATCH`.
fn version_tag(tag: &str) -> bool {
    let core = tag.strip_prefix('v').unwrap_or(tag);
    let parts: Vec<&str> = core.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
}

/// The warning for `pin` of the lock at `path`, whose repository is out
/// at `latest`.
fn release_line(path: &Path, pin: &TagInput, latest: &str) -> String {
    let input = shell_word(&pin.input);
    let default = path.as_os_str() == DEFAULT_LOCK;
    let flags = if default {
        String::new()
    } else {
        format!(
            "--flake {} --lock-file {} ",
            shell_word(&folder_of(path)),
            shell_word(&path.display().to_string())
        )
    };
    format!(
        "{}: {} is pinned at {}, and {latest} is out. Move it with flake-edit {flags}update {input} when ready.",
        path.display(),
        pin.input,
        pin.tag
    )
}

/// The line of the age check for the lock at `path`, judged at `now`.
///
/// `None` means silence. `named` tells whether the call named the lock: a
/// missing lock that the call did not name means the project has no flake.
/// The check reads the one file at `path` and nothing else.
#[must_use]
pub fn age_line(path: &Path, named: bool, now: jiff::Timestamp) -> Option<String> {
    judge_age(path, named, &load(path), now)
}

/// The line of the age check for `loaded`, the lock at `path`.
fn judge_age(path: &Path, named: bool, loaded: &Loaded, now: jiff::Timestamp) -> Option<String> {
    let shown = path.display();
    let lock = match loaded {
        Loaded::Lock(lock) => lock,
        Loaded::Missing => {
            return named.then(|| {
                format!("{shown}: no such file, so the age of the nixpkgs pin is unknown.")
            });
        }
        Loaded::Unreadable(reason) => return Some(unreadable(path, reason)),
    };
    let clock = match clock(lock) {
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
    let root_name = lock
        .get("root")
        .and_then(Value::as_str)
        .ok_or(Problem::NoRoot)?;
    let root = nodes.get(root_name).ok_or(Problem::NoRoot)?;
    let input = input_of(root, "nixpkgs").ok_or(Problem::NoNixpkgs)?;
    let name = resolve(nodes, root_name, input, 0).ok_or(Problem::NoNixpkgs)?;
    nodes.get(name).ok_or(Problem::NoNixpkgs)
}

/// The value of the input `name` of `node`.
fn input_of<'a>(node: &'a Value, name: &str) -> Option<&'a Value> {
    node.get("inputs")?.get(name)
}

/// The node name that an input value resolves to: the value itself when
/// it is a node name, or the end of its follows path, which starts at the
/// root node named `root_name`. An empty follows path names the root.
fn resolve<'a>(
    nodes: &'a Value,
    root_name: &'a str,
    input: &'a Value,
    depth: usize,
) -> Option<&'a str> {
    if let Some(name) = input.as_str() {
        return Some(name);
    }
    if depth >= MAX_FOLLOWS {
        return None;
    }
    let mut name = root_name;
    for step in input.as_array()? {
        let value = input_of(nodes.get(name)?, step.as_str()?)?;
        name = resolve(nodes, root_name, value, depth + 1)?;
    }
    Some(name)
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
