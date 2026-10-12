# 19 — What the CLI Owes a Setup Skill

A setup skill keeps the judgment: the intent, the questions to the operator, and the decisions a
merge needs. Every step a program can decide belongs to the tool's own command-line interface, so
the skill reads a result instead of following a procedure, and a test holds the step. This chapter
lists what that interface provides, in generic terms. The skill that reads it is
[18 — Setup skill skeleton](./18-setup-skill-skeleton.md).

The test for each item is the same: a step two agents could carry out differently from the same
evidence is a step the tool computes.

## Read the target

- A classification field in the status report. One field names what the target is: a first landing,
  a migration, a target whose documents or configuration have no recognized home and need the
  operator's decision, an upgrade, drift, current, or unreadable metadata. The skill's gate reads
  that one field. The rule that computes it is the one every landing verb applies, so the gate and
  the verbs can never disagree. A record newer than the binary reads no classification at all,
  because an older binary cannot judge it and "upgrade" would say it can.
- A diagnostic, not a report, for a record that does not parse. Absence and breakage are different
  facts, and a report that called a broken record absent invites a landing over it.
- The state an older release left on the host, by exact path, with whether it still binds a run. No
  verb of the current release reads it, so its removal is the operator's, after the older release
  finishes or abandons what it started.
- A status for the tool's own pin that reports a recorded host decision as its own state, beside "no
  manager" and "not wired", so a project that chose a host install reads as decided.

## Stage and compare

- A stage that defaults to the recorded values. Over an existing instance, the stage takes the
  recorded profile, scratch location, and every other recorded choice, and it refuses a different
  profile, because moving one is a migration somebody asks for by name.
- A compare verb that reports, per staged destination, three things.
  - State: equal, absent, differs, or retired, where retired is a destination the record names and
    the release no longer lands.
  - Ownership: managed, a marked region in a file the project owns, or adopted.
  - Provenance, for a destination that differs or retires: attributed only where the record vouches
    for the bytes on disk and the version-control history touches the path, reduced where one of the
    two holds, unknown where neither does.
- Whether a replace name resolves the destination: true only where the landing verb that follows
  accepts the path in its replace option and can then write it, judged by the check that verb
  applies to each name. The skill asks only about those, and reports any other collision as a
  refusal no option passes.
- The record half of the provenance is the same check the landing runs before it refuses unvouched
  bytes, so the comparison and the refusal name the same destinations. Where version control is
  absent, or the target is no repository, the history half reads as missing and the report says why.
- After a landing, the same compare is the proof: every managed and marked destination reads equal.

## Land

- A landing that reads its choices from the stage receipt. The operator states each choice once, at
  the stage, and the landing takes them from the receipt rather than from repeated flags. The
  landing still renders every byte again from the binary's own sources and reads no staged byte, so
  a stage is never a cache. It refuses a receipt another release wrote, and a receipt for another
  target.
- A named replace that saves a copy first. A conflict the operator names is saved, then overwritten.
  Every edit it does not name refuses atomically, before the first write, so no tool-owned file
  somebody edited is overwritten without an answer. The copy lives outside the stage, under the
  project's scratch location or the tool's state directory, and a copy already there with other
  bytes refuses rather than being overwritten.
- A step that removes or replaces state no version-control history holds, such as a forge's settings
  or a branch, saves that state before it acts, migration or not, and refuses where it cannot read
  it. A copy in a file is the only second copy that state has.
- An incremental landing, which the operator names, beside a corpus that stays in place: no archive
  runs, and each budget finding of the corpus is recorded as debt rather than refused. A migration
  target defaults to the archive.
- A dry run that lists every citation of a rule identifier the release no longer defines, by file
  and line, before the first write. Re-pointing a citation stays the author's, because the tool
  cannot know which rule the author meant.

## Environment and settings

- An environment verb. It reports the loader the repository records, read from the repository's own
  files rather than the host, and the state of each environment item the tool needs, such as a
  scratch-location variable and its ignore entry. An apply writes one approved item through that
  loader and nothing else.
- A lint report and a lint proof, for a tool that delivers a linter configuration. The report names
  each configuration that would replace the delivered one or drop a rule the tool depends on, with
  the rule it breaks and the kind of edit it needs, and reports "unknown" for a configuration whose
  grammar the binary does not read. The proof runs the delivered hook over a scratch file with a
  broken link, then a valid one, and removes the file, because a rule that silently does not load is
  the failure no other check sees.

## What stays with the skill

- The intent of the request, and the questions that only the operator can answer.
- Every decision the compare verb hands back as reduced or unknown provenance.
- The mapping of a migration: which sources group together, and what each becomes.
- The edit to a configuration the project owns, proposed and approved one change at a time.

A run keeps no state of its own. The repository, the stage receipt, and version control are the
records, and a later run reads the state from the status report rather than from a log of an earlier
run. A migration is the one exception, because it spans many runs, and
[20 — Migrating from an archive](./20-migrating-from-an-archive.md) states its workspace and
registry.

## See also

- [08 — Candidate artifacts and receipts](./08-candidate-artifacts-and-receipts.md) — the record the
  compare verb's record half reads.
- [09 — Disposable staging and direct landing](./09-disposable-staging-and-direct-landing.md) — the
  pipeline these verbs implement.
- [11 — Router and gates](./11-router-and-gates.md) — which checks belong to a program.
- [18 — Setup skill skeleton](./18-setup-skill-skeleton.md) — the skill that reads these verbs.
