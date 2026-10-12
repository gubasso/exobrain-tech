# 16 — Skill Format

[03 — Skills](./03-skills.md) states what a skill is and what each part of the package is for. This
chapter states the format a package follows once a project ships more than one skill, or one skill
that runs more than one task: how the package is laid out, how the manual loads its references, the
order of its sections, and how each instruction is written. The size of each file is
[17 — Skill size budget](./17-skill-size-budget.md). The evidence behind every convention is in
[21 — Skill format evidence](./21-skill-format-evidence.md) and
[22 — Skill format research](./22-skill-format-research.md).

## The package

1. A package holds `SKILL.md` and `references/`, and nothing else. The format also allows `scripts/`
   and `assets/`. A project whose deterministic steps are verbs of its own command-line tool ships
   no script, because a verb is what a test can hold. A package carries no README, no changelog, and
   no install guide, which every vendor guide keeps out of a skill.
2. `SKILL.md` is the control plane. It holds the workflow, the decisions, and the invariants that
   bind every run. Knowledge that only some runs need goes to a reference file, because a loaded
   body stays in context on every later turn and a reference loads only when the agent reads it.
3. `SKILL.md` links every reference file directly, by a path relative to the skill root, and no
   reference file links another one. An agent tends to preview a file it reached through another
   file rather than read it whole, so the graph stays one level deep.
4. Every link carries its activation condition, in the form "If the run includes `land`, read
   `references/land.md`." A bare list of reference files tells the agent nothing about which one
   applies.
5. Each fact lives in one file, either `SKILL.md` or one reference file. Two copies of one rule
   disagree after the first edit, and an agent that holds both can follow either.
6. Content two skills share is authored once and copied into each package at build or install time.
   A package never reads outside its own directory, because every tool that installs skills installs
   one whole directory per skill. The package closure is an allowlist, and a file name that two
   sources claim is an error.

## One file per task

A skill that serves several tasks keeps one reference file per task. The gate in `SKILL.md` picks
the tasks for the run, and the agent reads each picked task file into the context it already has. No
new agent starts, so nothing loads twice.

- A task file holds the pre-flight, the execution, and the post-flight of one task, and it links no
  other task file.
- Where one task needs another, `SKILL.md` states that composition, because it is the one file that
  links task files.
- Where several tasks share one procedure, that procedure is one more reference file, linked with
  the condition that names the tasks that need it.
- A task file holds the agent's procedure and the questions it asks. The method stays in the tool's
  own documentation, and the task file routes to it.
- A deterministic step stays a verb of the tool. A task file never computes what the tool can
  compute, which [19 — What the CLI owes a setup skill](./19-what-the-cli-owes-a-setup-skill.md)
  states for a setup skill.
- A branch small enough to stay in `SKILL.md` stays there. The split exists for the runs that do not
  need the branch.

## The section order

The sections of `SKILL.md` follow the stages of a run, in the order a run takes them.

```text
# <skill name>
<opening paragraph: what the skill does, and how a task is picked>

## Pre-flight     the checks that fail fast, then what the gate reads
## General rules  what binds every task, as standing instructions
## Gate           the table that picks the tasks, and its exits
## Tasks          each task link with its condition, and the composition
## Post-flight    what every run ends with, and the report template
```

Models favor the instructions they read first, and a host that compacts a session keeps only the
head of each skill, so what every run needs comes first, in the order the run needs it. A skill
whose task routing precedes its pre-flight picks a task before it knows the state of the target.
[18 — Setup skill skeleton](./18-setup-skill-skeleton.md) fills each section for a skill that sets
up a tool.

## Writing the instructions

1. Write each instruction in the imperative, one action per step, concrete enough to check. Name the
   exact command rather than an outcome: "run `tool verify --target .`", never "make sure it works".
2. Use a numbered list where order matters, a bulleted list for unordered rules, and a table for a
   closed choice. No format wins on every model, so the structure follows the content rather than a
   preference.
3. Write prose only as a reason of one clause, and only where the reason changes what the agent does
   at an edge case. A reason helps a model generalize a rule. Narration and background the model
   already knows cost context on every turn and change nothing.
4. Put the rules that matter most first. Write a rule that holds for the whole task as a standing
   instruction, such as "take every path from the status report", rather than as step 2 of one
   procedure, because a host does not read the skill again on later turns.
5. Give one default and the exception that overrides it, never a menu of equal options. A menu with
   no default makes the agent pick one at random, or ask a question the skill could have answered.
6. Write no capitals for emphasis. Newer models over-trigger on aggressive wording written for older
   ones. A rule that must hold on every run becomes a check of the tool or a test, not louder prose.
7. Use one term for one concept across the whole package, and no wording that is true only at the
   time of writing. "Stage" in the manual and "candidate directory" in a task file read as two
   things.
8. Keep the rule count low. Apply the deletion test to every line: if removing it does not change
   what the agent does, remove it. Compliance falls as the number of instructions rises, and focused
   skills outperform exhaustive ones.

## The description

The `description` states what the skill does, then when to use it, with the key terms first, because
a host that shortens the listing keeps the head. It stays in the third person, within 1,024
characters, and it carries no angle brackets. [03 — Skills](./03-skills.md) states why the
description is the whole trigger surface.

```yaml
description: >-
  Lands, upgrades, repairs, and tunes the tool's instance in a project, and provisions the
  tool itself. Use when asked to set up the tool, to migrate a project onto it, to upgrade
  or repair its files, or to pin the tool for a project.
```

## Holding the format

The conventions that a program can decide are tests over every package the project ships, so a
change that breaks one fails before it merges.

| Convention                           | The test                                                                              |
| ------------------------------------ | ------------------------------------------------------------------------------------- |
| The package holds only its two parts | every file under a skill directory is `SKILL.md` or a reference                       |
| Every reference is linked, one deep  | `SKILL.md` names each reference, and no reference names another                       |
| Every link carries its condition     | each sentence that names a reference opens with a condition word                      |
| A shared name has one source         | composing a package fails on a name two sources claim                                 |
| The sections follow the stage order  | the `##` headings of `SKILL.md` equal the order above                                 |
| No capitals for emphasis             | every capitals word outside frontmatter, code, and link targets is an allowed acronym |
| The description holds its contract   | its length, its voice, and no angle brackets                                          |
| The size budget                      | per [17](./17-skill-size-budget.md)                                                   |

The rest, one action per step, one fact in one file, the deletion test, is a reviewer's call, and
the project states it as such rather than as a gate.

## See also

- [03 — Skills](./03-skills.md) — what a skill is, and the description as the trigger surface.
- [04 — Skill distribution](./04-skill-distribution.md) — how a package installs, with what it
  shares copied in.
- [17 — Skill size budget](./17-skill-size-budget.md) — the caps, in characters.
- [18 — Setup skill skeleton](./18-setup-skill-skeleton.md) — the section order, filled for a setup
  skill.
- [21 — Skill format evidence](./21-skill-format-evidence.md) — the sources each convention rests
  on.
