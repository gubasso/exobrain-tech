# Decisions

Architecture Decision Records for changes to this spec.

## Format

One file per decision: `ADR-<slug>.md`, where the slug is the identifier. A filename carries no
counter and no digit, because two branches each allocating the next number are both right and the
merge leaves two records claiming one identity. A merged record's filename never changes, so a
citation stays resolvable; a replaced decision gets a new superseding record rather than an edit.

## Template

```markdown
# Title

**Status:** Proposed | Accepted | Superseded by ADR-<slug> **Date:** YYYY-MM-DD

## Context

What's the situation that requires a decision? What constraints apply? Cite specific chapters of the
spec or files in reference projects.

## Decision

What we're doing. One paragraph, declarative.

## Consequences

What changes after this decision: which chapters update, which templates update, which projects need
migration. Include the explicit downsides.

## Alternatives considered

Brief notes on options that were rejected and why.
```

## When to write one

- Changing a rule in any chapter (e.g. swapping figment for config-rs).
- Adding or removing a default dependency.
- Changing the canonical directory tree.
- Choosing between mutually exclusive options when a chapter says "pick one".

Don't write one for:

- Typo fixes, formatting, link updates.
- New examples added to existing chapters.
- The records themselves.
