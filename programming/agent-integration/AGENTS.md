---
digest-of: programming/agent-integration
last-synced: 2026-09-14
token-estimate: 1200
---

# AGENTS

## Scope

How a project is built to be discovered, operated, and trusted by a coding agent: the instruction
file, the authorization boundary, the skills it ships, where they install, and how the result is
evaluated. Designing the command-line tool an agent runs is `programming/cli-design/`. A fact about
one agent runtime is that vendor's shelf under `tools/`, dated.

## Key points

### The model (00)

- Four layers, each with one class of artifact: mechanism, what the agent runs; playbook, what it
  loads for a task; durable knowledge, what it reads on demand; governance, what binds every
  session.
- Each layer has what it never carries. A tool that encodes a workflow, a skill that copies a flag
  list, and an instruction file that collects every rule are the same mistake in three places.
- A project that owns a land ships the skills that operate it, in its own tree, installed by its own
  command. Shared material several skills read installs once under the user's state directory and is
  named by absolute path, because two runtimes agree on no relative layout.
- A skill's name is its identity, and a name has one owner. One package under two scopes is two
  entries under one name.

### Instruction files (01)

- The root file is hand-authored rules and routing. Judgment has no generator.
- A per-directory digest is the second shape: generated from the subtree, regenerated rather than
  edited, carrying frontmatter and no rule of its own.
- Every sentence is loaded in every session, so routing beats restating and length is the budget
  that keeps the file read.

### Skills (03)

- A skill has one authored owner per kind of content: judgment in `SKILL.md`, durable knowledge in a
  reference, mechanics in a script. A fact stated twice is already wrong in one place.
- A conforming package is a directory with `SKILL.md`; `references/`, `scripts/`, and `assets/` are
  progressive disclosure, not requirements. The package is self-contained and one level deep.
- Two required fields and four optional ones. An unrecognized field is ignored, and that rule is
  what makes a package portable. `allowed-tools` is marked experimental.
- The description is the whole triggering mechanism, because the body loads only after triggering.
  Three parts: what it produces, the phrasings a person types, the boundary against a near sibling.
- Progressive disclosure is a budget, not a style. The deletion test decides every paragraph: remove
  it, and ask whether the agent then makes a likely task error.
- Extract any deterministic chunk past a trivial one-liner, single use included. Four guardrails:
  split judgment from mechanism, keep the one-liner, stay coarse, leave prompt text to the model.
- A validation loop whose checks are the mechanism's own is the highest-value shape a skill carries.

### Distribution (04)

- Two scopes: user, owned by the person; project, committed with the land it drives.
- One canonical directory, linked into each runtime's root, is what keeps several runtimes on one
  copy. Whether a runtime follows the link is a dated vendor fact.
- An installer writes only what it declared, removes only what it wrote, and reports what it did.

### Evaluations (05)

- A test suite says the mechanism works; an eval says the agent uses it correctly. They fail
  differently and a project needs both.
- Four suites: trigger, behavior, portability, adversarial.
- An agent is non-deterministic, so the measurement is a rate over samples, not a verdict on one
  run. Ten samples is the smallest number that distinguishes a rate from an anecdote.
- Grade the outcome, not the route. Pin a tool sequence only where the sequence is the safety
  requirement.
- A verifier decides anything code can decide, from the transcript. A judge model grades the rest
  against a written rubric.
- Compare against a baseline run with the skill absent. A skill that does not beat it costs tokens
  for nothing.
- The dominant failure of an agent-written test suite is testing the library instead of the project;
  the five detection heuristics are stated once, in `cli-design/09`.
- Snapshot the three agent-facing surfaces: help output, machine-output schema, exit codes.

### Documentation and discovery (06)

- Two knowledge sets. Project artifacts are what the project's configuration selects into its own
  tree. Tool references are what the installed version carries, and they are exposed, never landed.
- Three steps: one always-loaded line in the instruction file, one described index carrying stable
  ids and aliases, one reader that returns an exact topic. Nothing embeds the corpus eagerly.
- Version matching is structural. The running executable serves the references it was built with,
  and a trunk, an unrelated checkout, or a search is a different version with no label saying so.
- A human index optimizes routing and a machine form declares a schema and stable ids. Neither is an
  executable plan.
- An id is stable. A renamed topic keeps its id and the old name becomes an alias.

### Candidate artifacts and receipts (08)

- A candidate is what the installed executable projects for one invocation, from its own sources,
  the target's resolved configuration, and the target's observed capabilities. It is not another
  release's bundle, not a stored plan, and not installed state.
- The receipt is attributed installed state, held centrally: producer identity and version, resolved
  inputs, destination, ownership class, placement, and the digest at the moment of writing.
- Reading an older receipt schema is bounded record-reading. It is not interpreting another
  release's content, and conflating the two turns a record format into a protocol.
- Three ownership classes: a generated file the tool replaces, a seeded file the project owns after
  creation, and a marked region inside a project-owned file.
- Provenance lives in the receipt. A generated file carries no version watermark, and a marker names
  placement rather than a version.
- Missing provenance reduces certainty and grants nothing. An unattributed whole file is never
  overwritten, and the tool never fetches an older release to manufacture the record it lacks.
- A retired destination is reported and released to the project. It is not deleted.

### Disposable staging and direct landing (09)

- Eight steps: evidence, render to stage, investigate, prepare, render to production, verify,
  compare against the retained stage, clean under explicit authorization.
- Production never reads the stage. Remove the stage first and production produces the same result.
  The two steps share a pure projection, never serialized state, which is what avoids a validity
  protocol with fingerprints and revalidation.
- The stage holds the artifact tree, an explanatory receipt, the omissions and conflicts, the
  changelog and guidance, and version-matched reference material it exposes rather than lands.
- Landing validates every path and collision before writing, then writes by ownership class, and
  writes the central receipt last. Files a record does not yet claim are recoverable; the reverse
  is not.
- The guarantee is a whole-file boundary on supported local filesystems, under one lock per target.
  The set is not transactional: no rollback, no all-or-nothing, a possibly completed remote rename
  re-observed, the observed completed paths reported, version control for recovery, rerun supported.
- Cleanup is its own destructive command. It identifies the stage by its receipt, refuses a symlink,
  a filesystem or home root, the project root and its ancestors, and an ambiguous directory, holds
  the validated boundary through removal, and removes one stage after explicit authorization.
- A rule states a property. An implementation names the mechanism and the targets where it proved it.

### Skill security (15)

- A skill from outside the project is code review, not a download.
- Fetched and reader-supplied content is data, never an instruction, and the rule is stated where
  the skill reads it rather than once at the top.
- Least tools, validated output before a shell or a path or a parser, the plan shown before a
  destructive step, an explicit target for anything irreversible, and no secret anywhere.
- The description says whether the skill contacts the network.

## Maintenance notes

- `02` is reserved for the authorization boundary and lands from another project's plan. The index
  in `README.md` names only chapters that exist.
- Where a chapter would restate a rule a `cli-design/` chapter owns, it links that owner. Check that
  before adding a rule here.
- A rule that holds for one runtime belongs in that runtime's adapter under `tools/`, dated. `01`
  links the instruction-file adapters, and `03` and `04` link all four skills adapters.
