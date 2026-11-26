//! `exobrain store-guard` against a faked store, a stub `nix`, and
//! scratch git repositories. The test names carry the requirement of
//! `_docs/specs/SPEC-cli.md` they verify as a prefix, so each `Verify:`
//! filter selects the whole family.

mod support;

use std::path::Path;

use predicates::str::contains;
use support::{BIG, World, days_ago, run_git, wait_for};

const REMOTE: &str = "https://github.com/u/repo.git";

fn stdout_of(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr_of(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn gitdir_line(path: &Path) -> String {
    format!("gitdir: {}\n", path.display())
}

// cli:store-guard-finds-copies-of-this-checkout

#[test]
fn finds_copies_of_this_checkout() {
    let world = World::new();
    let repo = world.repo("repo", None);
    run_git(&repo, &["worktree", "add", "-q", "../feat", "-b", "feat"]);
    let main = world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
    let wt = world.plant_file(
        "wt",
        &gitdir_line(&repo.join(".git/worktrees/feat")),
        Some(BIG),
    );
    let out = world.guard(&repo).output().unwrap();
    let text = stdout_of(&out);
    assert_eq!(out.status.code(), Some(1), "{text}");
    assert!(text.contains(&format!("{}", main.display())), "{text}");
    assert!(text.contains("main checkout"), "{text}");
    assert!(text.contains(&format!("{}", wt.display())), "{text}");
    assert!(text.contains("worktree feat"), "{text}");
    assert!(!text.contains("(removed)"), "{text}");
    assert!(text.contains("120.0 MB in 2 copies"), "{text}");
    assert!(text.contains("nix store gc"), "{text}");
    assert!(
        text.contains("project-platform:a-flake-is-read-from-its-git-tree"),
        "{text}"
    );
}

#[test]
fn finds_copies_of_this_checkout_through_a_matching_remote_only() {
    let world = World::new();
    let repo = world.repo("repo", Some(REMOTE));
    let ours = world.plant_dir("clone", Some("git@github.com:u/repo"), Some(BIG));
    let other = world.plant_dir("other", Some("https://github.com/u/other.git"), Some(BIG));
    let out = world.guard(&repo).output().unwrap();
    let text = stdout_of(&out);
    assert_eq!(out.status.code(), Some(1), "{text}");
    assert!(text.contains(&format!("{}", ours.display())), "{text}");
    assert!(text.contains("clone of github.com/u/repo"), "{text}");
    assert!(!text.contains(&format!("{}", other.display())), "{text}");
}

#[test]
fn finds_copies_of_this_checkout_ignoring_another_repository_and_small_copies() {
    let world = World::new();
    let repo = world.repo("repo", None);
    let elsewhere = world.repo("elsewhere", None);
    world.plant_file("foreign", &gitdir_line(&elsewhere.join(".git")), Some(BIG));
    world.plant_file("small", &gitdir_line(&repo.join(".git")), Some(1_000));
    let out = world.guard(&repo).output().unwrap();
    let text = stdout_of(&out);
    assert_eq!(out.status.code(), Some(0), "{text}");
    assert!(text.starts_with("no store copy of"), "{text}");
    assert!(!text.contains("origin unknown"), "{text}");

    // The same small copy counts once the minimum is below its size.
    world
        .guard(&repo)
        .args(["--min-size", "1K"])
        .assert()
        .code(1)
        .stdout(contains("main checkout"));
}

#[test]
fn finds_copies_of_this_checkout_and_reports_a_relative_or_malformed_gitdir_as_unknown() {
    let world = World::new();
    let repo = world.repo("repo", None);
    let relative = world.plant_file("rel", "gitdir: ../repo/.git/worktrees/feat\n", Some(BIG));
    let malformed = world.plant_file("bad", "not a pointer\n", Some(BIG));
    let out = world.guard(&repo).output().unwrap();
    let text = stdout_of(&out);
    assert_eq!(out.status.code(), Some(0), "{text}");
    assert!(text.contains("origin unknown:"), "{text}");
    assert!(
        text.contains(&format!("{}  relative gitdir", relative.display())),
        "{text}"
    );
    assert!(
        text.contains(&format!("{}  malformed .git file", malformed.display())),
        "{text}"
    );
}

#[test]
fn finds_copies_of_this_checkout_and_reports_a_gitdir_after_leading_text_as_unknown() {
    let world = World::new();
    let repo = world.repo("repo", None);
    // Git reads `gitdir: ` only from the first byte, so a pointer that a
    // later line holds is no gitfile, even when it names this checkout.
    let git_dir = std::fs::canonicalize(repo.join(".git")).unwrap();
    let text = format!("not a gitfile\ngitdir: {}\n", git_dir.display());
    let copy = world.plant_file("garbage", &text, Some(BIG));
    let out = world.guard(&repo).output().unwrap();
    let report = stdout_of(&out);
    assert_eq!(out.status.code(), Some(0), "{report}");
    assert!(
        report.contains(&format!("{}  malformed .git file", copy.display())),
        "{report}"
    );
    let daily = world.guard(&repo).arg("--daily").output().unwrap();
    assert_eq!(daily.status.code(), Some(0));
    assert!(daily.stderr.is_empty(), "{}", stderr_of(&daily));
}

#[test]
fn finds_copies_of_this_checkout_from_a_removed_worktree() {
    let world = World::new();
    let repo = world.repo("repo", None);
    let feat = world.root().join("feat");
    run_git(&repo, &["worktree", "add", "-q", "../feat", "-b", "feat"]);
    // The copy carries the worktree's own .git file, as a path copy would.
    let pointer = std::fs::read_to_string(feat.join(".git")).unwrap();
    let copy = world.plant_file("wt", &pointer, Some(BIG));
    run_git(&repo, &["worktree", "remove", "../feat"]);
    let out = world.guard(&repo).output().unwrap();
    let text = stdout_of(&out);
    assert_eq!(out.status.code(), Some(1), "{text}");
    assert!(text.contains(&format!("{}", copy.display())), "{text}");
    assert!(text.contains("worktree feat (removed)"), "{text}");
}

#[test]
fn finds_copies_of_this_checkout_dropping_a_path_nix_no_longer_reports() {
    let world = World::new();
    let repo = world.repo("repo", None);
    // No size: the stub reports the path as `null`, as nix does for a
    // path the collector removed between the listing and the query.
    world.plant_file("gone", &gitdir_line(&repo.join(".git")), None);
    let out = world.guard(&repo).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", stdout_of(&out));
    assert_eq!(world.nix_calls(), 1);
}

#[test]
fn finds_copies_of_this_checkout_unattributed_only_in_operator_mode() {
    let world = World::new();
    let repo = world.repo("repo", Some(REMOTE));
    let bare = world.plant_dir("noremote", None, Some(BIG));
    let out = world.guard(&repo).output().unwrap();
    let text = stdout_of(&out);
    assert_eq!(out.status.code(), Some(0), "{text}");
    assert!(
        text.contains(&format!("{}  no remote.origin.url", bare.display())),
        "{text}"
    );

    let daily = world.guard(&repo).arg("--daily").output().unwrap();
    assert_eq!(daily.status.code(), Some(0));
    assert!(daily.stdout.is_empty(), "{}", stdout_of(&daily));
    assert!(daily.stderr.is_empty(), "{}", stderr_of(&daily));
}

// cli:store-guard-exit-codes

#[test]
fn exit_codes_of_operator_mode() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.guard(&repo).assert().code(0);
    world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
    world.guard(&repo).assert().code(1);
    let outside = world.root().join("home");
    world
        .guard(&outside)
        .assert()
        .code(2)
        .stderr(contains("not inside a git work tree"));
    world
        .guard(&repo)
        .arg("--min-size")
        .arg("ten")
        .assert()
        .code(2);
}

#[test]
fn exit_codes_of_daily_mode() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
    let out = world.guard(&repo).arg("--daily").output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stdout.is_empty());
    let warning = stderr_of(&out);
    assert_eq!(warning.lines().count(), 3, "{warning}");
    assert!(
        warning.contains("1 store copy of this checkout hold 60.0 MB"),
        "{warning}"
    );
    assert!(warning.contains("nix store gc"), "{warning}");

    let outside = world.root().join("home");
    let out = world.guard(&outside).arg("--daily").output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stdout.is_empty() && out.stderr.is_empty());
}

