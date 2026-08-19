# Knowledge-base architecture

## The product is the knowledge

`exobrain-tech` is a knowledge base: a library of knowledge organized as directories and markdown files.
The **product** is that knowledge — the library, the content tree itself. This is the same relationship a code
project has:

| Code project                       | Knowledge base                        |
| ---------------------------------- | ------------------------------------- |
| The codebase is the product        | The markdown content is the product   |
| `_docs/` = metadata about the code | `_docs/` = metadata about the content |

So `_docs/` here is not where the knowledge lives. The knowledge lives in the content directories at
the top of the repository. `_docs/` holds the **metadata and specs about that product**: the definitions,
decisions, architecture, conventions, and patterns that govern how the knowledge base is structured
and maintained. When you want to record _how the knowledge base works_, write in `_docs/`. When you
want to record _knowledge_, write in the content tree.

## Content structure

The content tree is owned by the filesystem. Organize it by subject, with directories that group
related knowledge and semantic filenames that expose a file's topic before it is opened. Avoid
`notes.md`, `misc.md`, and `final-v2.md`; prefer durable nouns.

The knowledge is organized into subject buckets at the top of the repository:

| Bucket         | What it holds                                                      |
| -------------- | ------------------------------------------------------------------ |
| `programming/` | Language-agnostic craft: architecture, CLI design, testing, specs. |
| `languages/`   | Per-language guidance (`bash`, `python`, `rust`, `nix`, …).        |
| `systems/`     | Operating systems, shells, email, security.                        |
| `infra/`       | Cloud, containers, devops, networking, servers.                    |
| `tools/`       | Individual tools (`git`, `claude-code`, `nix`, `suckless`, …).     |
| `platforms/`   | Application platforms (`icp`, `solana`, `webdev`).                 |
| `workflows/`   | Cross-cutting personal and dev workflows.                          |
| `data/`        | Data engineering and databases.                                    |

Keep the private/public boundary: private equipment identity, security posture, recovery material,
credentials, and personal workflows belong in `exobrain-tech-vault`, not here.

Keep drafts out of the shipped content tree — normally under a gitignored `.draft/` — and promote a
draft by rewriting it into its durable home, then deleting the draft.

## The AGENTS.md digest standard

Every substantial content area carries an `AGENTS.md` **digest**: a concise map of that directory's
knowledge, loaded first by an agent (human or LLM) before it reads the underlying files. A digest is
a map, never the source of truth — the content files own the knowledge; the digest summarizes them.

A digest carries frontmatter naming the area it maps and the date it was last reconciled with that
area, so staleness is visible:

```yaml
---
digest-of: <path/to/this/area>
last-synced: <YYYY-MM-DD>
token-estimate: <approx tokens>
---
```

Regenerate a digest when the area's knowledge changes. A digest keeps no index of the directory: the
filesystem owns what exists, and a checked-in list drifts on the next add or rename. It names a file
when it has something to say about it. See
[ADR-filesystem-owns-disk-state](../decisions/ADR-filesystem-owns-disk-state.md). When a digest and a
source file disagree, the source file wins and the digest is regenerated. Copy
[`../reference/TEMPLATE-agents-digest.md`](../reference/TEMPLATE-agents-digest.md) into a content
area as its `AGENTS.md` to start one.

This standard is a default feature of `exobrain-tech`: agents entering any area get an accurate,
up-to-date map before spending attention on the full content. See
[Documentation conventions](../reference/docs-conventions.md) for the document that owns each of
the placement and single-source-of-truth rules the digests rest on.
