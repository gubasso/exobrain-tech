//! Shared plumbing for the integration suites.
//!
//! A `World` owns one scratch folder holding a faked store, a stub `nix`,
//! a state root, and the git repositories a test builds. Every `exobrain`
//! invocation it makes reaches only that folder: never the real
//! `/nix/store`, the real `nix`, or the developer's state.

// A helper that one integration binary does not call is still shared, and
// a helper unwraps because a broken fixture should abort the test.
#![allow(
    dead_code,
    clippy::unwrap_used,
    reason = "shared test plumbing, where a broken fixture aborts the test"
)]

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use assert_cmd::Command;

/// The variables a CI runner exports, which switch `--daily` off.
pub const CI_VARS: [&str; 6] = [
    "CI",
    "GITHUB_ACTIONS",
    "GITLAB_CI",
    "BUILDKITE",
    "CIRCLECI",
    "TF_BUILD",
];

/// A registration time the stub reports for every path.
pub const REGISTERED: i64 = 1_790_000_000;

/// A size over the default minimum of 50 MB.
pub const BIG: u64 = 60_000_000;

/// The stub `nix`: it logs each call beside itself, fails when
/// `STUB_FAIL` is set, and sleeps for `STUB_SLEEP` seconds. It reports a
/// path as registered only when the path holds a `nar-size` file, so a
/// path without one reads as collected.
const STUB_NIX: &str = r#"#!/bin/sh
echo call >> "$(dirname "$0")/nix.log"
if [ -n "${STUB_FAIL:-}" ]; then echo "error: stub failure" >&2; exit 1; fi
sleep "${STUB_SLEEP:-0}"
printf '{'
sep=
for p in "$@"; do
    case "$p" in /*) ;; *) continue ;; esac
    if [ -f "$p/nar-size" ]; then
        printf '%s"%s":{"narSize":%s,"registrationTime":1790000000}' "$sep" "$p" "$(cat "$p/nar-size")"
    else
        printf '%s"%s":null' "$sep" "$p"
    fi
    sep=,
done
printf '}\n'
"#;

/// A `git` invocation that acts on `dir` alone, with no inherited
/// configuration and no hook variables.
pub fn git(dir: &Path) -> std::process::Command {
    let mut command = std::process::Command::new("git");
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    command
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .args(["-c", "user.name=t", "-c", "user.email=t@t"])
        .current_dir(dir);
    command
}

/// Run `git` in `dir` and require success.
pub fn run_git(dir: &Path, args: &[&str]) {
    let status = git(dir).args(args).output().unwrap();
    assert!(
        status.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&status.stderr)
    );
}

/// One scratch world.
pub struct World {
    dir: tempfile::TempDir,
}

impl World {
    /// A fresh world with an empty store, a stub `nix`, and a state root.
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let world = Self { dir };
        std::fs::create_dir_all(world.store()).unwrap();
        std::fs::create_dir_all(world.root().join("home")).unwrap();
        let stub = world.root().join("bin/nix");
        std::fs::create_dir_all(stub.parent().unwrap()).unwrap();
        std::fs::write(&stub, STUB_NIX).unwrap();
        let mut perms = std::fs::metadata(&stub).unwrap().permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
        std::fs::set_permissions(&stub, perms).unwrap();
        world
    }

    /// The canonical scratch root.
    pub fn root(&self) -> PathBuf {
        std::fs::canonicalize(self.dir.path()).unwrap()
    }

    /// The faked store.
    pub fn store(&self) -> PathBuf {
        self.root().join("store")
    }

    /// The state root `XDG_STATE_HOME` points at.
    pub fn state(&self) -> PathBuf {
        self.root().join("state")
    }

    /// The folder the stamps and locks live in.
    pub fn guard_dir(&self) -> PathBuf {
        self.state().join("exobrain/store-guard")
    }

    /// How many times the stub `nix` ran.
    pub fn nix_calls(&self) -> usize {
        std::fs::read_to_string(self.root().join("bin/nix.log"))
            .map_or(0, |log| log.lines().count())
    }

    /// The stamps in the state folder.
    pub fn stamps(&self) -> Vec<PathBuf> {
        std::fs::read_dir(self.guard_dir()).map_or_else(
            |_| Vec::new(),
            |entries| {
                entries
                    .map(|entry| entry.unwrap().path())
                    .filter(|path| path.extension().is_some_and(|ext| ext == "stamp"))
                    .collect()
            },
        )
    }

    /// A git repository with one commit, and `origin` when `remote` is set.
    pub fn repo(&self, name: &str, remote: Option<&str>) -> PathBuf {
        let dir = self.root().join(name);
        std::fs::create_dir_all(&dir).unwrap();
        run_git(&dir, &["init", "-q", "-b", "main"]);
        std::fs::write(dir.join("README"), "x\n").unwrap();
        run_git(&dir, &["add", "README"]);
        run_git(&dir, &["commit", "-q", "-m", "init"]);
        if let Some(url) = remote {
            run_git(&dir, &["remote", "add", "origin", url]);
        }
        dir
    }

    /// A store entry named `<name>-source`, registered with `size` when set.
    fn entry(&self, name: &str, size: Option<u64>) -> PathBuf {
        let path = self
            .store()
            .join(format!("{}-{name}-source", "0".repeat(32)));
        std::fs::create_dir_all(&path).unwrap();
        if let Some(size) = size {
            std::fs::write(path.join("nar-size"), size.to_string()).unwrap();
        }
        path
    }

    /// Plant a copy whose `.git` is a file holding `text`.
    pub fn plant_file(&self, name: &str, text: &str, size: Option<u64>) -> PathBuf {
        let path = self.entry(name, size);
        std::fs::write(path.join(".git"), text).unwrap();
        path
    }

    /// Plant a copy whose `.git` is a folder, with `origin` when `remote` is set.
    pub fn plant_dir(&self, name: &str, remote: Option<&str>, size: Option<u64>) -> PathBuf {
        let path = self.entry(name, size);
        std::fs::create_dir_all(path.join(".git")).unwrap();
        let mut config = String::from("[core]\n\tbare = false\n");
        if let Some(url) = remote {
            let _ = write!(config, "[remote \"origin\"]\n\turl = {url}\n");
        }
        std::fs::write(path.join(".git/config"), config).unwrap();
        path
    }

    /// An `exobrain store-guard` invocation in `cwd`, inside this world,
    /// with every off switch cleared.
    pub fn guard(&self, cwd: &Path) -> Command {
        Command::from_std(self.guard_std(cwd))
    }

    /// The same invocation as a plain process, for a test that spawns it.
    pub fn guard_std(&self, cwd: &Path) -> std::process::Command {
        let mut command = std::process::Command::new(assert_cmd::cargo::cargo_bin!("exobrain"));
        for (name, _) in std::env::vars_os() {
            if name.to_string_lossy().starts_with("GIT_") {
                command.env_remove(name);
            }
        }
        for var in CI_VARS {
            command.env_remove(var);
        }
        command
            .env_remove("EXOBRAIN_STORE_GUARD")
            .env_remove("STUB_SLEEP")
            .env_remove("STUB_FAIL")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("EXOBRAIN_STORE_DIR", self.store())
            .env("EXOBRAIN_NIX_BIN", self.root().join("bin/nix"))
            .env("XDG_STATE_HOME", self.state())
            .env("HOME", self.root().join("home"))
            .current_dir(cwd)
            .arg("store-guard");
        command
    }
}
