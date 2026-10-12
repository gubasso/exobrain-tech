# 20 — Migrating from an Archive

A setup skill meets two kinds of predecessor: a document corpus written under another convention,
and another tool's configuration that does the same job. Both migrate the same way. The predecessor
moves into an archive, a clean first landing replaces it, and the project is rebuilt from the
archive one group at a time, with a registry recording every step. This chapter states that
procedure and why each step holds. The skill that runs it is
[18 — Setup skill skeleton](./18-setup-skill-skeleton.md), and the verbs it reads are
[19 — What the CLI owes a setup skill](./19-what-the-cli-owes-a-setup-skill.md).

## Why archive first

Rewriting a predecessor in place keeps two conventions alive until the last file converts, and a
migration that stops halfway leaves both. Archiving first retires the predecessor in one approved
step, lands the new convention clean, and turns the rest of the work into rewriting from a read-only
source. Nothing is overwritten, because nothing new lands where an old file still sits.

The operator can name a narrower scope instead: an incremental landing keeps the corpus where it is,
skips the archive, the map, the loop, and the close, and records each budget finding of the corpus
as debt. It is never the default, because it leaves the old convention alive beside the new one.

## The procedure

```text
pre-flight  every predecessor file matches the trunk
1 inventory every predecessor file, with its kind
2 snapshot  what no version-control history holds          (before any destruction)
3 archive   move the files into a workspace, write the registry   (gated)
4 land      the first-landing task, through the landing pipeline
5 map       groups and fates, approved by the operator
6 loop      one group at a time: read, write, verify, mark
7 close     every source done, nothing unmapped                     (gated)
```

### Pre-flight

The archive refuses while any predecessor file differs from the trunk's copy, a file deleted off
the trunk included, and writes nothing. A commit on the branch is not enough: the base commit the
registry records is the trunk's, so it outlives the branch the migration runs on. The trunk is the
branch the remote's default names, unless the operator names another.

### Inventory

The tool lists every predecessor file with its kind, its checksum, and whether its home is
recognized. Root files the project still needs during the migration, the readme, the license, the
changelog, and the agent instruction files, stay in place: each is registered as a source and
rewritten where it sits.

### Snapshot

Before any step that destroys state, the workspace snapshots what no version-control history holds:
a forge's protections, rulesets, and settings, and a bundle of every branch a step will delete. A
destructive step refuses while its snapshot is missing. A snapshot that needs a credential the
runtime lacks reads as unknown, never as empty, and the step that depends on it waits. A snapshot
has no second copy, so the report names its path.

### Archive

The predecessor's files move into a workspace under the user's state directory, beside a registry.
The tool writes a key into the repository that marks an open migration, so every clone sees one is
open, and the status report classifies the target as a migration until it closes.

Moving is safe because every byte has three homes before it moves: version control at the base
commit, the archive with its checksum, and a rebuild verb that recreates the archive from version
control at the base commit on any clone. The rebuild previews and writes only when applied, and on a
clone with no workspace it reconstructs the registry from the commit that opened the migration.

### Land

With the predecessor archived, the target reads as a first landing beneath the open migration, and
the first-landing task lands the tool's files through the landing pipeline.

### Map

The agent reads the sources in related sets and proposes each one's fate, and the operator approves
the map before anything is written from it.

| Predecessor       | A source becomes                                                        |
| ----------------- | ----------------------------------------------------------------------- |
| A document corpus | rewritten into one destination, split into several, merged, or dropped  |
| A tool's files    | replaced by the new tool, carried into a project-owned file, or dropped |

Mapping is many to many, and a dropped source records its reason. Decision records of the previous
convention get topic tags, and the records that share a tag form one group, read into one context.
The default result is one new record for the group, whose considered options name each old decision
with its why and its how. The history stays lean, and version control and the archive keep the
original bytes. A record of the previous convention is a migration source, not a record of the new
instance: once the merged record lands, the new convention's permanence rule binds it.

### Loop

One group at a time: read the whole group from the archive, write its destinations under the
authoring rules of the skill that owns the content, run the gates over them and read each
destination against its sources, then mark the group written and verified. A discovery, a file the
inventory missed, a split the map did not name, or a deletion nobody approved, returns to the map
and is never handled inline, because a scope widened one source at a time was never approved at any
size.

### Close

The close refuses while a source is unfinished, a verified group has no destination, or a
destination does not exist. It then removes the repository key. The archive stays, and keeping or
removing it is the operator's step, by its exact path.

## The registry

One file in the workspace, written only by the tool's migration verbs. Each status change appends a
dated entry to the source's history and never rewrites an earlier one, so the registry is also the
migration log.

```yaml
base_commit: 4f1c2e0
sources:
  - id: adr-0007
    path: docs/adr/0007-use-postgres.md
    sha256: 9f2c...
    kind: decision
    tags: [storage]
    group: storage
    status: verified
    history:
      - {at: 2026-10-08T14:02Z, status: mapped}
      - {at: 2026-10-08T15:40Z, status: verified}
groups:
  - id: storage
    sources: [adr-0003, adr-0007, adr-0012]
    destinations: [docs/decisions/ADR-store-state-in-postgres.md]
```

The statuses are pending, mapped, written, verified, and dropped.

## Who owns what

- The trunk stays releasable. The repository-side work runs on a branch the operator owns, and the
  forge-side steps run after that branch merges, so the forge never describes a trunk that does not
  exist yet.
- The operator owns the branch, the working copy, and the merge. The repository holds no predecessor
  between the archive and the close, so the skill warns once, at the gate, and the archive, the
  registry, and the base commit are the fallback it provides.
- A run keeps no state of its own, with this one exception. A migration spans many runs, so its
  workspace keeps the archive, the registry, and their history, and a later run resumes at the first
  source the registry has not finished.

## See also

- [10 — Versions and migration](./10-operator-selected-versions-and-agent-guided-migration.md) — the
  evidence order and the confidence classes.
- [18 — Setup skill skeleton](./18-setup-skill-skeleton.md) — the gate that picks the migrate task.
- [19 — What the CLI owes a setup skill](./19-what-the-cli-owes-a-setup-skill.md) — the verbs each
  step reads.
