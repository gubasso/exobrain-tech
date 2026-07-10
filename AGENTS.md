# AGENTS

Digest for `exobrain-tech`.

This repo is the public technical knowledge base. The KB product lives in the eight top-level buckets; governance docs live under `docs/` and follow the docs-design method. Root `AGENTS.md` intentionally keeps the existing concise no-frontmatter style; migrated per-directory digests carry refreshed frontmatter.

Source map:

| Topic | File |
| --- | --- |
| KB index | `README.md` |
| Authoring rules | `CLAUDE.md` |
| Zone index | `docs/README.md` |
| ADR template | `docs/decisions/template.md` |
| Governance decision | `docs/decisions/ADR-0001-documentation-governance.md` |
| Delegation architecture ADR | `docs/decisions/ADR-0002-in-session-subagent-delegation.md` |

Public/private boundary: do not place private equipment identity, security posture, recovery material, credentials, or personal workflows here; use `exobrain-tech-vault`. `docs/` is governance-only and migrated KB notes do not belong there.
