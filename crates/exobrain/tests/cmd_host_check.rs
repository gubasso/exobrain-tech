//! `exobrain host-check` against a scratch project, stub tools on a
//! scratch `PATH`, and a scratch cache. The test names carry the
//! requirement of `_docs/specs/SPEC-cli.md` they verify as a prefix, so
//! each `Verify:` filter selects the whole family.

#![allow(clippy::unwrap_used, reason = "a test aborts on a broken fixture")]

use std::ffi::OsStr;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::Output;

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

    /// Every entry of the version cache.
    fn cache_entries(&self) -> Vec<PathBuf> {
        std::fs::read_dir(self.cache().join("exobrain/host-check"))
            .map(|dir| dir.map(|e| e.unwrap().path()).collect())
            .unwrap_or_default()
    }

    fn run(&self, args: &[&str]) -> std::process::Output {
        self.run_in(&self.project(), args)
    }
}

/// One run with its own working folder, `PATH`, and cache.
fn run_with(dir: &Path, path: &OsStr, cache: &Path, args: &[&str]) -> Output {
    Command::cargo_bin("exobrain")
        .unwrap()
        .arg("host-check")
        .args(args)
        .current_dir(dir)
        .env_clear()
        .env("PATH", path)
        .env("XDG_CACHE_HOME", cache)
        .output()
        .unwrap()
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

#[test]
fn host_check_exit_codes_for_each_outcome() {
    let host = Host::new("schema = 1\n[tools.gh]\nversion = \">=2.57.0\"\n[tools.git]\n");
    host.tool("gh", "gh version 2.57.1");
    host.tool("git", "git version 2.51.0");
    assert_eq!(host.run(&[]).status.code(), Some(0), "pass");
    assert_eq!(host.run(&["jq"]).status.code(), Some(2), "unlisted name");
    std::fs::remove_file(host.bin().join("git")).unwrap();
    assert_eq!(host.run(&[]).status.code(), Some(1), "missing");
    host.tool("gh", "gh version 2.40.1");
    assert_eq!(host.run(&["gh"]).status.code(), Some(1), "mismatch");
    host.write_manifest("schema = 9\n");
    assert_eq!(host.run(&[]).status.code(), Some(2), "parse error");
    std::fs::remove_file(host.project().join("host-requirements.toml")).unwrap();
    assert_eq!(host.run(&[]).status.code(), Some(2), "no file");
}

// cli:host-check-validates-the-file, every problem in one run

#[test]
fn validates_the_file_reports_root_and_tool_problems_together() {
    let host = Host::new("schema = 2\nscope = \"x\"\n[tools.gh]\nversion = \"2.57.0\"\n");
    let out = host.run(&["--parse-only"]);
    assert_eq!(out.status.code(), Some(2));
    let err = stderr_of(&out);
    assert_eq!(err.lines().count(), 3, "{err}");
    assert!(err.contains("unknown key \"scope\""), "{err}");
    assert!(err.contains("unknown schema 2"), "{err}");
    assert!(err.contains("tool gh: version \"2.57.0\""), "{err}");
}

#[test]
fn validates_the_file_reports_two_problems_of_one_tool() {
    let host = Host::new(
        "schema = 1\n[tools.gh]\nflavor = \"x\"\nversion = \"2.57.0\"\nversion_cmd = 3\n",
    );
    let out = host.run(&["--parse-only"]);
    assert_eq!(out.status.code(), Some(2));
    let err = stderr_of(&out);
    assert_eq!(err.lines().count(), 3, "{err}");
    assert!(err.contains("tool gh: unknown key \"flavor\""), "{err}");
    assert!(err.contains("tool gh: version \"2.57.0\""), "{err}");
    assert!(err.contains("tool gh: `version_cmd` is a integer"), "{err}");
}

#[test]
fn validates_the_file_keeps_each_problem_on_one_line() {
    let host = Host::new(concat!(
        "schema = 1\n[tools.\"bad\\nname\"]\n[tools.\"two words\"]\n",
        "[tools.gh]\nversion = \">=1\\n2\"\nversion_cmd = \"-\\nV\"\n",
    ));
    let out = host.run(&["--parse-only"]);
    assert_eq!(out.status.code(), Some(2));
    let err = stderr_of(&out);
    let lines: Vec<&str> = err.lines().collect();
    assert_eq!(lines.len(), 4, "{err}");
    for line in &lines {
        assert!(line.contains("host-requirements.toml: tool "), "{err}");
    }
    assert!(
        err.contains("tool \"bad\\nname\": the name is not a plain command name"),
        "{err}"
    );
    assert!(err.contains("tool \"two words\":"), "{err}");
    assert!(
        err.contains("version_cmd \"-\\nV\" holds a control character"),
        "{err}"
    );
}

// cli:host-check-checks-the-named-tools, the lookup of a shell

#[test]
fn checks_the_named_tools_reading_an_empty_path_entry_as_the_working_folder() {
    let host = Host::new("schema = 1\n[tools.gh]\n");
    host.tool("gh", "gh version 2.57.1");
    let nowhere = host.root.path().join("nowhere");
    let path = std::env::join_paths([PathBuf::new(), nowhere.clone()]).unwrap();
    let out = run_with(&host.bin(), &path, &host.cache(), &[]);
    assert_eq!(out.status.code(), Some(2), "the bin folder holds no file");
    // From the project, whose sub folder holds the tool, an empty entry
    // finds it, and a PATH without the entry does not.
    let sub = host.project().join("sub");
    std::fs::copy(host.bin().join("gh"), sub.join("gh")).unwrap();
    let out = run_with(&sub, &path, &host.cache(), &[]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr_of(&out));
    let out = run_with(&sub, nowhere.as_os_str(), &host.cache(), &[]);
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn checks_the_named_tools_skipping_a_candidate_it_cannot_execute() {
    let host = Host::new("schema = 1\n[tools.gh]\nversion = \">=2.57.0\"\n");
    let first = host.root.path().join("first");
    std::fs::create_dir_all(&first).unwrap();
    let blocked = first.join("gh");
    std::fs::write(
        &blocked,
        "#!/bin/sh
exit 0
",
    )
    .unwrap();
    std::fs::set_permissions(&blocked, std::fs::Permissions::from_mode(0o001)).unwrap();
    let only_first = first.as_os_str().to_owned();
    let out = run_with(&host.project(), &only_first, &host.cache(), &[]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(stderr_of(&out), "host tool gh is missing\n");
    host.tool("gh", "gh version 2.57.1");
    let both = std::env::join_paths([first, host.bin()]).unwrap();
    let out = run_with(&host.project(), &both, &host.cache(), &[]);
    assert_eq!(out.status.code(), Some(0), "{}", stderr_of(&out));
    assert_eq!(host.calls("gh").len(), 1);
}

// cli:host-check-caches-versions, a cache that fails stays silent

#[test]
fn caches_versions_silently_without_a_writable_cache() {
    let host = Host::new("schema = 1\n[tools.gh]\nversion = \">=2.57.0\"\n");
    host.tool("gh", "gh version 2.57.1");
    let not_a_folder = host.root.path().join("cache-file");
    std::fs::write(&not_a_folder, "").unwrap();
    for _ in 0..2 {
        let out = run_with(&host.project(), host.bin().as_os_str(), &not_a_folder, &[]);
        assert_eq!(out.status.code(), Some(0));
        assert!(out.stderr.is_empty(), "{}", stderr_of(&out));
    }
    assert_eq!(host.calls("gh").len(), 2, "no cache, so each run probes");
}

#[test]
fn caches_versions_probing_again_over_a_malformed_entry() {
    let host = Host::new("schema = 1\n[tools.gh]\nversion = \">=2.57.0\"\n");
    host.tool("gh", "gh version 2.57.1");
    assert_eq!(host.run(&[]).status.code(), Some(0));
    let entries = host.cache_entries();
    assert_eq!(entries.len(), 1);
    std::fs::write(&entries[0], "not a version\n").unwrap();
    let out = host.run(&[]);
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty(), "{}", stderr_of(&out));
    assert_eq!(host.calls("gh").len(), 2, "a malformed entry was trusted");
    assert_eq!(host.run(&[]).status.code(), Some(0));
    assert_eq!(
        host.calls("gh").len(),
        2,
        "the probe did not repair the entry"
    );
}

#[test]
fn caches_versions_probing_again_over_an_unreadable_entry() {
    let host = Host::new("schema = 1\n[tools.gh]\nversion = \">=2.57.0\"\n");
    host.tool("gh", "gh version 2.57.1");
    assert_eq!(host.run(&[]).status.code(), Some(0));
    let entries = host.cache_entries();
    std::fs::set_permissions(&entries[0], std::fs::Permissions::from_mode(0o000)).unwrap();
    if std::fs::read(&entries[0]).is_ok() {
        // A user who reads any file, such as root, cannot run this case.
        return;
    }
    let out = host.run(&[]);
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty(), "{}", stderr_of(&out));
    assert_eq!(
        host.calls("gh").len(),
        2,
        "an unreadable entry stopped the probe"
    );
}
