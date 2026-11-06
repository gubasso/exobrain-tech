# Operate the documentation canon from a local projection

## Context and Problem Statement

`ADR-source-the-documentation-canon-upstream.md` chose a pinned local projection of the public
canon, and named two rules that enforce it. One of them, `distribution:verification-operates-offline`,
is a rule of the canon's own release spec, which no adopter receives, so no spec here defines it. A
decision record is not revised, so the citation cannot be corrected in place.

## Considered Options

- `a successor record that keeps the choice and cites only rules this repository states` — chosen.
- `a local rule restating the canon's offline verification` — rejected: the property belongs to the
  `sdd` binary, which this repository pins and does not build, so a local rule would bind nothing
  an author here does.

## Decision Outcome

Chosen option: `a successor record that keeps the choice and cites only rules this repository
states`. The choice stands: the public canon owns the method, and this checkout owns the projection
it operates, held locally and verified by `sdd verify` without network or canon access. That the
pinned `sdd` verifies offline is a property of the tool, recorded by its pin in `flake.nix`, not a
rule this repository states.

Enforced by `knowledge-base-boundary:no-knowledge-is-external`.

## Consequences

- Good: the record that binds now cites only live rules, so the citation gate holds it.
- Bad: offline verification has no rule here, and a reviewer of an upgrade confirms it.

## Status

Implemented by the pinned `sdd` in `flake.nix` and the managed projection under `.spec-driven-docs/`.
