# 07 — Lifecycle

Documents change, and the changes have to leave the corpus consistent. This chapter owns how a spec
changes, how exploratory material becomes durable, and how facts that expire are kept honest.

## Changing a spec

- When current behavior changes, the author MUST update the spec in the same change.
- A change MUST state what was added, modified, or removed, and MUST NOT restate what it left alone.

Three operations, and nothing else.

| Operation | Means                                           | Leaves behind       |
| --------- | ----------------------------------------------- | ------------------- |
| Add       | a new requirement binds                         | a new rule ID       |
| Modify    | an existing requirement now says something else | the same rule ID    |
| Remove    | a requirement stops binding                     | nothing in the spec |

Modify keeps the ID when the rule is still about the same thing, so every commit and comment citing it
still resolves. It gets a new ID when the subject changed, which makes it an add and a remove.

Remove deletes the requirement outright. No strikethrough, no `deprecated` marker, no note saying the
rule used to exist. The commit holds the diff and a decision record holds the reasoning.

## When a change earns a decision record

Add a record when the change clears the threshold in [04 — Decisions](./04-decisions.md): cross-cutting,
expensive to reverse, constraining, or rejecting a plausible alternative.

A change that adds an enforceable rule and clears the threshold produces two artifacts in one commit:
the requirement in the spec, and the record explaining the choice. The record cites the rule ID.

Most changes clear neither bar and produce only a spec edit. That is the normal case.

## Drafts

`.draft/` at the project root is the workshop, and it is gitignored. Discovery notes, raw outlines,
copied issue text, and half-shaped arguments live there.

- Exploratory material MUST stay outside the docs root.
- Promotion MUST be a rewrite into the owning zone.
- A promoted draft MUST be deleted.

Promotion is a rewrite because a draft contains uncertainty, repeated facts, and abandoned options,
and the durable document should contain only the result. Keeping the draft afterwards leaves a second
place for a reader to mistake for truth.

If a draft tangles exploration with something the project is already working under, split it: the
binding part goes to the plan zone under version control, and the rest stays in `.draft/` until it
resolves.

Promotion also strips project-private context when the target is project-agnostic material. Replace
local people, hosts, incidents, and workspace paths with placeholders; keep concrete public names only
where they are necessary examples.

## Perishable facts

Some documents hold facts that expire without any local change: benchmark results, vendor pricing,
model and tool rosters, external API shapes, security advisories, dependency lifecycle dates, platform
support matrices.

- A document holding a fact that depends on an external source MUST have an entry in the tracking
  registry.

The registry is machine-readable, one entry per artifact, and it records enough for a human or an
agent to know what is stale and how to check it.

```yaml
# <root>/reference/tracking.yaml
tracked:
  - path: _docs/reference/model-pricing.md
    last_checked: 2026-06-20
    cadence: 30d
    why: provider prices change without notice
    revalidate: re-fetch from the provider's official pricing page
    dependents:
      - _docs/guides/cost-estimation.md
```

The registry describes; it never becomes a second copy of the fact it tracks.

Revalidation is a scan for overdue entries, a re-fetch from the authoritative source, an update to the
artifact, and a bump to `last_checked`. Overdue is deterministic and belongs to tooling. Deciding what
the new truth is requires judgment and belongs to a person.

- An agent that cannot re-verify a claim MUST report it rather than overwrite it.

Do not track stable conceptual documents because they are old. A rule does not expire on a timer; it
is changed or removed by a decision.

## External-system bugs

A bug in a dependency, platform, or service that the project must work around is recorded once, in
reference, with the workaround and the condition that retires it.

- A temporary workaround MUST record the condition under which it is removed.

```yaml
# <root>/reference/known-issues/<slug>.md
---
upstream: https://github.com/<org>/<repo>/issues/1234
affects: <component>
workaround: <what the project does instead>
retire_when: upstream release >= 2.4.0
---
```

The retire condition is what stops a workaround outliving its bug. Without it the workaround becomes
permanent by default, and the next reader assumes it was a design choice.

When the condition is met, remove the workaround and delete the record in the same change. The record
existed to describe a live constraint; a constraint that lifted leaves no trace in prose.

## Sources

- OpenSpec, on delta descriptions stating what changes without restating the unchanged:
  <https://github.com/Fission-AI/OpenSpec/blob/main/docs/concepts.md>
