//! `exobrain hooks` against a scratch project, scratch sets, a scratch
//! state root, and a stub `gh` first on a scratch `PATH`. The stub serves
//! the releases of a scratch folder and logs each call, so no test reaches
//! the network. The test names carry the requirement of
//! `_docs/specs/SPEC-hooks.md` they verify as a prefix, so each `Verify:`
//! filter selects the whole family.

#![allow(clippy::unwrap_used, reason = "a test aborts on a broken fixture")]

use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;
use std::process::Output;

use assert_cmd::Command;
use tempfile::TempDir;

/// The stub `gh`: it logs each call, fails like a host with no network
/// when `STUB_OFFLINE` is set, and serves `releases/latest` and
/// `releases/<tag>/<set>.yaml` beside its folder.
const STUB_GH: &str = r#"#!/bin/sh
here="$(dirname "$0")"
echo "$*" >> "$here/gh.log"
if [ -n "${STUB_OFFLINE:-}" ]; then echo "error connecting to api.github.com" >&2; exit 1; fi
releases="$here/../releases"
case "$*" in
    *releases/latest*)
        if [ -f "$releases/latest" ]; then cat "$releases/latest"; exit 0; fi
        echo "gh: Not Found (HTTP 404)" >&2; exit 1 ;;
esac
for arg in "$@"; do case "$arg" in repos/*/contents/*) path="$arg" ;; esac; done
file="${path##*/}"; file="${file%%\?*}"; ref="${path##*ref=}"
if [ -f "$releases/$ref/$file" ]; then cat "$releases/$ref/$file"; exit 0; fi
echo "gh: Not Found (HTTP 404)" >&2; exit 1
"#;

/// The `common` set of the scratch release `v1.2.0`.
const COMMON_V12: &str = include_str!("fixtures/hooks/common.yaml");

/// The `rust` set of the scratch release `v1.2.0`.
const RUST_V12: &str = include_str!("fixtures/hooks/rust.yaml");

/// A project config at exobrain-tech v1.1.0, with typos behind its set,
/// local args and stages, a repository no set holds, and a release-kit
/// block that pins cargo-machete behind the `rust` set.
const PROJECT: &str = include_str!("fixtures/hooks/project.yaml");

struct World {
    root: TempDir,
}

impl World {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        for dir in ["project", "bin", "state", "sets", "releases/v1.2.0"] {
            std::fs::create_dir_all(root.path().join(dir)).unwrap();
        }
        let gh = root.path().join("bin/gh");
        std::fs::write(&gh, STUB_GH).unwrap();
        std::fs::set_permissions(&gh, std::fs::Permissions::from_mode(0o755)).unwrap();
        let world = Self { root };
        world.write("sets/common.yaml", COMMON_V12);
        world.write("sets/rust.yaml", RUST_V12);
        world.write("releases/latest", "v1.2.0\n");
        world.write("releases/v1.2.0/common.yaml", COMMON_V12);
        world.write("releases/v1.2.0/rust.yaml", RUST_V12);
        world.write("project/.pre-commit-config.yaml", PROJECT);
        world
    }

    fn path(&self, rel: &str) -> PathBuf {
        self.root.path().join(rel)
    }

    fn write(&self, rel: &str, text: &str) {
        std::fs::write(self.path(rel), text).unwrap();
    }

    fn read(&self, rel: &str) -> String {
        std::fs::read_to_string(self.path(rel)).unwrap()
    }

    fn config(&self) -> String {
        self.read("project/.pre-commit-config.yaml")
    }

    fn gh_calls(&self) -> Vec<String> {
        std::fs::read_to_string(self.path("bin/gh.log"))
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect()
    }

    fn run(&self, args: &[&str], env: &[(&str, &str)]) -> Output {
        let path = format!(
            "{}:{}",
            self.path("bin").display(),
            std::env::var("PATH").unwrap_or_default()
        );
        let mut cmd = Command::cargo_bin("exobrain").unwrap();
        cmd.current_dir(self.path("project"))
            .env("PATH", path)
            .env("XDG_STATE_HOME", self.path("state"))
            .env("HOME", self.path("state"))
            .env_remove("STUB_OFFLINE")
            .args(args);
        for (key, value) in env {
            cmd.env(key, value);
        }
        cmd.output().unwrap()
    }

    fn check(&self, sets: &[&str]) -> Output {
        let dir = self.path("sets");
        let mut args = vec!["hooks", "check", "--sets-dir", dir.to_str().unwrap()];
        for set in sets {
            args.extend(["--set", set]);
        }
        args.push(".pre-commit-config.yaml");
        self.run(&args, &[])
    }
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn code(output: &Output) -> i32 {
    output.status.code().unwrap()
}

/// The project with typos at the revision of its set.
fn aligned(world: &World) {
    world.write(
        "project/.pre-commit-config.yaml",
        &PROJECT.replace("'v1.40.0'", "'v1.52.2'"),
    );
}

// --- check ---

#[test]
fn hooks_check_compares_shared_revisions_and_passes_when_they_match() {
    let world = World::new();
    aligned(&world);
    let out = world.check(&["common", "rust"]);
    assert_eq!(code(&out), 0, "{}", stderr(&out));
    assert!(out.stdout.is_empty() && out.stderr.is_empty());
}

#[test]
fn hooks_check_compares_shared_revisions_and_names_the_one_that_differs() {
    let world = World::new();
    let out = world.check(&["common", "rust"]);
    assert_eq!(code(&out), 1);
    let err = stderr(&out);
    assert!(
        err.contains("https://github.com/crate-ci/typos pins v1.40.0, and set common of exobrain-tech v1.1.0 pins v1.52.2"),
        "{err}"
    );
    assert!(err.contains("fix: exobrain hooks sync"), "{err}");
    assert_eq!(err.lines().count(), 2, "only typos differs: {err}");
}

#[test]
fn hooks_check_compares_shared_revisions_and_ignores_a_repository_no_set_holds() {
    let world = World::new();
    aligned(&world);
    world.write(
        "project/.pre-commit-config.yaml",
        &world.config().replace("rev: v0.0.1", "rev: v9.9.9"),
    );
    assert_eq!(code(&world.check(&["common"])), 0);
}

#[test]
fn hooks_check_compares_shared_revisions_and_ignores_every_other_field() {
    let world = World::new();
    aligned(&world);
    // The project's typos entry differs from the set in args, stages, and
    // files, and its gitleaks entry adds an exclude.
    let out = world.check(&["common", "rust"]);
    assert_eq!(code(&out), 0, "{}", stderr(&out));
}

#[test]
fn hooks_check_compares_shared_revisions_and_skips_the_exobrain_tech_entry() {
    let world = World::new();
    aligned(&world);
    let set = format!(
        "{COMMON_V12}  - repo: https://github.com/gubasso/exobrain-tech\n    rev: v9.0.0\n    hooks:\n      - id: no-path-flake-reference\n"
    );
    world.write("sets/common.yaml", &set);
    assert_eq!(code(&world.check(&["common"])), 0);
}

#[test]
fn hooks_check_exit_codes_on_a_missing_set_and_a_malformed_config() {
    let world = World::new();
    let out = world.check(&["python"]);
    assert_eq!(code(&out), 2);
    assert!(stderr(&out).contains("set python"), "{}", stderr(&out));
    world.write(
        "project/.pre-commit-config.yaml",
        "repos:\n  - repo: https://github.com/x/y\n",
    );
    let out = world.check(&["common"]);
    assert_eq!(code(&out), 2);
    assert!(stderr(&out).contains("has no `rev`"), "{}", stderr(&out));
}

#[test]
fn hooks_skip_rendered_blocks_in_check() {
    let world = World::new();
    aligned(&world);
    // cargo-machete sits at v0.9.0 inside the release-kit block, and the
    // rust set pins v0.10.1.
    let out = world.check(&["rust"]);
    assert_eq!(code(&out), 0, "{}", stderr(&out));
}

#[test]
fn hooks_read_github_only_through_gh_and_check_never_does() {
    let world = World::new();
    let out = world.run(
        &[
            "hooks",
            "check",
            "--sets-dir",
            world.path("sets").to_str().unwrap(),
            "--set",
            "common",
            ".pre-commit-config.yaml",
        ],
        &[("STUB_OFFLINE", "1")],
    );
    assert_eq!(code(&out), 1, "{}", stderr(&out));
    assert!(world.gh_calls().is_empty(), "check reads no network");
    world.run(&["hooks", "sync", "--to", "v1.2.0"], &[]);
    let calls = world.gh_calls();
    assert!(!calls.is_empty());
    assert!(
        calls
            .iter()
            .all(|call| call.starts_with("api ") && call.contains("repos/gubasso/exobrain-tech/")),
        "{calls:?}"
    );
}

/// One failing sync: its name, its arguments, and its environment.
type Case<'a> = (&'a str, &'a [&'a str], &'a [(&'a str, &'a str)]);

