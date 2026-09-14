# 04 — Skill Distribution

A skill package is discovered by an agent because it sits in a root that agent scans. Where those
roots are is a vendor fact; how a project keeps one package reachable from several of them is not.

## The two scopes

Every runtime that reads skills distinguishes two scopes, whatever it calls them.

| Scope   | What it holds                                      | Who owns the lifetime                               |
| ------- | -------------------------------------------------- | --------------------------------------------------- |
| User    | Skills that follow one person across every project | The person, through whatever installs them          |
| Project | Skills that operate this project's own land        | The project, in its own tree, under version control |

A project-scoped package is committed with the project it drives, so a clone carries it and a
reviewer sees it change. A user-scoped package is not the project's business, and a project that
writes into a user's scope has taken a decision that was not its own.

## One source, many agents

Two runtimes scan different roots for the same package format. Keeping one copy per runtime means
keeping several copies in step, which nobody does for long.

The durable arrangement is one canonical directory plus a link from each root that scans for it.

```bash
ln -s "$HOME/code/skills/dispatch-messages" "$HOME/.<runtime-a>/skills/dispatch-messages"
ln -s "$HOME/code/skills/dispatch-messages" ".<runtime-b>/skills/dispatch-messages"
```

A runtime that follows a symbolic link while scanning serves the package from the canonical
directory, and the canonical directory is the only place an edit lands. Whether a given runtime
follows links is a vendor fact, dated in that vendor's adapter.

## One name, one owner

A skill's identity is its name, and an agent resolves a name across scopes. The same package
installed under two scopes makes two entries under one name, and which entry answers is decided by
the runtime's precedence rather than by the project. Install once, in the scope that owns the
skill's lifetime.

## What an installer owes

An installer that writes a package into a user's home carries obligations the package itself does
not:

- It writes only where it declared it would write, and it says so before it writes.
- It removes what it wrote, and nothing beside it. A package the user edited is the user's, and an
  uninstall that discards it destroys work the installer never owned.
- It reports what it did, in a form a later run can read back, so an install is a fact rather than a
  memory.

## The receipt makes those obligations keepable

A skill installer is a tool writing files into somebody else's tree, so it carries the same record
the landing chapters describe. The shape below is that record, narrowed to skills.

One package is authored once. What reaches each runtime's root is a materialization of it: the same
content, at a path that runtime scans. The authored copy is what an edit lands in, and a fix reaches
every runtime because there is one place to fix.

One installed skill name has one owner. A second installer writing a package under a name the record
already attributes is a collision, and it is reported rather than resolved, for the reason
[08 — Candidate artifacts and receipts](./08-candidate-artifacts-and-receipts.md) gives.

An upgrade and a removal both read the record. An upgrade replaces what the record attributes and
leaves everything else. A removal takes what the record attributes and stops. A file the record does
not claim stays, whoever put it there and however much it looks like something the installer would
have written.

Each file is written whole, so a runtime scanning mid-install sees the old package or the new one
rather than a half-written `SKILL.md`. The record is written last, which leaves a failed install as
files the record does not yet claim. Rerunning is the fix, and it is safe: the same package is
computed again and the files already correct are already correct.

Where those roots are is a vendor fact. Each runtime's adapter carries its own, dated, and the
adapters are linked below. This chapter states what an installer does with a root, never which ones
exist.

## See also

- [00 — The four layers](./00-model.md) — why the playbook layer has a distribution question.
- [03 — Skills](./03-skills.md) — the package this chapter distributes.

## The adapters

Where each runtime scans, in what order it resolves a name, and whether it reads a root another
runtime owns. Dated, from each runtime's own documentation. No adapter states a rule this chapter
lacks, and this chapter derives nothing from what the current set happens to have in common.

- [tools/claude-code — skills](../../tools/claude-code/skills.md)
- [tools/codex — skills](../../tools/codex/skills.md)
- [tools/gemini-cli — skills](../../tools/gemini-cli/skills.md)
- [tools/opencode — skills](../../tools/opencode/skills.md)
