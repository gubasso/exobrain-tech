# A comment cites the rule, not the record

## Context and Problem Statement

Common practice puts a decision-record number in a code comment near the code that record argued
for. That practice assumes the record is the only citable durable artifact, which stopped being true
when `programming/spec-driven-docs/` gave every rule an ID. A record is frozen, may be superseded,
and holds the argument rather than the obligation, so an agent that follows one from code loads what
was decided once instead of what binds now — and loads it into the context the shelf exists to keep
clean.

## Considered Options

- Keep the common practice: a comment names the decision record.
- A comment cites the rule by ID, and never names a record.
- Leave comments ungoverned.

## Decision Outcome

Chosen option: a comment cites the rule by ID. The grammar is the entry document's, extended to
code — the relation in capitals, then the rule ID — with `SATISFIES` for the code that implements a
rule and `VERIFIES` for the test that proves it. Requirements tracers converge on this shape:
OpenFastTrace writes coverage tags into comments against specification items, StrictDoc's
`@relation` markers carry implementation and verification roles, and DO-178C requires every source
code element to trace back to a requirement rather than to a rationale.

`programming/spec-driven-docs/09-spec-to-code.md` owns the rule, the two other things a comment may
hold, and the greps. Two hooks in the shelf's worked example enforce it.

## Consequences

- Good: a citation in code is checkable. The orphan grep fails a tag naming a rule no spec defines,
  which is what makes citing worth anything.
- Good: code becomes the fourth record set the coverage greps read, so traceability stays derived.
- Bad: the rule diverges from widely repeated ADR guidance, so an author arriving from that practice
  will write the wrong thing once.
- Bad: it presumes a specs zone. A project with no specs has no ID to cite, and gets only the
  prohibition.

## Status

Accepted.
