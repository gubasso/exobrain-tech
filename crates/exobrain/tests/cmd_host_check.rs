//! `exobrain host-check` against a scratch project, stub tools on a
//! scratch `PATH`, and a scratch cache. The test names carry the
//! requirement of `_docs/specs/SPEC-cli.md` they verify as a prefix, so
//! each `Verify:` filter selects the whole family.

#![allow(clippy::unwrap_used, reason = "a test aborts on a broken fixture")]

use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use tempfile::TempDir;

/// One scratch project, one scratch `PATH` folder, and one scratch cache.
struct Host {
    root: TempDir,
}

impl Host {
    fn new(manifest: &str) -> Self {
        let root = tempfile::tempdir().unwrap();
        for dir in ["project/sub", "bin", "cache"] {
            std::fs::create_dir_all(root.path().join(dir)).unwrap();
        }
        let host = Self { root };
        host.write_manifest(manifest);
        host
    }

    fn project(&self) -> PathBuf {
        self.root.path().join("project")
    }

    fn bin(&self) -> PathBuf {
        self.root.path().join("bin")
    }

    fn cache(&self) -> PathBuf {
        self.root.path().join("cache")
    }

    fn write_manifest(&self, text: &str) {
        std::fs::write(self.project().join("host-requirements.toml"), text).unwrap();
    }

