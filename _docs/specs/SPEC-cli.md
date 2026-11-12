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
  - [`cli:store-guard-stays-quiet-when-switched-off` — The daily guard stays off when switched off](#clistore-guard-stays-quiet-when-switched-off--the-daily-guard-stays-off-when-switched-off)
    - [Scenario: A CI runner exports an empty CI variable](#scenario-a-ci-runner-exports-an-empty-ci-variable)
  - [`cli:output-is-human-or-json` — Output is human text or one JSON object](#clioutput-is-human-or-json--output-is-human-text-or-one-json-object)
    - [Scenario: A script reads the report](#scenario-a-script-reads-the-report)
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

<!--TOC-->

## Purpose

These rules govern the `exobrain` binary that `crates/exobrain` builds: its verbs, their exit codes, and their output. The binary enforces rules that other specs own. `store-guard` finds the store copies that `project-platform:a-flake-is-read-from-its-git-tree` forbids. `SPEC-project-platform.md` owns that rule. This spec owns only how the guard finds, reports, and schedules the copies. `host-check` reads `host-requirements.toml`, the list of host tools of a project's dev environment, and checks them on the host. This spec owns the format of that file and how the checker reads it, checks it, and reports.

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

### `cli:store-guard-stays-quiet-when-switched-off` — The daily guard stays off when switched off

In either of these conditions, `store-guard --daily` MUST exit 0 without a scan and without a stamp:

- `EXOBRAIN_STORE_GUARD` is `0`.
- One of `CI`, `GITHUB_ACTIONS`, `GITLAB_CI`, `BUILDKITE`, `CIRCLECI`, and `TF_BUILD` holds a value other than empty, `0`, or `false` in any case.

The switches bind only `--daily`, and an operator run always scans.

#### Scenario: A CI runner exports an empty CI variable

- GIVEN an environment with `CI` set to the empty string
- WHEN `.envrc` runs `exobrain store-guard --daily`
- THEN the guard runs and stamps, because an empty value counts as an explicit no

Verify: `cargo nextest run -E 'test(daily_stays_quiet_when_switched_off)'`

### `cli:output-is-human-or-json` — Output is human text or one JSON object

The binary MUST print its result on stdout, as human text by default and as one JSON object under `--json`. It MUST print every diagnostic on stderr. `host-check` has no result, so it prints nothing on stdout.

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
