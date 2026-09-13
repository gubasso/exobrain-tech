# The canon binary serves its own gates

## Context and Problem Statement

ADR-source-the-documentation-canon-upstream vendored the canon's gate scripts and its verifier into
`.spec-driven-docs/`, and authored the marked pre-commit block by hand, so that routine work needed
nothing outside this checkout. The canon stopped shipping those scripts. It serves every delivered
gate from its own binary, renders the marked block from a declaration this repository owns, and
reads an instance record this repository's vendored verifier cannot parse. Staying on the vendored
projection means staying on a canon version that is three minor releases behind and cannot move.

## Considered Options

- `take the binary as a lock-pinned tool input` — chosen.
- `keep the vendored scripts and freeze the canon version` — rejected: a projection that cannot
  upgrade is a fork, and the rules it holds stop agreeing with the ones the canon states.
- `re-author every delivered gate here` — rejected: this repository would own a program it does not
  want, to enforce rules it did not write.

## Decision Outcome

Chosen option: `take the binary as a lock-pinned tool input`.

`flake.nix` names the canon as an input and `flake.lock` pins the revision, which
ADR-a-flake-pinned-tool-input-is-a-tool-dependency already reads as a tool dependency rather than an
external source. The boundary that matters is unchanged: no knowledge here is external, and every
check still runs offline from the store. What moves is the form of the tool. `.spec-driven-docs/`
now holds configuration, the debt record, and lint settings, never scripts. The marked pre-commit
block is rendered from `.spec-driven-docs/config.yaml` by the same tool, so the block stops being
hand-authored and the declaration becomes the thing this repository writes. `.hooks/` keeps only the
gates this repository adds, and `just test-gates` proves only those.

## Consequences

- Good: an upgrade is one pin move and one command, so the canon version can keep moving.
- Good: this repository's own harness shrinks to the gates it actually authors.
- Bad: `pre-commit` outside the dev shell now fails, because the binary resolves off the path.

## Status

Accepted
