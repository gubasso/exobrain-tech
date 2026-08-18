# Rename the existing numbered decision log to slug identifiers

## Context and Problem Statement

`ADR-spec-driven-docs-is-the-documentation-method` adopted a framework whose records are named
`ADR-<slug>.md` and must carry no digit, while this repository's own log was twenty-five records
named `ADR-0001-` through `ADR-0025-`. The adopted rule also says a merged record's filename never
changes, so the log could not be brought into compliance without breaking the rule that governs it.
Three ADR namespaces had meanwhile drifted into existence, each with its own `0001`.

## Considered Options

- Rename the whole log once, under a recorded exception — chosen.
- Freeze the numbered log and slug only new records — rejected: a permanently mixed directory
  defeats the scan the naming rule exists to serve, and the colliding namespaces stay colliding.
- Scope immutability to the slug rather than the filename — rejected: it reads as a rule that was
  weakened to fit, and the property worth protecting is the whole citation.

## Decision Outcome

Chosen option: rename once. Immutability binds from this record forward; the twenty-five renames are
the migration that makes it enforceable, and every citation in the tree moves with them. The counter
disappears from the record body too, because the slug in the filename is now the identifier.

Enforced by `docs-foundations:a-kind-prefix-carries-a-slug`.

## Consequences

- Good: one namespace, one identifier per record, and a gate that can finally be pointed at the
  directory rather than at the prefix.
- Bad: every external link to a numbered filename breaks, and the rename is a precedent that has to
  be refused next time.

## Status

Implemented
