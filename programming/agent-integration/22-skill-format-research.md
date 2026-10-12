# 22 — Skill Format Research

The research beneath [21](./21-skill-format-evidence.md), [16](./16-skill-format.md), and
[17](./17-skill-size-budget.md): context engineering, context windows, the papers, a measurement of
which size unit predicts tokens, the tools that install skills, the conflicts, and what could not be
verified. Every source was read 2026-10-08, except those under "Installing a skill package", read
2026-10-09, and every source was read again as raw text on 2026-10-10. A date in parentheses is the
publication date.

## Context engineering

Anthropic's post, at
<https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents> (2025-09-29):

- Good context engineering means "finding the smallest possible set of high-signal tokens that
  maximize the likelihood of some desired outcome."
- "Context, therefore, must be treated as a finite resource with diminishing marginal returns."
  Degradation is "a performance gradient rather than a hard cliff".
- The "right altitude" sits between hardcoded brittle logic and vague guidance that "fails to give
  the LLM concrete signals", and "minimal does not necessarily mean short".
- It warns against "a laundry list of edge cases" and prefers "diverse, canonical examples".
  Just-in-time loading keeps "lightweight identifiers (file paths, stored queries, web links, etc.)"
  and loads the content at runtime.
- "the exact formatting of prompts is likely becoming less important as models become more capable."

## Context windows

- Claude, <https://platform.claude.com/docs/en/docs/about-claude/models/overview> and
  <https://platform.claude.com/docs/en/build-with-claude/context-windows>: every current model has a
  1M-token window by default, and "more context isn't automatically better. As token count grows,
  accuracy and recall degrade, a phenomenon known as context rot." Its pricing,
  <https://platform.claude.com/docs/en/about-claude/pricing>: "A 900k-token request is billed at the
  same per-token rate as a 9k-token request", for Claude 4.6 and later models except Haiku 5.5.
- OpenAI, <https://developers.openai.com/api/docs/models>: current models list about 1.05M tokens,
  and a prompt above 272K input tokens costs twice as much per input token, per
  <https://developers.openai.com/api/docs/pricing>.
- Google, <https://ai.google.dev/gemini-api/docs/models/gemini-3.1-pro-preview>: 1,048,576 input
  tokens, and a prompt above 200K tokens costs twice as much, per
  <https://ai.google.dev/gemini-api/docs/pricing>. The long-context page,
  <https://ai.google.dev/gemini-api/docs/long-context>: "if you don't need tokens to be passed to
  the model, it is best to avoid passing them."
- The Claude Opus 4.6 announcement, <https://www.anthropic.com/news/claude-opus-4-6> (2026-02-05):
  on an 8-needle retrieval test at 1M tokens, "Opus 4.6 scores 76%, whereas Sonnet 4.5 scores just
  18.5%." Use of the full window varies widely between models.

## Long-context degradation

| Source                                                                                                       | Finding                                                                                                                                                                                |
| ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Liu et al., "Lost in the Middle", TACL. <https://arxiv.org/abs/2307.03172> (2023-07-06)                      | Performance "significantly degrades when models must access relevant information in the middle of long contexts, even for explicitly long-context models."                             |
| Hsieh et al., RULER, COLM 2024. <https://arxiv.org/abs/2404.06654> (2024-04-09)                              | "only half of them can maintain satisfactory performance at the length of 32K."                                                                                                        |
| Modarressi et al., NoLiMa, ICML 2025. <https://arxiv.org/abs/2502.05167> (2025-02-07)                        | "At 32K, for instance, 11 models drop below 50% of their strong short-length baselines."                                                                                               |
| Hong, Troynikov, Huber, "Context Rot", Chroma. <https://www.trychroma.com/research/context-rot> (2025-07-14) | "Even a single distractor reduces performance relative to the baseline." Focused prompts scored "significantly higher" than full prompts.                                              |
| Shi et al., ICML 2023. <https://arxiv.org/abs/2302.00093>                                                    | "Model performance is dramatically decreased when irrelevant information is included."                                                                                                 |
| Levy, Jacoby, Goldberg, ACL 2024. <https://arxiv.org/abs/2402.14848>                                         | Reasoning degrades "at much shorter input lengths than their technical maximum."                                                                                                       |
| Du et al., Findings of EMNLP 2025. <https://arxiv.org/abs/2510.05381> (2025-10-06)                           | "Even when models can perfectly retrieve all relevant information, their performance still degrades substantially (13.9%–85%) as input length increases." Length alone costs accuracy. |
| Bai et al., LongBench v2. <https://arxiv.org/abs/2412.15204> (2024-12-19)                                    | The best direct-answer model "achieves only 50.1% accuracy."                                                                                                                           |
| Paulsen, "Maximum Effective Context Window". <https://arxiv.org/abs/2509.21361>                              | The effective window is far below the advertised one. Weak evidence: one author, and an unusual method.                                                                                |

