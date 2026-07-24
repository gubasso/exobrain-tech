# Config generation from types

How a CLI gives users **accurate, self-documented config to copy** — an annotated
example file plus a machine schema — generated from the config types themselves,
kept honest by a pre-commit check, and **without the tool ever writing to
`$XDG_CONFIG_HOME`**.

This chapter resolves a tension the config rules create. Config is read-only at
runtime and the user authors it
([11 — XDG scaffolding](./11-xdg-scaffolding.md), and the _why_ in
[`design-decisions/config-state-ownership/`](../design-decisions/config-state-ownership/README.md)).
So the tool must **not** scaffold a starter file. But hand-written example configs
and docs drift from the real types the moment a field is added, renamed, or made
optional. The fix is to make the example a **generated artifact of the types**, so
it cannot drift, and let the user copy it. Call the rule **copy-don't-scaffold**.

## The rule: the type is the source of truth, the example is derived

The config type (Rust struct, Pydantic model, Go struct) is the single source of
truth for the config surface. Two artifacts are **generated** from it, never
hand-maintained:

| Artifact              | For                                            | Where it lands                               |
| --------------------- | ---------------------------------------------- | -------------------------------------------- |
| **JSON Schema**       | Editor validation, autocomplete, CI validation | `docs/reference/` or shipped with the binary |
| **Annotated example** | The file a user copies and edits               | `docs/reference/examples/*.example.<ext>`    |

The tool ships these in-repo and the user copies one into their config dir. The
tool then only ever **reads** the result — the file's sole writer stays the user
(the copy-don't-scaffold discipline of [11](./11-xdg-scaffolding.md)).

## What a good generated example looks like

- **Required keys active, optional keys commented out.** The uncommented file is a
  minimal valid config; uncommenting adds optional surface.
- **Every field annotated with its description**, pulled from the type's own
  doc-comments / field metadata — not a parallel doc that can rot.
- **Placeholder values, never real ones.** `REPLACE_ME`, `user@example.com`, the
  first enum variant. Secret fields get an obvious placeholder, never a live value.
- **A generated-file header**: "Generated from `<type>` — do not edit here. Copy to
  `$XDG_CONFIG_HOME/<app>/` and edit there; the tool never writes your config."
- **It parses.** The generated example must round-trip through the real loader in a
  test, or it is a lie.

```toml
# GENERATED from AppConfig — do not edit here.
# Copy to $XDG_CONFIG_HOME/pigeon/config.toml and edit there.
# The tool never writes your config.

# Backend endpoint (required).
endpoint = "REPLACE_ME"

# Request timeout, seconds (optional; default 30).
# timeout_secs = 30
```

## How to generate it

Reflect over the typed model, then render. The mechanism is language-specific but
the shape is identical everywhere:

- **Rust** — derive `serde` (the wire format) + `schemars`
  (`#[derive(JsonSchema)]`) for the schema; doc-comments and
  `#[schemars(description = "…")]` become field descriptions. Walk the schema to
  render the annotated `*.example.toml`. See the Rust config spec
  ([`languages/rust/cli-spec/05-config.md`](../../languages/rust/cli-spec/05-config.md)).
- **Python** — Pydantic models; `Field(description=...)` is the description source;
  `model_json_schema()` yields the schema.
- **Go** — struct tags plus a schema-reflect library.

**Make descriptions mandatory.** The generator should **hard-fail** if a public
config field lacks a description — that is what keeps the example self-documenting
instead of a wall of undocumented keys.

A `schema` / `--print-config` subcommand is the runtime companion to the committed
artifacts: expose the JSON Schema
([05 — `pigeon schema <type>`](./05-designing-for-llm-agents.md)) and let
`--print-config` dump the effective config
([03 — Schema discipline](./03-config-precedence.md#schema-discipline)). Same
source of truth, three surfaces: committed example, committed schema, live command.

## Freshness: a pre-commit `--check` gate

Generated files rot silently unless a gate proves they match the types. Add a
generator mode that **regenerates and compares**, exiting non-zero on drift, and
wire it as a pre-commit hook (and the same command in CI):

```yaml
# .pre-commit-config.yaml
- id: config-examples
  name: config examples are in sync with types
  entry: cargo run -p example-generator -- --check
  language: system
  pass_filenames: false
  always_run: true
```

Because the examples are a whole-model relationship, regenerate and stage **all**
of them, not per-changed-file (`pass_filenames: false`, `always_run: true`). See
[`best-practices/pre-commit.md`](../best-practices/pre-commit.md) for hook wiring.
The default (non-`--check`) mode regenerates and stages only the generated paths.

## What cannot be generated from types

Some config surfaces are not reflectable from a data type — an embedded DSL,
hand-written module files, a plugin written in the host language. Those cannot be
derived, so ship them as **hand-maintained example files** under the _same_
copy-don't-scaffold discipline: a committed `*.example.*` with a header, that the
user copies. Do not scaffold them into config either; the only difference is the
example is authored, not generated.

## Anti-patterns

- **Scaffolding the config file on `init`.** Violates read-only config
  ([11](./11-xdg-scaffolding.md)). Generate an example to copy instead.
- **Hand-maintained example configs for a reflectable type.** They drift; the next
  field addition makes them wrong and no check catches it.
- **Descriptions in a separate doc.** Put them on the field; generate the doc.
- **No freshness gate.** Without `--check` in pre-commit/CI, "generated" examples
  are just stale examples with extra steps.
- **Placeholder values that are valid live values.** A copyable example with a real
  endpoint or token invites accidental use; keep placeholders obviously fake.

## Checklist

- [ ] The config example is **generated from the config type**, not hand-written.
- [ ] Generation **fails** if any public field lacks a description.
- [ ] Required keys active, optional keys commented, placeholders obviously fake.
- [ ] A JSON Schema is generated for editor/CI validation.
- [ ] The generated example **round-trips** through the real loader in a test.
- [ ] A pre-commit `--check` (and CI) fails on drift; `pass_filenames: false`.
- [ ] The tool **copies-don't-scaffold**: it never writes the user's config
      ([11](./11-xdg-scaffolding.md), [config-state-ownership](../design-decisions/config-state-ownership/README.md)).

## See also

- [11 — XDG scaffolding & `init`](./11-xdg-scaffolding.md) — copy-don't-scaffold; where `init` may write.
- [03 — Config precedence](./03-config-precedence.md#schema-discipline) — schema discipline, `--print-config`.
- [05 — Designing for LLM agents](./05-designing-for-llm-agents.md) — `schema` subcommand, `--json`.
- [`design-decisions/config-state-ownership/`](../design-decisions/config-state-ownership/README.md) — config is read-only to the tool (the _why_).
- [`best-practices/pre-commit.md`](../best-practices/pre-commit.md) — freshness-gate hook wiring.

## References

External links, for further reading only.

- JSON Schema — annotations (`title`, `description`, `examples`) — <https://json-schema.org/understanding-json-schema/reference/annotations>
- `schemars` (Rust JSON Schema from types) — <https://graham.cool/schemars/>
- Pydantic — JSON Schema — <https://docs.pydantic.dev/latest/concepts/json_schema/>
