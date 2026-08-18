# Documentation governance

## Context and Problem Statement

`exobrain-tech` needs public technical knowledge without mixing content with
repo-governance rules. Contributors need a small, stable documentation method.

## Considered Options

- No governance scaffold.
- Use the docs-design method.
- Copy project internals into the public KB.

## Decision Outcome

Chosen option: **use the docs-design method**. Core KB content remains
free-form, while `_docs/` carries Diátaxis governance, lean ADRs, guides,
reference, and explanations.

## Consequences

- Good: public technical knowledge can grow naturally.
- Good: decisions and operating rules have predictable homes.
- Bad: contributors must keep private or personal material out of this repo.

## Status

Superseded by
[ADR-spec-driven-docs-is-the-documentation-method](./ADR-spec-driven-docs-is-the-documentation-method.md).
