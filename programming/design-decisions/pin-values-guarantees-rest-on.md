# ADR-0004: Pin the values a guarantee rests on, even when they match the upstream default

## Context and Problem Statement

A project's guarantees are usually enacted by flags or settings handed to an upstream tool. Where the
value a guarantee needs is already that tool's default, the tempting economy is to inherit it
silently. But an inherited default is a value the upstream owns: a later release can change it, and
the change lands with no diff in your repository, no review, and no failure naming the cause — the
guarantee is withdrawn by someone else's changelog. The trap has a second face: a wrapper, framework,
or distro layer between you and the tool often substitutes its own default, so "the default" is
ambiguous before it is stale.

## Considered Options

- Inherit the upstream default; state only the values that differ from it.
- Pin every value a guarantee depends on, whether or not it currently differs.
- Pin nothing and assert the resulting behavior in a test instead.

## Decision Outcome

Chosen option: **pin every value a guarantee depends on.** The selection test is not "does this
differ from the default?" but "would a change to it break something I promised?" — and that answer
does not change when the two values coincide today.

- Pin the tool's **version** alongside the value: a pinned setting inside an unpinned tool is half an
  answer, since the flag can be renamed or redefined.
- Record **why** each value is stated. A setting matching the default reads as redundant, and a
  future reader deletes it unless the reason sits next to it.
- Do **not** pin values no guarantee rests on: they are churn, and they bury the load-bearing ones.

## Consequences

- Good: an upstream default change becomes a no-op instead of a silent regression.
- Good: the load-bearing surface is enumerated in one place, so review and audit read it directly.
- Bad: verbose, and reviewers will ask why an apparent no-op is written down.
- Bad: pinning can freeze a worse value after upstream improves its default — revisit the pinned set
  on version bumps.

## Status

Accepted. Same shape as [ADR-0001](./version-source-of-truth.md): one place authors the value and
nothing load-bearing is left implicit.
