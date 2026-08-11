# Plan-Zone Artifacts

Copyable artifacts for a project's plan zone: a JSON Schema for one lane file, one for the optional
config, a dependency-free cross-file linter, a minimal valid fixture, and the linter's behavioral
test harness. These files are canonical reference, not a runtime dependency on this repository.

## Ownership split

| Artifact                                               | Owns                                                      |
| ------------------------------------------------------ | --------------------------------------------------------- |
| [`plan-lane.schema.json`](./plan-lane.schema.json)     | Field names, types, enums, and lane-local conditionals.   |
| [`plan-config.schema.json`](./plan-config.schema.json) | Parameters a script reads and the record cannot supply.   |
| [`check-plan`](./check-plan)                           | Every fact that spans files, plus explicit ranking modes. |
| [`test-check-plan`](./test-check-plan)                 | Writer properties and cross-file negative cases.          |
| [`example/`](./example/)                               | A minimal valid plan zone and worked-story fixture.       |

The schema owns one file's shape. The linter owns every fact that spans files. Neither reimplements
the other's rules.

## Record shape

```text
<project>/docs/plan/
  README.md
  charter.md
  config.yml            optional; what a script reads
  open-questions.md
  plan-lane.schema.json
  plan-config.schema.json
  lanes/
    backlog.yml
    todo.yml
    doing.yml
    review.yml
    closed.yml
  stories/
    <id>-<slug>.md
    <id>-<slug>/        optional non-narrative artifacts
  epics/                optional; one end state per document
    <id>-<slug>.md
```

All five lane files exist even when empty. A representative entry is:

```yaml
# yaml-language-server: $schema=../plan-lane.schema.json
lane: todo
stories:
  - id: "009"
    slug: release-readiness
    type: story
    points: 2
    needs: ["006"]
    epic: "014"
```

Lane membership comes from the filename, ranking from sequence position, and dependencies from
`needs`. Ids are quoted so YAML 1.1 readers do not interpret values such as `007` as octal. Lists
use flow style so a complete edge remains greppable.

The linter checks ids unique across stories and epics, filename agreement in both, epic-document
existence, dependency existence and cycles, work-lane gates, terminal fields, question edges, `Amends` paths in stories and epics alike,
type-conditional Example fences, and the two legal ranking rules described in
[03 — The Plan Record](../03-the-plan-record.md). It warns, without failing, when a close date sits
out of sequence in `closed.yml`.

## Modes

```bash
./check-plan docs/plan
./check-plan --rank-slots docs/plan
./check-plan --rank-fix docs/plan
```

The default validates without writing. `--rank-slots` reports legal candidates for each position.
`--rank-fix` is the only write mode: it performs a stable eligibility partition and stable
topological repair in `backlog.yml` and `todo.yml`, and a stable sort by close date in `closed.yml`,
moving original line blocks without changing lane membership or re-serializing YAML.

The writer is deterministic, byte-identical on legal input, idempotent, and non-canonical. It
preserves comments, modelines, blank lines, and free-form notes. A cyclic graph exits non-zero and
is unchanged, and no content failure lets it write at all. Never invoke `--rank-fix` from a hook;
validation must remain falsifiable.

A close date out of sequence is the one fact reported as a warning rather than a failure. It exits
zero in every mode, because `--rank-fix` can repair it and a gate on a repairable order buys
friction rather than proof.

## Runtime and canonical YAML

`check-plan` needs bash, coreutils, grep, sed, and awk. It accepts flat mappings, one entry nesting
level, and flow sequences. Anchors, aliases, block scalars, and multi-document YAML are rejected
because a plan record should remain human-scannable before a project toolchain exists.

## Adoption

1. Copy `plan-lane.schema.json` into `<project>/docs/plan/`, and `plan-config.schema.json` beside it
   if the project keeps a `config.yml`.
2. Copy `check-plan` into `<project>/scripts/` and make it executable.
3. Start from the fixture's lane files and keep each language-server modeline.
4. Add schema and cross-file validation to the project's hook runner.
5. Copy the story and epic heading gates from [the template](../template-heading-shapes.md).

```yaml
- repo: https://github.com/python-jsonschema/check-jsonschema
  rev: <pinned-version>
  hooks:
    - id: check-jsonschema
      name: plan lane files match the schema
      files: '^docs/plan/lanes/.*\.yml$'
      args: ['--schemafile', 'docs/plan/plan-lane.schema.json']

    - id: check-jsonschema
      alias: plan-config
      name: plan config matches the schema
      files: '^docs/plan/config\.yml$'
      args: ['--schemafile', 'docs/plan/plan-config.schema.json']

- repo: local
  hooks:
    - id: check-plan
      name: plan zone is coherent
      entry: scripts/check-plan
      language: system
      pass_filenames: false
      files: '^docs/plan/(lanes/|stories/|epics/|open-questions\.md|plan-lane\.schema\.json)'
```

`pass_filenames: false` is required because the rules read the plan as a whole. The hook stays
read-only; a close ceremony runs `--rank-fix` explicitly before validation.

The test harness uses only bash, coreutils, `diff`, and `mktemp`, all supplied by a normal stdenv.
This repository's `flake.nix` therefore needs no additional package for it.
