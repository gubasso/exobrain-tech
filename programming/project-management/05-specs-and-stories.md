# 05 — Specs and Stories

Where does durable behavior live once a story closes? A story records a change; a capability spec
records the current state.

## Diff and state

A story is future-facing and disposable as live guidance: it says what will be different and keeps
the agreement made for that change. A spec is present tense and durable: it says what the system
does now.

Specs are reference pages at `<project>/docs/reference/<capability>.md`. They are organized by
observable capability, never by story id. Not every story creates or changes one: a chore never
does, and internal work with unchanged behavior has nothing to specify.

## Acceptance becomes assertion

The story's `Acceptance` section specifies behavior before implementation and names the tests that
prove it. In the same commit as the behavior, those assertions are rewritten in present tense into
the capability spec. The story keeps its original acceptance as history; later stories may change
the live spec without rewriting earlier stories.

`Amends` declares the outbound obligation:

```markdown
## Amends

- `docs/reference/rate-limiting.md` — the threshold and window
- `docs/reference/http-errors.md` — new: the response body
```

Write `None` when the story changes no spec. `check-plan` verifies that every named path exists at
commit time; review verifies that the acceptance assertion actually reached it.

## Ownership boundary

The story carries what its work session must act on: goal, scope, sources, amended specs,
acceptance, tasks, and revisions. The lane entry carries what only the board needs: type, points,
dependencies, epic membership, labels, outcome, and close date.

Specs carry no inverse list of stories. Repository history already connects a spec line to the
commit and story id without creating a second index that can drift.

A spike produces an answer rather than behavior. It exits through an ADR, a revision to the story
that prompted it, or a closed measurement; it does not amend a behavior spec directly.
