# 004 — Decide Whether Prose Stays Unwrapped

## Goal

One line-break convention holds across the documentation root, and the gate that judges it reads
real paths.

## Example

The delivered gate is wired and declared to judge nothing, so the rule its message cites binds no
file.

```console
$ sdd gate --explain _docs/README.md
skipped      prose-stays-unwrapped  exclude **  (project)  types: [markdown]
```

## Core

The canon holds a paragraph to one line. This project wraps at 100 columns, in 69 documents under
the documentation root, 629 lines in all. Forty of those documents are decision records, whose lines
a record cannot have rewritten. The declaration parks the gate; it does not answer the question.

## In scope

- The choice: convert the documentation root, convert every zone a record does not own, or state
  the wrap as this project's own rule and keep the gate parked forever.
- Whatever the choice costs: the conversion itself, a record permitting a whitespace rewrite of an
  immutable record, or a local requirement that states the opposite of the canon's.
- The declaration entry, removed or narrowed to what the choice leaves.
- The same question for the eight library buckets, which the gate does not reach today.

## Out of scope

- `dprint.json`, whose `textWrap: maintain` accepts either convention already.
- Any other delivered gate's declaration.

## Governed by

- `_docs/specs/SPEC-docs-format.md` — the rule as the canon states it.
- `_docs/decisions/ADR-a-record-is-not-annotated-as-it-ages.md` — why a record's bytes are not
  rewritten to suit a later convention.
- `_docs/decisions/ADR-the-canon-binary-serves-its-own-gates.md` — why the gate arrived at all.

## Amends

- `.spec-driven-docs/config.yaml` — the entry that parks the gate.
- The documents the choice converts.

## Acceptance

- `sdd gate --explain` on a documentation-root file names a real decision, never `exclude **`.
- `just lint` passes with whatever the choice leaves declared.

## Tasks

- [ ] Take the choice, with the counts in front of the reader.
- [ ] Convert what it converts, in one change per zone.
- [ ] Narrow or remove the declaration entry.

## Rabbit holes

- Converting by hand, file by file — escape: a paragraph join is scriptable, and the gate names
  every line it would fail.
- Reopening the 100-column habit for the library buckets in the same change — escape: the buckets
  are a separate decision, taken after the documentation root proves the conversion is cheap.

## Done when

Every markdown file under the documentation root is judged by the gate, or the project states its
own rule and says why.

## Revisions

None.
