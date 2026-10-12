# 21 — Skill Format Evidence

What the Agent Skills specification and the vendors that adopt it say about a skill's format and
size, each read from its primary source on 2026-10-08 and read again as raw text on 2026-10-10.
[16 — Skill format](./16-skill-format.md) and [17 — Skill size budget](./17-skill-size-budget.md)
rest on it and on [22 — Skill format research](./22-skill-format-research.md). An entry states what
its source says, with a short quote where the wording matters.

## The Agent Skills specification

The specification, at <https://agentskills.io/specification>, with its source in
`docs/specification.mdx` of <https://github.com/agentskills/agentskills>:

- The layout is a required `SKILL.md` and optional `scripts/`, `references/`, and `assets/`.
- `name` is 1 to 64 characters of lowercase letters, digits, and hyphens, with no leading, trailing,
  or double hyphen, and it matches the parent directory. `description` is 1 to 1,024 characters and
  "Should describe both what the skill does and when to use it". `compatibility` is at most 500
  characters, and "Most skills do not need the `compatibility` field." `license`, `metadata` (a
  string-to-string map), and the experimental `allowed-tools` are optional.
- The body: "There are no format restrictions." Recommended contents are step-by-step instructions,
  examples, and edge cases.
- Progressive disclosure: metadata takes about 100 tokens at startup, the body "< 5000 tokens
  recommended" on activation, and resources load "only when required".
- "Keep your main `SKILL.md` under 500 lines. Move detailed reference material to separate files."
  Both figures are recommendations.
- References "use relative paths from the skill root". "Keep file references one level deep from
  `SKILL.md`. Avoid deeply nested reference chains." "Keep individual reference files focused.
  Agents load these on demand, so smaller files mean less use of context."
- `skills-ref validate` checks the frontmatter only, and its README says the library "is intended
  for demonstration purposes only".

The client implementation guide, at
<https://agentskills.io/client-implementation/adding-skills-support>, puts the catalog at "~50-100
tokens per skill". It says `.agents/skills/` at project and user scope "have emerged as a
widely-adopted convention for cross-client skill sharing", and that clients should "exempt skill
content from pruning" during compaction.

The best practices, at <https://agentskills.io/skill-creation/best-practices>:

- "Concise, stepwise guidance with a working example tends to outperform exhaustive documentation."
- The deletion test: "Would the agent get this wrong without this instruction? If the answer is no,
  cut it."
- "Provide defaults, not menus." "Favor procedures over declarations."
- "The highest-value content in many skills is a list of gotchas." The gotchas stay in `SKILL.md`.
- Templates are "more reliable than describing the format in prose, because agents pattern-match
  well against concrete structures".
- A link carries its loading condition: "Read `references/api-errors.md` if the API returns a
  non-200 status code" works better than "see references/".