## Instruction count and compliance

| Source                                                                           | Finding                                                                                                                                                                                                                                  |
| -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Zhou et al., IFEval. <https://arxiv.org/abs/2311.07911> (2023-11-14)             | The base benchmark for verifiable instructions.                                                                                                                                                                                          |
| Jiang et al., FollowBench, ACL 2024. <https://arxiv.org/abs/2310.20410>          | Compliance falls as constraints accumulate.                                                                                                                                                                                              |
| Jaroslawicz et al., IFScale. <https://arxiv.org/abs/2507.11538> (2025-07-15)     | "even the best frontier models only achieve 68% accuracy at the max density of 500 instructions". Models show a "bias towards earlier instructions".                                                                                     |
| Harada et al., ManyIFEval. <https://arxiv.org/abs/2509.21051> (2025-09-25)       | "performance consistently degrades as the number of instructions increases".                                                                                                                                                             |
| Qi et al., AgentIF. <https://arxiv.org/abs/2505.16944>                           | Real agent system prompts average "1,723 words" and "11.9 constraints per instruction". Current models "generally perform poorly".                                                                                                       |
| Ding et al., OctoBench. <https://arxiv.org/abs/2601.10343> (2026-01-15)          | "a systematic gap between task-solving and scaffold-aware compliance", across system prompts, repository files, and skills.                                                                                                              |
| Gloaguen et al., "Evaluating AGENTS.md". <https://arxiv.org/abs/2602.11988>      | Context files "do not generally improve task success rates, while increasing inference cost by over 20% on average". Repository overviews "are not helpful".                                                                             |
| Li et al., SkillsBench. <https://arxiv.org/abs/2602.12670>                       | "Focused Skills with at most three modules outperform larger or exhaustive bundles". In v4 (2026-06-14), compact skills gained 19.0 points and comprehensive ones 0.7, and the median `SKILL.md` is 4.8 KB. v1 to v3 give other figures. |
| Eliav, "Prompt Design at Scale". <https://arxiv.org/abs/2607.19257> (2026-07-21) | With 80 or more rules, the rate of responses that satisfy every rule is near zero for every model and format. Not peer reviewed.                                                                                                         |

## Prompt format

| Source                                                                                                                                      | Finding                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| He et al. <https://arxiv.org/abs/2411.10541> (2024-11-15)                                                                                   | "there is no universally optimal format, even within the same generational lineage." Larger models are more robust to format.                                                                                                                                                                                                                                                                                                                                    |
| "Quantifying Language Models' Sensitivity to Spurious Features in Prompt Design", ICLR 2024. <https://arxiv.org/abs/2310.11324>             | Spurious format changes moved accuracy by "up to 76 accuracy points" on a 13B model.                                                                                                                                                                                                                                                                                                                                                                             |
| Eliav, VeyraBench. <https://arxiv.org/abs/2607.19257>                                                                                       | "no model shows a consistent, meaningful markdown advantage". Markdown costs about 26% more tokens than plain text, and tables about 37% more.                                                                                                                                                                                                                                                                                                                   |
| Claude prompting best practices. <https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-prompting-best-practices> | "Provide instructions as sequential steps using numbered lists or bullet points when the order or completeness of steps matters." Explain the reason behind a rule: "Claude is smart enough to generalize from the explanation." Replace "CRITICAL: You MUST use this tool when…" with "Use this tool when…", because Claude Opus 4.5 and 4.6 over-trigger on aggressive wording written for earlier models. "Tell Claude what to do instead of what not to do." |
| Prompting Claude Sonnet 5. <https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-sonnet-5>             | The model "interprets prompts literally and explicitly... It does not silently generalize an instruction from one item to another." State each rule's scope.                                                                                                                                                                                                                                                                                                     |
| OpenAI GPT-4.1 prompting guide. <https://developers.openai.com/cookbook/examples/gpt4-1_prompting_guide> (2025-04-14)                       | Markdown sections are recommended. The model follows instructions "more literally". In a conflict, it follows the instruction closer to the end.                                                                                                                                                                                                                                                                                                                 |
| OpenAI GPT-5 prompting guide. <https://developers.openai.com/cookbook/examples/gpt-5/gpt-5_prompting_guide> (2025-08-07)                    | "contradictory or vague instructions can be more damaging".                                                                                                                                                                                                                                                                                                                                                                                                      |
| OpenAI, Using GPT-6. <https://developers.openai.com/api/docs/guides/latest-model>                                                           | "better able to follow longer instructions, but can also be more sensitive to information in context."                                                                                                                                                                                                                                                                                                                                                           |
| Google prompting strategies. <https://ai.google.dev/gemini-api/docs/prompting-strategies> (updated 2026-09-17)                              | "XML-style tags... or Markdown headings are effective". "Be precise and direct... Avoid unnecessary or overly persuasive language".                                                                                                                                                                                                                                                                                                                              |

