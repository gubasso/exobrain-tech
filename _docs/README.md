# exobrain-tech `_docs`

Governance and metadata about the public technical knowledge base — the specs
about the product, not the product itself. This page is only an index into the
zones. `specs/` states what binds now and is loaded per domain; the four
Diátaxis zones serve the reader needs beside it.

| Zone           | Reader need   | Contents                         |
| -------------- | ------------- | -------------------------------- |
| `specs/`       | What binds    | Requirements, by rule ID         |
| `decisions/`   | Why           | ADRs and the template            |
| `guides/`      | Tasks         | Runbooks and procedures          |
| `reference/`   | Lookup        | Facts, diagnostics, known issues |
| `explanation/` | Understanding | Mental models and architecture   |

Core KB content outside `_docs/` can use the structure that best fits the topic.

## Start here

- [Knowledge-base architecture](./explanation/knowledge-base-architecture.md) — the
  product↔metadata relationship, content buckets, and the AGENTS.md digest standard.
- [Documentation conventions](./reference/docs-conventions.md) — which document owns which rule,
  and the conventions local to this repository.
- [Documentation review checklist](./reference/docs-review-checklist.md) — the pre-merge
  guard for documentation and content changes.
- [AGENTS.md digest template](./reference/TEMPLATE-agents-digest.md) — the per-area
  content-digest standard.