// --- sync ---

#[test]
fn hooks_skip_rendered_blocks_in_sync() {
    let world = World::new();
    let out = world.run(&["hooks", "sync", "--to", "v1.2.0"], &[]);
    assert_eq!(code(&out), 0, "{}", stderr(&out));
    let config = world.config();
    assert!(config.contains(
        "# BEGIN release-kit\n  - repo: https://github.com/bnjbvr/cargo-machete\n    rev: v0.9.0\n"
    ));
}

#[test]
fn hooks_sync_rewrites_only_revisions_and_keeps_comments_and_order() {
    let world = World::new();
    let out = world.run(&["hooks", "sync", "--to", "v1.2.0"], &[]);
    assert_eq!(code(&out), 0, "{}", stderr(&out));
    let expected = PROJECT
        .replace("rev: v1.1.0", "rev: v1.2.0")
        .replace("'v1.40.0'", "'v1.52.2'");
    assert_eq!(world.config(), expected);
    assert!(
        stderr(&out).contains("v1.40.0 -> v1.52.2"),
        "{}",
        stderr(&out)
    );
    let after = world.check(&["common", "rust"]);
    assert_eq!(code(&after), 0, "{}", stderr(&after));
}

#[test]
fn hooks_sync_rewrites_only_revisions_and_sets_the_exobrain_tech_rev_to_the_tag() {
    let world = World::new();
    // A set that states a revision of exobrain-tech never moves the entry.
    let set = format!(
        "{COMMON_V12}  - repo: https://github.com/gubasso/exobrain-tech\n    rev: v0.0.9\n    hooks:\n      - id: no-path-flake-reference\n"
    );
    world.write("releases/v1.2.0/common.yaml", &set);
    let out = world.run(&["hooks", "sync", "--to", "v1.2.0"], &[]);
    assert_eq!(code(&out), 0, "{}", stderr(&out));
    let config = world.config();
    assert!(
        config.contains("exobrain-tech\n    rev: v1.2.0\n"),
        "{config}"
    );
    assert!(!config.contains("v0.0.9"));
}

