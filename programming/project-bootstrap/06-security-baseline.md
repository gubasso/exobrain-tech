# 06 — Security baseline

The minimum security posture a new project should have from day one. Keep it language-agnostic here; the language binding adds ecosystem tools (e.g. `cargo-deny` / `cargo-audit` for Rust).

## Secrets hygiene

Never commit secrets. Enforce it on three fronts:

- **Ignore** — `.env` and credential files are in `.gitignore` (chapter [01](./01-repository-foundation.md)) so they cannot be staged by accident.
- **Scan** — enable secret scanning / push protection on the forge to block a secret that slips through.
- **Rotate** — if a secret is ever committed, rotate it; scrubbing history is not enough.

Prefer keyless auth (OIDC) wherever a workflow supports it, so there is no long-lived secret to leak.

Where a token is needed, prefer one that a host program mints and that expires within hours. Rotate a long-lived key once a year, and at once on any suspicion of a leak. Rotate it by overlap:

1. Add the new key.
2. Make sure that the check passes with the new key.
3. Remove the old key.

A token that rotates itself needs three more guards:

- The rotation writes the new token atomically to a persistent file that the sandbox denies. The program that uses the token reads that file on each request, so a rotation needs no restart.
- A health check fails when the last good rotation is older than the rotation period allows, and it names the step that re-mints the token.
- The operator's own client re-mints the token, never through the credential gateway of the agents.

A project that agents push to protects its trunk on the forge against the agent identity. [The authorization boundary](../agent-integration/authorization-boundary.md#the-forge-bounds-the-agent-identity) states the rule.

## Dependency review & audit

Track and vet dependencies:

- Enable automated dependency-update PRs so you learn about advisories promptly.
- Run a vulnerability audit that fails on known-vulnerable dependencies at pre-push, and in CI as a repeat of the push stage (chapter [05](./05-ci.md)).
- Review new dependencies before adding them — every dependency is attack surface.

## OpenSSF Scorecard

[OpenSSF Scorecard](https://github.com/ossf/scorecard) is a measurable baseline: it scores a repo on branch protection, pinned dependencies, token permissions, CI-test presence, and more. Use its checks as the target checklist for the security posture — most map directly to steps already in this runbook (CI in chapter [05](./05-ci.md), least-privilege CI tokens, etc.).