## Token proxies and the measurement

The vendors' rules of thumb:

- Anthropic: the models overview states "1M tokens is roughly 555k words or 2.5M Unicode characters
  on the current tokenizer (introduced with Claude Opus 4.7)", about 2.5 characters per token. The
  glossary, <https://platform.claude.com/docs/en/about-claude/glossary>, gives the earlier tokenizer
  "approximately 3.5 English characters" per token, and the newer one "approximately 30 percent more
  tokens for the same text". Token counting,
  <https://platform.claude.com/docs/en/build-with-claude/token-counting>, is free.
- Google, <https://ai.google.dev/gemini-api/docs/tokens>: "a token is equivalent to about 4
  characters. 100 tokens is equal to about 60-80 English words."

The research: Roberts, Han, and Albanie, <https://arxiv.org/abs/2601.11518> (2026-01-16), find
"token counts are not a stable unit of length", with compression ratios differing by 20% to 50%
between tokenizers, and by 100% for emoji and numbers, and call the common rule of 0.75 words per
token "an oversimplification". Petrov et al., NeurIPS 2023, <https://arxiv.org/abs/2305.15425>, find
the same text in different languages differs in token length "up to 15 times".

The measurement, run 2026-10-08:

- Method: the `tiktoken` tokenizers `o200k_base` and `cl100k_base` ran over two corpora. One pooled
  ratio per proxy predicted each file's token count, and the error is the per-file error of that
  prediction. Claude's tokenizer is not public, so the figures come from OpenAI tokenizers.
- Corpus 1: 145 Markdown files of one documentation project, written as unwrapped prose.
- Corpus 2: 90 skill files from the skill collections installed on one host.

| Corpus                 | Proxy      | Correlation with tokens | Median error | 90th-percentile error | Worst error |
| ---------------------- | ---------- | ----------------------- | ------------ | --------------------- | ----------- |
| Documentation Markdown | Characters | 0.989                   | 4%           | 12%                   | 39%         |
|                        | Words      | 0.965                   | 9%           | 19%                   | 42%         |
|                        | Lines      | 0.964                   | 12%          | 46%                   | 126%        |
| Skill files            | Characters | 0.995                   | 5%           | 9%                    | 12%         |
|                        | Words      | 0.971                   | 12%          | 21%                   | 27%         |
|                        | Lines      | 0.71                    | 27%          | 70%                   | 118%        |

Characters predict tokens best in both corpora. Tokens per line ranged from 6.5 to 34, so a 500-line
cap allows anything from about 3,000 to about 17,000 tokens. The Markdown averaged about 4.4
characters per token on these tokenizers. With Anthropic's stated 30% increase that is about 3.4,
and Anthropic's own 2.5 characters per token is the conservative bound a character cap converted
from tokens uses.

## Installing a skill package

The question these sources answer: where does a tool that installs its own skills put the content
two skills share?

The specification defines a skill as one directory holding at least `SKILL.md`, and it names no
location two skills share.

