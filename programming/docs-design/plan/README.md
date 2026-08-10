# Plan-zone artifacts

Working drop-ins for a project's plan zone: a JSON Schema that defines one kanban lane file, and a
dependency-free linter that checks everything spanning two files. Copy them; they are not a
dependency, and nothing here runs against your project from this repository.

> **Status.** These artifacts implement the lane-based plan record. [07 — Plan and Slices](../07-plan-and-slices.md) still documents the previous single-file `milestones.md` record,
> and the chapters are being rewritten to match. Where the two disagree, treat these artifacts as
> the newer design and the chapter as the one currently described in prose. The record format itself
> is unaffected by that gap: the schema and the linter are gated in this repository and are correct
> as they stand. See [ADR-0006](../../../_docs/decisions/ADR-0006-executable-artifacts-in-the-library.md)
> for why they ship as files rather than as fenced blocks.

## What each artifact owns

| Artifact                                           | Owns                                                               |
| -------------------------------------------------- | ------------------------------------------------------------------ |
| [`plan-lane.schema.json`](./plan-lane.schema.json) | the shape of one lane file                                         |
| [`check-plan`](./check-plan)                       | every fact that spans two files                                    |
| [`example/`](./example/)                           | a minimal valid plan zone, which is also this repository's fixture |

The split is the design decision worth internalizing, because getting it wrong doubles the code:

> The schema owns the shape of one file. The linter owns every fact that spans two.

**Schema** — field names and types, the closed lane vocabulary, id and slug patterns, `outcome`
required in `closed.yml` and forbidden elsewhere, `succeeded_by` required exactly when
`outcome: reshaped`. All declarative, all enforced by a stock hook, and all surfaced in the editor
before a commit exists, because each lane file opens with a `yaml-language-server` modeline.

**Linter** — an id in two lanes, a `needs` edge pointing at nothing, a self-reference, a cycle, a
slice directory with no entry, an entry with no directory, a drifted appetite cache, a slice in
`doing.yml` whose dependencies are still open, a question blocking something already in flight, and
a question whose every target has closed.

Neither can do the other's job. A schema sees one document; a cross-file linter has no business
re-deriving what a schema already declares.

## The record

```text
<project>/docs/plan/
  charter.md              what the project is for
  open-questions.md       questions that could change a decision, and what each blocks
  plan-lane.schema.json   this schema, copied
  lanes/
    backlog.yml           shaped enough to preserve, not selected
    todo.yml              selected as next work
    doing.yml             implementation owns it
    review.yml            completion claimed, evidence pending
    closed.yml            done, cut, or reshaped
  slices/<id>-<slug>/     one directory per unit of work
```

All five lane files exist from day one, even empty. A missing file is an error rather than an
implicit empty lane, because otherwise "no `review.yml`" cannot be told apart from "review not yet
adopted".

One entry:

```yaml
# yaml-language-server: $schema=../plan-lane.schema.json
lane: todo
slices:
  - id: "009"
    slug: release-readiness
    appetite: 1 session
    needs: ["006"]
    tags: [release]
```

Three properties that are easy to miss:

- **There is no `status` field.** The file is the lane. Carrying both would put status in two
  places, which is the failure the whole shape exists to avoid.
- **There is no `priority` field.** Position in `slices:` is the human ranking. Position cannot
  collide and never needs renumbering.
- **Ids are quoted.** Under YAML 1.1, still implemented by several parsers, `007` is octal 7. The
  schema types ids as strings matching `^[0-9]{3}$`, so the unquoted form is rejected structurally.

`needs` and `tags` are flow sequences on one line. That is not a parser shortcut — it keeps a whole
dependency edge greppable, which a block sequence would lose.

## Running it

```bash
./check-plan example                 # the worked example
./check-plan docs/plan               # a real project; this is the default path
PLAN_WIP_LIMIT=1 ./check-plan docs/plan
```

Failures name the file and line, so the fix is one edit away:

```text
docs/plan/lanes/doing.yml:3: in doing.yml but needs 006, which is still todo
docs/plan/lanes/doing.yml:3: appetite cache: entry says '9 sessions', docs/plan/slices/004-profile-composition/README.md says '4 sessions'
docs/plan/open-questions.md:11: stale: every slice it blocks is closed — this question should have exited
```

`check-plan` needs only bash, coreutils, grep and awk. That is deliberate: a `yq` dependency would
mean the first thing a new project does is install a YAML processor to read its own plan. The price
is that it accepts a canonical _subset_ of YAML — flat mappings, one nesting level, flow sequences
for lists — and refuses anything else with a message naming the line. Anchors, aliases, multi-document
files and block scalars are out of scope, and a plan record that needs them is a plan record no human
scans.

## Adopting

1. Copy `plan-lane.schema.json` to `<project>/docs/plan/`.
2. Copy `check-plan` to `<project>/scripts/`, and `chmod +x` it.
3. Copy `example/lanes/*.yml` as empty starting lanes. **Keep the modeline** — it is the line that
   carries most of the daily value, and the step people skip.
4. Add the two hook entries below.

```yaml
  # --- Plan zone ---
  #
  # Split deliberately: check-jsonschema owns the shape of one lane file and
  # gives the editor the same contract via each file's yaml-language-server
  # modeline; scripts/check-plan owns every fact that spans two files.
  - repo: https://github.com/python-jsonschema/check-jsonschema
    rev: 0.37.4
    hooks:
      - id: check-jsonschema
        name: plan lane files match the schema
        files: '^docs/plan/lanes/.*\.yml$'
        args: ['--schemafile', 'docs/plan/plan-lane.schema.json']

  - repo: local
    hooks:
      - id: check-plan
        name: plan zone is coherent across lanes and slices
        entry: scripts/check-plan
        language: system
        pass_filenames: false
        files: '^docs/plan/(lanes/|slices/|open-questions\.md|plan-lane\.schema\.json)'
```

`pass_filenames: false` because every rule but the first is cross-file: handing the hook one changed
path would let a slice rename slip through whenever the lane file happened not to be staged.

## Improving them

Edit the copy in this repository, not only the copy in your project. `just test` gates the schema
against its metaschema and its worked example, runs `shellcheck` over the linter, and runs the
linter against `example/`.
