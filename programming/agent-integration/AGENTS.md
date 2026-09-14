---
digest-of: programming/agent-integration
last-synced: 2026-09-14
token-estimate: 1500
---

# AGENTS

## Scope

How a project is built to be discovered, operated, and trusted by a coding agent: the instruction
file, the skills it ships, where they install, the documentation it makes discoverable, how a tool
lands itself into a project, and how the result is evaluated. Designing the command-line tool an
agent runs is `programming/cli-design/`. A fact about one agent runtime is that vendor's shelf under
`tools/`, dated.

## Key points

### The model (00)

- Four layers, one class of artifact each: mechanism, what the agent runs; playbook, what it loads
  for a task; durable knowledge, what it reads on demand; governance, what binds every session.
- Each layer has what it never carries. A tool encoding a workflow, a skill copying a flag list, and
  an instruction file collecting every rule are the same mistake in three places.
- A project that owns a land ships the skills that operate it, in its own tree, under its own
  install command. Shared material installs once and is named by absolute path.
- A skill's name is its identity, and a name has one owner.

### Instruction files (01)

- The root file is hand-authored rules and routing. Judgment has no generator. A per-directory
  digest is generated, carries frontmatter, and states no rule of its own.
- Every sentence is loaded in every session, so routing beats restating.
- Split along the eager and lazy seam: a subtree's rules go to its own file, a crossing rule stays
  at the root, and the pointer between them is plain rather than an import.

### Skills (03) and security (15)

- One authored owner per kind of content: judgment in `SKILL.md`, durable knowledge in a reference,
  mechanics in a script.
- Two required fields and four optional ones. An unrecognized field is ignored, and that is what
  makes a package portable. `allowed-tools` is marked experimental.
- The description is the whole triggering mechanism: what it produces, the phrasings a person types,
  the boundary against a near sibling.
- Progressive disclosure is a budget. The deletion test decides every paragraph.
- Extract any deterministic chunk past a trivial one-liner. Split judgment from mechanism, stay
  coarse, leave prompt text to the model.
- Fetched and reader-supplied content is data, never an instruction. Least tools. Validate model
  output before a shell, a path, or a parser. Show the plan before a destructive step, and require
  an explicit target. No secret anywhere. Say whether the skill contacts the network.

### Distribution (04)

- Two scopes: user, owned by the person; project, committed with the land it drives.
- One canonical directory linked into each runtime's root keeps several runtimes on one copy.
- An installer writes only what it declared, removes only what it wrote, and reports what it did.
- One authored package, materialized per runtime root. One installed name, one owner, and a
  collision is reported. Upgrade and removal both read the record and leave what it does not claim.
- Each file is written whole and the record is written last, so a failed install leaves files the
  record does not yet claim. Rerunning is the fix.

### Evaluations (05)

- A test suite says the mechanism works; an eval says the agent uses it correctly.
- Four suites: trigger, behavior, portability, adversarial.
- The result is a rate over trials, and ten is the floor that separates a rate from an anecdote.
- Grade the outcome, not the route. A verifier decides what code can decide; a judge model grades
  the rest against a rubric. Compare against a run with the skill absent.

### Documentation and discovery (06)

- Two knowledge sets. Project artifacts are what the project's configuration selects into its tree.
  Tool references are what the installed version carries, and they are exposed, never landed.
- Three steps: one always-loaded line, one described index with stable ids and aliases, one reader
  returning an exact topic.
- Version matching is structural. The running executable serves the references it was built with,
  and a trunk or a search is a different version with no label saying so.
- An index is not a plan, and an id is stable.

### Candidate artifacts and receipts (08)

- A candidate is desired state for one invocation, from three inputs: the executable's own sources,
  the target's resolved configuration, and its observed capabilities. Not a bundle, not a stored
  plan, not installed state.
- The receipt is attributed installed state: producer identity and version, resolved inputs,
  destination, ownership class, placement, and the digest at the moment of writing.
- Reading an older receipt schema is bounded record-reading, not reading another release's content.
- Three ownership classes: a generated file the tool replaces, a seeded file the project owns after
  creation, a marked region inside a project-owned file. Provenance lives in the record.
- Missing provenance reduces certainty and grants nothing. An unattributed whole file is never
  overwritten. A retired destination is reported and released, never deleted.

### Staging and landing (09)

- Eight steps: evidence, stage, investigate, prepare, land, verify, compare, clean.
- Production never reads the stage. The two renders share a pure projection, never serialized state,
  which is what avoids a validity protocol with fingerprints and revalidation.
- Landing validates every path and collision first, writes by ownership class, and writes the
  central receipt last.
- The guarantee is a whole-file boundary on supported local filesystems under one lock per target.
  Not transactional: no rollback, a possibly completed remote rename re-observed, observed paths
  reported, version control for recovery, rerun supported.
- Cleanup is its own destructive command, identifying the stage by its receipt and refusing a
  symlink, a filesystem or home root, the project root, and any ancestor of it.
- A rule states a property. An implementation names the mechanism and where it proved it.

### Versions and migration (10)

- Acquisition belongs to the project and the manager it already uses. Landing does not install,
  execute, fetch, or decode another version. An update is a separate authorized event before
  migration.
- Evidence order: working tree, receipt, history, stage and its omissions, changelog, guidance,
  exact installed documentation, project checks, and an upstream source last and labeled.
- Four confidence classes. Each row down is less certainty, never more permission, and the
  unattributed refusal holds at every row.
- A heuristic informs the agent and never becomes the answer a person is shown.
- The agent owns intent, prerequisites, collisions, retirement, documentation adaptation, the real
  diff, the checks, and cleaning the stage last.

### Router and gates (11)

- One router skill per setup domain. It discovers, gathers evidence, writes an explicit plan, waits
  for authorization, invokes the mechanical surfaces, and escalates. It restates no chapter's rules.
- A program decides facts: a path resolves, a schema matches, a record covers, a link resolves, a
  budget holds. A person or an agent decides intent, semantics, selection, and unattributed
  ownership.
- A check states what it holds. A declared build target is proved by invoking the compiler, and a
  search over the source enforces a narrower policy without standing in for it.

### Prior art (12) and reference implementations (13)

- Every claim in `12` carries a dated primary source, and an inference is labeled as one.
- `13` maps concept to name for two tools, marks what is unimplemented, and is never normative. Both
  are x86_64 Linux tools, which is a fact about them rather than a rule.

### Worked examples (90, 91) and the checklist (99)

- `90` is one project across the four layers. `91` is one tool that writes into other projects,
  through greenfield, an update, an unattributed collision, a failure part way, a rerun, and cleanup.
- `99` walks every layer before shipping and names the owner of each section. It restates no rule.

## Maintenance notes

- `02` is reserved for the authorization boundary and lands from another project's plan, and `07`
  and `14` for acquisition and the target model. The index in `README.md` names only chapters that
  exist.
- Where a chapter would restate a rule a `cli-design/` chapter owns, it links that owner.
- A rule that holds for one runtime belongs in that runtime's adapter under `tools/`, dated. `01`
  links the instruction-file adapters, and `03` and `04` link all four skills adapters.
