---
name: exobrain-hooks-setup
description: Sets up the prek hooks of a project from the reference hook sets of exobrain-tech, moves a project from pre-commit to prek, takes the exobrain CLI from the host, and enables the hook-revisions gate. Use when asked to set up or adopt the reference hook sets, to migrate a project from pre-commit to prek, to swap pre-commit-hooks for the prek built-ins, to enable hook-revisions, to take exobrain out of a devshell, or to provide the exobrain CLI in CI. Triggers include prek, pre-commit, .pre-commit-config.yaml, hook sets, hook-revisions, exobrain hooks sync, exobrain hooks status, built-in hooks, and host-requirements.toml.
license: MIT
compatibility: Needs git, prek 0.5.7 or newer, gh, and the exobrain CLI on the host. Reads the reference sets and the chapters of exobrain-tech at the tag that the project pins. Changes only the project that it runs in.
---

# exobrain-hooks-setup

Set up the git hooks of one project to the canon of exobrain-tech: prek is the only runner, the project copies the reference sets of its stacks, the hook `hook-revisions` holds each shared `rev` to the exobrain-tech tag of the project, and the host supplies the `exobrain` CLI. Where a file named here is not on disk, `exobrain skill show exobrain-hooks-setup --file <path>` prints it, for example `--file references/runner.md`.

## Sources

This skill names each source in exobrain-tech as `$EXOBRAIN_TECH/<path>`. Read a source at the tag that the project pins, never at another revision. Where `$EXOBRAIN_TECH` holds a checkout, run `git -C "$EXOBRAIN_TECH" show <tag>:<path>`. Otherwise run `gh api "repos/gubasso/exobrain-tech/contents/<path>?ref=<tag>" -H 'Accept: application/vnd.github.raw'`.

- `$EXOBRAIN_TECH/programming/project-bootstrap/03-local-dev-environment.md`: the owner of each dependency, the host requirements, and the `.envrc` warnings.
- `$EXOBRAIN_TECH/programming/project-bootstrap/04-quality-gates.md`: the two hook stages, the reference sets, the exception classes of a `language: system` hook, and the generated files.
- `$EXOBRAIN_TECH/programming/project-bootstrap/05-ci.md`: the CI job that repeats the hook stages, and the step that provides the `exobrain` CLI.
- The sets: `$EXOBRAIN_TECH/programming/project-bootstrap/hook-sets/common.yaml`, `$EXOBRAIN_TECH/programming/project-bootstrap/hook-sets/rust.yaml`, `$EXOBRAIN_TECH/programming/project-bootstrap/hook-sets/python.yaml`, `$EXOBRAIN_TECH/programming/project-bootstrap/hook-sets/bash.yaml`, and `$EXOBRAIN_TECH/programming/project-bootstrap/hook-sets/nix.yaml`.

## Pre-flight

1. Make sure that the target is the repository the request means: `git rev-parse --show-toplevel` and `git remote get-url origin`.
2. Read the `AGENTS.md` and the `README.md` of the project for how to run a command in its environment, such as `direnv exec . <cmd>` or `nix develop --command <cmd>`. Run every project command that way, `git commit` included.
3. Make sure that the host has the tools: `prek --version` prints 0.5.7 or newer, and `command -v gh exobrain` names both. `exobrain hooks --help` lists `check`, `sync`, and `status`. A missing or older tool stops the run. Ask the operator to install it, and never install a host tool yourself.
4. Pick the tag. It is the `rev` of the exobrain-tech entry in `.pre-commit-config.yaml` where the project has one. Otherwise it is the latest release: `gh release view --repo gubasso/exobrain-tech --json tagName -q .tagName`. A tag older than the first release that carries `hook-revisions` cannot hold the sets, so the run moves the project to the latest release.
5. Read the project. Note each of these that exists: `.pre-commit-config.yaml`, `flake.nix` and `nix/`, `.envrc`, `host-requirements.toml`, `justfile`, `Makefile`, `scripts/`, `.github/workflows/`, and `.gitlab-ci.yml`. Note the hooks folder, `git rev-parse --git-path hooks`, and `git config core.hooksPath`.
6. Pick the sets from the files that git tracks. `common` always. `rust` where a `Cargo.toml` exists. `python` where a `pyproject.toml` or a Python script exists. `bash` where a shell script exists. `nix` where a `flake.nix` or another `.nix` file exists. The request can add or drop a set, and the report names the reason.
7. Run both hook stages as the baseline, before any change: `prek run --all-files` and `prek run --all-files --hook-stage pre-push`. prek reads a pre-commit configuration too. Record each hook that fails, with its output.

