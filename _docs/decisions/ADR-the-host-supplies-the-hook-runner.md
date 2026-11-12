# The host supplies the git hook runner

## Context and Problem Statement

The canon put the git hook runner in the devshell of each project. Git runs an installed hook from every shell, so the nixpkgs pre-commit baked its store path into each hook, and a garbage collection broke every hook. `mkShell` also exported the Python of that runner on `PYTHONPATH`, which shadowed the packages of a uv project, so the canon taught an `env -u PYTHONPATH` prefix. The leak also hid a nixpkgs defect: its `virtualenv` omits `packaging`.

## Considered Options

- `prek on the host` — chosen.
- `the runner in the devshell` — rejected: it keeps the baked store path, the leak, and the prefix.
- `the runner in the manifest of each project` — rejected: a project without a Python manifest has no place for it.
- `git-hooks.nix` — rejected: the hooks lose the `rev` pins that CI and the hooks share.
- `pre-commit as a uv tool on the host` — rejected: the host still carries a Python runner.

## Decision Outcome

Chosen option: `prek on the host` — prek is a Rust runner that reads the same `.pre-commit-config.yaml` and needs no Python, so it sits beside git with nothing to leak. The operator decided it on 2026-11-11.

- The host supplies `prek`, and CI installs a pinned prek. A project calls `prek` by name, with no probe or fallback, and lists it in its README.
- `.pre-commit-config.yaml` pins every hook by `rev` and names the oldest runner it accepts with `minimum_prek_version`.
- No devshell holds a runner. A line that installs it on a host carries `hook-runner-ok: <reason>`.
- prek builds a Python hook with the uv of the host.

Enforced by `project-platform:the-host-supplies-the-hook-runner`.

## Consequences

- Good: an installed hook survives a garbage collection, because the prek shim falls back to `prek` on `PATH`.
- Good: `uv run` in a devshell needs no prefix.
- Bad: every host and every CI job installs prek before a hook runs.
- Bad: every caller of `pre-commit` changes, because prek ships no `pre-commit` command.

## Status

Accepted
