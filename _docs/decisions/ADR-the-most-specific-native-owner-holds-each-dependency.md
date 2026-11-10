# The most specific native owner holds each dependency

## Context and Problem Statement

The canon named three owners for the tools of a project: the devshell, pre-commit, and the host. It
left out the manifest of the language stack, so nothing stopped a flake from holding a Python
library, and hook tools stayed in the devshell as `language: system` after the reason for it was
gone. The Python pages also disagreed on the default manager: one chose uv, four chose Poetry.

## Considered Options

- `the most specific native owner` — chosen.
- `three owners: the devshell, pre-commit, and the host` — rejected: it gives a language library no
  owner of its own, so the flake takes it.
- `everything from the devshell` — rejected: every hook loses its pinned `rev`, and a commit outside
  the devshell fails.

## Decision Outcome

Chosen option: `the most specific native owner` — a dependency lives in the one file that declares
its kind natively, so the manifest of a project travels with its code. The operator decided it on
2026-11-09.

- The owners run from the manifest of the stack, through `.pre-commit-config.yaml` and the
  devshell, to the host.
- A hook that keeps `language: system`, `script`, or `unsupported` carries an `# owner:` comment
  with one of five classes: `no-hook-language`, `project-environment`, `recorded-failure`,
  `outside-caller`, or `base-tool`.
- Python uses uv for scripts and projects. Poetry stays as a note for a project that already uses
  it.
- The host supplies the standard ELF loader, so a hook wheel with a prebuilt binary runs.

Enforced by `project-platform:the-most-specific-native-owner-holds-each-dependency`.

## Consequences

- Good: a dependency has one place, and each hook tool has one pinned version for the hook and CI.
- Bad: a managed hook downloads its environment once on each machine.
- Bad: a NixOS host needs nix-ld before a hook with a dynamically linked binary runs there.

## Status

Accepted
