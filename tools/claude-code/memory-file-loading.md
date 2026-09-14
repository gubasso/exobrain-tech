# Claude Code instruction-file adapter

Verified 2026-09-14 against <https://code.claude.com/docs/en/memory>.

Claude Code's instruction-file facts, against the portable rules in
[agent-integration/01 — Instruction files](../../programming/agent-integration/01-instruction-files.md).

## Which files Claude Code reads

`CLAUDE.md`, not `AGENTS.md`. A repository that keeps its instructions in `AGENTS.md` for other
runtimes writes a `CLAUDE.md` that imports it with `@AGENTS.md`, or symlinks the one to the other
where no Claude-specific content is needed.

| Scope          | Location                                                                         |
| -------------- | -------------------------------------------------------------------------------- |
| Managed policy | `/etc/claude-code/CLAUDE.md` on Linux, or the `claudeMd` key in managed settings |
| User           | `~/.claude/CLAUDE.md`                                                            |
| Project        | `./CLAUDE.md` or `./.claude/CLAUDE.md`                                           |
| Local          | `./CLAUDE.local.md`, ignored by version control                                  |

They concatenate rather than override, in that order, so a project instruction is read after a user
one. A managed policy file cannot be excluded.

## What loads eagerly and what loads lazily

The working directory and every directory above it are read at launch. A `CLAUDE.md` in a
subdirectory below the working directory is discovered but held back, and enters context when Claude
reads a file in that subdirectory. Inside one directory, `CLAUDE.local.md` is appended after
`CLAUDE.md`.

Ordering runs from the filesystem root down, so the file closest to where the session started is
read last.

`--add-dir` adds a directory's files without its instructions.
`CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` loads those too.

## Imports are eager wherever they sit

`@path/to/file` expands into context alongside the file that names it, at the moment that file
loads. A root import therefore saves nothing; a nested file's import loads only when the nested file
does.

A relative path resolves against the importing file, never the working directory, so a nested
`docs/x/CLAUDE.md` names its sibling as `@AGENTS.md`. Imports recurse to four hops. An `@` inside a
code span or a fenced block is text rather than an import.

An import in a project file that resolves outside the working directory is external, and the first
one prompts for approval. A declined prompt disables external imports for that project and does not
return.

## Path-scoped rules are the lazy mechanism

`.claude/rules/*.md` is discovered recursively. A rule with no `paths` frontmatter loads at launch,
with the same weight as `.claude/CLAUDE.md`. A rule carrying `paths` globs loads only when Claude
reads a file that matches one.

```markdown
---
paths:
  - "src/api/**/*.ts"
---
```

`~/.claude/rules/` holds the same shape for every project on the machine, and loads before the
project's own.

## Size

The documented target is under 200 lines per file. A file over 4 MiB is skipped whole. Splitting
into imports organizes the text without reducing what a session pays for; scoping a rule to a path
is what reduces it.

## Survival across compaction

A project-root `CLAUDE.md` is re-read from disk and re-injected after compaction. A nested file and
a path-scoped rule return only when Claude next reads a file they apply to.

## See also

- [agent-integration/01 — Instruction files](../../programming/agent-integration/01-instruction-files.md) —
  what the root file carries whatever the vendor.
- [skills](./skills.md) — the same vendor's skill facts, and where a procedure goes instead of here.
