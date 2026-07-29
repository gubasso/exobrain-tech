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

### Where the generator lives

The generator is **development tooling and must not ship in the user's binary**. It
needs a schema library, a comment-preserving renderer, and read access to the
config types — none of which a user installing the tool should pay for. Three homes
come up; they are not equivalent:

| Home                        | Verdict                                                                            |
| --------------------------- | ---------------------------------------------------------------------------------- |
| **Separate build target**   | **Prefer.** `cargo xtask` / a dev-only package / a `tools/` module                 |
| Hidden subcommand           | Avoid — ships schema deps to every user, and puts a dev concern on the CLI surface |
| Second binary, same package | Workable, but the dev dependencies stay in the shipped graph                       |

In Rust this is a `cargo xtask` workspace member; in Python, a dev-only package
in the same repo (`example_generator`, not shipped in the wheel); in Go, a
`tools/` main package excluded from the release build.

There is a catch worth planning for in Rust: an `xtask` in a separate crate cannot
import config types from a **binary-only** crate. The package needs a library
target exposing just those types. That is a real structural consequence — if a
project has a documented "one binary crate, no workspace until X" rule, the
generator is usually the thing that trips it, and the honest move is to note that
the trigger fired rather than to route around it with a hidden subcommand.

A `schema` / `--print-config` subcommand is the runtime companion to the committed
artifacts: expose the JSON Schema
([05 — `pigeon schema <type>`](./05-designing-for-llm-agents.md)) and let
`--print-config` dump the effective config
([03 — Schema discipline](./03-config-precedence.md#schema-discipline)). Same
source of truth, three surfaces: committed example, committed schema, live command.

## Freshness: a pre-commit gate

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

### Staleness is a comparison, not a cache

**Render every artifact in memory and compare it byte for byte with what is on
disk.** A file whose contents differ is stale; a missing file is stale. That is the
entire mechanism.

The instinct to add a cache — hash the model, stamp a timestamp, skip the render
when the hash matches — is the wrong one, and it is worth naming because it looks
like an optimization. Rendering a config example is microseconds of work; the cache
buys nothing measurable and costs three real things:

- **An artifact to manage.** The stamp has to be committed (so CI can read it) or
  gitignored (so CI cannot), and both are wrong in a different way.
- **An invalidation rule to get wrong.** Every cache has a failure mode where it
  reports fresh and the file is not — which is the exact condition the gate exists
  to catch. A gate that can be fooled by its own cache is worse than no gate.
- **A second source of truth** about what "current" means: the types, and the stamp.

Render-and-compare has none of these because it recomputes the answer every time.
What makes it viable is **deterministic, byte-stable rendering** — same types in,
identical bytes out, with stable key order and no timestamps or absolute paths in
the output. Get that right and a commit touching no config field is a natural
no-op, which is the property the cache was reaching for anyway.

Byte-stability is a real constraint on the renderer, not a freebie: a serializer
that reorders map keys between versions, or writes the generation date into the
header, turns every unrelated commit into a diff. Pin the behaviour and test it.

### Regenerate-and-stage, not verify-only

The hook has two plausible shapes. Prefer the first:

| Shape                    | Behaviour on drift                                                  | Cost                 |
| ------------------------ | ------------------------------------------------------------------- | -------------------- |
| **Regenerate-and-stage** | Rewrites stale files, `git add`s those paths, fails the commit once | Whole-file staging   |
| Verify-only              | Reports and fails; the author reruns by hand                        | An extra manual step |

Regenerate-and-stage is better because the model change and its regenerated example
land in **one commit**, which is what makes the history readable — a field addition
whose example arrives two commits later has a window where the repo contradicts
itself. Use `--check` (verify-only, writes nothing) in CI, where staging is
meaningless and a mutation would be a surprise.

The cost is real and worth stating up front: **generated files must be staged
whole.** An author using `git commit -p` on a generated example commits something
the generator did not produce, and neither shape of the gate can distinguish that
from ordinary staleness.

## What cannot be generated from types

Some config surfaces are not reflectable from a data type — an embedded DSL,
hand-written module files, a plugin written in the host language, or **a document
whose schema belongs to another program**. Those cannot be derived, so ship them as
**hand-maintained example files** under the _same_ copy-don't-scaffold discipline:
a committed `*.example.*` with a header, that the user copies. Do not scaffold them
into config either; the only difference is the example is authored, not generated.

### Most real tools have a mix

A tool with several config surfaces will usually reflect some and not others, and
the split is not a design smell — it falls out of **who owns each schema**. Ask it
per surface, not per project:

| Surface                                | Owner        | Kind            |
| -------------------------------------- | ------------ | --------------- |
| The tool's own config file             | You          | Generated       |
| A typed profile/manifest of your own   | You          | Generated       |
| A fragment in another program's format | That program | Hand-maintained |
| An embedded DSL or module file         | The language | Hand-maintained |

A wrapper is the sharpest case: its own config reflects cleanly, while the settings
fragments it composes are the wrapped program's format and evolve on the wrapped
program's schedule. Reflecting those would mean pinning a schema you do not own,
which is the same coupling a wrapper exists to avoid.

Keep both kinds in **one directory** with the same header convention, and say in
the header which kind a file is. A reader copying an example should not have to
know; a contributor about to edit one must, because editing a generated file is
wasted work the gate will overwrite.

## Anti-patterns

- **Scaffolding the config file on `init`.** Violates read-only config
  ([11](./11-xdg-scaffolding.md)). Generate an example to copy instead.
- **Hand-maintained example configs for a reflectable type.** They drift; the next
  field addition makes them wrong and no check catches it.
- **Descriptions in a separate doc.** Put them on the field; generate the doc.
- **No freshness gate.** Without `--check` in pre-commit/CI, "generated" examples
  are just stale examples with extra steps.
- **A cache in front of the generator.** Rendering is microseconds; a hash or
  timestamp stamp adds an artifact, an invalidation rule, and a way for the gate to
  report fresh when it is not. Compare rendered bytes instead.
- **Non-deterministic rendering.** A generation date in the header, an unstable map
  order, or an absolute path in the output makes every unrelated commit a diff and
  defeats the comparison the gate depends on.
- **Shipping the generator in the user's binary.** A hidden subcommand puts schema
  dependencies in every install and a dev concern on the CLI surface.
- **Placeholder values that are valid live values.** A copyable example with a real
  endpoint or token invites accidental use; keep placeholders obviously fake.

## Checklist

- [ ] The config example is **generated from the config type**, not hand-written.
- [ ] Generation **fails** if any public field lacks a description.
- [ ] Required keys active, optional keys commented, placeholders obviously fake.
- [ ] A JSON Schema is generated for editor/CI validation.
- [ ] The generated example **round-trips** through the real loader in a test.
- [ ] Rendering is **deterministic and byte-stable** — no dates, no unstable key
      order, no absolute paths.
- [ ] Staleness is **rendered-bytes vs on-disk bytes**, with no cache or stamp.
- [ ] The pre-commit hook **regenerates and stages**; CI runs the same command with
      `--check`.
- [ ] The generator is a **separate build target**, not shipped in the user's binary.
- [ ] Each example's header says whether it is generated or hand-maintained.
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
