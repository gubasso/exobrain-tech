# ADR-0006: The library may ship executable artifacts, and must gate them

## Context and Problem Statement

This knowledge base has shipped only prose, embedding machine-readable artifacts as fenced code
blocks — `template-heading-shapes.md` carries `MD043` configuration that way. That works for eight
lines of illustrative JSONC. It fails for a JSON Schema and a 250-line linter, because **a fenced
block cannot be validated**: no hook can run `check-jsonschema` against a schema that exists only
inside markdown, or `shellcheck` against a script that does. The KB would publish an artifact it
asserts is correct and never checks — the failure `07-plan-and-slices.md` itself names, since an
unenforced reference reads as verified.

## Considered Options

- Keep embedding artifacts as fenced blocks; accept that they are untested.
- Extract fenced blocks to temporary files in CI and validate those.
- Ship artifacts as real files inside the owning content bucket, and gate them here.

## Decision Outcome

Chosen option: **ship them as real files inside the owning bucket, and gate them** — a drop-in a
reader copies is only trustworthy if this repository proves it runs. Extraction was rejected as a
parser between the artifact and its test, which is one more thing to be wrong.

An executable artifact is library content, not `_docs/` metadata: it is knowledge the library
serves, so it lives in its bucket beside the chapter that explains it (ADR-0004 unaffected). Three
obligations attach to shipping one: a gate in `.pre-commit-config.yaml`, a case in `just test`, and
any tool it needs added to `flake.nix` in the same change. A fenced block remains correct for
illustration; the line is whether a reader is expected to copy the thing and run it.

## Consequences

- Good: published drop-ins are proven, not asserted; `cp` replaces extract-from-fence; a schema
  gives editor completion where a fenced copy gave none.
- Bad: this repository is no longer markdown-only. `just test` and `just build` stop being
  documented no-ops, the devShell grows per-artifact tooling, and a contributor may now break a
  build.

## Status

Accepted. Enacted by `programming/docs-design/plan/`, the plan-zone hooks in
`.pre-commit-config.yaml`, the `test` recipe in `justfile`, and the validator packages in
`flake.nix`.
