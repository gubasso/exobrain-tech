# 18 — Setup Skill Skeleton

A setup skill sets up the tool it ships with in a project: it provisions the tool, lands the tool's
files, migrates a project that documents or configures itself some other way, upgrades and repairs
what landed, and tunes the tool's settings. This chapter fills the section order of
[16 — Skill format](./16-skill-format.md) for that skill. The verbs the skill reads instead of
following a
procedure are [19 — What the CLI owes a setup skill](./19-what-the-cli-owes-a-setup-skill.md), and
the migration is [20 — Migrating from an archive](./20-migrating-from-an-archive.md).

## The scope principle

A tool intervenes in a project's workflow as little as it can. The principle binds every task, every
landed file, and every question the skill asks.

1. A skill holds only what its tool owns: setting up the tool, the files and settings the tool
   lands, and the tool's own process.
2. Universal development workflow stays the project's and the agent's own: starting a branch, the
   working-copy layout, integrating a branch onto the trunk, and pruning after a merge. Agents and
   people do this on every project, with or without the tool.
3. The tool may ship helpers for that workflow. A helper is a verb the tool and its documentation
   describe, and no task, landed prompt, question, or gate requires it.
4. A landing asks only what the tool needs to render its own files, and lands nothing that tells a
   project how to work beyond what the tool's own process requires.
5. A landed rule or a question that exists only to shape the project's workflow leaves, or becomes
   an opt-in the operator asks for by name.

## The shape of a run

```text
GENERAL PRE-FLIGHT   stop checks that fail fast, then observe checks for the gate
GENERAL EXECUTION    general rules, then the gate: state x request -> ordered tasks
  each task          its pre-flight, its execution, its post-flight
GENERAL POST-FLIGHT  env items, the checks, cleaning the stage, the report
```

Every check lives at exactly one level. The general pre-flight owns the host, the installed skill,
and the target as a whole. A task's pre-flight owns only what that task needs before its plan. A
task's post-flight owns only that task's proofs. The general post-flight owns what every run ends
with.

## Pre-flight

Two kinds of check run here. A stop check finds a fact no task of the skill can change, and it ends
the run with the evidence and the operator's step. An observe check finds a fact that classifies the
target, and the gate picks tasks from it.

- A gate both of the tool's skills share holds the host stops: a failed hard probe of the tool's
  doctor, skills and a binary from different builds, and no repository, or not the one the request
  means. It hands back the status report, or the diagnostic, and judges neither.
- The skill holds the target stops: a record or a declaration the tool cannot read. Nothing is
  written, and an unreadable record never counts as an absent one, because a skill that lands over
  it destroys what it could not read.
- Whether the tool is on PATH is the one fact the skill observes by hand, because no verb of the
  tool runs without it. A missing binary, a record newer than the binary, and a repository that
  records no way to obtain the tool all route to the `provision` task.
- Everything else the gate reads comes from the tool's status report: the classification, the paths,
  and the soft probes, which become constraints on the plan.

## Provision

A setup skill installs the tool it serves where the host lacks it, is older than the repository's
record, or the repository records no way to obtain it. It asks two questions, each on its own,
because a project can pin through a manager and still load the environment by hand.

1. How the project obtains the tool. The default is the manager the repository already carries, and
   the options are a flake devshell, mise, asdf, another manager the tool serves, and a native host
   install. Each option states its cost, and a host install records no version in a file.
2. How the development environment loads. The options are direnv, another loader, and none. With
   direnv, the loader line and the line that keeps the pin fresh land in `.envrc`.

Every answer lands in a file the contributors share: the manager file and its lock, and `.envrc`. A
host install is recorded as an answer in the tool's own configuration, with the lowest version it
accepts, so the tool's status reads it as a decision rather than a gap and no later run asks again.
A decline is recorded the same way. Each install runs as a step the operator approves, and the
post-flight confirms the tool reports the pinned version before the pre-flight runs again.

## General rules

The general rules bind every task, so they hold nothing a task alone needs. A skill of this shape
needs about seven.

1. Route, never restate: find an answer in the tool's documentation index, a rule in its rule
   reader, and never copy a chapter into the skill.
2. Take every path from the paths section of the status report, never by hand.
3. Decide from a verb's machine-readable output, never from its human text.
4. Leave the tool's managed bytes to the tool, marked regions included.
5. Keep one instance of the tool per repository, at its root, and the skills at user scope, never
   inside the project.
