# 17 — Skill Size Budget

A skill is paid for in tokens, but no author can count tokens: every host tokenizes differently, and
the most widely used tokenizer is not public. This chapter states the proxy a budget counts instead,
the caps, and why each cap sits where it does. The sources and the measurement are in
[22 — Skill format research](./22-skill-format-research.md).

## Count characters

A size budget is measured in characters, never in lines or words.

The vendor guidance names three units for one limit: 500 lines, 5,000 tokens, and 5,000 words. The
three disagree as soon as prose is written one paragraph per line, which is how a documentation
project that diffs by sentence writes it. A measurement over two corpora, 145 Markdown documents and
90 skill files, ran two public tokenizers and asked which proxy predicts each file's token count.

| Proxy      | Median error | Worst error  |
| ---------- | ------------ | ------------ |
| Characters | 4% to 5%     | 12% to 39%   |
| Words      | 9% to 12%    | 27% to 42%   |
| Lines      | 12% to 27%   | 118% to 126% |

Lines are the worst proxy by a wide margin. Tokens per line ranged from 6.5 to 34, so a 500-line cap
admits anything from about 3,000 to about 17,000 tokens. Characters track tokens closely on both
corpora.

Characters stay a proxy. Code, tables, and text in a language other than English carry more tokens
per character, and the caps leave room for that rather than pretending to precision.

## The caps

| File             | Cap               | Target                | Also                                                         |
| ---------------- | ----------------- | --------------------- | ------------------------------------------------------------ |
| `SKILL.md` body  | 12,000 characters | 8,000 or fewer        | the frontmatter is not counted                               |
| A reference file | 32,000 characters | as small as it can be | above 8,000 characters, it opens with a list of its sections |

### The manual

12,000 characters is about 4,800 tokens at 2.5 characters per token, the rate Anthropic states for
its current tokenizer: one million tokens to roughly 2.5 million characters. The measured corpora
averaged about 4.4 characters per token on the public tokenizers, and about 3.4 after the 30%
increase Anthropic states for its newer tokenizer, so 2.5 is the conservative bound and the cap uses
it.

The figure keeps a body under two limits that apply to it directly. The Agent Skills specification
recommends a body under 5,000 tokens on activation. Claude Code re-attaches only the first 5,000
tokens of each invoked skill after it compacts a session, within a combined 25,000-token budget, so
a body longer than that loses its tail at the point a long session needs it most.

The target of 8,000 characters leaves room for the session around the skill. A body near the cap is
a body that should move a task to a reference file.

### A reference file

32,000 characters follows the 32 KiB that Codex sets by default as the combined size of the
instruction files it loads, the one published byte limit for a file an agent reads as instructions.
No primary source sets a cap for an individual reference file beyond "focused" and "smaller", so
this cap is a ceiling, not a target: the reference a run reads should be as small as its task
allows, because accuracy falls as input grows even when the model retrieves every relevant fact.

A reference file over 8,000 characters opens with a list of its sections. An agent tends to preview
a long file by its head, and a list of sections at the head tells it whether the rest holds what it
needs. The vendor guides set the same rule at 100 or 300 lines, and 8,000 characters is the stricter
end for prose written one paragraph per line.

## Why a larger window changes none of this

Current models offer about one million tokens of context. The window is not the constraint a skill
budget answers.

- Budgets far below the window already apply: the specification recommends a body under 5,000
  tokens on activation, and the loaders keep the first 5,000 tokens after compaction and cap the
  skill listing at 1% or 2% of the window.
- Accuracy falls as input grows. One irrelevant passage already lowers it, and length alone lowers
  it even with perfect retrieval.
- Compliance falls as the instruction count rises. In one benchmark that is not peer reviewed,
  almost no response satisfies every rule once a prompt holds 80 or more.
- A loaded skill stays in context for every later turn, so each character is paid again on each
  turn.

A larger window raises the ceiling on what fits. It does not raise the accuracy of what is read,
which is the reason a skill stays lean.

## Holding the budget

The caps are tests over every package the project ships, counted in characters, with the frontmatter
excluded from a manual's body. A failure names the file and its count, and the fix is a move, never
a raised cap: a task procedure leaves `SKILL.md` for a reference file, and an oversized reference
splits along its sections.

## See also

- [03 — Skills](./03-skills.md) — progressive disclosure, the layers a budget prices.
- [16 — Skill format](./16-skill-format.md) — what goes in the manual and what goes in a reference.
- [22 — Skill format research](./22-skill-format-research.md) — the measurement, the papers, and the
  token rates.
