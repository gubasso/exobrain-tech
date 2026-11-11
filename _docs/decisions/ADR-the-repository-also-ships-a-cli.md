# The repository also ships a CLI

## Context and Problem Statement

The library states rules that other projects follow, such as reading a flake from its git tree. A
lexical hook enforces such a rule line by line, but it cannot see what a program assembles at run
time, so the rule also needs a runtime guard. The repository ships only documents and the scripts a
reader copies, so the guard had no home where it shares a release with the rule it enforces.

## Considered Options

- `a Rust workspace under crates/` — chosen.
- `a separate repository for the tools` — rejected: it splits the rule from its check, so a change
  to one is reviewed and released without the other.
- `a root crate beside the buckets` — rejected: it puts `src/` and `Cargo.toml` sources beside the
  eight buckets, where a reader looks for knowledge.

## Decision Outcome

Chosen option: `a Rust workspace under crates/` — the library stays the knowledge product, and
`crates/` holds the programs that serve the projects which follow the canon, starting with the
`exobrain` CLI.

- The placement test gains a third answer: a program the repository ships lives in `crates/`.
- The software is MIT under `LICENSE-MIT`, and the knowledge stays CC BY 4.0 under `LICENSE`.
- A program in `crates/` follows the repository's Rust gates and its own contract in
  `_docs/specs/SPEC-cli.md`.
- This changes the placement test of `ADR-docs-vs-library-boundary` and the shipped-artifact scope of
  `ADR-executable-artifacts-in-the-library`. Both records stay as they are, and the specs and
  `AGENTS.md` state what binds now.

Enforced by `project-platform:the-hooks-are-the-only-gate`.

## Consequences

- Good: a rule, its hook, and its guard change in one commit and share one release.
- Good: a consumer pins one tag for the hook and the binary.
- Bad: the repository carries a Rust toolchain, a lock file, and Rust gates beside the docs gates.
- Bad: two licenses, so each file falls on one side of the split.

## Status

Accepted
