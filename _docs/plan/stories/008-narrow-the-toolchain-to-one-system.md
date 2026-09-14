# 008 — Narrow the Toolchain to One System

## Goal

What the flake offers a reader is what a required check proves, and the narrowing binds this
checkout's toolchain without narrowing what a chapter may be about.

## Example

The flake advertises a development shell for systems nothing exercises.

```console
$ nix flake show --all-systems | grep -c '└───'
4
```

One required check runs, on x86_64 Linux. No runner has ever built the shell the flake offers a
Darwin or ARM host, and the pinned canon binary is fetched per system from an upstream that builds
one target.

## Core

One system is bound in a `let` and every system-dependent output is keyed by that binding. A check
evaluates the flake's own outputs and refuses a second key, because a text scan over `flake.nix`
cannot make the claim: Nix spells one attribute tree several ways.

## In scope

- The flake's output shape, the unused `flake-utils` input, and its lock node.
- A spec that binds the support claim, and a decision record that holds the reasoning.
- The evaluated check, its recipe, and the tool it needs in the development shell.

## Out of scope

- The library's subject matter. A chapter may still describe any operating system.
- Adding a runner for a second system, which is what adding a second system would require first.

## Governed by

- `_docs/specs/SPEC-project-platform.md` — the rules this story lands.
- `AGENTS.md` — the rule that this checkout reads only local files and its pinned tools.

## Amends

- `flake.nix` and `flake.lock` — one system, one input fewer.
- `.hooks/check-one-system.sh` and `.hooks/one-system.nix` — the evaluated check.
- `justfile` — the recipe, wired into `just test`.
- `_docs/specs/SPEC-project-platform.md` and
  `_docs/decisions/ADR-linux-is-the-only-supported-project-target.md` — created.

## Acceptance

- `nix flake show` exposes `x86_64-linux` and nothing else.
- The check fails against a second system declared inside a nested attribute set, which is the case
  a text scan misses.
- `just check` passes.

## Tasks

- [ ] Bind one system in the flake and drop the input that mapped over a set.
- [ ] Write the evaluated check and prove it fails.
- [ ] Bind the claim in a spec, with the decision recorded beside it.

## Rabbit holes

- Turning the support cut into a portable rule for the library — escape: the spec states the
  boundary, and a portable chapter names a property rather than an operating system.
- Holding the claim with a text scan over `flake.nix` — escape: a nested attribute set declares an
  output with no dotted family before its key, so the check evaluates instead.

## Done when

The flake declares one system, a check proves it, and a spec says why the claim stops there.

## Revisions

None.
