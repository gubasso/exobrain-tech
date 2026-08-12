# ADR-0016: Plan Parameters Live in a Validated Config File

## Context and Problem Statement

Velocity is defined as points closed per iteration, but the iteration cadence lived only as prose in
`charter.md`. Nothing could join that sentence to the close dates in `closed.yml`, so a measure the
shelf calls portable and committed was not computable by any tool. The same gap would appear for
every future parameter a plan-zone script needs and the record cannot supply.

## Considered Options

- Parse the cadence out of charter prose.
- Drop named iterations and report only rolling windows.
- Keep parameters in a separate, schema-validated config file.

## Decision Outcome

Chosen option: **an optional `docs/plan/config.yml`, validated by
`plan-config.schema.json`**. Parsing prose makes a sentence load-bearing and breaks on rewording.
Rolling windows avoid the anchor but lose the ability to say what one iteration delivered, which is
the unit the charter declares.

The file carries the iteration anchor and length today, and is the home for any later parameter of
the same kind. Presence at the known path is the whole contract: nothing links to it, and no
document indexes it, so it costs no maintained reference. The charter keeps stating the cadence for
a reader, because a human reading the charter should not have to open a config to learn how the
project works; the two say the same thing to two audiences.

YAML rather than TOML, because the repository already validates YAML with `check-jsonschema` and the
linter's parser already reads flat YAML mappings. TOML would need a new parser and a new devShell
entry to express the same three values.

## Consequences

- Good: velocity becomes computable, the schema gives editor completion, and future parameters have
  an obvious home.
- Bad: the cadence is now stated twice, in prose and in config, and the two can disagree. Nothing
  gates that, and a reviewer owns it.

## Status

Superseded by [ADR-0017](./ADR-0017-the-planning-method-moves-to-plan-xp.md) — the method and its artifacts now live at [plan-xp](https://github.com/gubasso/plan-xp).