// cli:store-guard-daily-runs-once-a-day

#[test]
fn daily_runs_once_a_day() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));

    world.guard(&repo).arg("--daily").assert().code(0);
    assert_eq!(world.nix_calls(), 1);
    let stamps = world.stamps();
    assert_eq!(stamps.len(), 1);
    let stamp = &stamps[0];
    let name = stamp.file_name().unwrap().to_string_lossy().into_owned();
    assert!(name.starts_with("repo-"), "{name}");
    assert_eq!(name.len(), "repo-".len() + 16 + ".stamp".len(), "{name}");
    let today = exobrain::stamp::today();
    assert_eq!(std::fs::read_to_string(stamp).unwrap().trim(), today);
    // The atomic write leaves only the stamp; the lock is gone too.
    let left: Vec<_> = std::fs::read_dir(world.guard_dir()).unwrap().collect();
    assert_eq!(left.len(), 1);

    let again = world.guard(&repo).arg("--daily").output().unwrap();
    assert_eq!(again.status.code(), Some(0));
    assert!(again.stdout.is_empty() && again.stderr.is_empty());
    assert_eq!(
        world.nix_calls(),
        1,
        "the second run of the day scans nothing"
    );

    std::fs::write(stamp, "2000-01-01\n").unwrap();
    world
        .guard(&repo)
        .arg("--daily")
        .assert()
        .code(0)
        .stderr(contains("nix store gc"));
    assert_eq!(world.nix_calls(), 2, "a stamp from another day scans again");
    assert_eq!(std::fs::read_to_string(stamp).unwrap().trim(), today);
}

