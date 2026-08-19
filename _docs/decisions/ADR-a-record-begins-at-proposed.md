# A record begins at Proposed, and exploration stays in .draft/

## Context and Problem Statement

The status vocabulary carried seven values and now carries six: `Ideation` left with the file it
lived in rather than as the subject of a decision, so the narrowing of every future record's
vocabulary happened with its reasoning written nowhere. The argument for a low state is real. A
lifecycle whose lowest value already means "ready for review" has no cheap entry, and this log shows
the symptom — thirty-one records, none of them `Proposed`, `Rejected`, or `Deprecated`.

## Considered Options

- Keep the six values and name `.draft/` as the state below `Proposed` — chosen.
- Restore `Ideation` — rejected: it needs a deletable record, and the permanence rule is what makes
  the log worth keeping.
- Add a mutable pre-record artifact under `_docs/` — rejected: it is `.draft/` with a second name and
  a maintenance cost.

## Decision Outcome

Chosen option: keep the six values. A record begins at `Proposed`. Exploration that is not yet a
proposal stays in `.draft/`, and promotion is a rewrite into the log rather than a move. The log
therefore holds proposals and outcomes, never disposable notes, and every record in it is permanent
from the moment it merges.

Artifact class decides this, not taste. MADR and Nygard both stop at `proposed`; Kubernetes is the
one scheme with a lower state, and it works because a KEP is edited after merge. A record here is
frozen at merge, so importing a low state means importing a deletable record — the combination no
surveyed scheme runs, and a hole in the permanence rule.

The empty `Rejected` count is a separate problem and this record does not fix it. A rejection with no
positive counterpart already earns a record; the review checklist asks it.

## Consequences

- Good: the log stays permanent with no exception argued per status.
- Good: half-shaped arguments have a stated home instead of no home.
- Bad: exploration is invisible to a fresh session until it is promoted.

## Status

Accepted
