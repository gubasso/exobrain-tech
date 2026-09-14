# 006 — Split the Vendor Facts by Vendor

## Goal

A reader who needs one agent runtime's facts opens that runtime's shelf and finds them dated and
sourced. A reader who needs a rule that holds for any runtime never opens a vendor shelf at all.

## Example

One vendor directory holds four runtimes' facts, another product's contracts, and one person's
cache-clearing command.

```console
$ ls tools/claude-code tools/opencode.md
tools/opencode.md

tools/claude-code:
AGENTS.md  invocation-cheatsheet.md  memory-file-loading.md  orchestration  skill-authoring
```

`skill-authoring/skill-spec.md` carries a Claude field table and a Codex comparison in one file.
`skill-style.md` documents a dotfiles checkout. `orchestration/` and `invocation-cheatsheet.md`
document `cog`'s skills and its helper binary. `tools/opencode.md` is two lines of one home
directory.

## Core

One shelf per vendor, one adapter per vendor per subject. An adapter carries a verified date and the
vendor's own documentation as its source. It states the discovery roots and their precedence, the
fields the vendor reads beyond the specification's, activation and reload behavior, path-derived
identity where the vendor derives it, and the permission model where the vendor has one. It never
states a rule the portable baseline lacks.

## In scope

- `tools/claude-code/`, `tools/codex/`, `tools/gemini-cli/`, and `tools/opencode/`, each an index of
  dated adapters.
- The split of `skill-spec.md` into the Claude Code and Codex skills adapters.
- The split of `memory-file-loading.md`: the Claude Code half stays, the Codex half becomes an
  adapter, and the design consequence moves to the chapter that owns it.
- The adapter links from `01`, `03`, and `04`.
- The retirement of what is personal, another product's, or a rule the baseline owns.

## Out of scope

- The portable skill standard, which story 007 writes from the specification and these adapters.
- Any chapter another repository's plan lands on the shelf.
- Any file under another product's source tree.

## Governed by

- `_docs/specs/SPEC-knowledge-base-boundary.md` — what belongs in a bucket and what does not.
- `_docs/specs/SPEC-docs-format.md` — the register and the size budgets.
- `AGENTS.md` — the product boundary, and the rule that each fact has one source of truth.

## Amends

- `tools/claude-code/`, reduced to Claude Code's own dated facts.
- `tools/codex/`, `tools/gemini-cli/`, `tools/opencode/`, created.
- `programming/agent-integration/01-instruction-files.md`, `03-skills.md`,
  `04-skill-distribution.md` — the adapter links and the design consequence.

## Acceptance

- Every file under the four vendor directories is a dated vendor fact or an index of them.
- No file there names `agent-helper`, `dotfiles`, `skill-builder`, `codex-conventions.md`, or
  `skill-script-extraction.md`.
- Every vendor-specific claim in the shelf is a link to an adapter, and no adapter states a rule the
  baseline does not.
- `just check` passes, and `lychee` resolves every vendor URL the adapters carry.

## Tasks

- [ ] Split the two Claude Code pages that hold more than one vendor's facts.
- [ ] Write the Gemini CLI and OpenCode adapters from each vendor's own documentation.
- [ ] Retire what is personal or another product's, with every link to it.
- [ ] Index each vendor directory and link every adapter from the chapter it adapts.

## Rabbit holes

- Writing the baseline into an adapter, because the vendor page states it too — escape: a rule that
  holds for any runtime goes to the shelf chapter, and the adapter states the vendor's version.
- Reconstructing a retired page's content somewhere else — escape: the page is another product's,
  and that product documents it.

## Done when

Each vendor shelf holds that vendor's facts as dated adapters and nothing else, and the shelf
chapters link every adapter they have.

## Revisions

None.
