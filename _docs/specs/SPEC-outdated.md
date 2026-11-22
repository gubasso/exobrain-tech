# Outdated Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`outdated:warns-when-nixpkgs-ages` — outdated warns when the nixpkgs pin of a lock is over 30 days old](#outdatedwarns-when-nixpkgs-ages--outdated-warns-when-the-nixpkgs-pin-of-a-lock-is-over-30-days-old)
    - [Scenario: A project holds two locks of different ages](#scenario-a-project-holds-two-locks-of-different-ages)
  - [`outdated:age-reads-only-the-lock` — The age check reads only the lock](#outdatedage-reads-only-the-lock--the-age-check-reads-only-the-lock)
    - [Scenario: A laptop loads a project offline](#scenario-a-laptop-loads-a-project-offline)
  - [`outdated:always-exits-0` — outdated exits 0 in every case](#outdatedalways-exits-0--outdated-exits-0-in-every-case)
    - [Scenario: A project deletes a lock that its `.envrc` names](#scenario-a-project-deletes-a-lock-that-its-envrc-names)
  - [`outdated:warns-about-a-newer-release` — outdated warns when an input pinned at a version tag has a newer release](#outdatedwarns-about-a-newer-release--outdated-warns-when-an-input-pinned-at-a-version-tag-has-a-newer-release)
    - [Scenario: A project pins release-kit below its latest release](#scenario-a-project-pins-release-kit-below-its-latest-release)
  - [`outdated:reads-each-release-once-a-week` — outdated reads the latest tag of a repository once a week](#outdatedreads-each-release-once-a-week--outdated-reads-the-latest-tag-of-a-repository-once-a-week)
    - [Scenario: Two checkouts pin the same repository](#scenario-two-checkouts-pin-the-same-repository)
  - [`outdated:bounds-each-read` — outdated ends each read of the tags after 10 seconds](#outdatedbounds-each-read--outdated-ends-each-read-of-the-tags-after-10-seconds)
    - [Scenario: The network hangs during a shell load](#scenario-the-network-hangs-during-a-shell-load)

<!--TOC-->

## Purpose

These rules govern `exobrain outdated [<lock>...]`, the verb of the `exobrain` binary that reports what is behind in a project and changes nothing. A project runs it from one line in `.envrc`, so it runs on each shell load. This spec owns the age check of the nixpkgs pin of a flake lock: which node is the clock, how the command judges its age, what it prints, and what it reads. It also owns the release check of each input pinned at a version tag: which inputs it judges, how it reads the latest release, how it caches and bounds that read, and what it prints. A call names zero or more locks, and the default is `flake.lock` alone. `SPEC-cli.md` owns the rest of the binary, its output streams included.

## Requirements

### `outdated:warns-when-nixpkgs-ages` — outdated warns when the nixpkgs pin of a lock is over 30 days old

When `exobrain outdated` runs, the command MUST judge each lock on its own and print one line on stderr for each lock whose nixpkgs pin is over 30 days old, and nothing for the other locks.

The clock of a lock is the node that the `nixpkgs` input of its root node resolves to. Where that input is a node name, the clock is that node. Where it is a follows path, a list of input names, the command walks the path from the root node. A node named `nixpkgs` that the root does not reach is not the clock.

The age is the whole days between the `locked.lastModified` of the clock and now, rounded down. A `lastModified` in the future counts as age 0. The limit is 30 days, and no flag, environment variable, or project file changes it. A pin of exactly 30 days stays silent.

The line names the lock as the call gave it and the folder of the lock as a flake reference, `.` for `flake.lock` and `./nix` for `nix/flake.lock`:

```text
nix/flake.lock: nixpkgs pin is 41 days old (limit 30). Move it with nix flake update --flake ./nix when ready.
```

#### Scenario: A project holds two locks of different ages

- GIVEN a `flake.lock` whose nixpkgs pin is 10 days old, and a `nix/flake.lock` whose pin is 41 days old
- WHEN `.envrc` runs `exobrain outdated flake.lock nix/flake.lock`
- THEN stderr holds one line, for `nix/flake.lock`, and it names `--flake ./nix`

Verify: `cargo nextest run -E 'test(/^outdated_(warns|silent|judges|names|follows)_/)'`

### `outdated:age-reads-only-the-lock` — The age check reads only the lock

The age check MUST read only the lock file of each lock that it judges. It reads no network, no cache, and no state file, calls no `nix`, and writes no file.

#### Scenario: A laptop loads a project offline

- GIVEN an empty `XDG_STATE_HOME` and `XDG_CACHE_HOME`, and a `PATH` whose `nix` and `gh` only log their calls
- WHEN the age check judges a lock whose nixpkgs pin is 41 days old
- THEN it returns the warning, no `nix` or `gh` call is logged, and no file in the scratch world changes

Verify: `cargo nextest run -E 'test(outdated_age_reads_only_the_lock)'`

### `outdated:always-exits-0` — outdated exits 0 in every case

The command MUST exit 0 and print nothing on stdout in every case. It never writes a lock, `flake.nix`, or any other file of the project.

Where a lock gives no clock, the command prints one line on stderr for it and goes on with the next lock:

- The path does not exist: `<path>: no such file, so the age of the nixpkgs pin is unknown.`
- The file does not parse as a lock, or it has no root node: `<path>: cannot read the lock (<error>), so the age of the nixpkgs pin is unknown.`
- The root has no `nixpkgs` input, or its follows path names a missing input: `<path>: the root of the lock has no nixpkgs input, so the lock has no clock.`
- The clock has no `locked.lastModified`, such as a `path` input: `<path>: the nixpkgs pin carries no lastModified, so its age is unknown.`

A lock that the call names comes from the `.envrc` of the project, so its line repeats on each shell load until the developer fixes it. Where the call names no lock and `flake.lock` does not exist, the project has no flake, and the command prints nothing.

#### Scenario: A project deletes a lock that its `.envrc` names

- GIVEN an `.envrc` that runs `exobrain outdated flake.lock nix/flake.lock`, and no `nix/flake.lock`
- WHEN direnv loads the project
- THEN stderr holds the no-such-file line for `nix/flake.lock`, and the exit code is 0

Verify: `cargo nextest run -E 'test(/^outdated_(without|missing|unparsable|no)_/)'`

### `outdated:warns-about-a-newer-release` — outdated warns when an input pinned at a version tag has a newer release

When `exobrain outdated` runs, the command MUST print one line on stderr for each root input of each lock that is pinned at a version tag of a GitHub repository whose latest tag is a greater version.

The command judges an input of the root node that names a node, not a follows path. Its node `original` has `type` `github` and a `ref` of the form `vMAJOR.MINOR.PATCH` or `MAJOR.MINOR.PATCH`. The command skips a branch, a commit, an input of every other type, and a follows path in silence, and reads no tag for them.

The latest tag is the one that `flake-edit update` moves to. The command reads the first page of 100 tags of the repository, and reads the next pages only while every page is full and holds no version tag. It drops a `refs/tags/` prefix and every character before the first digit, pads `1` and `1.2` to three parts, and keeps each tag that then parses as a semantic version. The latest tag is the highest of them by semantic version precedence, a pre-release tag included. The command warns only when that tag is a greater version than the pinned tag.

The line names the lock as the call gave it. For `flake.lock`, the move is `flake-edit update <input>`. For every other lock, the move adds `--flake <folder> --lock-file <lock>`, because `flake-edit` reads `flake.nix` and `flake.lock` of the current folder by default:

```text
flake.lock: release-kit is pinned at v0.10.1, and v0.10.3 is out. Move it with flake-edit update release-kit when ready.
nix/flake.lock: tool is pinned at v1.0.0, and v1.1.0 is out. Move it with flake-edit --flake ./nix --lock-file nix/flake.lock update tool when ready.
```

#### Scenario: A project pins release-kit below its latest release

- GIVEN a `flake.lock` whose root input `release-kit` pins `github:gubasso/release-kit/v0.10.1`, and a repository whose highest tag is `v0.10.3`
- WHEN `.envrc` runs `exobrain outdated`
- THEN stderr holds the line that names `v0.10.3` and `flake-edit update release-kit`, and the exit code is 0

Verify: `cargo nextest run -E 'test(/^outdated_(warns_about|orders|names_the_lock|silent_on|skips|reads_two)_/)'`

### `outdated:reads-each-release-once-a-week` — outdated reads the latest tag of a repository once a week

The command MUST keep the latest tag of each repository in a cache that every checkout of the host shares, and read the tags again only when the cache is due.

The cache of `<owner>/<repo>` is the file `${XDG_STATE_HOME:-$HOME/.local/state}/exobrain/releases/<owner>/<repo>.json`, with the owner and the repository in lower case, so two repositories never share a file. The file holds the time of the last attempt, the time of the last success, the latest tag, and the error of the last attempt:

```json
{ "attempted": 1790000000, "succeeded": 1790000000, "latest": "v0.10.3", "error": null }
```

A read is due 7 days after the last success, or 1 hour after a failed attempt. A missing or unreadable cache, or a time in the future, is due. `--refresh` makes every read due. One call reads each repository once, however many inputs and locks pin it, and runs the due reads in parallel. Between reads, the command judges each input against the latest tag of the cache.

A due read takes the lock `<repo>.lock` beside the cache, an `O_EXCL` file that holds the process id of its owner, under the takeover rules of the store-guard lock. A run that finds the lock held reads nothing and judges from the cache it has.

#### Scenario: Two checkouts pin the same repository

- GIVEN two checkouts whose `flake.lock` pins `github:gubasso/release-kit/v0.10.1`, and an empty cache
- WHEN `exobrain outdated` runs in the first checkout and then in the second
- THEN the first run reads the tags once, and the second run reads nothing and prints the same warning

Verify: `cargo nextest run -E 'test(/^outdated_(reads_each|retries|shares|keeps)_/)'`

### `outdated:bounds-each-read` — outdated ends each read of the tags after 10 seconds

The command MUST read every tag through `gh api`, as `hooks:read-github-only-through-gh` holds for the hooks verbs, and end the `gh` process of a read that runs longer than 10 seconds. The due reads of one call run in parallel, so they hold a shell load for 10 seconds at most.

A failed read, a timeout, a missing `gh`, and a `gh` with no login each record the error in the cache and print one line on stderr, once, at the failed attempt:

```text
cannot read the latest release of gubasso/release-kit (gh is not on PATH); the next read runs in 1 hour, or now with --refresh
```

The command still exits 0, and judges the input against the latest tag of the last success, where the cache holds one.

#### Scenario: The network hangs during a shell load

- GIVEN a `flake.lock` that pins a GitHub input at a version tag, an empty cache, and a `gh` that hangs
- WHEN `exobrain outdated` runs
- THEN the command ends the `gh` process after 10 seconds, prints the failed-read line, and exits 0

Verify: `cargo nextest run -E 'test(/^outdated_(bounds|without_gh)_/)'`
