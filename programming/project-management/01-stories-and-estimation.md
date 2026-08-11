# 01 — Stories and Estimation

What is a unit of work, how large may it be, and what is the budget? The unit is a story, and its
points estimate the human judgment required to accept it.

## The work unit

A story is one vertical, demonstrable change. It crosses the necessary layers, names an observable
outcome, and is small enough to accept as one conversation. The lane entry gives it one type:

- `story` for customer-visible behavior;
- `spike` for time-boxed research whose result is an answer;
- `chore` for necessary work without a customer-visible outcome.

Stories decompose into tasks, but tasks do not become separate plan records. The story remains the
unit of intent, ranking, acceptance, and history.

## Points count judgment

A point is one unit of irreducible human judgment: a decision that a test cannot settle and a human
must accept by eye. The schema permits only `1`, `2`, or `3`.

```text
1  Confirmation only: named tests settle acceptance, with at most one unchanged contract.
2  One judgment: an interface, name, message, or existing contract needs human reasoning.
3  Two judgments, or one hard-to-reverse judgment such as a schema or security control.
4+ Not a value. Split the story along the judgments already named in Acceptance.
```

This scale estimates the scarce resource in an agent-implemented workflow: reviewer attention. It
does not predict elapsed time, agent effort, diff size, file count, or task count.

## Core and cuts

Each story names a non-negotiable `Core` and an ordered `In scope` remainder. When the work exceeds
what one story can carry, cut the least valuable remainder first; never cut correctness, tests,
review, or security. If the core alone requires more than three points, split the guarantee before
work begins. A split leaves each piece carrying its own goal and none carrying the goal they share;
[06 — Epics](./06-epics.md) is where that shared end state lives.

Changes to the story after work begins remain visible in `Revisions`. Cutting planned remainder is
the normal response and needs no revision entry; changing the goal, core, acceptance, or other
agreement does.

## Velocity is the iteration budget

Velocity is the sum of points from entries with `outcome: done` and `closed` inside an iteration.
It is derived and never stored. `cut` delivers no value to count, and `reshaped` is counted through
its successor.

Read velocity as the number of judgments one reviewer can accept per iteration. More agent
throughput raises the review queue, not this capacity. The charter states the cadence for a reader
and `config.yml` carries it in the form a script reads; changing it restarts the series.

Planning against velocity and calibrating the scale are separate practices and are not specified by
this shelf.
