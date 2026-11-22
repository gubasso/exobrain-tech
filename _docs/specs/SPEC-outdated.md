# Outdated Specification

## Purpose

These rules govern `exobrain outdated [<lock>...]`, the verb of the `exobrain` binary that reports what is behind in a project and changes nothing. A project runs it from one line in `.envrc`, so it runs on each shell load. This spec owns the age check of the nixpkgs pin of a flake lock: which node is the clock, how the command judges its age, what it prints, and what it reads. A call names zero or more locks, and the default is `flake.lock` alone. `SPEC-cli.md` owns the rest of the binary, its output streams included.

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
