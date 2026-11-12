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
  - [`project-platform:the-hooks-are-the-only-gate` — The git hooks are the only gate](#project-platformthe-hooks-are-the-only-gate--the-git-hooks-are-the-only-gate)
    - [Scenario: A check is added outside the hooks](#scenario-a-check-is-added-outside-the-hooks)
  - [`project-platform:hooks-exclude-generated-files` — The hooks leave generated files to their generator](#project-platformhooks-exclude-generated-files--the-hooks-leave-generated-files-to-their-generator)
    - [Scenario: sdd rewrites its manifest](#scenario-sdd-rewrites-its-manifest)
  - [`project-platform:a-flake-is-read-from-its-git-tree` — A flake reference reads the git tree](#project-platforma-flake-is-read-from-its-git-tree--a-flake-reference-reads-the-git-tree)
    - [Scenario: A hook evaluates its own flake](#scenario-a-hook-evaluates-its-own-flake)

<!--TOC-->

## Purpose

Rules governing which systems this checkout's toolchain supports, where its tools come from, what its
git hooks run, and the boundary between those choices and the knowledge the library
publishes. They bind development and verification here. They do not narrow what a chapter may be
about.

## Requirements

### `project-platform:one-supported-system` — The flake exposes one system

The flake MUST key every system-dependent output by `x86_64-linux` and by no other system.

#### Scenario: A second system is added inside a nested attribute set

- GIVEN a `devShells` set that binds `aarch64-linux` beside the supported system
- WHEN the check evaluates the flake's own outputs
- THEN it reports both keys and fails, because a nested set declares an output no text scan sees

Verify: `just test-one-system`

### `project-platform:every-system-has-a-native-check` — Every supported system has a native required check

The project MUST run its required check, the commit and push hook stages, natively on each system it
advertises, and MUST NOT advertise a system no check exercises.

#### Scenario: A system is advertised with no host behind it

- GIVEN a proposal to expose a second system from the flake
- WHEN no host of that system runs the hook stages
- THEN the support claim is refused, because an untested claim is a promise the project does not keep

Verify: `[ "$(uname -s)-$(uname -m)" = Linux-x86_64 ] && just lint`

### `project-platform:library-topics-stay-platform-neutral` — Library topics stay open

The author MUST NOT narrow a chapter's subject to this project's supported system, and MUST scope a
platform-specific claim to the implementation that proves it.

#### Scenario: A portable chapter describes an access-control mode

- GIVEN a rule that requires bounded access to a staged directory
- WHEN an implementation proves owner-only modes on Linux
- THEN the chapter states the portable property and attributes the mode to that implementation

Verify: reviewer confirms the chapter states no operating system as a universal requirement

### `project-platform:the-most-specific-native-owner-holds-each-dependency` — The most specific native owner holds each dependency

The project MUST give each dependency exactly one owner, the most specific file that declares it
natively. `.pre-commit-config.yaml` pins each hook tool for which the hook runner builds an
environment, and it owns a tool that only a hook uses, even when the tool reads the files of this
repository. `flake.nix` holds the toolchain, the task runner, the project tools such as `sdd`, each
tool that no hook language can build, and each runtime that a hook takes from the system, such as
`nodejs` for a node hook under `language_version: system`. `flake.nix` holds no language library
and no hook runner. A hook that keeps `language: system` or `script` carries an `# owner:` comment
with its class. The host supplies only `git`, the hook runner `prek`, `bash`, `awk`, `sed`, `grep`,
the coreutils, `nix`, `direnv`, `uv`, and the standard ELF loader. The project uses a host tool by
name, as it uses `git`, and holds no probe and no fallback for one: where the host lacks it, the
command fails, and the developer installs it. `README.md` lists these host requirements. uv builds Python environments, so the host supplies it, and uv
owns the interpreter it installs: the devshell holds neither uv nor a Python for a uv project, which
pins uv with `[tool.uv] required-version` and the interpreter patch with `.python-version`. The one
Python in the flake of a uv project builds its uv2nix package: the nixpkgs interpreter of that
patch, held equal to it by a flake check. The chapter that states the general rule is
`programming/project-bootstrap/03-local-dev-environment.md`, "Who owns each dependency".

#### Scenario: A hook that prek can build

- GIVEN an empty prek home and a `PATH` that holds only the host tools
- WHEN prek builds the environment of every hook, the rust environment of the secret scanner
  included
- THEN each managed hook builds from its pinned `rev`, the rust hook with the cargo of the devshell,
  and the python hooks with the uv of the host

Verify: `d="$(mktemp -d)"; env -i HOME="$HOME" NIX_SSL_CERT_FILE="$NIX_SSL_CERT_FILE" PATH="$(dirname "$(command -v nix)"):$(dirname "$(command -v prek)"):$(dirname "$(command -v uv)"):/usr/bin" PREK_HOME="$d" PREK_UV_SOURCE=none nix develop --command prek prepare-hooks; rc=$?; chmod -R u+w "$d"; rm -rf "$d"; [ "$rc" -eq 0 ]`

The verification rebuilds every hook environment, so it runs once for each change of `flake.nix` or
`.pre-commit-config.yaml`, never on every check. `PREK_UV_SOURCE=none` stops prek from installing a uv of its own, so
the python hooks prove the uv of the host
([prek language support](https://prek.j178.dev/reference/language-support/)).

### `project-platform:the-host-supplies-the-hook-runner` — The host supplies the git hook runner

The host MUST supply the git hook runner, `prek`, and `flake.nix` MUST NOT hold one. Git runs an
installed hook from every shell, with or without the devshell, so the runner belongs beside git. A
runner written in Python, placed in a devshell, also exports its own Python on `PYTHONPATH`, which
shadows the packages of a uv project. `.pre-commit-config.yaml` names the oldest runner that the project accepts with
`minimum_prek_version`. A line of a flake that installs the runner on a host carries
`hook-runner-ok: <reason>`. The hook `no-hook-runner-in-flake`, which this repository publishes in
`.pre-commit-hooks.yaml`, enforces the rule line by line in `flake.nix` and under `nix/`. It reads
one line at a time, so it cannot see the runner among other items on one line of a `with pkgs;`
list that spans several lines. The chapter that states the general rule is
`tools/nix/02-per-project-devshell.md`, "Git hooks".

#### Scenario: A devshell lists the hook runner

- GIVEN a `flake.nix` whose devshell lists the runner beside the task runner
- WHEN the commit stage runs
- THEN the hook refuses the line, because the host supplies the runner

Verify: `prek run no-hook-runner-in-flake --all-files`

### `project-platform:the-hooks-are-the-only-gate` — The git hooks are the only gate

The project MUST run every check in its commit stage or its push stage, each check in exactly one of
them, except `dist build`, which only CI runs. A fast check on the staged files runs at commit, and
a costly check, such as a flake evaluation or the link check, runs at push. The push stage MUST NOT
rerun a commit-stage hook. A branch reaches the trunk through `rk integrate` and a push from a
checkout that ran both stages, so a change that passed its hooks is the change that lands. CI MUST
hold only three kinds of workflow:

- The validation workflow, `ci.yml`. Every `run:` step of its job `test` is one `prek run` call of
  one hook stage, alone on its line and bare or through `nix develop --command`, so a push that
  passed its hooks passes `test`. A `uses:` step pinned by commit SHA installs prek. A hook skip
  list goes in the step's `env:`.
- Beside `test`, `ci.yml` MAY hold the `dist-plan` and `dist-build` jobs that `rk binding rust`
  serves, as `test` needs, and no other job. The `dist-plan` hook runs the same plan at push.
  `dist build` is the one check that only CI runs. It builds the release archives that users
  download, and rk judges from the workflow alone that every release target builds before a
  merge. The operator keeps this costly build out of the local hooks.
- The release workflows, `release-plz.yml` and `release.yml`, which create tags, releases, and
  artifacts and check nothing the hooks do not check.
- The pull-request checks, `pr-commits.yml`, which judge what only a pull request has. No push
  starts them.

The chapter that states the general rule is `programming/project-bootstrap/04-quality-gates.md`,
"Git hooks".

#### Scenario: A check is added outside the hooks

- GIVEN a new check that a CI step runs and no hook stage runs
- WHEN a contributor pushes a change that the check rejects
- THEN the push passed and CI failed, so the check moves into the commit or the push stage

Verify: `for f in .github/workflows/*; do [ -e "$f" ] || continue; case "${f##*/}" in ci.yml) awk '/^  test:/{t=1;next} /^  [^ #]/{t=0} t' "$f" | grep -E '^[[:space:]]*(- )?run:' | grep -Evq '^[[:space:]]*(- )?run:[[:space:]]+(nix develop --command )?prek run( [-A-Za-z0-9=_.,]+)*[[:space:]]*$' && exit 1; awk '/^jobs:/{j=1;next} /^[^ #]/{j=0} j && /^  [A-Za-z0-9_-]+:/' "$f" | grep -Evq '^  (test|dist-plan|dist-build):' && exit 1 ;; release-plz.yml|release.yml|pr-commits.yml) ;; *) exit 1 ;; esac; done; just lint`

### `project-platform:hooks-exclude-generated-files` — The hooks leave generated files to their generator

The project MUST exclude every file a tool generates from every hook that rewrites or judges it, in
the tool's own configuration where the tool has one, and MUST keep it under the hooks that only read
it. The generated files are `flake.lock` (nix), `.release-kit/manifest.json`,
`.github/workflows/release-plz.yml`, and `.github/workflows/pr-commits.yml` (rk),
`.spec-driven-docs/manifest.json` and `_docs/.markdownlint-cli2.jsonc` (sdd), `Cargo.lock`
(cargo), `CHANGELOG.md` (release-plz), and `.github/workflows/release.yml` (cargo-dist). The chapter that states the general rule is
`programming/project-bootstrap/04-quality-gates.md`, "Generated files".

#### Scenario: sdd rewrites its manifest

- GIVEN a version upgrade that rewrites `.spec-driven-docs/manifest.json` in sdd's own layout
- WHEN the commit stage runs
- THEN no formatter, fixer, spell check, or editorconfig check changes or rejects the file, and the
  JSON parser and the secret scan still read it

Verify: `nix develop --command bash -c 'p="$(dprint output-file-paths)" || exit 1; printf "%s\n" "$p" | grep -qE "/(flake\.lock|\.release-kit/manifest\.json|\.spec-driven-docs/manifest\.json|_docs/\.markdownlint-cli2\.jsonc|Cargo\.lock|CHANGELOG\.md)$" && exit 1; for f in flake.lock .release-kit/manifest.json .spec-driven-docs/manifest.json _docs/.markdownlint-cli2.jsonc Cargo.lock CHANGELOG.md .github/workflows/release-plz.yml .github/workflows/pr-commits.yml .github/workflows/release.yml; do grep -qF "\"$f\"" _typos.toml && grep -qF "\"^${f//./\\\\.}\$\"" .editorconfig-checker.json || exit 1; done'`

### `project-platform:a-flake-is-read-from-its-git-tree` — A flake reference reads the git tree

When a command, a script, or a hook names a flake in a project working tree, it MUST read the git
tree: `git+file://` with the root, with `?dir=` for a flake in a subfolder and `?shallow=1` where CI
clones shallowly, or `.` from the CLI inside the repository. It MUST NOT name the tree as a `path:`
reference or pass a bare path to `builtins.getFlake`, because either one copies every file of the
folder into the store, ignored files included. A `path:` reference stays allowed for a `/nix/store`
path, for a relative input inside a generated flake, and for a throwaway fixture folder, and such a
line carries `nix-path-ok: <reason>`. The hook `no-path-flake-reference`, which this repository
publishes in `.pre-commit-hooks.yaml`, enforces the rule line by line. It cannot follow a reference
that a program assembles from parts. The chapter that states the general rule is
`languages/nix/05-tooling-gotchas-resources.md`, gotcha 1.

#### Scenario: A hook evaluates its own flake

- GIVEN a worktree with a built `target/` folder that git ignores
- WHEN a hook evaluates the flake of the worktree through `builtins.getFlake` on its bare path
- THEN the hook refuses the line, because each run would copy `target/` into the store again

Verify: `prek run no-path-flake-reference --all-files`
