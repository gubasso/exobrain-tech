# Planning lives outside the repository

## Context and Problem Statement

ADR-the-repository-names-no-planning-method removed the method but kept the record, so the docs root
held a planning directory whose shape no linter checked. That record duplicated the one kept outside
the checkout: one unit of work existed in both places, and each copy cited the other. A second store
of the same fact drifts on the next change to either side, which this repository forbids everywhere
else.

## Considered Options

- Hold no planning record here at all — chosen.
- Keep the in-tree record and stop duplicating it outside — rejected: the work spans several
  repositories, and a record inside one of them cannot order work in the others.
- Keep both and name which one wins — rejected: it states the drift as a rule instead of removing
  it, and a reader still has two files to read.

## Decision Outcome

Chosen option: hold no planning record here. This supersedes
ADR-the-repository-names-no-planning-method, whose outcome allowed the docs root to hold a planning
directory.

The docs root holds the specs, the decision log, and the three Diátaxis zones, and nothing else. What
this repository is building next is tracked outside the checkout, and no file here names where.

Enforced by `knowledge-base-boundary:no-method-is-named`.

## Consequences

- Good: one unit of work has one record, and no file here goes stale when that record moves.
- Good: the last gate reading a planning path goes, and with it the heading contract it enforced.
- Bad: a reader of this checkout alone cannot see what is planned, and must be told where to look.
- Bad: a change that enacts planned work carries no in-repo entry document, so the rules citing one
  are held by review against a document outside the tree.

## Status

Accepted