#[test]
fn hooks_sync_rewrites_only_revisions_and_moves_to_the_latest_release() {
    let world = World::new();
    let out = world.run(&["hooks", "sync"], &[]);
    assert_eq!(code(&out), 0, "{}", stderr(&out));
    assert!(world.config().contains("exobrain-tech\n    rev: v1.2.0\n"));
    let calls = world.gh_calls();
    assert!(
        calls[0].contains("repos/gubasso/exobrain-tech/releases/latest"),
        "{calls:?}"
    );
}

#[test]
fn hooks_sync_writes_all_or_nothing_on_each_failure() {
    let world = World::new();
    let cases: [Case<'_>; 3] = [
        ("an unknown tag", &["hooks", "sync", "--to", "v7.0.0"], &[]),
        (
            "a failed download",
            &["hooks", "sync", "--to", "v1.2.0"],
            &[("STUB_OFFLINE", "1")],
        ),
        (
            "no latest release",
            &["hooks", "sync"],
            &[("STUB_OFFLINE", "1")],
        ),
    ];
    for (case, args, env) in cases {
        let out = world.run(args, env);
        assert_eq!(code(&out), 1, "{case}: {}", stderr(&out));
        assert!(
            stderr(&out).contains("is unchanged"),
            "{case}: {}",
            stderr(&out)
        );
        assert_eq!(world.config(), PROJECT, "{case}");
    }
    // A set that the release lacks.
    world.write(
        "project/.pre-commit-config.yaml",
        &PROJECT.replace("--set, rust]", "--set, rust, --set, python]"),
    );
    let before = world.config();
    let out = world.run(&["hooks", "sync", "--to", "v1.2.0"], &[]);
    assert_eq!(code(&out), 1);
    assert!(stderr(&out).contains("set python"), "{}", stderr(&out));
    assert_eq!(world.config(), before);
    // A malformed config.
    let malformed = format!("{PROJECT}\t- repo: local\n");
    world.write("project/.pre-commit-config.yaml", &malformed);
    let out = world.run(&["hooks", "sync", "--to", "v1.2.0"], &[]);
    assert_eq!(code(&out), 1);
    assert_eq!(world.config(), malformed);
    assert!(
        world
            .gh_calls()
            .iter()
            .all(|call| !call.contains("v1.2.0/python"))
    );
}

#[test]
fn hooks_sync_writes_all_or_nothing_and_leaves_no_temporary_file() {
    let world = World::new();
    let out = world.run(&["hooks", "sync", "--to", "v1.2.0"], &[]);
    assert_eq!(code(&out), 0, "{}", stderr(&out));
    let names: Vec<_> = std::fs::read_dir(world.path("project"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(names, [".pre-commit-config.yaml"]);
}

// --- The verbs ---

#[test]
fn hooks_carries_only_check_and_sync() {
    let world = World::new();
    let out = world.run(&["hooks", "status"], &[]);
    assert_eq!(code(&out), 2, "{}", stderr(&out));
    assert!(
        stderr(&out).contains("unrecognized subcommand 'status'"),
        "{}",
        stderr(&out)
    );
    let help = world.run(&["hooks", "--help"], &[]);
    assert_eq!(code(&help), 0, "{}", stderr(&help));
    let text = String::from_utf8(help.stdout).unwrap();
    let verbs: Vec<&str> = text
        .lines()
        .skip_while(|line| !line.starts_with("Commands:"))
        .skip(1)
        .take_while(|line| !line.trim().is_empty())
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    assert_eq!(verbs, ["check", "sync", "help"], "{text}");
}