The evaluation pages, at <https://agentskills.io/skill-creation/evaluating-skills> and
<https://agentskills.io/skill-creation/optimizing-descriptions>, hold that "Reasoning-based
instructions ('Do X because Y tends to cause Z') work better than rigid directives ('ALWAYS do X,
NEVER do Y')." An evaluation runs output scenarios with and without the skill, and tests about 20
trigger queries, 8 to 10 that should trigger and 8 to 10 that should not, near misses being the most
valuable negatives, split 60/40 into training and validation.

## Anthropic and Claude

Skill authoring best practices, at
<https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices>:

- "The context window is a public good." "Default assumption: Claude is already very smart. Only add
  context Claude doesn't already have." Each paragraph answers "Does this paragraph justify its
  token cost?"
- Degrees of freedom: "Match the level of specificity to the task's fragility and variability."
  Exact commands for fragile steps, heuristics where many approaches work.
- "Keep SKILL.md body under 500 lines for optimal performance", a recommendation.
- "Keep references one level deep from SKILL.md. All reference files should link directly from
  SKILL.md", because Claude "might use commands like `head -100`" on a nested file. "For reference
  files longer than 100 lines, include a table of contents at the top."
- "Break complex operations into clear, sequential steps." A complex workflow gets a checklist the
  agent copies and checks off. Feedback loops: "Run validator → fix errors → repeat."
- "Choose one term and use it throughout the Skill." No time-sensitive information, and legacy
  material goes into a labeled "Old patterns" section.
- Descriptions: "Always write in third person", stating what the skill does and when to use it.
  `name` and `description` carry no XML tags, and `name` carries no "anthropic" or "claude".
- "Prefer scripts for deterministic operations", and say whether the agent runs a script or reads
  it.
- "Create evaluations BEFORE writing extensive documentation." "There is not currently a built-in
  way to run these evaluations."

The Agent Skills overview, at
<https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview>, states the loading
levels as metadata "~100 tokens per Skill", instructions "Under 5k tokens", and resources "None
until accessed". "Scripts run through bash, and only their output enters context."

Claude Code's skills page, at <https://code.claude.com/docs/en/skills>:

- "Keep `SKILL.md` under 500 lines. Move detailed reference material to separate files."
- "Keep the body itself concise. Once a skill loads, its content stays in context across turns, so
  every line is a recurring token cost. State what to do rather than narrating how or why."
- "Claude Code does not re-read the skill file on later turns, so write guidance that should apply
  throughout a task as standing instructions rather than one-time steps."
- After compaction it re-attaches "the first 5,000 tokens of each" skill, within a "combined budget
  of 25,000 tokens", so the most important instructions go near the top.
- The listing truncates `description` plus `when_to_use` at 1,536 characters, and the listing budget
  scales at 1% of the context window. A rule that must hold every time moves into a hook. Only the
  six specification fields work outside Claude Code.

The vendor's runtime facts are dated in
[tools/claude-code — skills](../../tools/claude-code/skills.md).

Claude Code's memory and best-practices pages, at <https://code.claude.com/docs/en/memory> and
<https://code.claude.com/docs/en/best-practices>:

- "target under 200 lines per CLAUDE.md file. Longer files consume more context and reduce
  adherence."
- "group related instructions under markdown headers and bullets. Organized sections are easier for
  Claude to follow than dense paragraphs."
- Instructions are "concrete enough to verify": "Use 2-space indentation", not "Format code
  properly".
- "if two instructions contradict each other, Claude may pick one arbitrarily." "Bloated CLAUDE.md
  files cause Claude to ignore your actual instructions!" "If you emphasize many lines, none of them
  stands out."
- "For each line, ask: 'Would removing this cause Claude to make mistakes?' If not, cut it."

The engineering post, at
<https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills>
(2025-10-16, update note dated 2025-12-18), defines skills as "organized folders of instructions,
scripts, and resources that agents can discover and load dynamically". Its practices: start with
evaluation, split an unwieldy `SKILL.md`, keep mutually exclusive contexts in separate files, and
iterate with the model. It states no numeric size limit.

The skill-creator skill, at
<https://github.com/anthropics/skills/blob/main/skills/skill-creator/SKILL.md>:

- "Prefer using the imperative form in instructions." Every "when to use" fact goes in the
  description, not the body.
- "Try to explain to the model why things are important in lieu of heavy-handed musty MUSTs." "If
  you find yourself writing ALWAYS or NEVER in all caps … that's a yellow flag." "Keep the prompt
  lean. Remove things that aren't pulling their weight."
- A reference file over 300 lines gets a table of contents, which conflicts with the 100 lines of
  the best-practices page.
- In the same repository `claude-api/SKILL.md` runs 616 lines at commit `dbd4588` (2026-10-09), so
  the 500-line figure is not enforced there.

The Complete Guide to Building Skills for Claude, at
<https://resources.anthropic.com/hubfs/The-Complete-Guide-to-Building-Skill-for-Claude.pdf>
(2026-01):

- "Keep SKILL.md under 5,000 words", a third unit beside lines and tokens. "Don't include README.md
  inside your skill folder."
- A description is "[What it does] + [When to use it] + [Key capabilities]", with negative triggers
  where an adjacent skill exists.
- Specific and actionable: "Run `python scripts/validate.py --input {filename}`", not "Validate the
  data before proceeding." "Put critical instructions at the top." "Use bullet points and numbered
  lists." "Code is deterministic; language interpretation isn't."

## OpenAI

The skills guide for Codex and ChatGPT, at <https://learn.chatgpt.com/docs/build-skills>: "Keep each
skill focused on one job." "Prefer instructions over scripts unless you need deterministic behavior
or external tooling." "Write imperative steps with explicit inputs and outputs." The skill list
takes "at most 2% of the model's context window, or 8,000 characters when the context window is
unknown", so "Front-load the key use case and trigger words so a host can still match the skill if
descriptions are shortened."

The skill-creator skill, at
<https://github.com/openai/skills/blob/main/skills/.system/skill-creator/SKILL.md>: "Always use
imperative/infinitive form." "Information should live in either SKILL.md or references files, not
both." A skill holds no README, install guide, quick reference, or changelog. The body stays under
"<5k words" and 500 lines, and a reference file over 100 lines gets a table of contents.

AGENTS.md in Codex, at <https://learn.chatgpt.com/docs/agent-configuration/agents-md>: Codex "stops
adding files once the combined size reaches the limit defined by project_doc_max_bytes (32 KiB by
default)". The runtime fact is dated in
[tools/codex — instruction files](../../tools/codex/instruction-files.md).

## Other adopters

- GitHub Copilot,
  <https://docs.github.com/en/copilot/how-tos/copilot-on-github/customize-copilot/customize-cloud-agent/add-skills>:
  "custom instructions for simple instructions relevant to almost every task" and "skills for more
  detailed instructions that Copilot should only access when relevant."
- VS Code, <https://code.visualstudio.com/docs/agent-customization/agent-skills>: "Names with
  invalid characters cause the skill to silently fail to load."
- Cursor, <https://cursor.com/docs/skills>: it reads `.agents/skills/`, `.cursor/skills/`,
  `.claude/skills/`, and `.codex/skills/`.
- Gemini CLI, <https://geminicli.com/docs/cli/skills/> and
  <https://geminicli.com/docs/cli/creating-skills/>: the description "is how Gemini decides when to
  use the skill". No size limit is stated.
- AGENTS.md, <https://agents.md/>: "just standard Markdown. Use any headings you like", with no size
  guidance.

## See also

- [22 — Skill format research](./22-skill-format-research.md) — context engineering, the papers, the
  measurement, the installers, and the conflicts these sources leave open.
- [03 — Skills](./03-skills.md) — the portable baseline these sources describe.
- [tools/codex — skills](../../tools/codex/skills.md) and
  [tools/gemini-cli — skills](../../tools/gemini-cli/skills.md) — the runtime facts above, dated per
  vendor.
