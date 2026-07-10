# Migration Report (Public Safe)

Source count: 610 files. Public delivered rows: 610. Vault-only rows: 0. Split rows: 1. Secret-redact dropped rows: 0.

Public reorg decisions: `main.md` became `README.md`; the retained KB buckets are `programming/`, `languages/`, `platforms/`, `infra/`, `systems/`, `tools/`, `data/`, `workflows/`; `projects/specs/*` was folded into `programming/specs/*` after reading the chatbot system spec and confirming it is a software architecture spec. `docs/` remains governance-only.

Scrubbing categories applied: personal home paths, personal names/emails, example usernames, personal hostnames, and secret-like example assignments in conceptual secret-management docs. No confirmed real source secret was copied or dropped.

Verification artifacts are in the run review directory recorded by the vault report: source counts, manifests, scrub log, staged scans, link checks, collision report, and final reconciliation.

## Post-review corrections (2026-07-10)

- Restored 37 wrongly redacted public internal references from `link-redactions.tsv`; every restored target was verified to exist in the public repository.
- Scrubbed leaked private identity/work-org tokens from the public repository while preserving the explicit public-project attribution URLs for `github.com/gubasso/cog` and `github.com/gubasso/dotfiles`.
- Set root `README.md` mode to `644`.
- Re-verification passed: public placeholder recheck empty, public identity scan contains only kept public-project attribution hits, public secret scan empty, and public local markdown link check empty.
