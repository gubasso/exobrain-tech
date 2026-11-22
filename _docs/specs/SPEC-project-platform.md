# Project Platform Specification

<!--TOC-->

- [Purpose](#purpose)
- [Requirements](#requirements)
  - [`project-platform:one-supported-system` — The flake exposes one system](#project-platformone-supported-system--the-flake-exposes-one-system)
    - [Scenario: A second system is added inside a nested attribute set](#scenario-a-second-system-is-added-inside-a-nested-attribute-set)
  - [`project-platform:every-system-has-a-native-check` — Every supported system has a native required check](#project-platformevery-system-has-a-native-check--every-supported-system-has-a-native-required-check)
    - [Scenario: A system is advertised with no host behind it](#scenario-a-system-is-advertised-with-no-host-behind-it)
  - [`project-platform:library-topics-stay-platform-neutral` — Library topics stay open](#project-platformlibrary-topics-stay-platform-neutral--library-topics-stay-open)
    - [Scenario: A portable chapter describes an access-control mode](#scenario-a-portable-chapter-describes-an-access-control-mode)
  - [`project-platform:the-most-specific-native-owner-holds-each-dependency` — The most specific native owner holds each dependency](#project-platformthe-most-specific-native-owner-holds-each-dependency--the-most-specific-native-owner-holds-each-dependency)
    - [Scenario: A hook that prek can build](#scenario-a-hook-that-prek-can-build)
  - [`project-platform:the-host-supplies-the-hook-runner` — The host supplies the git hook runner](#project-platformthe-host-supplies-the-hook-runner--the-host-supplies-the-git-hook-runner)
    - [Scenario: A devshell lists the hook runner](#scenario-a-devshell-lists-the-hook-runner)
  - [`project-platform:host-requirements-list-the-host-tools` — host-requirements.toml lists the host tools](#project-platformhost-requirements-list-the-host-tools--host-requirementstoml-lists-the-host-tools)
    - [Scenario: A recipe needs a tool that the host lacks](#scenario-a-recipe-needs-a-tool-that-the-host-lacks)
  - [`project-platform:the-hooks-are-the-only-gate` — The git hooks are the only gate](#project-platformthe-hooks-are-the-only-gate--the-git-hooks-are-the-only-gate)
    - [Scenario: A check is added outside the hooks](#scenario-a-check-is-added-outside-the-hooks)
    - [Scenario: A hook that CI cannot run](#scenario-a-hook-that-ci-cannot-run)
    - [Scenario: A hook that only CI can run](#scenario-a-hook-that-only-ci-can-run)
  - [`project-platform:hooks-exclude-generated-files` — The hooks leave generated files to their generator](#project-platformhooks-exclude-generated-files--the-hooks-leave-generated-files-to-their-generator)
    - [Scenario: sdd rewrites its manifest](#scenario-sdd-rewrites-its-manifest)
  - [`project-platform:a-flake-is-read-from-its-git-tree` — A flake reference reads the git tree](#project-platforma-flake-is-read-from-its-git-tree--a-flake-reference-reads-the-git-tree)
    - [Scenario: A hook evaluates its own flake](#scenario-a-hook-evaluates-its-own-flake)
  - [`project-platform:a-large-build-folder-gets-a-warning` — A large build folder gets a warning](#project-platforma-large-build-folder-gets-a-warning--a-large-build-folder-gets-a-warning)
    - [Scenario: A worktree restores its dependencies from the compile cache](#scenario-a-worktree-restores-its-dependencies-from-the-compile-cache)
  - [`project-platform:a-rust-project-builds-through-the-host-compile-cache` — A Rust project builds through the compile cache of the host](#project-platforma-rust-project-builds-through-the-host-compile-cache--a-rust-project-builds-through-the-compile-cache-of-the-host)
    - [Scenario: CI builds a Rust project](#scenario-ci-builds-a-rust-project)
- [Unenforced](#unenforced)

<!--TOC-->

## Purpose

These rules govern four choices of this checkout:

- the systems that its toolchain supports
- where its tools come from
- what its git hooks run
- the boundary between those choices and the knowledge that the library publishes

They bind development and verification here. They do not narrow what a chapter can be about.

## Requirements

### `project-platform:one-supported-system` — The flake exposes one system

The flake MUST key every system-dependent output by `x86_64-linux` and by no other system.

#### Scenario: A second system is added inside a nested attribute set

- GIVEN a `devShells` set that binds `aarch64-linux` beside the supported system
- WHEN the check evaluates the flake's own outputs
- THEN it reports both keys and fails, because a nested set declares an output no text scan sees

Verify: `just test-one-system`

### `project-platform:every-system-has-a-native-check` — Every supported system has a native required check

The project MUST run its required check natively on each system that it advertises. The required check is the commit and push hook stages. The project MUST NOT advertise a system that no check exercises.

#### Scenario: A system is advertised with no host behind it

- GIVEN a proposal to expose a second system from the flake
- WHEN no host of that system runs the hook stages
- THEN the support claim is refused, because an untested claim is a promise the project does not keep

Verify: `[ "$(uname -s)-$(uname -m)" = Linux-x86_64 ] && just lint`

### `project-platform:library-topics-stay-platform-neutral` — Library topics stay open

The author MUST NOT narrow a chapter's subject to this project's supported system, and MUST scope a platform-specific claim to the implementation that proves it.

#### Scenario: A portable chapter describes an access-control mode

- GIVEN a rule that requires bounded access to a staged directory
- WHEN an implementation proves owner-only modes on Linux
- THEN the chapter states the portable property and attributes the mode to that implementation

Verify: reviewer confirms the chapter states no operating system as a universal requirement

### `project-platform:the-most-specific-native-owner-holds-each-dependency` — The most specific native owner holds each dependency

The project MUST give each dependency exactly one owner, the most specific file that declares it natively. `.pre-commit-config.yaml` pins each hook tool for which the hook runner builds an environment. It owns a tool that only a hook uses, even when the tool reads the files of this repository.

`flake.nix` holds these items:

- the toolchain
- the task runner
- the project tools, such as `sdd`
- each tool that no hook language can build
- each runtime that a hook takes from the system, such as `nodejs` for a node hook under `language_version: system`

`flake.nix` holds no language library and no hook runner. A hook that keeps `language: system` or `script` carries an `# owner:` comment with its class. A hook that calls a host tool other than a base tool, such as `hook-revisions` calling `exobrain`, takes the class `host-tool` with the owner `host`.

The host supplies only these tools:

- `git`, `bash`, `awk`, `sed`, `grep`, and the coreutils
- the hook runner `prek`
- `nix`, `direnv`, and `uv`
- the standard ELF loader

The project uses a host tool by name, as it uses `git`, and lists it in `host-requirements.toml` (`project-platform:host-requirements-list-the-host-tools`).

uv builds Python environments, so the host supplies it. uv owns the interpreter that it installs. The devshell holds neither uv nor a Python for a uv project. Such a project pins uv with `[tool.uv] required-version` and the interpreter patch with `.python-version`. The one Python in the flake of a uv project builds its uv2nix package. That Python is the nixpkgs interpreter of the pinned patch, and a flake check holds the two equal.

The chapter that states the general rule is `programming/project-bootstrap/03-local-dev-environment.md`, "Who owns each dependency".

#### Scenario: A hook that prek can build

- GIVEN an empty prek home and a `PATH` that holds only the host tools
- WHEN prek builds the environment of every hook, the rust environment of the secret scanner included
- THEN each managed hook builds from its pinned `rev`, the rust hook with the cargo of the devshell, and the python hooks with the uv of the host

Verify: `d="$(mktemp -d)"; env -i HOME="$HOME" NIX_SSL_CERT_FILE="$NIX_SSL_CERT_FILE" PATH="$(dirname "$(command -v nix)"):$(dirname "$(command -v prek)"):$(dirname "$(command -v uv)"):/usr/bin" PREK_HOME="$d" PREK_UV_SOURCE=none nix develop --command prek prepare-hooks; rc=$?; chmod -R u+w "$d"; rm -rf "$d"; [ "$rc" -eq 0 ]`

The verification rebuilds every hook environment. It therefore runs once for each change of `flake.nix` or `.pre-commit-config.yaml`, never on every check. `PREK_UV_SOURCE=none` stops prek from installing a uv of its own, so the python hooks prove the uv of the host ([prek language support](https://prek.j178.dev/reference/language-support/)).

### `project-platform:the-host-supplies-the-hook-runner` — The host supplies the git hook runner

The host MUST supply the git hook runner, `prek`, and `flake.nix` MUST NOT hold one. Git runs an installed hook from every shell, with or without the devshell, so the runner belongs beside git. In a devshell, a runner written in Python also exports its own Python on `PYTHONPATH`. That Python shadows the packages of a uv project. `.pre-commit-config.yaml` names the oldest runner that the project accepts with `minimum_prek_version`.

A line of a flake that installs the runner on a host carries `hook-runner-ok: <reason>`. The hook `no-hook-runner-in-flake`, which this repository publishes in `.pre-commit-hooks.yaml`, enforces the rule line by line in `flake.nix` and under `nix/`. It reads one line at a time. A `with pkgs;` list can span several lines and hold the runner among other items on one line. The hook cannot see the runner there. The chapter that states the general rule is `tools/nix/02-per-project-devshell.md`, "Git hooks".

#### Scenario: A devshell lists the hook runner

- GIVEN a `flake.nix` whose devshell lists the runner beside the task runner
- WHEN the commit stage runs
- THEN the hook refuses the line, because the host supplies the runner

Verify: `prek run no-hook-runner-in-flake --all-files`

### `project-platform:host-requirements-list-the-host-tools` — host-requirements.toml lists the host tools

The project MUST list each host tool that these callers use, once, in `host-requirements.toml` at its root:

- its `justfile`
- its `Makefile`
- its git hooks
- its scripts

The project MUST check a tool with `exobrain host-check` where it uses one. `SPEC-cli.md` owns the format of the file and the behavior of the checker. `version` is a Cargo requirement with an explicit operator. A tool that enforces its own version from its own configuration takes `version = "*"`, such as prek with `minimum_prek_version`. The checker then checks only its presence. The file lists no tool that a manifest, `.pre-commit-config.yaml`, or the devshell owns.

A call site names the tools that it needs. A `justfile` checks the tools of every recipe once, in a top-level assignment such as `_host := shell("exobrain host-check git wt")`. A recipe or a script that alone uses a tool checks it in its first line. A bare `exobrain host-check` checks every tool of the file, so it stands only where every recipe needs every listed tool.

A `Makefile` checks in the first line of each dev target that needs the tool, such as `lint`, `test`, or `check`. A package build runs the same file where `exobrain` is absent. The `Makefile` therefore never checks in a `$(shell ...)` at parse time, or in `all`, `install`, or a build target. A git hook needs no check, because prek names a missing executable.

Nothing checks when a shell enters the folder. No check picks another tool or falls back. Where a tool is missing, the developer installs it. `README.md` links the file and repeats none of it. The hook `host-requirements`, which this repository publishes in `.pre-commit-hooks.yaml`, validates the file each time it changes.

A project without the exobrain-tech flake input checks natively in its `justfile` with `require()` and `semver_matches()`. These functions read the requirement strings unchanged. Such a project cannot run the hook, and just halts on a malformed requirement at the first check that reads it.

A project with no call site lists its tools with no `version`, because no check enforces one. This repository is such a project. Its checker is its own build, which a clean clone lacks until `cargo build`.

A shipped application is outside this rule. It checks the tools that it needs at runtime in its own code, at the start of a command. That code holds the list. The chapter that states the general rule is `programming/project-bootstrap/03-local-dev-environment.md`, "Host requirements".

#### Scenario: A recipe needs a tool that the host lacks

- GIVEN a `host-requirements.toml` that lists `git` and `gh`, and a release recipe that alone uses `gh`
- WHEN a host without `gh` runs any recipe, and then the release recipe
- THEN every other recipe runs, and the release recipe stops at its first line with `host tool gh is missing`

Verify: `cargo run -q -p exobrain -- host-check --parse-only`

### `project-platform:the-hooks-are-the-only-gate` — The git hooks are the only gate

The project MUST run every check in exactly one of its hook stages, the commit stage or the push stage. A hook that only CI can run takes the `manual` stage and the group `ci-only`, and the one other exception is `dist build`, which only CI runs. A fast check on the staged files runs at commit. A costly check, such as a flake evaluation or the link check, runs at push. The push stage MUST NOT rerun a commit-stage hook.

A branch reaches the trunk through `rk integrate` and a push from a checkout that ran both stages. The change that lands is therefore the change that passed its hooks. CI MUST hold only three kinds of workflow:

- The validation workflow, `ci.yml`. Every `run:` step of its job `test` is one `prek run` call of one hook stage. The call stands alone on its line, bare or through `nix develop --command`. A push that passed its hooks therefore passes `test`. A `uses:` step pinned by commit SHA installs prek.
- Beside `test`, `ci.yml` MAY hold the `dist-plan` and `dist-build` jobs that `rk binding rust` serves, as `test` needs, and no other job. The `dist-plan` hook runs the same plan at push. `dist build` is the one check outside the hooks that only CI runs. It builds the release archives that users download. From the workflow alone, rk judges that every release target builds before a merge. The operator keeps this costly build out of the local hooks.
- The release workflows, `release-plz.yml` and `release.yml`. They create tags, releases, and artifacts, and they check nothing that the hooks do not check.
- The pull-request checks, `pr-commits.yml`, which judge what only a pull request has. No push starts them.

The hook configuration MUST declare the scope of each hook that one runtime cannot run, as a prek group, and no committed file MAY set `SKIP`. `groups: [local-only]` marks a hook that CI cannot run, and a CI-only hook takes `stages: [manual]` and `groups: [ci-only]`. The comment above each grouped hook states why the other runtime cannot run it. The job `test` selects its hooks with these steps, and each step names its stage, because a group filter with no `--stage` also runs the `manual` hooks:

```yaml
- run: nix develop --command prek run --all-files --stage pre-commit --no-group local-only
- run: nix develop --command prek run --all-files --stage pre-push --no-group local-only
- run: nix develop --command prek run --all-files --stage manual --group ci-only
```

A project with no `ci-only` hook leaves the third step out, because prek refuses a `--group` that matches no hook. `SKIP` stays for a one-time run in one runtime.

The chapter that states the general rule is `programming/project-bootstrap/04-quality-gates.md`, "Git hooks".

#### Scenario: A check is added outside the hooks

- GIVEN a new check that a CI step runs and no hook stage runs
- WHEN a contributor pushes a change that the check rejects
- THEN the push passed and CI failed, so the check moves into the commit or the push stage

#### Scenario: A hook that CI cannot run

- GIVEN a link check at push that an outside site blocks on a CI runner
- WHEN the hook takes `groups: [local-only]` with its reason in the comment
- THEN the push stage runs it on a developer host, the step `--stage pre-push --no-group local-only` leaves it out of CI, and no workflow sets `SKIP`

#### Scenario: A hook that only CI can run

- GIVEN a test that needs a secret that only the forge holds
- WHEN the hook takes `stages: [manual]` and `groups: [ci-only]` with its reason in the comment
- THEN no git hook fires it, and the step `--stage manual --group ci-only` runs it in CI

Verify: `for f in .github/workflows/*; do [ -e "$f" ] || continue; grep -Eq 'SKIP[=:]' "$f" && exit 1; grep -E 'prek run' "$f" | grep -E -- '--(no-)?group' | grep -Evq -- '--(hook-)?stage[ =]' && exit 1; case "${f##*/}" in ci.yml) awk '/^  test:/{t=1;next} /^  [^ #]/{t=0} t' "$f" | grep -E '^[[:space:]]*(- )?run:' | grep -Evq '^[[:space:]]*(- )?run:[[:space:]]+(nix develop --command )?prek run( [-A-Za-z0-9=_.,]+)*[[:space:]]*$' && exit 1; awk '/^jobs:/{j=1;next} /^[^ #]/{j=0} j && /^  [A-Za-z0-9_-]+:/' "$f" | grep -Evq '^  (test|dist-plan|dist-build):' && exit 1 ;; release-plz.yml|release.yml|pr-commits.yml) ;; *) exit 1 ;; esac; done; just lint`

### `project-platform:hooks-exclude-generated-files` — The hooks leave generated files to their generator

The project MUST exclude every file that a tool generates from every hook that rewrites or judges it. It MUST place that exclusion in the configuration of the tool, where the tool has one. The project MUST keep the file under the hooks that only read it. These are the generated files, each with its generator:

- `flake.lock` (nix)
- `.release-kit/manifest.json`, `.github/workflows/release-plz.yml`, and `.github/workflows/pr-commits.yml` (rk)
- `.spec-driven-docs/manifest.json` and `_docs/.markdownlint-cli2.jsonc` (sdd)
- `Cargo.lock` (cargo)
- `CHANGELOG.md` (release-plz)
- `.github/workflows/release.yml` (cargo-dist)

The chapter that states the general rule is `programming/project-bootstrap/04-quality-gates.md`, "Generated files".

#### Scenario: sdd rewrites its manifest

- GIVEN a version upgrade that rewrites `.spec-driven-docs/manifest.json` in sdd's own layout
- WHEN the commit stage runs
- THEN no formatter, fixer, spell check, or editorconfig check changes or rejects the file, and the JSON parser and the secret scan still read it

Verify: `nix develop --command bash -c 'p="$(dprint output-file-paths)" || exit 1; printf "%s\n" "$p" | grep -qE "/(flake\.lock|\.release-kit/manifest\.json|\.spec-driven-docs/manifest\.json|_docs/\.markdownlint-cli2\.jsonc|Cargo\.lock|CHANGELOG\.md)$" && exit 1; for f in flake.lock .release-kit/manifest.json .spec-driven-docs/manifest.json _docs/.markdownlint-cli2.jsonc Cargo.lock CHANGELOG.md .github/workflows/release-plz.yml .github/workflows/pr-commits.yml .github/workflows/release.yml; do grep -qF "\"$f\"" _typos.toml && grep -qF "\"^${f//./\\\\.}\$\"" .editorconfig-checker.json || exit 1; done'`

### `project-platform:a-flake-is-read-from-its-git-tree` — A flake reference reads the git tree

When a command, a script, or a hook names a flake in a project working tree, it MUST read the git tree. It reads the tree in one of two forms:

- `git+file://` with the root, with `?dir=` for a flake in a subfolder and `?shallow=1` where CI clones shallowly
- `.` from the CLI inside the repository

It MUST NOT name the tree as a `path:` reference or pass a bare path to `builtins.getFlake`. Either one copies every file of the folder into the store, ignored files included. A `path:` reference stays allowed in three cases:

- a `/nix/store` path
- a relative input inside a generated flake
- a throwaway fixture folder

Such a line carries `nix-path-ok: <reason>`. The hook `no-path-flake-reference`, which this repository publishes in `.pre-commit-hooks.yaml`, enforces the rule line by line. It cannot follow a reference that a program assembles from parts. The chapter that states the general rule is `languages/nix/05-tooling-gotchas-resources.md`, gotcha 1.

#### Scenario: A hook evaluates its own flake

- GIVEN a worktree with a built `target/` folder that git ignores
- WHEN a hook evaluates the flake of the worktree through `builtins.getFlake` on its bare path
- THEN the hook refuses the line, because each run would copy `target/` into the store again

Verify: `prek run no-path-flake-reference --all-files`

### `project-platform:a-large-build-folder-gets-a-warning` — A large build folder gets a warning

A Rust project that follows this canon MUST run `exobrain store-guard --daily` from its `.envrc`, and the guard MUST warn when the build folder of the checkout holds more than 10 GB of exclusive bytes. Exclusive bytes are the bytes that deleting the folder frees, so a copy-on-write clone of a compile-cache entry counts only its changed blocks. The warning names `cargo clean --target-dir <folder>`. Nothing deletes the folder on its own: a person or an agent runs the command. `SPEC-cli.md` owns how the guard finds and measures the folder, in `cli:store-guard-daily-warns-about-a-large-build-folder`. The chapter that states the convention is `programming/project-bootstrap/03-local-dev-environment.md`, "Build output cleanup".

#### Scenario: A worktree restores its dependencies from the compile cache

- GIVEN a worktree whose build folder holds 12 GB by `du`, nearly all of it clones of kache entries
- WHEN `.envrc` runs `exobrain store-guard --daily`
- THEN the guard prints nothing, because deleting the folder would free far less than 10 GB

Verify: `cargo nextest run -E 'test(large_build_folder)'`

### `project-platform:a-rust-project-builds-through-the-host-compile-cache` — A Rust project builds through the compile cache of the host

A Rust project that follows this canon MUST build through kache, the compile cache that the host owns. These parts carry the rule:

- The host installs one kache, writes its configuration, and runs its daemon as a user service. Two versions of kache never share one cache.
- The project lists `kache` in `host-requirements.toml`.
- The `.envrc` of the project sets `RUSTC_WRAPPER=kache` after `use flake`. The devshell and the host never set it, because CI enters the devshell without direnv and has no kache.
- Each worktree keeps its own build folder. The worktrees of a project never share one, because cargo judges freshness by file times and can report the build of another worktree fresh.
- The dev profile sets `debug = "line-tables-only"`, and `[profile.dev.package."*"]` sets `debug = false`. A build that needs a debugger passes `--config` for full debug information on one command.
- The kache daemon and a weekly `kache clean --stale 14d --yes` of the host delete old build output in the background, and `project-platform:a-large-build-folder-gets-a-warning` warns about the rest.

`programming/project-bootstrap/examples/rust.envrc` is the `.envrc` of such a project, and the shipped-scripts gate holds it (`knowledge-base-boundary:a-shipped-script-is-gated`). The chapter that states the convention is `programming/project-bootstrap/03-local-dev-environment.md`, "Rust builds".

#### Scenario: CI builds a Rust project

- GIVEN a Rust project whose `.envrc` sets `RUSTC_WRAPPER=kache`, and a CI runner without kache
- WHEN CI enters the devshell with `nix develop --command` and runs `cargo build`
- THEN cargo calls rustc directly, because the devshell sets no wrapper and CI reads no `.envrc`

Verify: `just test-rust-envrc`

## Unenforced

No command decides these rules, or a command decides only part of each. A reviewer confirms the rest.

| Rule                                                    | Reviewer confirms                                                 |
| ------------------------------------------------------- | ----------------------------------------------------------------- |
| `project-platform:library-topics-stay-platform-neutral` | The chapter states no operating system as a universal requirement |
