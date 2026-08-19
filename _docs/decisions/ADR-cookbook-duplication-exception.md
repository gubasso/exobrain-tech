# Cookbooks may duplicate spec content; they are exempt from DRY

## Context and Problem Statement

The repository is a single source of truth: every technical fact lives in one canonical place and
other pages link to it, which is what keeps the deep-dive specs from drifting apart. A cookbook — a
lean top-to-bottom runbook for one concrete task — is useful only if it is self-contained, and a
reader executing it wants every command in one file rather than a scavenger hunt across thirty
linked pages. Without an explicit exemption, a later de-duplication sweep strips the inlined
snippets and guts the cookbook's purpose.

## Considered Options

- Cookbooks are exempt from DRY and may inline snippets from the canonical specs — chosen.
- No cookbooks; force everything through the linked deep-dive specs — rejected: it removes the
  runbook genre rather than reconciling it.
- Cookbooks allowed but still DRY, link-only — rejected: it destroys the self-containment that makes
  a cookbook worth having.

## Decision Outcome

Chosen option: a cookbook MAY inline-duplicate canonical content to stay self-contained, subject to
three guardrails.

1. Footnote the canon. Every inlined snippet links to the spec that owns it, so a reader reaches the
   why and the authoritative version.
2. Never the source of truth. Where a cookbook snippet disagrees with its footnoted spec, the spec
   wins.
3. Marked and recognizable. A cookbook lives in a `cookbook/` directory or opens with a cookbook
   header, so tooling and reviewers identify an exempt file at a glance.

De-duplication does not apply to cookbook files, and no sweep collapses their snippets to links.

## Consequences

- Good: runbooks stay self-contained and fast to execute without eroding the specs they distill.
- Good: the exemption is explicit, so a sweep skips cookbooks by rule rather than by accident.
- Bad: a cookbook can fall out of sync. The footnote keeps the canonical source one click away, and
  the cookbook's `AGENTS.md` flags re-checking when a footnoted spec changes materially.

## Status

Accepted. Operative rule lives in the repo's `AGENTS.md`.