#[test]
fn daily_runs_once_a_day_even_when_the_scan_fails() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
    let out = world
        .guard(&repo)
        .arg("--daily")
        .env("STUB_FAIL", "1")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(stderr_of(&out).lines().count(), 1, "{}", stderr_of(&out));
    assert_eq!(world.nix_calls(), 1);
    assert_eq!(world.stamps().len(), 1, "the stamp landed before the scan");

    world.guard(&repo).arg("--daily").assert().code(0);
    assert_eq!(world.nix_calls(), 1, "no second try the same day");
}

#[test]
fn daily_runs_once_a_day_across_concurrent_runs() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
    let spawn = || {
        world
            .guard_std(&repo)
            .arg("--daily")
            .env("STUB_SLEEP", "1")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap()
    };
    let (mut first, mut second) = (spawn(), spawn());
    assert!(first.wait().unwrap().success());
    assert!(second.wait().unwrap().success());
    assert_eq!(world.nix_calls(), 1);
}

#[test]
fn daily_runs_once_a_day_and_skips_while_another_run_holds_the_lock() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
    // A first run names the key; then a live process holds the lock.
    world.guard(&repo).arg("--daily").assert().code(0);
    let stamp = world.stamps().remove(0);
    std::fs::remove_file(&stamp).unwrap();
    let lock = stamp.with_extension("lock");
    std::fs::write(&lock, format!("pid={}\n", std::process::id())).unwrap();

    let out = world.guard(&repo).arg("--daily").output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty(), "{}", stderr_of(&out));
    assert_eq!(world.nix_calls(), 1, "a held lock skips the scan");
    assert!(world.stamps().is_empty(), "a held lock skips the stamp");
    assert!(lock.exists(), "the run leaves another owner's lock alone");
}

#[test]
fn daily_runs_once_a_day_and_skips_an_unwritable_state_folder() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
    // A regular file where the state root belongs: no folder can be made.
    std::fs::write(world.state(), "not a folder").unwrap();
    let out = world.guard(&repo).arg("--daily").output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(stderr_of(&out).lines().count(), 1, "{}", stderr_of(&out));
    assert_eq!(world.nix_calls(), 0);
}

// cli:store-guard-daily-runs-prek-gc-once-a-week

#[test]
fn weekly_prek_gc_runs_when_the_stamp_is_older_than_seven_days() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.stub_prek();
    let out = world.guard(&repo).arg("--daily").output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    let text = stderr_of(&out);
    assert_eq!(text.lines().count(), 1, "{text}");
    assert!(text.contains("prek cache gc freed 12.5MiB in"), "{text}");
    assert!(
        text.contains(&world.prek_store().display().to_string()),
        "{text}"
    );
    assert_eq!(world.gc_calls(), 1, "a missing stamp runs gc");
    let stamps = world.gc_stamps();
    assert_eq!(stamps.len(), 1);
    let today = exobrain::stamp::today();
    assert_eq!(std::fs::read_to_string(&stamps[0]).unwrap().trim(), today);

    world.clear_daily();
    std::fs::write(&stamps[0], days_ago(8)).unwrap();
    world.guard(&repo).arg("--daily").assert().code(0);
    assert_eq!(world.gc_calls(), 2, "a stamp of eight days ago runs gc");
    assert_eq!(std::fs::read_to_string(&stamps[0]).unwrap().trim(), today);
}

