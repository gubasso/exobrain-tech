//! This checkout's identity, and the attribution of one store copy to it.
//!
//! A copy whose `.git` is a file is the copy of a linked worktree or of a
//! checkout with a separate git dir; its `gitdir:` line names the git dir
//! it came from. A copy whose `.git` is a folder is the copy of a main
//! checkout or a clone, and only its `remote.origin.url` can tie it to this
//! checkout. Anything that cannot be decided is reported as origin
//! unknown, never guessed.

use std::path::{Component, Path, PathBuf};
use std::process::Command;

use crate::error::Error;
use crate::store::{Candidate, GitKind};

/// The checkout the guard runs for.
#[derive(Debug, Clone)]
pub struct Checkout {
    /// The canonical top level of the work tree.
    pub root: PathBuf,
    /// The canonical git common dir, shared by every worktree.
    pub common_dir: PathBuf,
    /// The normalized `remote.origin.url`, where one is set.
    pub remote: Option<String>,
}

/// What a store copy is to this checkout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attribution {
    /// A copy of this checkout, with a human description of its origin.
    Ours(String),
    /// A copy whose origin cannot be decided, with the reason.
    Unknown(String),
    /// A copy of something else.
    Foreign,
}

/// A `git` command with the variables a git hook exports removed, so it
/// judges the folder it runs in.
fn git() -> Command {
    let mut command = Command::new("git");
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    command
}

/// Identify the checkout that holds the working folder.
///
/// # Errors
///
/// [`Error::NotGit`] when the folder is not inside a git work tree.
pub fn checkout() -> Result<Checkout, Error> {
    let output = git()
        .args([
            "rev-parse",
            "--path-format=absolute",
            "--git-common-dir",
            "--show-toplevel",
        ])
        .output()
        .map_err(|e| Error::NotGit(format!("git did not run: {e}")))?;
    let text = String::from_utf8_lossy(&output.stdout);
    let mut lines = text.lines();
    let (Some(common), Some(root), true) = (lines.next(), lines.next(), output.status.success())
    else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::NotGit(
            stderr.lines().next().unwrap_or("").to_owned(),
        ));
    };
    let canonical = |path: &str| {
        std::fs::canonicalize(path).map_err(|source| Error::Io {
            what: format!("canonicalize {path}"),
            source,
        })
    };
    let remote = git()
        .args(["config", "--get", "remote.origin.url"])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| normalize_url(String::from_utf8_lossy(&out.stdout).trim()))
        .filter(|url| !url.is_empty());
    Ok(Checkout {
        root: canonical(root)?,
        common_dir: canonical(common)?,
        remote,
    })
}

/// Attribute one store copy to `checkout`, or not.
#[must_use]
pub fn attribute(candidate: &Candidate, checkout: &Checkout) -> Attribution {
    match candidate.git {
        GitKind::File => attribute_file(&candidate.path.as_std_path().join(".git"), checkout),
        GitKind::Dir => attribute_dir(&candidate.path.as_std_path().join(".git"), checkout),
    }
}

fn attribute_file(dot_git: &Path, checkout: &Checkout) -> Attribution {
    let Ok(text) = std::fs::read_to_string(dot_git) else {
        return Attribution::Unknown("unreadable .git file".to_owned());
    };
    // Git reads a gitfile as `gitdir: <path>` from the first byte, with only
    // trailing whitespace after the path, and rejects anything else.
    let Some(pointer) = text
        .strip_prefix("gitdir: ")
        .map(str::trim_end)
        .filter(|p| !p.is_empty() && !p.contains('\n'))
    else {
        return Attribution::Unknown("malformed .git file".to_owned());
    };
    let pointer = Path::new(pointer);
    if !pointer.is_absolute() {
        return Attribution::Unknown(format!("relative gitdir {}", pointer.display()));
    }
    let pointer = normalize_lexically(pointer);
    if pointer == checkout.common_dir {
        return Attribution::Ours("main checkout".to_owned());
    }
    let worktrees = checkout.common_dir.join("worktrees");
    let Ok(rest) = pointer.strip_prefix(&worktrees) else {
        return Attribution::Foreign;
    };
    let Some(Component::Normal(name)) = rest.components().next() else {
        return Attribution::Foreign;
    };
    let name = name.to_string_lossy();
    if worktrees.join(&*name).exists() {
        Attribution::Ours(format!("worktree {name}"))
    } else {
        Attribution::Ours(format!("worktree {name} (removed)"))
    }
}

fn attribute_dir(dot_git: &Path, checkout: &Checkout) -> Attribution {
    let config = dot_git.join("config");
    let output = git()
        .arg("config")
        .arg("--file")
        .arg(&config)
        .args(["--get", "remote.origin.url"])
        .output();
    let remote = match output {
        // Exit 1 is git's "the key is not set".
        Ok(out) if out.status.success() => {
            normalize_url(String::from_utf8_lossy(&out.stdout).trim())
        }
        Ok(out) if out.status.code() == Some(1) && config.is_file() => String::new(),
        _ => return Attribution::Unknown("unreadable .git/config".to_owned()),
    };
    if remote.is_empty() {
        return Attribution::Unknown("no remote.origin.url".to_owned());
    }
    match &checkout.remote {
        Some(ours) if *ours == remote => Attribution::Ours(format!("clone of {remote}")),
        _ => Attribution::Foreign,
    }
}

/// Resolve `.` and `..` in an absolute path without touching the
/// filesystem, because the folder it names may already be gone.
#[must_use]
pub fn normalize_lexically(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

/// A remote URL reduced to host and path: no scheme, no user, no trailing
/// slash, no trailing `.git`, and an scp-style `host:path` read as
/// `host/path`.
#[must_use]
pub fn normalize_url(url: &str) -> String {
    let url = url.trim();
    let (rest, had_scheme) = url
        .split_once("://")
        .map_or((url, false), |(_, rest)| (rest, true));
    let rest = match rest.split_once('@') {
        Some((user, host)) if !user.contains('/') => host,
        _ => rest,
    };
    let rest = if had_scheme || rest.starts_with('/') {
        rest.to_owned()
    } else {
        rest.replacen(':', "/", 1)
    };
    let rest = rest.trim_end_matches('/');
    rest.strip_suffix(".git")
        .unwrap_or(rest)
        .trim_end_matches('/')
        .to_owned()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{normalize_lexically, normalize_url};

    #[test]
    fn remote_forms_of_one_repository_normalize_equal() {
        let forms = [
            "https://github.com/gubasso/exobrain-tech",
            "https://github.com/gubasso/exobrain-tech.git",
            "https://github.com/gubasso/exobrain-tech/",
            "git@github.com:gubasso/exobrain-tech.git",
            "ssh://git@github.com/gubasso/exobrain-tech",
        ];
        for form in forms {
            assert_eq!(
                normalize_url(form),
                "github.com/gubasso/exobrain-tech",
                "{form}"
            );
        }
        assert_ne!(
            normalize_url("https://github.com/other/x"),
            normalize_url(forms[0])
        );
    }

    #[test]
    fn a_path_normalizes_without_the_filesystem() {
        assert_eq!(
            normalize_lexically(Path::new("/a/b/./c/../d")),
            Path::new("/a/b/d")
        );
    }
}
