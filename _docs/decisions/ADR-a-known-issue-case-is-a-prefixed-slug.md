# Name a known-issue case KI-<slug> and gate it

## Context and Problem Statement

The framework required a suppression to name its case id but never said what a case id was. The one
example used `KI-0007`, a counter inherited from a retired shelf, and the record it pointed at was
specified as an unprefixed `<slug>.md`. So the citation token, the filename, and the example
disagreed, and nothing could check any of them.

## Considered Options

- Make the record `KI-<slug>.md`, and the filename the case id — chosen.
- Keep an unprefixed filename and carry the id in frontmatter — rejected: a citation in source then
  resolves only by reading every record, which is what the prefix exists to avoid.
- Drop case ids and cite the upstream issue URL — rejected: a URL rots, cannot be grepped as one
  token, and does not exist yet while a bug is still being characterized.

## Decision Outcome

Chosen option: `KI-<slug>.md` under `<root>/reference/known-issues/`, with the filename as the id.
This makes it the fourth kind prefix, which the placement rule already anticipated by generating the
table from "prefix a file when its shape is fixed and gated" — so the prefix arrives with two hooks
rather than as an assertion.

Enforced by `docs-foundations:artifact-filenames-carry-a-kind-prefix` and
`spec-to-code:a-suppression-names-its-case`.

## Consequences

- Good: a suppression's reason string resolves to exactly one file, and a mask that outlives its bug
  is now findable rather than merely regrettable.
- Bad: a fourth prefix is one more thing to teach, and the resolution check is one-way, so a record
  no suppression cites is never reported.

## Status

Implemented