#[test]
fn weekly_prek_gc_waits_seven_days_across_checkouts() {
    let world = World::new();
    let repo = world.repo("repo", None);
    let other = world.repo("other", None);
    world.stub_prek();
    world.guard(&repo).arg("--daily").assert().code(0);
    assert_eq!(world.gc_calls(), 1);

    // Another checkout on the same store shares the weekly stamp.
    let out = world.guard(&other).arg("--daily").output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty(), "{}", stderr_of(&out));
    assert_eq!(
        world.gc_calls(),
        1,
        "a second checkout the same day runs no gc"
    );
    assert_eq!(world.gc_stamps().len(), 1, "one weekly stamp per store");
    assert_eq!(
        world.daily_stamps().len(),
        2,
        "one daily stamp per checkout"
    );

    world.clear_daily();
    std::fs::write(&world.gc_stamps()[0], days_ago(6)).unwrap();
    world.guard(&repo).arg("--daily").assert().code(0);
    assert_eq!(world.gc_calls(), 1, "a stamp of six days ago runs no gc");
}

#[test]
fn weekly_prek_gc_skips_without_prek() {
    let world = World::new();
    let repo = world.repo("repo", None);
    let out = world.guard(&repo).arg("--daily").output().unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(
        out.stdout.is_empty() && out.stderr.is_empty(),
        "{}",
        stderr_of(&out)
    );
    assert!(
        world.gc_stamps().is_empty(),
        "no prek writes no weekly stamp"
    );
}

#[test]
fn weekly_prek_gc_prints_nothing_when_it_frees_nothing() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.stub_prek();
    let out = world
        .guard(&repo)
        .arg("--daily")
        .env("STUB_PREK_OUT", "Nothing to clean")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty(), "{}", stderr_of(&out));
    assert_eq!(world.gc_calls(), 1);
}

#[test]
fn weekly_prek_gc_error_keeps_the_exit_code() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
    world.stub_prek();
    let out = world
        .guard(&repo)
        .arg("--daily")
        .env("STUB_PREK_FAIL", "1")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let text = stderr_of(&out);
    assert_eq!(
        text.lines().count(),
        4,
        "the copy warning and one gc line: {text}"
    );
    assert!(
        text.contains("exobrain store-guard: prek cache gc: error: stub gc failure"),
        "{text}"
    );
    assert_eq!(world.gc_stamps().len(), 1, "the stamp landed before gc");

    world.clear_daily();
    world.guard(&repo).arg("--daily").assert().code(0);
    assert_eq!(world.gc_calls(), 1, "no second try within the week");
}

#[test]
fn weekly_prek_gc_runs_once_across_concurrent_checkouts() {
    let world = World::new();
    let first_repo = world.repo("first", None);
    let second_repo = world.repo("second", None);
    world.stub_prek();
    let spawn = |repo: &Path| {
        world
            .guard_std(repo)
            .arg("--daily")
            .env("STUB_PREK_SLEEP", "1")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap()
    };
    let (mut first, mut second) = (spawn(&first_repo), spawn(&second_repo));
    assert!(first.wait().unwrap().success());
    assert!(second.wait().unwrap().success());
    assert_eq!(world.gc_calls(), 1);
    assert_eq!(world.daily_stamps().len(), 2);
}

