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
    for name in ["renamed.lock", "follows.lock", "empty-follows.lock"] {
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
    for name in [
        "no-root.lock",
        "missing-root-field.lock",
        "wrong-root-type.lock",
    ] {
        std::fs::copy(fixture(name), dir.path().join(name)).unwrap();
    }
    let out = outdated(
        dir.path(),
        &[
            "flake.lock",
            "no-root.lock",
            "missing-root-field.lock",
            "wrong-root-type.lock",
        ],
    );
    assert_lines(
        &out,
        &[
            "flake.lock: cannot read the lock (expected ident at line 1 column 2), so the age of the nixpkgs pin is unknown.",
            "no-root.lock: cannot read the lock (no root node), so the age of the nixpkgs pin is unknown.",
            "missing-root-field.lock: cannot read the lock (no root node), so the age of the nixpkgs pin is unknown.",
            "wrong-root-type.lock: cannot read the lock (no root node), so the age of the nixpkgs pin is unknown.",
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

/// A stub that logs each call beside itself and names itself on stderr,
/// so a call to `nix` or `gh` leaves a trace. It finds its folder with a
/// parameter expansion, because its `PATH` holds no other tool.
const STUB: &str =
    "#!/bin/sh\necho \"$0 $*\" >> \"${0%/*}/calls.log\"\necho \"stub $0 called\" >&2\nexit 1\n";

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
    // The positive control: under the same PATH, each stub logs its call.
    for tool in ["nix", "gh"] {
        let control = std::process::Command::new("/bin/sh")
            .args(["-c", &format!("{tool} --version")])
            .env_clear()
            .env("PATH", root.join("bin"))
            .output()
            .unwrap();
        assert!(!control.status.success(), "the {tool} stub exits 1");
        let log = std::fs::read_to_string(root.join("bin/calls.log")).unwrap();
        assert!(log.contains(&format!("/{tool} --version")), "{log}");
        std::fs::remove_file(root.join("bin/calls.log")).unwrap();
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
    let stderr = stderr_of(&out);
    assert!(out.status.success(), "{stdout}{stderr}");
    assert!(!stderr.contains("stub "), "{stderr}");
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

// The release check. Each test runs the binary in a scratch world whose
// `PATH` holds only the stub `gh`, and whose `XDG_STATE_HOME` is its own.

/// The stub `gh`: it logs each call beside itself, sleeps for
/// `STUB_SLEEP` seconds, fails when `STUB_FAIL` is set, and prints the
/// file `tags/<owner>/<repo>` beside itself for the first page of tags.
const STUB_GH: &str = r#"#!/bin/sh
echo "$*" >> "${0%/*}/gh.log"
if [ -n "${STUB_SLEEP:-}" ]; then @SLEEP@ "$STUB_SLEEP"; fi
if [ -n "${STUB_FAIL:-}" ]; then echo "error: stub failure" >&2; exit 1; fi
case "$2" in
repos/*"&page=1")
    p=${2#repos/}; p=${p%%/tags*}
    f="${0%/*}/tags/${p%%/*}/${p#*/}"
    if [ -f "$f" ]; then @CAT@ "$f"; fi
    ;;
esac
"#;

/// One scratch world for the release check.
struct Gh {
    dir: TempDir,
}

impl Gh {
    /// A world with the stub `gh` on its `PATH`.
    fn new() -> Self {
        let world = Self::bare();
        let stub = STUB_GH
            .replace("@SLEEP@", &tool("sleep"))
            .replace("@CAT@", &tool("cat"));
        let path = world.bin().join("gh");
        std::fs::write(&path, stub).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        world
    }

    /// A world with an empty `PATH` folder.
    fn bare() -> Self {
        let world = Self {
            dir: TempDir::new().unwrap(),
        };
        for sub in ["bin", "state", "home"] {
            std::fs::create_dir_all(world.root().join(sub)).unwrap();
        }
        world
    }

    fn root(&self) -> PathBuf {
        std::fs::canonicalize(self.dir.path()).unwrap()
    }

    fn bin(&self) -> PathBuf {
        self.root().join("bin")
    }

    /// A project folder `name` inside the world.
    fn project(&self, name: &str) -> PathBuf {
        let dir = self.root().join(name);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Make the stub report `tags` for `slug`, newest first.
    fn tags(&self, slug: &str, tags: &[&str]) {
        let path = self.bin().join("tags").join(slug);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut text = tags.join("\n");
        text.push('\n');
        std::fs::write(path, text).unwrap();
    }

    /// The calls of the stub `gh`.
    fn calls(&self) -> Vec<String> {
        std::fs::read_to_string(self.bin().join("gh.log"))
            .map(|log| log.lines().map(ToOwned::to_owned).collect())
            .unwrap_or_default()
    }

    /// The cache file of `slug`.
    fn cache(&self, slug: &str) -> PathBuf {
        self.root()
            .join("state/exobrain/releases")
            .join(format!("{slug}.json"))
    }

    /// Run `exobrain outdated <args>` in `cwd`, with `env` set.
    fn run(&self, cwd: &Path, args: &[&str], env: &[(&str, &str)]) -> Output {
        let mut command = Command::new(assert_cmd::cargo::cargo_bin!("exobrain"));
        command
            .env_clear()
            .env("PATH", self.bin())
            .env("HOME", self.root().join("home"))
            .env("XDG_STATE_HOME", self.root().join("state"))
            .arg("outdated")
            .args(args)
            .current_dir(cwd);
        for (key, value) in env {
            command.env(key, value);
        }
        command.output().unwrap()
    }
}

/// The absolute path of the host tool `name`, for the stub to call.
fn tool(name: &str) -> String {
    std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
        .unwrap_or_else(|| panic!("{name} is on PATH"))
        .display()
        .to_string()
}

/// Write the fixture `name` to `dir/rel`, with its clocks `days` days
/// before now and its release-kit input pinned at `tag`.
fn write_fixture(dir: &Path, rel: &str, name: &str, days: i64, tag: &str) {
    let text = std::fs::read_to_string(fixture(name)).unwrap();
    let stamp = Timestamp::now().as_second() - days * DAY;
    let text = text
        .replace(&PINNED.to_string(), &stamp.to_string())
        .replace("@PINNED_TAG@", tag);
    let path = dir.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

/// A fresh `flake.lock` in `dir` that pins release-kit at `tag`.
fn write_tags(dir: &Path, tag: &str) {
    write_fixture(dir, "flake.lock", "tags.lock", 0, tag);
}

/// Write a cache entry for `slug` whose last attempt is `age` seconds old.
fn write_cache(world: &Gh, slug: &str, age: i64, latest: &str, error: Option<&str>) {
    let now = Timestamp::now().as_second();
    let succeeded = if error.is_some() {
        now - 30 * DAY
    } else {
        now - age
    };
    let entry = serde_json::json!({
        "attempted": now - age,
        "succeeded": succeeded,
        "latest": latest,
        "error": error,
    });
    let path = world.cache(slug);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, entry.to_string()).unwrap();
}

const RK: &str = "gubasso/release-kit";

fn rk_line(from: &str, to: &str) -> String {
    format!(
        "flake.lock: release-kit is pinned at {from}, and {to} is out. Move it with flake-edit update release-kit when ready."
    )
}

fn failed_line(slug: &str, error: &str) -> String {
    format!(
        "cannot read the latest release of {slug} ({error}); the next read runs in 1 hour, or now with --refresh"
    )
}

// outdated:warns-about-a-newer-release

#[test]
fn outdated_warns_about_a_newer_release() {
    let world = Gh::new();
    let project = world.project("p");
    write_tags(&project, "v0.10.1");
    world.tags(RK, &["v0.10.3", "v0.10.2", "v0.10.1"]);
    assert_lines(
        &world.run(&project, &[], &[]),
        &[&rk_line("v0.10.1", "v0.10.3")],
    );
    assert_eq!(
        world.calls(),
        [format!(
            "api repos/{RK}/tags?per_page=100&page=1 --jq .[].name"
        )]
    );
}

#[test]
fn outdated_orders_tags_like_flake_edit() {
    let world = Gh::new();
    let project = world.project("p");
    write_tags(&project, "v1.0.0");
    world.tags(RK, &["v1.5.0", "v2.0.0-beta.1", "nightly"]);
    assert_lines(
        &world.run(&project, &[], &[]),
        &[&rk_line("v1.0.0", "v2.0.0-beta.1")],
    );
}

#[test]
fn outdated_orders_versions_by_semver() {
    let world = Gh::new();
    let project = world.project("p");
    write_tags(&project, "v0.9.8");
    world.tags(RK, &["v0.10.1"]);
    assert_lines(
        &world.run(&project, &[], &[]),
        &[&rk_line("v0.9.8", "v0.10.1")],
    );
    let world = Gh::new();
    let project = world.project("p");
    write_tags(&project, "v0.10.1");
    world.tags(RK, &["v0.9.8"]);
    assert_lines(&world.run(&project, &[], &[]), &[]);
}

#[test]
fn outdated_names_the_lock_of_a_subfolder() {
    let world = Gh::new();
    let project = world.project("p");
    write_fixture(&project, "nix/flake.lock", "tags.lock", 0, "v0.10.1");
    world.tags(RK, &["v0.10.3"]);
    assert_lines(
        &world.run(&project, &["nix/flake.lock"], &[]),
        &[
            "nix/flake.lock: release-kit is pinned at v0.10.1, and v0.10.3 is out. Move it with flake-edit --flake ./nix --lock-file nix/flake.lock update release-kit when ready.",
        ],
    );
}

#[test]
fn outdated_silent_on_the_latest_release() {
    let world = Gh::new();
    let project = world.project("p");
    write_tags(&project, "v0.10.3");
    world.tags(RK, &["v0.10.3", "v0.10.2"]);
    assert_lines(&world.run(&project, &[], &[]), &[]);
    assert_eq!(world.calls().len(), 1);
}

#[test]
fn outdated_skips_branch_and_commit_inputs() {
    // The fixture also holds a GitLab input at a tag and a follows path to
    // a tag input, which the check skips too. Only release-kit is read.
    let world = Gh::new();
    let project = world.project("p");
    write_tags(&project, "main");
    assert_lines(&world.run(&project, &[], &[]), &[]);
    assert_eq!(world.calls(), Vec::<String>::new());
    write_tags(&project, "v0.10.1");
    assert_lines(&world.run(&project, &[], &[]), &[]);
    let calls = world.calls();
    assert_eq!(calls.len(), 1, "{calls:?}");
    assert!(calls[0].contains(RK), "{calls:?}");
}

#[test]
fn outdated_reads_two_locks_in_one_call() {
    let world = Gh::new();
    let project = world.project("p");
    write_fixture(&project, "flake.lock", "tags.lock", 41, "v0.10.1");
    write_lock(&project, "nix/flake.lock", 41);
    world.tags(RK, &["v0.10.3"]);
    assert_lines(
        &world.run(&project, &["flake.lock", "nix/flake.lock"], &[]),
        &[
            "flake.lock: nixpkgs pin is 41 days old (limit 30). Move it with nix flake update --flake . when ready.",
            &rk_line("v0.10.1", "v0.10.3"),
            "nix/flake.lock: nixpkgs pin is 41 days old (limit 30). Move it with nix flake update --flake ./nix when ready.",
        ],
    );
    assert_eq!(world.calls().len(), 1);
}

// outdated:reads-each-release-once-a-week

#[test]
fn outdated_keeps_two_owners_apart() {
    let world = Gh::new();
    let project = world.project("p");
    write_fixture(&project, "flake.lock", "owners.lock", 0, "");
    world.tags("a-b/c", &["v1.1.0"]);
    world.tags("a/b-c", &["v1.2.0"]);
    assert_lines(
        &world.run(&project, &[], &[]),
        &[
            "flake.lock: first is pinned at v1.0.0, and v1.1.0 is out. Move it with flake-edit update first when ready.",
            "flake.lock: second is pinned at v1.0.0, and v1.2.0 is out. Move it with flake-edit update second when ready.",
        ],
    );
    let first: serde_json::Value =
        serde_json::from_slice(&std::fs::read(world.cache("a-b/c")).unwrap()).unwrap();
    let second: serde_json::Value =
        serde_json::from_slice(&std::fs::read(world.cache("a/b-c")).unwrap()).unwrap();
    assert_eq!(first["latest"], "v1.1.0");
    assert_eq!(second["latest"], "v1.2.0");
    assert_eq!(first["error"], serde_json::Value::Null);
}

#[test]
fn outdated_reads_each_release_once_a_week() {
    let world = Gh::new();
    let project = world.project("p");
    write_tags(&project, "v0.10.1");
    world.tags(RK, &["v0.10.3"]);
    write_cache(&world, RK, DAY, "v0.10.2", None);
    assert_lines(
        &world.run(&project, &[], &[]),
        &[&rk_line("v0.10.1", "v0.10.2")],
    );
    assert_eq!(world.calls().len(), 0, "a cache 1 day old is fresh");
    write_cache(&world, RK, 8 * DAY, "v0.10.2", None);
    assert_lines(
        &world.run(&project, &[], &[]),
        &[&rk_line("v0.10.1", "v0.10.3")],
    );
    assert_eq!(world.calls().len(), 1, "a cache 8 days old is due");
    world.tags(RK, &["v0.11.0"]);
    assert_lines(
        &world.run(&project, &["--refresh"], &[]),
        &[&rk_line("v0.10.1", "v0.11.0")],
    );
    assert_eq!(world.calls().len(), 2, "--refresh reads now");
}

#[test]
fn outdated_retries_a_failed_read_after_an_hour() {
    let world = Gh::new();
    let project = world.project("p");
    write_tags(&project, "v0.10.1");
    world.tags(RK, &["v0.10.3"]);
    write_cache(&world, RK, 30 * 60, "v0.10.2", Some("offline"));
    assert_lines(
        &world.run(&project, &[], &[]),
        &[&rk_line("v0.10.1", "v0.10.2")],
    );
    assert_eq!(world.calls().len(), 0, "a failure 30 minutes old waits");
    write_cache(&world, RK, 2 * 3600, "v0.10.2", Some("offline"));
    assert_lines(
        &world.run(&project, &[], &[]),
        &[&rk_line("v0.10.1", "v0.10.3")],
    );
    assert_eq!(world.calls().len(), 1, "a failure 2 hours old is due");
}

#[test]
fn outdated_shares_the_cache_across_checkouts() {
    let world = Gh::new();
    let first = world.project("first");
    let second = world.project("second");
    write_tags(&first, "v0.10.1");
    write_tags(&second, "v0.10.1");
    world.tags(RK, &["v0.10.3"]);
    for project in [&first, &second] {
        assert_lines(
            &world.run(project, &[], &[]),
            &[&rk_line("v0.10.1", "v0.10.3")],
        );
    }
    assert_eq!(world.calls().len(), 1);
}

// outdated:bounds-each-read

#[test]
fn outdated_bounds_each_read() {
    let world = Gh::new();
    let project = world.project("p");
    write_tags(&project, "v0.10.1");
    world.tags(RK, &["v0.10.3"]);
    let started = std::time::Instant::now();
    let out = world.run(&project, &[], &[("STUB_SLEEP", "30")]);
    let took = started.elapsed();
    assert!(took < std::time::Duration::from_secs(20), "took {took:?}");
    assert_lines(&out, &[&failed_line(RK, "gh timed out after 10 seconds")]);
    let entry: serde_json::Value =
        serde_json::from_slice(&std::fs::read(world.cache(RK)).unwrap()).unwrap();
    assert_eq!(entry["error"], "gh timed out after 10 seconds");
}

#[test]
fn outdated_bounds_each_read_reports_a_failed_gh() {
    let world = Gh::new();
    let project = world.project("p");
    write_tags(&project, "v0.10.1");
    write_cache(&world, RK, 8 * DAY, "v0.10.2", None);
    assert_lines(
        &world.run(&project, &[], &[("STUB_FAIL", "1")]),
        &[
            &rk_line("v0.10.1", "v0.10.2"),
            &failed_line(RK, "error: stub failure"),
        ],
    );
}

#[test]
fn outdated_without_gh_exits_0() {
    let world = Gh::bare();
    let project = world.project("p");
    write_tags(&project, "v0.10.1");
    assert_lines(
        &world.run(&project, &[], &[]),
        &[&failed_line(RK, "gh is not on PATH")],
    );
    assert_lines(&world.run(&project, &[], &[]), &[]);
}