Stop and ask the operator in each of these cases. State the evidence, and write nothing:

- `core.hooksPath` is set, so prek does not own the hooks folder.
- The hooks folder holds a hook that neither prek nor pre-commit installed. `references/runner.md` says how to tell them apart.
- A host tool is missing or older than the run needs.
- The baseline fails for a reason that this run does not touch, such as a failing test. Ask whether to go on over it.

## General rules

1. Route, never restate. The chapters own the reason for each rule, and the sets own each hook. Copy a hook from its set at the tag, never from memory.
2. Leave each rendered block as its tool wrote it. Never edit a line between `# BEGIN release-kit` and `# END release-kit`, or between `# BEGIN spec-driven-docs` and `# END spec-driven-docs`. `exobrain hooks sync` skips them too.
3. Give each tool one owner, as chapter 03 says. The devshell holds no hook runner and no `exobrain`. Never install a project tool on the host.
4. Keep each project choice that differs from a set, and record the reason in a comment directly above the hook. `hook-revisions` reads only the `rev`, so `args`, `stages`, `files`, and `exclude` stay the project's. A `rev` is never a project choice: `exobrain hooks sync` owns it.
5. Keep each hook of the project that no set holds. A `language: system` hook carries the `# owner:` comment of chapter 04.
6. Work on the branch that the workflow of the project names. Never pass `--no-verify`, and never set `SKIP`.
7. Commit with a type that cuts no release, because the hooks ship nothing: `build(hooks)` for the hook configuration, `build(nix)` for the flake, and `ci` for a workflow. A commit header names the change, such as `build(hooks): adopt the common and rust reference hook sets`.

## Tasks

Present the plan before the first write: the tag, the sets, the tasks below that apply, and the files that each task writes. Wait for a yes, unless the order that started the run already names them. Run the tasks in this order, and read each task file into this context before its task. Skip a task whose pre-flight finds the work done, and say so in the report.

1. `runner`: move the project from pre-commit to prek, and install the prek shims. Read `references/runner.md`.
2. `host-cli`: take the `exobrain` package out of the devshell, and provide the CLI in CI. Read `references/host-cli.md`.
3. `sets`: write or merge `.pre-commit-config.yaml` from the sets, and swap the hooks of `pre-commit/pre-commit-hooks` and `check-json5` for the prek built-ins. Read `references/sets.md`.
4. `gate`: enable `hook-revisions`, run `exobrain hooks sync`, list `exobrain` in `host-requirements.toml`, and add the warnings to `.envrc`. Read `references/gate.md`.

## Post-flight

Run these once, after the last task, in the environment of the project.

1. Run `prek validate-config`.
2. Run `prek run hook-revisions --all-files`. It passes.
3. Run `exobrain host-check --parse-only`, then `exobrain host-check`.
4. Run both hook stages: `prek run --all-files` and `prek run --all-files --hook-stage pre-push`. Each hook that passed in the baseline passes now. A new failure is a finding: fix the file in its own commit, or report it.
5. List the hooks folder. Each type of `default_install_hook_types` holds a prek shim, and no `.legacy` file is left.

Fill this template from observation:

```text
Result: <the last line of the pre-push run, or the first failing line>
Tag: <tag>; sets: <names>, because <the files that picked them>
Tasks:
  - <task>: <files written or removed>; proof: <post-flight evidence>
Project choices kept:
  - <hook>: <field and value>; reason: <the comment above the hook>
Findings:
  - <hook>: <file and failure>; <fixed in <commit> | reported>
Commits:
  - <hash> <header>
Still the operator's:
  - <exact command>: <what it changes>
Next: <one action>
```

Print "Project choices kept", "Findings", and "Still the operator's" only where the run has an entry for them.
