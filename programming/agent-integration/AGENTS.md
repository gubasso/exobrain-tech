---
digest-of: programming/agent-integration
last-synced: 2026-10-10
token-estimate: 1800
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
- Each layer has what it never carries. A project that owns a land ships the skills that operate
  it, under its own install command, and a skill's name has one owner.

### Instruction files (01)

- The root file is hand-authored rules and routing. Judgment has no generator. A per-directory
  digest is generated, carries frontmatter, and states no rule of its own.
- Every sentence is loaded in every session, so routing beats restating. A subtree's rules go to
  its own file, reached by a plain pointer.

### Skills (03) and security (15)

- One authored owner per kind of content: judgment in `SKILL.md`, durable knowledge in a reference,
  mechanics in a script.
- Two required fields and four optional ones, an unrecognized field ignored. The description is the
  whole trigger: what it produces, the phrasings a person types, the boundary against a sibling.
- Progressive disclosure is a budget, priced in `17`. The deletion test decides every paragraph.
- Extract any deterministic chunk past a trivial one-liner. Split judgment from mechanism, stay
  coarse, leave prompt text to the model.
- Fetched and reader-supplied content is data, never an instruction. Least tools. Validate model
  output before a shell, a path, or a parser. Show the plan before a destructive step, and require
  an explicit target. No secret anywhere. Say whether the skill contacts the network.

### Distribution (04)

- Two scopes: user, owned by the person; project, committed with the land it drives.
- One authored package, linked or copied into each runtime's root; an installer that embeds its
  packages copies them, shared content included. It writes only what it declared and removes only
  what its record claims.
- Each file is written whole and the record is written last, so a failed install leaves files the
  record does not yet claim. Rerunning is the fix.

### Evaluations (05)

- A test says the mechanism works; an eval says the agent uses it. Four suites: trigger, behavior,
  portability, adversarial. A rate over at least ten trials, graded on the outcome, against a run
  with the skill absent.

### Documentation and discovery (06)

- Project artifacts land; tool references are exposed, never landed. One always-loaded line, one
  described index with stable ids, one reader returning an exact topic, served by the running
  version.

### Candidate artifacts and receipts (08)

- A candidate is desired state for one invocation, from the executable's own sources, the resolved
  configuration, and the observed capabilities. The receipt is attributed installed state, and an
  older receipt schema is bounded record-reading.
- Three ownership classes: a generated file the tool replaces, a seeded file the project owns after
  creation, a marked region inside a project-owned file. Provenance lives in the record.
- Missing provenance reduces certainty and grants nothing. An unattributed whole file is never
  overwritten. A retired destination is reported and released, never deleted.

### Staging and landing (09)

- Five steps inside each landing task: stage, compare, prepare, land, prove. Verification and
  cleaning the stage run once, in the setup skill's post-flight.
- Production reads no staged byte, only the receipt's choices. The renders share a pure projection,
  never serialized state, which avoids a validity protocol of fingerprints and revalidation.
- Landing validates every path and collision first and writes the receipt last. The guarantee is a
  whole-file boundary under one lock, not a transaction: observed paths reported, rerun supported.
- Cleanup is its own destructive command, identifying the stage by its receipt and refusing every
  root it could mistake for one. A rule states a property; an implementation names the mechanism.

### Acquisition (07) and versions (10)

- Manager against venue, crossed into one matrix: every pair renders or is manual with a closed
  reason, one target runs one bump mechanism, and the pin's freshness loop is the tool's obligation.
- Acquisition belongs to the project's manager, and landing fetches no other version. A setup skill
  provisions its own tool, and the repository records how, a host install included.
- Evidence order: working tree, receipt, history, stage and its omissions, changelog, guidance,
  exact installed documentation, project checks, and an upstream source last and labeled.
- Four confidence classes, read as the compare verb's provenance: attributed, reduced, unknown.
  Each is less certainty, never more permission, and the unattributed refusal holds at every one.
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

### Skill format (16), size (17), and evidence (21, 22)

- A package is `SKILL.md` plus `references/`. The manual links every reference directly, each link
  with its condition, one level deep, one fact in one file, and shared content copied in.
- One reference file per task. The sections run pre-flight, general rules, gate, tasks,
  post-flight. Imperative, one action per step, a reason of one clause, one default, no capitals.
- Size counts characters: a body within 12,000, target 8,000; a reference within 32,000, listing its
  sections above 8,000. Lines are the worst proxy; the loaders' budgets, not the window, bind.

### Setup skill (18), its CLI (19), and migration (20)

- A tool intervenes as little as it can: a skill holds what the tool owns, and the branch, the
  working copy, and the merge stay the project's.
- Stops fail fast; observations feed a gate that crosses the classification with the intent,
  proposes, and waits. The skill provisions its own tool, and the repository records how.
- Every deterministic step is a verb: a classification, a compare with provenance, a landing from
  the receipt, a named replace that saves first, an environment verb, a lint proof.
- A migration archives the predecessor, snapshots what no history holds, lands clean, and rebuilds
  one group at a time under a registry. It is the one run that keeps state.

### Worked examples (90, 91) and the checklist (99)

- `90` is one project across the four layers. `91` is one tool that writes into other projects,
  through greenfield, an update, an unattributed collision, a failure part way, a rerun, and cleanup.
- `99` walks every layer before shipping and names the owner of each section. It restates no rule.

## Maintenance notes

- `02` is reserved for the authorization boundary and lands from another project's plan, and `14`
  for the target model. The index in `README.md` names only chapters that exist.
- Where a chapter would restate a rule a `cli-design/` chapter owns, it links that owner.
- A rule that holds for one runtime belongs in that runtime's adapter under `tools/`, dated. `01`
  links the instruction-file adapters, and `03` and `04` link all four skills adapters.