    /// A stub tool that logs each call beside itself and prints `output`.
    fn tool(&self, name: &str, output: &str) -> PathBuf {
        let path = self.bin().join(name);
        let script = format!("#!/bin/sh\necho \"$*\" >> \"$0.log\"\nprintf '%s\\n' '{output}'\n");
        std::fs::write(&path, script).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    fn calls(&self, name: &str) -> Vec<String> {
        std::fs::read_to_string(self.bin().join(format!("{name}.log")))
            .unwrap_or_default()
            .lines()
            .map(str::to_string)
            .collect()
    }

    fn run_in(&self, dir: &Path, args: &[&str]) -> std::process::Output {
        Command::cargo_bin("exobrain")
            .unwrap()
            .arg("host-check")
            .args(args)
            .current_dir(dir)
            .env_clear()
            .env("PATH", self.bin())
            .env("XDG_CACHE_HOME", self.cache())
            .output()
            .unwrap()
    }

    fn run(&self, args: &[&str]) -> std::process::Output {
        self.run_in(&self.project(), args)
    }
}

fn stderr_of(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

const PLAN_EXAMPLE: &str = r#"schema = 1

[tools.git]
version = ">=2.36.0"

[tools.gh]
version = ">=2.57.0"

[tools.prek]
version = "*"
"#;

// cli:host-check-validates-the-file

#[test]
fn validates_the_file_and_passes_the_plan_example() {
    let host = Host::new(PLAN_EXAMPLE);
    let out = host.run(&["--parse-only"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr_of(&out));
    assert!(out.stdout.is_empty() && out.stderr.is_empty());
}

#[test]
fn validates_the_file_and_names_every_problem() {
    let host = Host::new(
        "schema = 1\n[tools.gh]\nversion = \"2.57.0\"\n[tools.git]\nversion = \">=2\"\nscope = \"global\"\n",
    );
    host.tool("gh", "gh version 2.57.0");
    let out = host.run(&["--parse-only"]);
    assert_eq!(out.status.code(), Some(2));
    let err = stderr_of(&out);
    assert!(
        err.contains("host-requirements.toml: tool gh: version \"2.57.0\""),
        "{err}"
    );
    assert!(err.contains("no operator"), "{err}");
    assert!(err.contains("tool git:") && err.contains("scope"), "{err}");
    assert!(host.calls("gh").is_empty(), "--parse-only probed the host");
}

#[test]
fn validates_the_file_and_rejects_a_toml_error_with_its_line() {
    let host = Host::new("schema = 1\n[tools.gh\n");
    let out = host.run(&["--parse-only"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr_of(&out).contains("line 2"), "{}", stderr_of(&out));
}

#[test]
fn validates_the_file_found_in_a_parent_folder() {
    let host = Host::new("schema = 2\n");
    let out = host.run_in(&host.project().join("sub"), &["--parse-only"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr_of(&out).contains("unknown schema 2"));
}

// cli:host-check-checks-the-named-tools

#[test]
fn checks_the_named_tools_or_every_tool() {
    let host = Host::new(PLAN_EXAMPLE);
    host.tool("git", "git version 2.51.0");
    host.tool("prek", "prek 0.5.7");
    let bare = host.run(&[]);
    assert_eq!(bare.status.code(), Some(1));
    assert_eq!(stderr_of(&bare), "host tool gh is missing\n");
    let named = host.run(&["git", "prek"]);
    assert_eq!(named.status.code(), Some(0), "{}", stderr_of(&named));
    assert!(named.stdout.is_empty() && named.stderr.is_empty());
}

#[test]
fn checks_the_named_tools_and_refuses_an_unlisted_name() {
    let host = Host::new(PLAN_EXAMPLE);
    host.tool("git", "git version 2.51.0");
    let out = host.run(&["git", "jq"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr_of(&out).contains("host tool jq is not listed in"));
    assert!(
        host.calls("git").is_empty(),
        "an unlisted name still probed"
    );
}

#[test]
fn checks_the_named_tools_and_leaves_a_wildcard_version_to_the_tool() {
    let host = Host::new(PLAN_EXAMPLE);
    host.tool("prek", "no version at all");
    let out = host.run(&["prek"]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr_of(&out));
    assert!(host.calls("prek").is_empty(), "a `*` tool was probed");
}

#[test]
fn checks_the_named_tools_with_a_two_part_version_and_a_version_cmd() {
    let host = Host::new("schema = 1\n[tools.py]\nversion = \">=3.12.0\"\nversion_cmd = \"-V\"\n");
    host.tool("py", "Python 3.12");
    let out = host.run(&[]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr_of(&out));
    assert_eq!(host.calls("py"), ["-V"]);
}

// cli:host-check-reports-every-failure

#[test]
fn reports_every_failure_one_line_each() {
    let host = Host::new(
        "schema = 1\n[tools.gh]\nversion = \">=2.57.0\"\n[tools.git]\n[tools.jq]\nversion = \">=1.7\"\n",
    );
    host.tool("gh", "gh version 2.40.1 (2023-12-13)");
    host.tool("jq", "nothing useful");
    let out = host.run(&[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert_eq!(
        stderr_of(&out),
        concat!(
            "host tool gh 2.40.1 does not match >=2.57.0 (host-requirements.toml)\n",
            "host tool git is missing\n",
            "host tool jq printed no version for `jq --version` (host-requirements.toml)\n",
        )
    );
}

// cli:host-check-caches-versions

#[test]
fn caches_versions_so_a_second_run_spawns_no_probe() {
    let host = Host::new("schema = 1\n[tools.gh]\nversion = \">=2.57.0\"\n");
    host.tool("gh", "gh version 2.57.1");
    assert_eq!(host.run(&[]).status.code(), Some(0));
    assert_eq!(host.run(&[]).status.code(), Some(0));
    assert_eq!(host.calls("gh").len(), 1, "the second run probed again");
}

#[test]
fn caches_versions_and_misses_after_an_upgrade_in_place() {
    let host = Host::new("schema = 1\n[tools.gh]\nversion = \">=2.57.0\"\n");
    host.tool("gh", "gh version 2.57.1");
    assert_eq!(host.run(&[]).status.code(), Some(0));
    // A package manager rewrites the binary at the same path.
    std::thread::sleep(std::time::Duration::from_millis(20));
    host.tool("gh", "gh version 2.40.1");
    let out = host.run(&[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr_of(&out).contains("gh 2.40.1 does not match"));
}

// cli:host-check-exit-codes

#[test]
fn host_check_exit_codes_without_a_file() {
    let host = Host::new("schema = 1\n");
    std::fs::remove_file(host.project().join("host-requirements.toml")).unwrap();
    let out = host.run(&[]);
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr_of(&out).contains("no host-requirements.toml in"));
}
