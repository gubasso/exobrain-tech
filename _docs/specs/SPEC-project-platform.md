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
  - [`project-platform:the-devshell-supplies-every-project-tool` — The devshell supplies every project tool](#project-platformthe-devshell-supplies-every-project-tool--the-devshell-supplies-every-project-tool)
    - [Scenario: A hook builds its environment on a machine with no curl](#scenario-a-hook-builds-its-environment-on-a-machine-with-no-curl)
  - [`project-platform:the-hooks-are-the-only-gate` — The git hooks are the only gate](#project-platformthe-hooks-are-the-only-gate--the-git-hooks-are-the-only-gate)
    - [Scenario: A check is added outside the hooks](#scenario-a-check-is-added-outside-the-hooks)
  - [`project-platform:hooks-exclude-generated-files` — The hooks leave generated files to their generator](#project-platformhooks-exclude-generated-files--the-hooks-leave-generated-files-to-their-generator)
    - [Scenario: sdd rewrites its manifest](#scenario-sdd-rewrites-its-manifest)

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

### `project-platform:the-devshell-supplies-every-project-tool` — The devshell supplies every project tool

The project MUST supply from `flake.nix` every tool that a recipe, a `language: system` hook, or a
`language: script` hook calls, and every tool that pre-commit's installers need to build a hook
environment from a clean machine. The host supplies only `git`, `bash`, `awk`, `sed`, `grep`, the
coreutils, `nix`, and `direnv`. The chapter that states the general rule is
`programming/project-bootstrap/03-local-dev-environment.md`, "Who supplies each tool".

#### Scenario: A hook builds its environment on a machine with no curl

- GIVEN an empty pre-commit cache and a `PATH` that holds only the host tools
- WHEN pre-commit builds the rust environment of the secret scanner
- THEN `rustup-init` finds `curl` in the devshell and the build succeeds

Verify: `env -i HOME="$HOME" NIX_SSL_CERT_FILE="$NIX_SSL_CERT_FILE" PATH="$(dirname "$(command -v nix)"):/usr/bin" PRE_COMMIT_HOME="$(mktemp -d)" nix develop --command pre-commit install-hooks`

### `project-platform:the-hooks-are-the-only-gate` — The git hooks are the only gate

The project MUST run every check in its commit stage or its push stage, each check in exactly one of
them, and MUST run no CI. A fast check on the staged files runs at commit, and a costly check, such
as a flake evaluation or the link check, runs at push. The push stage MUST NOT rerun a commit-stage
hook. A branch reaches the trunk through `rk integrate` and a push from a checkout that ran both
stages, so a change that passed its hooks is the change that lands. The chapter that states the
general rule is `programming/project-bootstrap/04-quality-gates.md`, "Git hooks".

#### Scenario: A check is added outside the hooks

- GIVEN a new check that a recipe runs and no hook stage runs
- WHEN a contributor pushes a change that the check rejects
- THEN nothing stopped the push, so the check moves into the commit or the push stage

Verify: `[ ! -e .github/workflows ] && just lint`

### `project-platform:hooks-exclude-generated-files` — The hooks leave generated files to their generator

The project MUST exclude every file a tool generates from every hook that rewrites or judges it, in
the tool's own configuration where the tool has one, and MUST keep it under the hooks that only read
it. The generated files are `flake.lock` (nix), `.spec-driven-docs/manifest.json`, and
`_docs/.markdownlint-cli2.jsonc` (sdd). The chapter that states the general rule is
`programming/project-bootstrap/04-quality-gates.md`, "Generated files".

#### Scenario: sdd rewrites its manifest

- GIVEN a version upgrade that rewrites `.spec-driven-docs/manifest.json` in sdd's own layout
- WHEN the commit stage runs
- THEN no formatter, fixer, spell check, or editorconfig check changes or rejects the file, and the
  JSON parser and the secret scan still read it

Verify: `nix develop --command bash -c '! { dprint output-file-paths; typos --files; editorconfig-checker -dry-run; } | grep -E "(^|/)(flake\.lock|\.spec-driven-docs/manifest\.json|_docs/\.markdownlint-cli2\.jsonc)$"'`