/// This test documents a race of prek that `SPEC-cli.md` records, against
/// the host's prek. On prek 0.5.7 to 0.5.11, a hook runs outside the store
/// lock, and `gc` keeps only what the configurations on disk pin. When the
/// configuration moves its `rev` during a run, the weekly `gc` removes the
/// clone that the running hook reads, and that hook run fails. On prek
/// 0.5.12 and later, the run holds a shared lock on the store, and `gc`
/// refuses while the hook runs.
///
/// For a version before 0.5.7 the test prints the version and the reason on
/// stderr and returns, and `.config/nextest.toml` shows that output.
#[test]
fn weekly_prek_gc_during_a_hook_run_whose_config_moved_its_rev() {
    let world = World::new();
    let prek = world.real_prek();
    let version = support::prek_version(&prek);
    let racy = semver::VersionReq::parse(">=0.5.7, <=0.5.11").unwrap();
    let fixed = semver::VersionReq::parse(">=0.5.12").unwrap();
    if racy.matches(&version) {
        eprintln!("prek {version} is inside 0.5.7 to 0.5.11: the test asserts the race");
    } else if fixed.matches(&version) {
        eprintln!("prek {version} is 0.5.12 or later: the test asserts the refusal");
    } else {
        let rule = "cli:store-guard-daily-runs-prek-gc-once-a-week";
        eprintln!("SKIP: prek {version} is before 0.5.7, outside the versions that {rule} records");
        return;
    }
    let sync = world.root().join("sync");
    std::fs::create_dir_all(&sync).unwrap();

    let hooks = slow_hook_repo(&world, &sync);

    // A project that pins v1, and a hook run that waits inside the hook.
    let project = world.repo("project", None);
    let config = project.join(".pre-commit-config.yaml");
    let pinned = |rev: &str| {
        format!(
            "repos:\n  - repo: {}\n    rev: {rev}\n    hooks:\n      - id: slow\n",
            hooks.display()
        )
    };
    std::fs::write(&config, pinned("v1")).unwrap();
    run_git(&project, &["add", "-A"]);
    // The run keeps the host's PATH for its hook, and none of the variables
    // that git or an outer hook run exports.
    let run = std::process::Command::new(&prek)
        .args(["run", "--all-files"])
        .current_dir(&project)
        .env_clear()
        .envs(std::env::vars_os().filter(|(name, _)| {
            let name = name.to_string_lossy();
            !name.starts_with("GIT_")
                && !name.starts_with("PREK_")
                && !name.starts_with("PRE_COMMIT")
        }))
        .env("PREK_HOME", world.prek_store())
        .env("PREK_COLOR", "never")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    wait_for(&sync.join("started"));
    let clones: Vec<_> = std::fs::read_dir(world.prek_store().join("repos"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    assert_eq!(clones.len(), 1, "the run cloned v1");

    // The configuration moves to v2 while the hook runs, and the weekly gc runs.
    std::fs::write(&config, pinned("v2")).unwrap();
    let guard = world.guard(&project).arg("--daily").output().unwrap();
    assert_eq!(guard.status.code(), Some(0));
    let text = stderr_of(&guard);
    if racy.matches(&version) {
        assert!(text.contains("prek cache gc freed"), "{text}");
        assert!(
            !clones[0].exists(),
            "gc removed the clone of the running hook"
        );
    } else {
        assert!(
            text.contains("exobrain store-guard: prek cache gc: error: Cache at")
                && text.contains("is in use"),
            "{text}"
        );
        assert!(clones[0].exists(), "gc kept the clone of the running hook");
    }

    std::fs::write(sync.join("go"), "").unwrap();
    let out = run.wait_with_output().unwrap();
    let report = format!("{}{}", stdout_of(&out), stderr_of(&out));
    if racy.matches(&version) {
        assert!(!out.status.success(), "{report}");
        assert!(
            report.contains("payload: No such file or directory"),
            "{report}"
        );
    } else {
        // prek still fails the run, because the configuration changed during
        // it, so the test judges the output and not the exit.
        assert!(report.contains("payload of v1"), "{report}");
        assert!(!report.contains("No such file or directory"), "{report}");
    }
}

/// A hook repository with one script hook, tagged v1 and v2. The hook
/// writes `sync/started`, waits up to 30 seconds for `sync/go`, and then
/// prints the `payload` file of its clone.
#[expect(
    clippy::unwrap_used,
    reason = "a fixture helper, where a broken fixture aborts the test"
)]
fn slow_hook_repo(world: &World, sync: &Path) -> std::path::PathBuf {
    let hooks = world.repo("hooks", None);
    std::fs::write(
        hooks.join(".pre-commit-hooks.yaml"),
        "- id: slow\n  name: slow\n  entry: slow.sh\n  language: script\n  pass_filenames: false\n  always_run: true\n",
    )
    .unwrap();
    let script = format!(
        "#!/bin/sh\nd=$(dirname \"$0\")\n: > '{started}'\ni=0\nwhile [ ! -e '{go}' ] && [ $i -lt 300 ]; do sleep 0.1; i=$((i+1)); done\ncat \"$d/payload\"\n",
        started = sync.join("started").display(),
        go = sync.join("go").display(),
    );
    std::fs::write(hooks.join("slow.sh"), script).unwrap();
    let mut perms = std::fs::metadata(hooks.join("slow.sh"))
        .unwrap()
        .permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    std::fs::set_permissions(hooks.join("slow.sh"), perms).unwrap();
    std::fs::write(hooks.join("payload"), "payload of v1\n").unwrap();
    run_git(&hooks, &["add", "-A"]);
    run_git(&hooks, &["commit", "-q", "-m", "v1"]);
    run_git(&hooks, &["tag", "v1"]);
    std::fs::write(hooks.join("payload"), "payload of v2\n").unwrap();
    run_git(&hooks, &["commit", "-q", "-am", "v2"]);
    run_git(&hooks, &["tag", "v2"]);
    hooks
}

// cli:store-guard-daily-warns-about-a-large-build-folder

