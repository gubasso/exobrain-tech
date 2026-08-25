# The upstream canon is named once

## Context and Problem Statement

This repository is an instance of the documentation canon, not a sibling of it, so every mention of
the canon is an upstream dependency reference. Nineteen such references had accumulated: library
chapters citing canon passages as further reading, a shelf digest pointing at a sibling directory
that no longer exists, and a reference table repeating a pinned tag nine times. Each pin was a
second home for a version the projection manifest already records, and each lagged the manifest the
moment an upgrade landed.

## Considered Options

- `one unversioned pointer in the root digest` — chosen.
- `keep the pins and gate them against the manifest` — rejected: the version still has two homes,
  and every upgrade has to rewrite prose to keep them agreeing.
- `keep the citations, drop only the versions` — rejected: an unpinned citation from a library
  chapter still makes a bucket depend on how this checkout is governed.

## Decision Outcome

Chosen option: `one unversioned pointer in the root digest` — the root `AGENTS.md` names the project
and nothing else does, while `.spec-driven-docs/manifest.json` stays the single record of the
version implemented. Library content names the canon nowhere, because a bucket serves knowledge and
the method that governs this checkout is not that knowledge.

Enforced by `knowledge-base-boundary:the-canon-is-named-once`.

## Consequences

- Good: an upgrade rewrites the manifest and no prose, so the version cannot go stale.
- Good: a reader following a reference reaches the project, not a passage frozen at a past release.
- Bad: an argument a canon passage makes at length is no longer reachable from the page that
  restates it; the local spec is the only home for the rule.
- Bad: the decision log keeps citations to paths and tags that no longer resolve, which stays
  correct under `ADR-preserve-historical-citations-verbatim` and reads as rot to a newcomer.

## Status

Implemented — `.hooks/canon-named-once.sh`, wired as `canon-named-once`.