The Atlassian TWG CLI, at <https://developer.atlassian.com/cloud/twg-cli/agents/skills/>, installs
the skills that operate it. "It always writes the canonical skill bundle to `~/.agents/skills`", and
where Claude Code is installed, "Claude Code receives a copy under `~/.claude/skills`". The copy is
a copy, not a link. Each skill holds its own `.agents/skills/<skill>/references/`, "Detailed
guidance loaded on demand", and the page shows no file shared outside a skill directory. "To repair
or refresh skills manually, run: `twg skills install`."

`gh skill install`, at <https://cli.github.com/manual/gh_skill_install>, lands skills "at either
project scope (inside the current git repository) or user scope (in your home directory, available
everywhere)". At project scope, several agents "share the `.agents/skills` directory", and a skill
bound for one destination from several hosts "is installed there only once". Local skills "are
copied (not symlinked) with local-path tracking metadata injected into frontmatter", and that
metadata "enables `gh skill update` to detect changes".

The Vercel `skills` CLI, at <https://github.com/vercel-labs/skills>, keeps one canonical copy and
links it into each agent's own root, and issue 1025 below names `~/.agents/skills` as the store of a
global install. Symlink is the
recommended mode, and `--copy` "creates independent copies for each agent", for where symlinks are
not supported. Its issue tracker records what the links cost:

- <https://github.com/vercel-labs/skills/issues/744>: the skill installs to `~/.agents/skills`, no
  link reaches `~/.claude/skills`, and Claude Code cannot see it.
- <https://github.com/vercel-labs/skills/issues/1199>: `update` turns copied skills back into links,
  because the lock file does not record the install mode, and links do not travel across platforms.
- <https://github.com/vercel-labs/skills/issues/1025>: a request for a command that rebuilds the
  links, because rerunning the install over the canonical store is unsafe.

What the sources establish:

1. Every tool installs a skill as one whole directory, and each directory carries its own
   `references/`. No tool reads a supporting file from outside the skill directory.
2. A tool that serves several agent roots either copies the directory into each root or links each
   root to one canonical copy. The links fail in ways the copies do not.
3. Inference, not stated by any source: a tool that writes the copies from one embedded source keeps
   one authored file, so a fix still lands once.

## Where the sources conflict

| Topic                       | Positions                                                                                                                                                             | The choice taken                                                                                       |
| --------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| Size unit                   | 500 lines (specification, Claude docs, OpenAI skill-creator). 5,000 tokens (specification, Claude overview). 5,000 words (Anthropic guide, OpenAI skill-creator).     | Characters, because the measurement ranks them first and the three units disagree for unwrapped prose. |
| Table of contents threshold | 100 lines (Claude best practices, OpenAI skill-creator). 300 lines (Anthropic skill-creator).                                                                         | 8,000 characters, the stricter end for unwrapped prose.                                                |
| Reasons in rules            | Explain the why (Agent Skills, Anthropic skill-creator, Claude prompting). State what to do, not how or why (Claude Code skills).                                     | A reason of one clause, only where it changes behavior.                                                |
| Emphasis                    | A worked example suggests "MUST filter" over "always filter" (Claude skill best practices). Dial back aggressive wording (Claude prompting, Anthropic skill-creator). | No all-caps emphasis. A hard rule becomes a check.                                                     |
| Description voice           | Third person (Claude). Imperative "Use this skill when" (Agent Skills).                                                                                               | "[What it does]. Use when …", which satisfies both.                                                    |

## What the research could not verify

- No primary source prescribes Markdown decision tables. Choosing them for closed choices rests on
  reasoning, not on a source.
- No primary source sets a size cap for one reference file beyond "focused" and "smaller". A
  32,000-character cap follows the Codex 32 KiB limit for instruction files by analogy.
- The OpenAI tokens-per-character help article returned HTTP 403 on 2026-10-08 and 2026-10-10, so
  its rule was confirmed only through secondary quotes.

## See also

- [21 — Skill format evidence](./21-skill-format-evidence.md) — the specification and vendor
  guidance these findings sit beneath.
- [04 — Skill distribution](./04-skill-distribution.md) — the scopes and roots the installers above
  write into.
- [05 — Evaluations](./05-evaluations.md) — how a format choice is checked against agent behavior
  rather than assumed.