/// Put a wrapper that runs the `cargo` of the test process on the `PATH`
/// of `exobrain`. `cargo metadata --no-deps` reads only the manifest.
#[expect(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a fixture helper, where a broken fixture aborts the test"
)]
fn link_cargo(world: &World) {
    let cargo = std::env::var_os("CARGO")
        .map(std::path::PathBuf::from)
        .or_else(|| support::which("cargo"))
        .expect("cargo runs this test");
    let wrapper = world.path_dir().join("cargo");
    std::fs::write(
        &wrapper,
        format!("#!/bin/sh\nexec '{}' \"$@\"\n", cargo.display()),
    )
    .unwrap();
    let mut perms = std::fs::metadata(&wrapper).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    std::fs::set_permissions(&wrapper, perms).unwrap();
}

/// A git repository that holds a one-crate Cargo project, with the real
/// `cargo` on the `PATH` of `exobrain`.
#[expect(
    clippy::unwrap_used,
    reason = "a fixture helper, where a broken fixture aborts the test"
)]
fn rust_repo(world: &World, name: &str) -> std::path::PathBuf {
    let repo = world.repo(name, None);
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(
        repo.join("Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    std::fs::write(repo.join("src/lib.rs"), "").unwrap();
    run_git(&repo, &["add", "Cargo.toml", "src/lib.rs"]);
    run_git(&repo, &["commit", "-q", "-m", "crate"]);
    link_cargo(world);
    repo
}

/// Write a file of `bytes` bytes at `path`, with its parents.
#[expect(
    clippy::unwrap_used,
    reason = "a fixture helper, where a broken fixture aborts the test"
)]
fn fill(path: &Path, bytes: usize) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, vec![7_u8; bytes]).unwrap();
}

/// A daily run in `cwd` with a build-folder limit of `limit`, and no
/// inherited build-folder setting.
#[expect(
    clippy::unwrap_used,
    reason = "a fixture helper, where a broken fixture aborts the test"
)]
fn daily_with_limit(world: &World, cwd: &Path, limit: &str) -> std::process::Output {
    world
        .guard(cwd)
        .arg("--daily")
        .env("EXOBRAIN_TARGET_LIMIT", limit)
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("CARGO_BUILD_TARGET_DIR")
        .output()
        .unwrap()
}

#[test]
fn large_build_folder_over_the_limit_gets_one_line() {
    let world = World::new();
    let repo = rust_repo(&world, "repo");
    fill(&repo.join("target/debug/deps/libbig.rlib"), 300_000);
    let out = daily_with_limit(&world, &repo, "100K");
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stdout.is_empty(), "{}", stdout_of(&out));
    let text = stderr_of(&out);
    let target = repo.join("target");
    assert_eq!(text.lines().count(), 1, "{text}");
    assert!(
        text.contains(&format!("build folder {} holds 0.3 MB", target.display())),
        "{text}"
    );
    assert!(
        text.contains(&format!("`cargo clean --target-dir {}`", target.display())),
        "{text}"
    );
    assert!(
        target.join("debug/deps/libbig.rlib").exists(),
        "nothing deleted"
    );
}

#[test]
fn large_build_folder_under_the_limit_gets_no_line() {
    let world = World::new();
    let repo = rust_repo(&world, "repo");
    fill(&repo.join("target/debug/deps/libbig.rlib"), 300_000);
    let out = daily_with_limit(&world, &repo, "");
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty(), "{}", stderr_of(&out));
    // The default limit is 10 GB; the limit itself is not over it.
    let out = daily_with_limit(&world, &repo, "1M");
    assert!(out.stderr.is_empty(), "{}", stderr_of(&out));
}

#[test]
fn large_build_folder_is_skipped_without_a_cargo_manifest() {
    let world = World::new();
    let repo = world.repo("repo", None);
    link_cargo(&world);
    fill(&repo.join("target/debug/deps/libbig.rlib"), 300_000);
    let out = daily_with_limit(&world, &repo, "1K");
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty(), "{}", stderr_of(&out));
}

#[test]
fn large_build_folder_outside_the_checkout_is_measured() {
    let world = World::new();
    let repo = rust_repo(&world, "repo");
    let outside = world.root().join("builds/repo");
    fill(&outside.join("debug/deps/libbig.rlib"), 300_000);
    fill(&repo.join("target/debug/deps/libbig.rlib"), 300_000);
    let out = world
        .guard(&repo)
        .arg("--daily")
        .env("EXOBRAIN_TARGET_LIMIT", "100K")
        .env("CARGO_TARGET_DIR", &outside)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let text = stderr_of(&out);
    assert_eq!(text.lines().count(), 1, "{text}");
    assert!(
        text.contains(&format!("--target-dir {}`", outside.display())),
        "{text}"
    );
    assert!(
        !text.contains(&repo.join("target").display().to_string()),
        "{text}"
    );
}

