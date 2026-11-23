# CLI Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`cli:store-guard-finds-copies-of-this-checkout` — store-guard finds the store copies of this checkout](#clistore-guard-finds-copies-of-this-checkout--store-guard-finds-the-store-copies-of-this-checkout)
    - [Scenario: A worktree is removed after its copy reached the store](#scenario-a-worktree-is-removed-after-its-copy-reached-the-store)
  - [`cli:store-guard-exit-codes` — store-guard exits by what it found](#clistore-guard-exit-codes--store-guard-exits-by-what-it-found)
    - [Scenario: The guard runs outside git](#scenario-the-guard-runs-outside-git)
  - [`cli:store-guard-daily-runs-once-a-day` — The daily guard scans at most once a day](#clistore-guard-daily-runs-once-a-day--the-daily-guard-scans-at-most-once-a-day)
    - [Scenario: The first scan of the day fails](#scenario-the-first-scan-of-the-day-fails)
  - [`cli:store-guard-daily-runs-prek-gc-once-a-week` — The daily guard runs prek cache gc once a week](#clistore-guard-daily-runs-prek-gc-once-a-week--the-daily-guard-runs-prek-cache-gc-once-a-week)
    - [Scenario: A configuration moves its rev during a hook run](#scenario-a-configuration-moves-its-rev-during-a-hook-run)
  - [`cli:store-guard-daily-warns-about-a-large-build-folder` — The daily guard warns about a large build folder](#clistore-guard-daily-warns-about-a-large-build-folder--the-daily-guard-warns-about-a-large-build-folder)
    - [Scenario: Two worktrees build into folders outside the checkout](#scenario-two-worktrees-build-into-folders-outside-the-checkout)
  - [`cli:store-guard-stays-quiet-when-switched-off` — The daily guard stays off when switched off](#clistore-guard-stays-quiet-when-switched-off--the-daily-guard-stays-off-when-switched-off)
    - [Scenario: A CI runner exports an empty CI variable](#scenario-a-ci-runner-exports-an-empty-ci-variable)
  - [`cli:output-is-human-or-json` — Output is human text or one JSON object](#clioutput-is-human-or-json--output-is-human-text-or-one-json-object)
    - [Scenario: A script reads the report](#scenario-a-script-reads-the-report)
  - [`cli:skill-prints-the-embedded-skills` — skill prints the agent skills that the binary carries](#cliskill-prints-the-embedded-skills--skill-prints-the-agent-skills-that-the-binary-carries)
    - [Scenario: A skill gains a reference file](#scenario-a-skill-gains-a-reference-file)
  - [`cli:host-check-validates-the-file` — host-check parses the file with a typed parser](#clihost-check-validates-the-file--host-check-parses-the-file-with-a-typed-parser)
    - [Scenario: A version has no operator](#scenario-a-version-has-no-operator)
  - [`cli:host-check-checks-the-named-tools` — host-check checks the tools a call names](#clihost-check-checks-the-named-tools--host-check-checks-the-tools-a-call-names)
    - [Scenario: A host lacks a tool that one recipe uses](#scenario-a-host-lacks-a-tool-that-one-recipe-uses)
  - [`cli:host-check-reports-every-failure` — host-check reports every failure before it exits](#clihost-check-reports-every-failure--host-check-reports-every-failure-before-it-exits)
    - [Scenario: Two tools fail at once](#scenario-two-tools-fail-at-once)
  - [`cli:host-check-caches-versions` — host-check caches each version it reads](#clihost-check-caches-versions--host-check-caches-each-version-it-reads)
    - [Scenario: A package manager upgrades a tool in place](#scenario-a-package-manager-upgrades-a-tool-in-place)
  - [`cli:host-check-exit-codes` — host-check exits by what it found](#clihost-check-exit-codes--host-check-exits-by-what-it-found)
    - [Scenario: A project has no file](#scenario-a-project-has-no-file)
  - [`cli:a-verb-fills-a-named-gap` — Each verb fills a gap that the feature document names](#clia-verb-fills-a-named-gap--each-verb-fills-a-gap-that-the-feature-document-names)
    - [Scenario: A verb repeats a native command](#scenario-a-verb-repeats-a-native-command)
- [Unenforced](#unenforced)

<!--TOC-->

## Purpose

These rules govern the `exobrain` binary that `crates/exobrain` builds: its verbs, their exit codes, and their output. Each verb fills a gap that no native tool covers, and `programming/project-bootstrap/COMPARISON-dev-tool-features.md` names each gap. The binary enforces rules that other specs own. `store-guard` finds the store copies that `project-platform:a-flake-is-read-from-its-git-tree` forbids. `SPEC-project-platform.md` owns that rule. This spec owns only how the guard finds, reports, and schedules the copies. The daily mode of the guard also warns about a large build folder of the checkout and runs `prek cache gc` once a week, and this spec owns that check and that schedule too. `host-check` reads `host-requirements.toml`, the list of host tools of a project's dev environment, and checks them on the host. This spec owns the format of that file and how the checker reads it, checks it, and reports. `hooks` holds the hook revisions of a project to the reference sets of its exobrain-tech tag, and `SPEC-hooks.md` owns its rules. `outdated` reports what is behind in a project and changes nothing, and `SPEC-outdated.md` owns its rules. `skill` prints the agent skills that the binary carries, and this spec owns where a skill lives and how the binary serves it.

## Requirements

### `cli:store-guard-finds-copies-of-this-checkout` — store-guard finds the store copies of this checkout

When `exobrain store-guard` runs inside a git work tree, the guard MUST report each `-source` entry of the Nix store that meets both of these conditions:

- Its `.git` ties it to this checkout.
- Its registered NAR size reaches `--min-size`.

For each entry, the guard reports these fields:

- the store path
- the NAR size
- the registration date
- the origin

A `.git` file ties a copy when its absolute `gitdir:` equals the git common dir or lies under `<common dir>/worktrees/`. A `.git` folder ties a copy when its normalized `remote.origin.url` equals the checkout's.

The guard lists the store once. It stats one `.git` per entry. It reads size and age from one `nix path-info --json` call.

These copies go under "origin unknown":

- a copy with a relative or malformed `gitdir:`
- an unreadable copy
- a copy whose `.git` folder has no remote

The guard drops a path that `nix` no longer reports.

#### Scenario: A worktree is removed after its copy reached the store

- GIVEN a store copy whose `.git` file points under `<common dir>/worktrees/feat`
- WHEN `git worktree remove` deletes that worktree and the guard runs from the main checkout
- THEN the guard still reports the copy, with the origin `worktree feat (removed)`

Verify: `cargo nextest run -E 'test(finds_copies_of_this_checkout)'`

### `cli:store-guard-exit-codes` — store-guard exits by what it found

The guard MUST exit with one of these codes:

- 0 when the checkout has no store copy
- 1 when it has at least one
- 2 on a usage or environment error

With `--daily`, the guard MUST exit 0 in every case.

#### Scenario: The guard runs outside git

- GIVEN a folder that no git work tree holds
- WHEN the operator runs `exobrain store-guard` there, and `.envrc` runs `exobrain store-guard --daily` there
- THEN the operator run exits 2 with the reason on stderr, and the daily run exits 0 and prints nothing

Verify: `cargo nextest run -E 'test(exit_codes)'`

### `cli:store-guard-daily-runs-once-a-day` — The daily guard scans at most once a day

When `exobrain store-guard --daily` runs, the guard MUST take these steps in order:

1. It takes the lock of the checkout.
2. If the stamp holds today's local date, it skips the scan.
3. Otherwise, it writes today's date to the stamp atomically before it scans.

The lock is an `O_EXCL` file beside the stamp that holds the owner's pid. A run takes the lock over when procfs shows its owner gone, or after 15 minutes where procfs cannot judge. A second concurrent run exits without a scan. The stamp is `${XDG_STATE_HOME:-$HOME/.local/state}/exobrain/store-guard/<basename>-<digest16>.stamp`, and the digest is the SHA-256 of the canonical checkout path. If the guard cannot write the state folder, it skips the scan and prints one line on stderr. On a clean store, the guard prints nothing. When it finds copies, it prints a three-line warning on stderr.

#### Scenario: The first scan of the day fails

- GIVEN a checkout whose stamp holds yesterday's date
- WHEN the daily run writes today's stamp and its `nix path-info` call then fails
- THEN no later run scans again that day, because the stamp landed before the scan

Verify: `cargo nextest run -E 'test(daily_runs_once_a_day)'`

### `cli:store-guard-daily-runs-prek-gc-once-a-week` — The daily guard runs prek cache gc once a week

After the daily scan of a checkout, `exobrain store-guard --daily` MUST take these steps in order:

1. If `prek` is not on `PATH`, it stops without a message.
2. It resolves the prek store with `prek cache dir`.
3. It takes the lock of that store.
4. If the weekly stamp of that store is less than 7 days old, it skips `prek cache gc`.
5. Otherwise, it writes today's date to the weekly stamp atomically, and then runs `prek cache gc`.

The weekly stamp and its lock belong to the prek store, not to a checkout. They are `${XDG_STATE_HOME:-$HOME/.local/state}/exobrain/store-guard/gc-<basename>-<digest16>.stamp` and the `.lock` beside it, and the digest is the SHA-256 of the store path. Every checkout on one store shares them, so two checkouts that run at once start one `gc` at most. The lock follows the rules of the daily lock. The daily stamp comes first, so the guard calls `prek` at most once a day per checkout.

When `gc` frees space, the guard prints one line on stderr with the freed size. When `gc` frees nothing, the guard prints nothing. When `gc` fails, the guard prints the error on one line and still exits 0.

On prek 0.5.7 to 0.5.11, `gc` can remove the files of a running hook. `prek run` holds the store lock only while it prepares and installs the hooks, and the hooks run outside the lock. `gc` keeps only the clones and environments that the configurations on disk pin at that moment. It also deletes the `scratch/` folder of the store, where a hook with a shell entry writes its script as it starts. If a configuration moves a `rev` while its hooks run, a `gc` in that window removes the clone and the environment of the old `rev`. The effect is one failed hook run. A rerun installs the new `rev` and passes, and nothing else is lost. [`KI-prek-cache-gc-removes-a-running-hook-environment`](../reference/known-issues/KI-prek-cache-gc-removes-a-running-hook-environment.md) records the defect and the prek release that retires it.

#### Scenario: A configuration moves its rev during a hook run

- GIVEN a prek hook run whose hook reads a file of its clone, and a configuration on disk that moves the `rev` of that repository during the run
- WHEN the weekly `prek cache gc` runs before the hook reads the file
- THEN `gc` removes the clone, and the hook run fails

The test of this scenario asserts the race only for prek 0.5.7 to 0.5.11. For any other version, it prints the version and skips, and this rule needs a review for that version.

Verify: `cargo nextest run -E 'test(weekly_prek_gc)'`

### `cli:store-guard-daily-warns-about-a-large-build-folder` — The daily guard warns about a large build folder

After the daily scan of a checkout, and before the weekly `prek cache gc`, `exobrain store-guard --daily` MUST take these steps in order:

1. If the top level of the checkout holds no `Cargo.toml`, or `cargo` is not on `PATH`, it stops without a message.
2. It reads the build folder from the `target_directory` that `cargo metadata --no-deps --format-version 1` reports at the top level, so `CARGO_TARGET_DIR` and `build.target-dir` apply. If the folder does not exist, it stops without a message.
3. It measures the exclusive bytes of the folder, the bytes that deleting the folder would free.
4. If the folder holds more exclusive bytes than the limit, it prints one line on stderr with the folder, its size, and the command `cargo clean --target-dir <folder>`.

The limit is 10 GB, and `EXOBRAIN_TARGET_LIMIT` overrides it with a size in the form of `--min-size`. The guard measures each regular file with `measure_file` of the `kache-fs` crate, at the version of the kache that the host runs, and counts its `unique` bytes: the extents that the `FIEMAP` ioctl does not flag as shared. A copy-on-write clone of a kache entry therefore counts only the blocks that the build changed. Where the filesystem has no `FIEMAP` and `unique` is unknown, a file counts its allocated bytes, because such a filesystem shares no extent. A file with several hard links counts once, by device and inode, and only when every link lies inside the folder. The walk follows no symbolic link, and a file that a running build deletes counts as zero. The guard never uses `du`, which counts each shared extent in full, and calls no other tool for the measure.

One limit is known. The count is per file, so an extent that two files of the same folder share counts as shared in both, and the total can read a little low.

The guard measures only the folder of the checkout that runs it. Each worktree runs its own daily guard with its own environment, so each one reports its own folder. Under the limit, the guard prints nothing. When `cargo metadata` or the walk fails, the guard prints the error on one line and still exits 0. The guard never deletes the folder.

#### Scenario: Two worktrees build into folders outside the checkout

- GIVEN a main checkout with `CARGO_TARGET_DIR=builds/main` and a worktree with `CARGO_TARGET_DIR=builds/feat`, both over the limit
- WHEN `.envrc` runs `exobrain store-guard --daily` in each one
- THEN each run prints one line that names its own folder and never the folder of the other

The tests run on a temp dir and read no network. The test of a clone runs in a temp dir on the filesystem of the build output, and it reports a skip where that filesystem cannot reflink.

Verify: `cargo nextest run -E 'test(large_build_folder)'`

### `cli:store-guard-stays-quiet-when-switched-off` — The daily guard stays off when switched off

In either of these conditions, `store-guard --daily` MUST exit 0 without a scan, without a build-folder check, without `prek cache gc`, and without a stamp:

- `EXOBRAIN_STORE_GUARD` is `0`.
- One of `CI`, `GITHUB_ACTIONS`, `GITLAB_CI`, `BUILDKITE`, `CIRCLECI`, and `TF_BUILD` holds a value other than empty, `0`, or `false` in any case.

The switches bind only `--daily`, and an operator run always scans.

#### Scenario: A CI runner exports an empty CI variable

- GIVEN an environment with `CI` set to the empty string
- WHEN `.envrc` runs `exobrain store-guard --daily`
- THEN the guard runs and stamps, because an empty value counts as an explicit no

Verify: `cargo nextest run -E 'test(daily_stays_quiet_when_switched_off)'`

### `cli:output-is-human-or-json` — Output is human text or one JSON object

The binary MUST print its result on stdout, as human text by default and as one JSON object under `--json`. It MUST print every diagnostic on stderr. `host-check`, `hooks`, and `outdated` have no result, so they print nothing on stdout. The result of `skill` is a document or a list of names, which it prints as it stands, with no `--json`.

The `store-guard` object holds these keys:

- `checkout`
- `copies`, with `path`, `nar_size`, `registered`, and `origin` for each copy
- `unattributed`, with `path` and `reason` for each copy of unknown origin
- `total_nar_size`

`registered` is an RFC 3339 timestamp in UTC. The human report ends with these items:

- the total
- the remedy "run `nix store gc` once no process uses these paths"
- the rule ID `project-platform:a-flake-is-read-from-its-git-tree`

#### Scenario: A script reads the report

- GIVEN a checkout with one store copy and one copy of unknown origin
- WHEN a script runs `exobrain store-guard --json`
- THEN stdout parses as one object with exactly the documented keys, and the exit code is 1

Verify: `cargo nextest run -E 'test(output_is_human_or_json)'`

### `cli:skill-prints-the-embedded-skills` — skill prints the agent skills that the binary carries

The binary MUST carry each skill of `crates/exobrain/skills/<name>/` and print it on request, in the form that `sdd skill` and `rk skill` use. A skill is a package folder: a `SKILL.md` with `name` and `description` frontmatter, and the files that it names, such as `references/<task>.md`. The binary embeds every file of the folder at compile time, so a project reads the skill of the CLI on its host with no checkout of exobrain-tech. The binary serves two commands:

- `exobrain skill list` prints the name of each skill, one per line.
- `exobrain skill show <name> [--file <path>]` prints the `SKILL.md` of the skill, or the file at `<path>` relative to its folder, byte for byte.

An unknown name or path prints its reason on stderr, prints nothing on stdout, and exits 2. A skill names each set and chapter of exobrain-tech that it reads as `$EXOBRAIN_TECH/<path>`, and the hook `skill-sources` fails on each such path that the checkout does not hold. `exobrain-hooks-setup` sets up the git hooks of a project from the reference sets, as chapter 04 of `programming/project-bootstrap/` says.

#### Scenario: A skill gains a reference file

- GIVEN a skill folder that holds a new `references/<task>.md`, and a binary that does not embed it
- WHEN the test suite runs `exobrain skill show <name> --file references/<task>.md`
- THEN the command exits 2, and the test fails until the binary embeds the file

Verify: `cargo nextest run -E 'test(skill_prints_the_embedded_skills)'`

### `cli:host-check-validates-the-file` — host-check parses the file with a typed parser

When `exobrain host-check` runs, the checker MUST read the first `host-requirements.toml` in the working folder or one of its parent folders. It MUST accept only `schema = 1` and `[tools.<name>]` tables, with an optional `version` and an optional `version_cmd` each. `<name>` is a plain command name, with no whitespace, no control character, and no `/`. `version_cmd` holds no control character. `version` is a Cargo version requirement, or it is `"*"` alone. Each comma-separated comparator of the requirement starts with `>=`, `>`, `<=`, `<`, `=`, `~`, or `^`.

The checker MUST reject each of these inputs:

- an unknown key
- another `schema`
- a comparator with no operator, such as `"2.57.0"`, `"2.*"`, or `"2.x"`
- a requirement that Cargo's grammar does not parse

It reports every problem that the TOML syntax lets it reach, one line each, with the file, the tool, and the reason. It quotes each value of the file with escapes, so no value breaks a line. With `--parse-only`, the checker stops after the parse and runs no tool.

#### Scenario: A version has no operator

- GIVEN a file whose `gh` table holds `version = "2.57.0"` and whose `git` table holds an unknown key
- WHEN the `host-requirements` hook runs `exobrain host-check --parse-only`
- THEN the checker exits 2 with one line for `gh` and one for `git`, and runs no tool

Verify: `cargo nextest run -E 'test(validates_the_file)'`

### `cli:host-check-checks-the-named-tools` — host-check checks the tools a call names

When `exobrain host-check` runs with no tool name, the checker MUST check every tool of the file. With tool names, it MUST check only those, and MUST exit 2 without a check when a name is not in the file. For each tool, the checker looks up the first executable of that name on `PATH`. For a tool whose `version` is a requirement, it runs the tool with `version_cmd`. `version_cmd` is `--version` by default, and the checker passes it as one argument. The checker reads the first version of stdout, then of stderr. It takes two or three numbers and pads a two-part version to `x.y.0`. A tool with no `version` or with `version = "*"` gets a presence check only, and the checker never runs it. No check picks another tool or falls back.

#### Scenario: A host lacks a tool that one recipe uses

- GIVEN a file that lists `git`, `gh`, and `prek`, on a host without `gh`
- WHEN a `justfile` runs `exobrain host-check git prek` for every recipe
- THEN the check passes, and only a call that names `gh` fails

Verify: `cargo nextest run -E 'test(checks_the_named_tools)'`

### `cli:host-check-reports-every-failure` — host-check reports every failure before it exits

The checker MUST check every selected tool before it exits. It MUST print one line on stderr for each failure, in one of these forms:

- `host tool <name> is missing`
- `host tool <name> <version> does not match <requirement> (host-requirements.toml)`
- `host tool <name> printed no version for` the probe command

#### Scenario: Two tools fail at once

- GIVEN a host with `gh` 2.40.1 against `>=2.57.0`, and no `git`
- WHEN a recipe runs `exobrain host-check`
- THEN stderr holds one line for `gh` and one for `git`, and the exit code is 1

Verify: `cargo nextest run -E 'test(reports_every_failure)'`

### `cli:host-check-caches-versions` — host-check caches each version it reads

The checker MUST cache each version that it reads under `${XDG_CACHE_HOME:-$HOME/.cache}/exobrain/host-check/`. These values key each entry:

- the real path of the binary
- its device, inode, size, and modification time
- its `version_cmd`

A cache hit runs no tool. If the checker cannot read or write a cache entry, it runs the tool.

#### Scenario: A package manager upgrades a tool in place

- GIVEN a cached version of `/usr/bin/gh`
- WHEN the package manager replaces `/usr/bin/gh` with an older build, and a recipe runs the check
- THEN the modification time no longer matches, so the checker runs the tool and reports the new version

Verify: `cargo nextest run -E 'test(caches_versions)'`

### `cli:host-check-exit-codes` — host-check exits by what it found

The checker MUST exit with one of these codes:

- 0 when every selected tool passes
- 1 when at least one tool is missing or misses its requirement
- 2 when the checker finds no file, the file fails the parse, or a call names a tool that the file does not list

#### Scenario: A project has no file

- GIVEN a folder with no `host-requirements.toml` in it or above it
- WHEN a recipe runs `exobrain host-check`
- THEN the checker exits 2 and names the folder it searched from

Verify: `cargo nextest run -E 'test(host_check_exit_codes)'`

### `cli:a-verb-fills-a-named-gap` — Each verb fills a gap that the feature document names

The binary MUST carry only verbs that fill a gap that `programming/project-bootstrap/COMPARISON-dev-tool-features.md` names in the exobrain column of its tables.

A gap is a feature of a dev environment that no native tool covers, such as a check of the host tools or a warning about the age of a pin. Where a native command covers a feature, a project runs that command, and the binary adds no verb for it. Where a tool outside the managers covers a gap, such as `flake-edit` for the move of a flake tag pin, the document names that tool, and the binary adds no verb either. The test reads each subcommand of the binary and requires the text `exobrain <verb>` in a table row of the document.

#### Scenario: A verb repeats a native command

- GIVEN a new verb `exobrain bump` that moves a flake tag pin, which `flake-edit update <input>` already moves
- WHEN the document names no gap for `exobrain bump`
- THEN the test fails and names `bump`, and a review of the document finds the native command

Verify: `cargo nextest run -E 'test(a_verb_fills_a_named_gap)'`

## Unenforced

No command decides these rules, or a command decides only part of each. A reviewer confirms the rest.

| Rule                           | Reviewer confirms                                                                                        |
| ------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `cli:a-verb-fills-a-named-gap` | Each gap that the comparison document names has no native command in any tool that the document compares |
