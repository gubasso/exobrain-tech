# Linux is the only supported project target

## Context and Problem Statement

The flake mapped its development shell over `flake-utils.lib.eachDefaultSystem`, so it advertised a
shell for Darwin and ARM as well as x86_64 Linux. One required check runs, on x86_64 Linux, and no
Darwin or ARM runner has ever exercised the toolchain the flake offers them. The pinned canon binary
is fetched per system from an upstream that builds one target. An advertised system with nothing
behind it is a promise this project does not keep.

## Considered Options

- `declare x86_64-linux and nothing else` — chosen.
- `keep the default system set and add runners` — rejected: three more required checks and three
  more toolchain surfaces, for a repository of markdown that one person builds on one machine.
- `keep the default set and document that only Linux is tested` — rejected: the flake is the claim,
  and a note beside it does not change what `nix develop` offers a reader on another host.

## Decision Outcome

Chosen option: `declare x86_64-linux and nothing else`.

`flake.nix` binds one system in a `let` and keys its outputs by that binding. The `flake-utils`
input and its lock node go, because nothing maps any more.
[SPEC-project-platform](../specs/SPEC-project-platform.md) binds the claim, and
`.hooks/check-one-system.sh` holds it by evaluating the flake's own outputs rather than reading
`flake.nix` as text. A text scan cannot make this claim: Nix spells one attribute tree several ways,
and a nested set declares an output with no dotted family before its key.

The cut governs this checkout's toolchain alone. A chapter may still describe Linux, macOS, or
Windows when that is its subject, and a portable rule states a property rather than an operating
system.

## Consequences

- Good: what the flake offers is what a required check proves.
- Good: one fewer input, and a shell that resolves the canon binary for the one system it exists for.
- Bad: a reader on another host gets no shell, and adding one back means adding its runner first.

## Status

Accepted
