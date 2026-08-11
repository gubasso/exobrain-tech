# Glossary

One descriptive sentence per term this shelf uses, each naming the chapter whose rules govern it. An
entry identifies a term; it never states a rule, threshold, condition, or procedure.

A chapter defines a term inline only when the term is that chapter's subject. A chapter that uses a
term it does not own MUST NOT redefine it; it may add a short appositive for the immediate sentence,
and links `./glossary.md` when a reader might not know the term at all. If an entry here needs a
second sentence, the definition belongs in the owning chapter instead.

- `ADR` — an architecture decision record: one file recording why one option was chosen over serious
  alternatives. Rules: [02 — Lean ADRs](./02-lean-adrs.md).
- `amendment` — a pointer added under a record's status when a later decision changed part of it
  without reversing it. Rules: [02 — Lean ADRs](./02-lean-adrs.md).
- `ARID` — Avoid Repetition In Documentation: every durable fact has one owner and every other
  mention is a link. Rules: [00 — Foundations](./00-foundations.md).
- `artifact token` — an upper-snake `<ANGLE>` name for one thing a phase of a guide produces.
  Rules: [09 — Procedure Artifacts](./09-procedure-artifacts.md).
- `author-instructions file` — the file a project's humans and agents load before editing, such as
  `CLAUDE.md` or `AGENTS.md`. Rules: [04 — Agent Context](./04-agent-context.md).
- `cadence` — how often a perishable fact is re-verified.
  Rules: [06 — Tracking and Revalidation](./06-tracking-and-revalidation.md).
- `chapter` — one numbered file in this shelf, owning one topic.
  Rules: [README](./README.md).
- `context filter` — an entry document that names the sources a session loads, so the session loads
  those and nothing else. Rules: [04 — Agent Context](./04-agent-context.md).
- `decorative markdown` — formatting that carries no structure a reader or parser can use, chiefly
  bold and italics. Rules: [08 — Lean Markdown](./08-lean-markdown.md).
- `Deprecated` — a decision that no longer applies and has no successor.
  Rules: [02 — Lean ADRs](./02-lean-adrs.md).
- `digest` — a derived per-directory map for agents, such as an `AGENTS.md`, which is never a rules
  home. Rules: [04 — Agent Context](./04-agent-context.md).
- `docs root` — the directory the zones sit under: `docs/` in a code project, `_docs/` in a knowledge
  base. Rules: [01 — Diataxis Zones](./01-diataxis-zones.md).
- `draft` — provisional material that is not yet project state, held outside shipped docs.
  Rules: [05 — Drafts and Promotion](./05-drafts-and-promotion.md).
- `entry document` — the single document a session reads first for one unit of work or one
  subsystem. Rules: [04 — Agent Context](./04-agent-context.md).
- `fixed-shape document` — a document whose heading list is a contract other documents, tools, and
  sessions rely on, so it is gated rather than allowed to grow.
  Rules: [08 — Lean Markdown](./08-lean-markdown.md).
- `honest-fail` — a test left failing so it does not hide a bug in an external system.
  Rules: [07 — Known Issues](./07-known-issues.md).
- `inputs line` — the line under a phase heading naming the artifacts it consumes and where each was
  produced. Rules: [09 — Procedure Artifacts](./09-procedure-artifacts.md).
- `knowledge base` — a project whose product is its content tree of directories and markdown files
  rather than a codebase. Rules: [01 — Diataxis Zones](./01-diataxis-zones.md).
- `known-issue case` — one tracked bug in a system the project does not own, filed as its own
  directory. Rules: [07 — Known Issues](./07-known-issues.md).
- `load-bearing comment` — a comment whose removal would confuse a future maintainer.
  Rules: [00 — Foundations](./00-foundations.md).
- `mask` — a temporary workaround in the tree that hides a known issue and must be revert-tracked.
  Rules: [07 — Known Issues](./07-known-issues.md).
- `MUST` / `SHOULD` — normative keywords, binding only when capitalized; see
  <https://www.rfc-editor.org/rfc/rfc8174.txt>.
- `outputs block` — the fenced list closing a phase of a guide, naming each artifact it produced.
  Rules: [09 — Procedure Artifacts](./09-procedure-artifacts.md).
- `placeholder` — a lowercase `<angle>` name standing in for anything project-specific, distinct from
  an artifact token by case. Rules: [08 — Lean Markdown](./08-lean-markdown.md).
- `product` — what a project exists to produce, which its documentation is about rather than part of.
  Rules: [01 — Diataxis Zones](./01-diataxis-zones.md).
- `promotion` — rewriting a draft into a durable document in its owning zone.
  Rules: [05 — Drafts and Promotion](./05-drafts-and-promotion.md).
- `recipe form` — a guide phase written as numbered actions with commands in fences, rather than as
  prose. Rules: [09 — Procedure Artifacts](./09-procedure-artifacts.md).
- `Rejected` — a decision the project explicitly chose not to take.
  Rules: [02 — Lean ADRs](./02-lean-adrs.md).
- `shape config` — the one file holding a fixed shape's `MD043` heading array, applied to that shape's
  documents by one hook entry. Rules: [08 — Lean Markdown](./08-lean-markdown.md).
- `shelf` — one topic directory of numbered chapters in this knowledge base.
  Rules: [README](./README.md).
- `structural markdown` — formatting that carries shape a reader and a parser can both use:
  headings, lists, tables, fences, inline code, links.
  Rules: [08 — Lean Markdown](./08-lean-markdown.md).
- `subsystem page` — the living explanation page owning one subsystem's current design.
  Rules: [03 — Subsystem Pages](./03-subsystem-pages.md).
- `Superseded` — a decision replaced by a later one, which exists.
  Rules: [02 — Lean ADRs](./02-lean-adrs.md).
- `tracking file` — a machine-readable registry of perishable facts and their re-check cadence.
  Rules: [06 — Tracking and Revalidation](./06-tracking-and-revalidation.md).
- `zone` — one home in the docs layout, defined by the reader need it serves.
  Rules: [01 — Diataxis Zones](./01-diataxis-zones.md).