6. The repository records how it obtains the tool. `provision` proposes the method, and the operator
   picks it.
7. A run keeps no state of its own. The repository, the stage receipt, and version control are the
   records, and a later run reads the state from the status report. A migration is the one
   exception, because it spans many runs.

## The gate

The gate crosses two inputs. The state comes from the tool's classification, never from the request.
The intent and the orientation come from the request: the task the operator wants, and any
instruction, such as a scratch location or two records to keep together. The state decides which
tasks are possible, and the intent picks among them. A cell is an ordered list of tasks that share
one plan and one approval, and a run carries as many tasks as its cell lists.

| State          | set up  | upgrade | fix     | settings           |
| -------------- | ------- | ------- | ------- | ------------------ |
| setup          | land    | ask     | ask     | land               |
| migration      | migrate | ask     | ask     | migrate            |
| needs-decision | ask     | ask     | ask     | ask                |
| upgrade        | upgrade | upgrade | upgrade | upgrade, then tune |
| drift          | repair  | repair  | repair  | repair, then tune  |
| current        | done    | done    | done    | tune               |

- Ask: state the evidence and the options, wait, and read the gate again with no new pre-flight.
- Hand off: a failure inside content another skill owns goes to that skill.
- Done: report from observation.

The gate proposes and the operator confirms. It presents the state, the intent it read, and the
tasks, then waits, and a redirect reads the gate again. An item of the request that fits becomes a
constraint on the plan. An item that does not fit the state gets the evidence and the task that
fits, then a question, and an item that breaks a rule gets that rule's identifier, then a question.
Before each task the skill names it in one line and waits, and it prints one progress line at each
move between a task's pre-flight, execution, and post-flight.

The plan is approved after the gate picks the tasks and their pre-flights have run, so it names each
task, the verbs it runs, and the files it writes, and it covers every execution and post-flight in
the cell.

## Tasks

One reference file per task, linked from `SKILL.md` with its condition, each with its own
pre-flight, execution, and post-flight. The tasks that land files share one pipeline file, stage,
compare, prepare, land, and prove, so each task file holds only what differs from it. `SKILL.md`
states which tasks compose: a first landing runs the settings task's pre-flight before it stages, so
a path the operator reserves joins the stage and a lint edit runs while preparing.

A repair restores the tool's own bytes through that pipeline, at the same version. A failure inside
content the project owns goes to the skill that owns that content, or, where none does, the setup
skill edits it as a step the operator approves, with the project's answer.

## Post-flight

It runs once, after the last task, in this order, so every check judges the final tree and the
report describes it.

1. Env: apply the environment items the operator approved in the plan, through the tool's
   environment verb.
2. Check: the tool's own judge, then the repository's full gate suite.
3. Clean: remove the stage, and nothing else, on the operator's yes. Saved copies, snapshots, and a
   migration workspace are backups kept outside it, and the report lists each path.
4. Report, from the fixed template.

A failed check keeps the stage, reports, and routes: a failure inside content goes to the skill that
owns it, and drift goes to a new run whose gate picks `repair`. The post-flight never repairs.

```text
Result: <the exact line of the tool's judge, or the first failing line>
Tasks:
  - <task>: <files written or removed>; proof: <the task's post-flight evidence>
Answers recorded:
  - <question>: <answer>
Git actions:
  - <action>: <run, under the term the request named | printed for the operator>
Saved before changing:
  - <path>: <what it holds>
Still the operator's:
  - <exact command>: <what it changes>
Stage: <cleaned | kept at <path>, because <reason>>
Next: <one action>
```

Result, Tasks, Answers recorded, Still the operator's, Stage, and Next are the core lines every
setup skill shares. Git actions, forge actions, and Saved before changing print only where the run
has an entry for them, and a tool may add one evidence line of its own. Every answer the operator
gave goes in, so a declined offer reads as asked and answered.

## See also

- [11 — Router and gates](./11-router-and-gates.md) — the router this skeleton refines.
- [16 — Skill format](./16-skill-format.md) — the section order and the writing conventions.
- [19 — What the CLI owes a setup skill](./19-what-the-cli-owes-a-setup-skill.md) — the verbs each
  section reads.
- [20 — Migrating from an archive](./20-migrating-from-an-archive.md) — the migrate task in full.
