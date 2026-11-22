//! `exobrain outdated` against the fixture locks of `tests/fixtures/outdated/`
//! and scratch projects. The age tests call the age check with a fixed
//! `now`, and the command tests run the binary in a scratch folder. The
//! test names carry the requirement of `_docs/specs/SPEC-outdated.md` they
//! verify, and each `Verify:` filter selects its family.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "a test aborts on a broken fixture"
)]

use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::Output;

use assert_cmd::Command;
use exobrain::commands::outdated::age_line;
use jiff::Timestamp;
use tempfile::TempDir;

/// The `lastModified` of every fixture clock.
const PINNED: i64 = 1_790_000_000;

/// The seconds of one day.
const DAY: i64 = 86_400;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/outdated")
        .join(name)
}

/// The time `seconds` after the fixture pin.
fn after(seconds: i64) -> Timestamp {
    Timestamp::from_second(PINNED + seconds).unwrap()
}

/// The age line of a fixture that the call names.
fn line_of(name: &str, now: Timestamp) -> Option<String> {
    age_line(&fixture(name), true, now)
}

/// Write the fixture `nixpkgs.lock` to `dir/rel`, with its clock `days`
/// days before now.
fn write_lock(dir: &Path, rel: &str, days: i64) {
    let text = std::fs::read_to_string(fixture("nixpkgs.lock")).unwrap();
    let stamp = Timestamp::now().as_second() - days * DAY;
    let text = text.replace(&PINNED.to_string(), &stamp.to_string());
    let path = dir.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

/// Run `exobrain outdated <args>` in `dir`.
fn outdated(dir: &Path, args: &[&str]) -> Output {
    Command::new(assert_cmd::cargo::cargo_bin!("exobrain"))
        .arg("outdated")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap()
}

fn stderr_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// Assert that the command exited 0, printed nothing on stdout, and
/// printed exactly `lines` on stderr.
fn assert_lines(out: &Output, lines: &[&str]) {
    let stderr = stderr_of(out);
    assert_eq!(out.status.code(), Some(0), "{stderr}");
    assert!(
        out.stdout.is_empty(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert_eq!(stderr.lines().collect::<Vec<_>>(), lines, "{stderr}");
}

// outdated:warns-when-nixpkgs-ages

#[test]
fn outdated_warns_over_30_days() {
    let line = line_of("nixpkgs.lock", after(41 * DAY)).unwrap();
    let shown = fixture("nixpkgs.lock");
    let folder = shown.parent().unwrap();
    assert_eq!(
        line,
        format!(
            "{}: nixpkgs pin is 41 days old (limit 30). Move it with nix flake update --flake {} when ready.",
            shown.display(),
            folder.display()
        )
    );
    assert!(line_of("nixpkgs.lock", after(31 * DAY)).is_some());
}

#[test]
fn outdated_warns_over_30_days_from_the_binary() {
    let dir = TempDir::new().unwrap();
    write_lock(dir.path(), "flake.lock", 41);
    assert_lines(
        &outdated(dir.path(), &[]),
        &[
            "flake.lock: nixpkgs pin is 41 days old (limit 30). Move it with nix flake update --flake . when ready.",
        ],
    );
}

#[test]
fn outdated_silent_at_30_days() {
    assert_eq!(line_of("nixpkgs.lock", after(30 * DAY)), None);
    assert_eq!(line_of("nixpkgs.lock", after(31 * DAY - 1)), None);
    assert_eq!(line_of("nixpkgs.lock", after(0)), None);
}

#[test]
fn outdated_silent_in_the_future() {
    assert_eq!(line_of("nixpkgs.lock", after(-5 * DAY)), None);
}

#[test]
fn outdated_judges_each_lock_on_its_own() {
    let dir = TempDir::new().unwrap();
    write_lock(dir.path(), "flake.lock", 10);
    write_lock(dir.path(), "nix/flake.lock", 41);
    assert_lines(
        &outdated(dir.path(), &["flake.lock", "nix/flake.lock"]),
        &[
            "nix/flake.lock: nixpkgs pin is 41 days old (limit 30). Move it with nix flake update --flake ./nix when ready.",
        ],
    );
}

#[test]
fn outdated_names_the_folder_of_the_lock() {
    let dir = TempDir::new().unwrap();
    write_lock(dir.path(), "flake.lock", 41);
    write_lock(dir.path(), "nix/flake.lock", 41);
    assert_lines(
        &outdated(dir.path(), &["flake.lock", "nix/flake.lock"]),
        &[
            "flake.lock: nixpkgs pin is 41 days old (limit 30). Move it with nix flake update --flake . when ready.",
            "nix/flake.lock: nixpkgs pin is 41 days old (limit 30). Move it with nix flake update --flake ./nix when ready.",
        ],
    );
}

#[test]
fn outdated_follows_the_root_input() {
    // The decoy node named `nixpkgs` is 100 days older than the clock, so
    // a check that read it would warn at 41 days.
    for name in ["renamed.lock", "follows.lock"] {
        assert_eq!(line_of(name, after(20 * DAY)), None, "{name}");
        let line = line_of(name, after(41 * DAY)).unwrap();
        assert!(
            line.contains("nixpkgs pin is 41 days old"),
            "{name}: {line}"
        );
    }
}

// outdated:always-exits-0

#[test]
fn outdated_without_a_flake_is_silent() {
    let dir = TempDir::new().unwrap();
    assert_lines(&outdated(dir.path(), &[]), &[]);
}

#[test]
fn outdated_missing_file_exits_0() {
    let dir = TempDir::new().unwrap();
    assert_lines(
        &outdated(dir.path(), &["flake.lock"]),
        &["flake.lock: no such file, so the age of the nixpkgs pin is unknown."],
    );
}

#[test]
fn outdated_unparsable_lock_exits_0() {
    let dir = TempDir::new().unwrap();
    std::fs::copy(fixture("not-json.lock"), dir.path().join("flake.lock")).unwrap();
    std::fs::copy(fixture("no-root.lock"), dir.path().join("no-root.lock")).unwrap();
    let out = outdated(dir.path(), &["flake.lock", "no-root.lock"]);
    assert_lines(
        &out,
        &[
            "flake.lock: cannot read the lock (expected ident at line 1 column 2), so the age of the nixpkgs pin is unknown.",
            "no-root.lock: cannot read the lock (no root node), so the age of the nixpkgs pin is unknown.",
        ],
    );
}

#[test]
fn outdated_no_nixpkgs_input_exits_0() {
    let dir = TempDir::new().unwrap();
    std::fs::copy(fixture("no-nixpkgs.lock"), dir.path().join("flake.lock")).unwrap();
    std::fs::copy(
        fixture("broken-follows.lock"),
        dir.path().join("follows.lock"),
    )
    .unwrap();
    assert_lines(
        &outdated(dir.path(), &[]),
        &["flake.lock: the root of the lock has no nixpkgs input, so the lock has no clock."],
    );
    assert_lines(
        &outdated(dir.path(), &["follows.lock"]),
        &["follows.lock: the root of the lock has no nixpkgs input, so the lock has no clock."],
    );
}

#[test]
fn outdated_no_last_modified_exits_0() {
    let dir = TempDir::new().unwrap();
    std::fs::copy(fixture("path-input.lock"), dir.path().join("flake.lock")).unwrap();
    assert_lines(
        &outdated(dir.path(), &[]),
        &["flake.lock: the nixpkgs pin carries no lastModified, so its age is unknown."],
    );
}

// outdated:age-reads-only-the-lock

/// The variable that hands the probe its scratch world.
const PROBE: &str = "EXOBRAIN_OUTDATED_PROBE";

/// A stub that logs each call beside itself, so a call to `nix` or `gh`
/// leaves a trace.
const STUB: &str = "#!/bin/sh\necho \"$0 $*\" >> \"$(dirname \"$0\")/calls.log\"\nexit 1\n";

/// Every file under `dir`, with its length and modification time.
fn snapshot(dir: &Path) -> Vec<(PathBuf, u64, std::time::SystemTime)> {
    let mut found = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(next) = stack.pop() {
        for entry in std::fs::read_dir(&next).unwrap() {
            let path = entry.unwrap().path();
            let meta = std::fs::symlink_metadata(&path).unwrap();
            if meta.is_dir() {
                stack.push(path.clone());
            }
            found.push((path, meta.len(), meta.modified().unwrap()));
        }
    }
    found.sort();
    found
}

/// The child half of `outdated_age_reads_only_the_lock`: it runs the age
/// check alone, in the process environment that the parent set, and prints
/// its line. Outside the parent it does nothing.
#[test]
fn outdated_age_reads_only_the_lock_probe() {
    let Some(world) = std::env::var_os(PROBE) else {
        return;
    };
    let lock = Path::new(&world).join("project/flake.lock");
    let line = age_line(&lock, true, after(41 * DAY)).unwrap_or_default();
    println!("probe-line: {line}");
}

#[test]
fn outdated_age_reads_only_the_lock() {
    let world = TempDir::new().unwrap();
    let root = world.path();
    for sub in ["project", "state", "cache", "home", "bin"] {
        std::fs::create_dir(root.join(sub)).unwrap();
    }
    std::fs::copy(fixture("nixpkgs.lock"), root.join("project/flake.lock")).unwrap();
    for tool in ["nix", "gh"] {
        let stub = root.join("bin").join(tool);
        std::fs::write(&stub, STUB).unwrap();
        std::fs::set_permissions(&stub, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    let before = snapshot(root);
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "outdated_age_reads_only_the_lock_probe",
            "--nocapture",
        ])
        .env_clear()
        .env(PROBE, root)
        .env("PATH", root.join("bin"))
        .env("HOME", root.join("home"))
        .env("XDG_STATE_HOME", root.join("state"))
        .env("XDG_CACHE_HOME", root.join("cache"))
        .current_dir(root.join("project"))
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{stdout}{}", stderr_of(&out));
    assert!(
        stdout.contains("flake.lock: nixpkgs pin is 41 days old (limit 30)"),
        "{stdout}"
    );
    assert_eq!(snapshot(root), before, "the age check wrote a file");
    assert!(
        !root.join("bin/calls.log").exists(),
        "the age check called nix or gh"
    );
}

#[test]
fn outdated_warns_over_30_days_names_the_limit_in_its_help() {
    let dir = TempDir::new().unwrap();
    let out = outdated(dir.path(), &["--help"]);
    let help = String::from_utf8_lossy(&out.stdout);
    assert_eq!(out.status.code(), Some(0), "{help}");
    assert!(help.contains("over 30 days old"), "{help}");
    assert!(help.contains("The limit is fixed."), "{help}");
    assert!(help.contains("`flake.lock`"), "{help}");
}
