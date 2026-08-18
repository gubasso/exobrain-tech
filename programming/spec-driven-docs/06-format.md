# 06 — Format

Markdown is the house format. This chapter keeps the constructs that carry structure, drops the
inline emphasis that carries decoration, sets the register every document is written in, and owns the
size budget the whole framework is measured against.

## Budgets

| Artifact                         | Budget                                               |
| -------------------------------- | ---------------------------------------------------- |
| Root author-instructions file    | 100 lines                                            |
| Subtree author-instructions file | 150 lines                                            |
| Spec                             | 300 lines excluding the TOC; TOC generated above 100 |
| Requirement statement            | one sentence                                         |
| Decision record                  | 350 words                                            |
| Chapter                          | 200 lines                                            |
| Reference depth                  | one level from the entry document                    |

- A document MUST stay within the budget for its artifact class.

The numbers are not arbitrary and they are not measured constants either. Retrieval accuracy falls as
input length grows, non-uniformly and on tasks as simple as finding one sentence, so a cap is the
crude instrument that keeps documents in the range where retrieval is reliable. Where a first-party
figure exists it anchors the class: an instruction file read in full is held under a few hundred
lines, and a file over a hundred lines is assumed to be read in part.

When a chapter wants more room, the excess is a requirement in a spec or a decision record, not a
longer chapter.

## Structural markdown is kept

- Headings, which carry the document's shape and are what retrieval keys on.
- Ordered and unordered lists, for genuinely parallel items or sequences.
- Tables, for comparisons and exact mappings.
- Fenced code blocks, always with a language; use `text` when none applies.
- Inline code, for identifiers, paths, flags, commands, keywords, and status values.
- Links.
- Blockquotes, for quoted material only.

## Decorative markdown is dropped

- A document MUST mark identifiers with inline code, normativity with RFC 2119 keywords, and
  structure with headings.
- A document MUST NOT contain bold or italic text.

Emphasis was doing three jobs and each has a better home. An identifier becomes inline code. A binding
statement becomes a MUST. A phrase tempting to bold as a mini-heading becomes a heading, and if that
produces too many headings the section was carrying more than one idea.

The rule is zero, not sparingly. A density budget invites a judgment call on every line and loses it
slowly; zero is checkable, and it is the only version applied consistently.

Do not justify this with token savings. Emphasis markers are one to two percent of a corpus. The
dominant costs are restating a fact another document owns and loading documents the work does not
need, and [00 — Model](./00-model.md) and [05 — Agent Context](./05-agent-context.md) fix those.

## Register

The framework's documents are read by someone about to act, not by someone studying.

- A document MUST put its most consequential content first.
- A document MUST spend prose only on a decision, a hazard, or a non-obvious constraint.
- A command MUST appear in a fence, not inside a sentence.
- A document MUST use numbered steps only where order is load-bearing.

Two habits produce documents that fail this, and both look like helpfulness.

The first is narration: a sentence introducing the step, the command, then a sentence describing what
just happened. The introduction restates the heading and the description restates the command's own
output. Neither survives deletion.

The second is inlined rationale: the concept behind a rule explained at the rule. It reads as thorough
and it is a placement failure. The explanation belongs to whoever owns that fact, and this document
links it.

The test: strip the prose out and read what is left. If the remaining rules are still executable, the
prose was earning its place. If the document no longer makes sense, the rules were incomplete and the
prose was carrying them.

A rule that reads as arbitrary is the case where prose is required. Two sentences saying why a
counterintuitive rule exists prevent the next author from deleting it as pointless.

## One default, not a survey

- A document MUST recommend one option and MAY name one escape hatch.
- A document MUST NOT compare alternatives it does not recommend.

A list of four approaches leaves the reader to choose and leaves an agent to pick arbitrarily. Name
the default, name the one case that justifies departing from it, and stop. The comparison that
produced the default belongs in the decision record.

## No document narrates its own history

- A document MUST state what is true now.
- A document MUST NOT record what it used to say, what it replaces, or why something is absent.

No `formerly`, no `used to`, no `this replaces`, no `inherited from`, no note explaining an absence. A
reader arriving today has no idea what yesterday looked like, and telling them costs a sentence they
cannot act on.

The test: delete the clause. If nothing a reader can act on disappeared, it was archaeology.

Decision records are the exemption, because holding history is their entire job. Git holds the rest,
and holds it better. This binds the change that removes something too: a deletion leaves no trace in
prose, only in the log.

## Consistent terminology

- A project MUST use one term for one concept across every document.

Mixing spec, specification, contract, and requirements doc for one artifact weakens retrieval for all
four. Pick one, put it in the glossary, and use it.

## Mechanics

- One `#` per file. `##` for sections, `###` only for genuinely parallel sub-parts.
- An unheaded opening states what the file owns, in two or three sentences.
- Hand-wrap at 100 columns.
- Relative links carry an explicit prefix: `./name.md`, `../dir/name.md`, or a multi-segment path.
- `<angle>` placeholders stand for project-specific values.
- Cite a source inline at the claim it supports; collect them in a `## Sources` section when a
  document carries three or more.
- No `## See also` section; an index and inline links carry navigation.

## Sources

- Chroma, on degradation with input length and on position sensitivity:
  <https://www.trychroma.com/research/context-rot>
- He et al., on prompt format changing task accuracy substantially for weaker models:
  <https://arxiv.org/abs/2411.10541>
- Anthropic, on conciseness, consistent terminology, and avoiding a survey of options:
  <https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices>