#[test]
fn large_build_folder_of_each_worktree_is_its_own() {
    let world = World::new();
    let repo = rust_repo(&world, "repo");
    run_git(&repo, &["worktree", "add", "-q", "../feat", "-b", "feat"]);
    let feat = world.root().join("feat");
    let builds = world.root().join("builds");
    fill(&builds.join("main/debug/libmain.rlib"), 300_000);
    fill(&builds.join("feat/debug/libfeat.rlib"), 300_000);
    let run = |cwd: &Path, folder: &str| {
        let out = world
            .guard(cwd)
            .arg("--daily")
            .env("EXOBRAIN_TARGET_LIMIT", "100K")
            .env("CARGO_TARGET_DIR", builds.join(folder))
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(0));
        stderr_of(&out)
    };
    let main_text = run(&repo, "main");
    let feat_text = run(&feat, "feat");
    for (text, own, other) in [(&main_text, "main", "feat"), (&feat_text, "feat", "main")] {
        assert_eq!(text.lines().count(), 1, "{text}");
        let own = builds.join(own).display().to_string();
        let other = builds.join(other).display().to_string();
        assert!(text.contains(&format!("build folder {own} ")), "{text}");
        assert!(!text.contains(&other), "{text}");
    }
}

#[test]
fn large_build_folder_counts_a_hard_link_once_and_only_when_it_is_whole() {
    let world = World::new();
    let repo = rust_repo(&world, "repo");
    let target = repo.join("target");
    // Cargo links each binary of `deps/` into its profile folder.
    fill(&target.join("debug/deps/tool-0123"), 300_000);
    std::fs::hard_link(
        target.join("debug/deps/tool-0123"),
        target.join("debug/tool"),
    )
    .unwrap();
    let out = daily_with_limit(&world, &repo, "400K");
    assert!(
        out.stderr.is_empty(),
        "two links count once: {}",
        stderr_of(&out)
    );
    world.clear_daily();
    let out = daily_with_limit(&world, &repo, "200K");
    assert_eq!(stderr_of(&out).lines().count(), 1, "{}", stderr_of(&out));

    // A cache that holds another link keeps the blocks after a clean.
    world.clear_daily();
    std::fs::hard_link(target.join("debug/tool"), world.root().join("cache-entry")).unwrap();
    let out = daily_with_limit(&world, &repo, "1K");
    assert!(
        out.stderr.is_empty(),
        "a shared link counts zero: {}",
        stderr_of(&out)
    );
}

#[test]
fn large_build_folder_counts_only_the_unshared_bytes_of_a_clone() {
    let world = World::new();
    let repo = rust_repo(&world, "repo");
    // The scratch root of the world can be a tmpfs, which cannot reflink,
    // so the clone lives in a temp dir beside the build output of this
    // test, which shares the filesystem of the checkout.
    let disk = tempfile::tempdir_in(env!("CARGO_TARGET_TMPDIR")).unwrap();
    let entry = disk.path().join("cache/entry");
    fill(&entry, 4_000_000);
    let target = disk.path().join("target");
    let clone = target.join("debug/deps/libdep.rlib");
    std::fs::create_dir_all(clone.parent().unwrap()).unwrap();
    let cloned = std::process::Command::new("cp")
        .arg("--reflink=always")
        .arg(&entry)
        .arg(&clone)
        .output()
        .unwrap();
    if !cloned.status.success() {
        eprintln!(
            "skipped: the filesystem of {} cannot reflink: {}",
            disk.path().display(),
            stderr_of(&cloned).trim()
        );
        return;
    }
    let run = || {
        world
            .guard(&repo)
            .arg("--daily")
            .env("EXOBRAIN_TARGET_LIMIT", "1M")
            .env("CARGO_TARGET_DIR", &target)
            .output()
            .unwrap()
    };
    // A clone of a 4 MB cache entry frees nearly nothing, as a target that
    // kache restored does, so it stays under a limit of 1 MB.
    let out = run();
    assert!(out.stderr.is_empty(), "{}", stderr_of(&out));
    // Bytes that the target alone holds count in full.
    world.clear_daily();
    fill(&target.join("debug/deps/libown.rlib"), 2_000_000);
    let out = run();
    let text = stderr_of(&out);
    assert_eq!(text.lines().count(), 1, "{text}");
    assert!(text.contains("holds 2.0 MB"), "{text}");
}

