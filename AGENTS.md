# AGENTS

Digest for `exobrain-tech`.

This repo is the public technical knowledge base. The KB — the library, the content — is the product and lives in the eight top-level buckets, exactly as a code project's product is its codebase; `_docs/` holds the metadata and specs _about_ that product (governance, following the docs-design method), never the product itself. Root `AGENTS.md` intentionally keeps the existing concise no-frontmatter style; migrated per-directory digests carry refreshed frontmatter.

<!-- self-containment -->

## Self-Containment

Non-negotiable: this project is self-contained. The knowledge it depends on is held in-repo. An external reference is allowed only as a public link or citation for further reading — never as a load-bearing dependency on a resource outside the repository, and in particular never on an external, local, personalized, or mutating repository, path, or tool. If an external document, repo, or personal path is required to understand, build, or operate this project, copy its essential knowledge into the repository (a doc, an ADR, or an inline comment) so the repo stays complete on its own. See `_docs/decisions/ADR-0003-self-containment.md`.

## Decisions

Non-negotiable: record every significant, hard-to-reverse decision as an ADR under `_docs/decisions/`, one decision per file, using the MADR-minimal `template.md` (at or below 350 words, exactly one `Status`), so the rationale lives with the code. Accepted ADRs are not deleted; a changed decision gets a new superseding ADR.

Source map:

| Topic                        | File                                                         |
| ---------------------------- | ------------------------------------------------------------ |
| KB index                     | `README.md`                                                  |
| Authoring rules              | `CLAUDE.md`                                                  |
| Zone index                   | `_docs/README.md`                                            |
| ADR template                 | `_docs/decisions/template.md`                                |
| Governance decision          | `_docs/decisions/ADR-0001-documentation-governance.md`       |
| Delegation architecture ADR  | `_docs/decisions/ADR-0002-in-session-subagent-delegation.md` |
| Self-containment ADR         | `_docs/decisions/ADR-0003-self-containment.md`               |
| Docs-vs-library boundary ADR | `_docs/decisions/ADR-0004-docs-vs-library-boundary.md`       |

## Docs vs library content

Non-negotiable: the product is the library — the knowledge itself. The knowledge articles and books the product exists to serve live in the eight top-level buckets (`programming/`, `languages/`, `systems/`, `infra/`, `tools/`, `platforms/`, `workflows/`, `data/`), just as a code project's product is its codebase. `_docs/` is **not** the product: it is reserved for the metadata and specs _about_ that product — governance, ADRs, conventions, architecture, guides, and explanation about how this knowledge base itself works. Never place a knowledge article under `_docs/`; place it in the owning bucket and cross-link from `_docs/` only when the product reference must mention it. Placement test: "Is this about how the KB works?" → `_docs/`. "Is this knowledge the library serves?" → the content bucket. See `_docs/decisions/ADR-0004-docs-vs-library-boundary.md`.

Public/private boundary: do not place private equipment identity, security posture, recovery material, credentials, or personal workflows here; use `exobrain-tech-vault`. `_docs/` is governance/metadata-only and migrated KB notes do not belong there.
