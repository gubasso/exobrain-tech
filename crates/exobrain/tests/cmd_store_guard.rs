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
/// the host's prek. A hook runs outside the store lock, and `gc` keeps only
/// what the configurations on disk pin. When the configuration moves its
/// `rev` during a run, the weekly `gc` removes the clone that the running
/// hook reads, and that hook run fails. If prek starts to keep the files
/// of a running hook, this test fails, and the spec text changes with it.
#[test]
fn weekly_prek_gc_removes_the_clone_of_a_running_hook_whose_config_moved_its_rev() {
    let world = World::new();
    let prek = world.real_prek();
    let sync = world.root().join("sync");
    std::fs::create_dir_all(&sync).unwrap();

    // A hook repository with a script hook, tagged v1 and v2.
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
    std::fs::write(hooks.join("payload"), "v1\n").unwrap();
    run_git(&hooks, &["add", "-A"]);
    run_git(&hooks, &["commit", "-q", "-m", "v1"]);
    run_git(&hooks, &["tag", "v1"]);
    std::fs::write(hooks.join("payload"), "v2\n").unwrap();
    run_git(&hooks, &["commit", "-q", "-am", "v2"]);
    run_git(&hooks, &["tag", "v2"]);

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
    assert!(text.contains("prek cache gc freed"), "{text}");
    assert!(
        !clones[0].exists(),
        "gc removed the clone of the running hook"
    );

    std::fs::write(sync.join("go"), "").unwrap();
    let out = run.wait_with_output().unwrap();
    let report = format!("{}{}", stdout_of(&out), stderr_of(&out));
    assert!(!out.status.success(), "{report}");
    assert!(
        report.contains("payload: No such file or directory"),
        "{report}"
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