#[test]
fn large_build_folder_measure_error_keeps_the_exit_code() {
    let world = World::new();
    let repo = rust_repo(&world, "repo");
    std::fs::write(repo.join("Cargo.toml"), "[package\n").unwrap();
    let out = daily_with_limit(&world, &repo, "1K");
    assert_eq!(out.status.code(), Some(0));
    let text = stderr_of(&out);
    assert_eq!(text.lines().count(), 1, "{text}");
    assert!(
        text.contains("exobrain store-guard: cargo metadata: "),
        "{text}"
    );

    // An unreadable folder inside the build folder fails the walk.
    run_git(&repo, &["checkout", "-q", "--", "Cargo.toml"]);
    let locked = repo.join("target/debug/locked");
    fill(&locked.join("file"), 10);
    let mode = |bits| {
        let mut perms = std::fs::metadata(&locked).unwrap().permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut perms, bits);
        std::fs::set_permissions(&locked, perms).unwrap();
    };
    mode(0o000);
    world.clear_daily();
    let out = daily_with_limit(&world, &repo, "1K");
    mode(0o755);
    assert_eq!(out.status.code(), Some(0));
    let text = stderr_of(&out);
    assert_eq!(text.lines().count(), 1, "{text}");
    assert!(
        text.contains(&format!("cannot measure {}", repo.join("target").display())),
        "{text}"
    );
}

// cli:store-guard-stays-quiet-when-switched-off

#[test]
fn daily_stays_quiet_when_switched_off() {
    let world = World::new();
    let repo = world.repo("repo", None);
    world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
    for (var, value) in [
        ("EXOBRAIN_STORE_GUARD", "0"),
        ("CI", "1"),
        ("CI", "true"),
        ("GITHUB_ACTIONS", "true"),
        ("TF_BUILD", "True"),
    ] {
        let out = world
            .guard(&repo)
            .arg("--daily")
            .env(var, value)
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(0));
        assert!(
            out.stdout.is_empty() && out.stderr.is_empty(),
            "{var}={value}"
        );
        assert_eq!(world.nix_calls(), 0, "{var}={value} scans nothing");
        assert!(world.stamps().is_empty(), "{var}={value} stamps nothing");
    }
    // The switches bind only --daily: an operator run still scans.
    world.guard(&repo).env("CI", "1").assert().code(1);
}

#[test]
fn daily_stays_quiet_when_switched_off_but_not_by_an_explicit_no() {
    for (var, value) in [
        ("CI", ""),
        ("CI", "0"),
        ("CI", "false"),
        ("EXOBRAIN_STORE_GUARD", "1"),
        ("EXOBRAIN_STORE_GUARD", ""),
    ] {
        let world = World::new();
        let repo = world.repo("repo", None);
        world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
        world
            .guard(&repo)
            .arg("--daily")
            .env(var, value)
            .assert()
            .code(0)
            .stderr(contains("nix store gc"));
        assert_eq!(world.stamps().len(), 1, "{var}={value:?} runs and stamps");
    }
}

// cli:output-is-human-or-json

#[test]
fn output_is_human_or_json() {
    let world = World::new();
    let repo = world.repo("repo", Some(REMOTE));
    let copy = world.plant_file("main", &gitdir_line(&repo.join(".git")), Some(BIG));
    let unknown = world.plant_dir("noremote", None, Some(BIG));
    let out = world.guard(&repo).arg("--json").output().unwrap();
    assert_eq!(out.status.code(), Some(1));
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let mut keys: Vec<&str> = value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        ["checkout", "copies", "total_nar_size", "unattributed"]
    );
    assert_eq!(value["checkout"], repo.display().to_string());
    assert_eq!(value["total_nar_size"], BIG);
    let copies = value["copies"].as_array().unwrap();
    assert_eq!(copies.len(), 1);
    let mut copy_keys: Vec<&str> = copies[0]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    copy_keys.sort_unstable();
    assert_eq!(copy_keys, ["nar_size", "origin", "path", "registered"]);
    assert_eq!(copies[0]["path"], copy.display().to_string());
    assert_eq!(copies[0]["nar_size"], BIG);
    assert_eq!(copies[0]["origin"], "main checkout");
    let registered = jiff::Timestamp::from_second(support::REGISTERED).unwrap();
    assert_eq!(copies[0]["registered"], registered.to_string());
    let unattributed = value["unattributed"].as_array().unwrap();
    assert_eq!(unattributed.len(), 1);
    assert_eq!(unattributed[0]["path"], unknown.display().to_string());
    assert!(unattributed[0]["reason"].is_string());

    let human = world.guard(&repo).output().unwrap();
    let text = stdout_of(&human);
    assert!(text.starts_with("store copies of "), "{text}");
    assert!(serde_json::from_str::<serde_json::Value>(&text).is_err());
}
