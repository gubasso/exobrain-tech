# 99 — Checklist

Walk this before shipping. Each section names the chapter that owns its rules, and restates none of
them. An item that fails sends you to the owner, not to this page.

## Governance

- [ ] The root instruction file is hand-authored rules and routing, and it points rather than
      restates.
- [ ] A rule that binds one subtree lives in that subtree's file, reached by a plain pointer.
- [ ] A per-directory digest is generated, carries frontmatter, and states no rule of its own.
- [ ] The file is short enough to be read rather than skimmed.

Owner: [01 — Instruction files](./01-instruction-files.md)

## Playbook

- [ ] The directory name and the frontmatter `name` match, and `name` and `description` are present.
- [ ] No frontmatter field the target runtime rejects, and no angle bracket anywhere in it.
- [ ] The description states what the skill produces, the phrasings a person types, and the boundary
      against a near sibling.
- [ ] Every paragraph of the body survives the deletion test.
- [ ] Every reference pointer carries the condition for reading it, and the package is one level
      deep.
- [ ] Every deterministic routine past a trivial one-liner is a script, and judgment stayed prose.
- [ ] Each script runs without a terminal, bounds its output, and returns a distinct exit code.
- [ ] Every tool dependency is declared, checked, and fails legibly.

Owner: [03 — Skills](./03-skills.md)

## Safety

- [ ] Fetched and reader-supplied content is handled as data, at the point the skill reads it.
- [ ] The skill asks for the smallest tool set the task needs.
- [ ] Model output is validated before a shell, a path, or a parser consumes it.
- [ ] A destructive step shows its plan, waits, and requires an explicit target.
- [ ] No secret appears in a prompt, a log, an example, a fixture, or an error.
- [ ] The description says whether the skill contacts the network.

Owner: [15 — Skill security](./15-skill-security.md)

## Distribution

- [ ] One authored package, materialized into each root, with one owner per installed name.
- [ ] The installer writes only what it declared and removes only what it wrote.
- [ ] A file the record does not claim stays, whoever wrote it.
- [ ] Each file is written whole, and the record is written last.

Owner: [04 — Skill distribution](./04-skill-distribution.md)

## Durable knowledge

- [ ] Project artifacts and tool references are distinguishable, and only the first kind lands.
- [ ] One always-loaded line names the entry point, and nothing embeds the corpus.
- [ ] The index carries a stable id, a summary, the aliases, and the kind for each topic.
- [ ] A reader returns one exact topic by id.
- [ ] The references a tool serves are the ones the running version was built with.

Owner: [06 — Documentation and discovery](./06-documentation-and-discovery.md)

## Landing

- [ ] Acquisition belongs to the project's manager, and landing installs, fetches, and decodes
      nothing.
- [ ] The candidate is computed from the executable's own sources, the resolved configuration, and
      the observed capabilities, and from nothing else.
- [ ] Production reads no staged byte. Removing the stage first changes nothing it produces.
- [ ] Every path and collision is validated before the first write.
- [ ] An unattributed whole file is refused rather than overwritten.
- [ ] A retired destination is reported and released, never deleted.
- [ ] The central receipt is written last.
- [ ] The failure text promises a whole-file boundary and no transaction, names the paths observed
      completed, and says rerun is supported.
- [ ] Cleanup is its own command, identifies the stage by its receipt, and refuses a symlink, a
      filesystem or home root, the project root, and any ancestor of it.

Owners: [08 — Candidate artifacts and receipts](./08-candidate-artifacts-and-receipts.md),
[09 — Disposable staging and direct landing](./09-disposable-staging-and-direct-landing.md), and
[10 — Versions and migration](./10-operator-selected-versions-and-agent-guided-migration.md)

## Routing and checks

- [ ] One router skill drives the sequence, links the rules, and restates none of them.
- [ ] Every mechanical gate decides a fact, and every opinion is escalated.
- [ ] Every check states what it holds, and no proxy stands in for the property it resembles.
- [ ] A declared build target is proved by invoking its compiler.

Owner: [11 — Router and gates](./11-router-and-gates.md)

## Evidence

- [ ] Trigger fixtures cover positive, negative, and near-miss prompts.
- [ ] A behavior fixture states the expected artifacts and the deterministic assertions.
- [ ] Adversarial fixtures cover the threats the skill plausibly faces.
- [ ] A nondeterministic suite runs at least ten trials and records the rate.
- [ ] The recorded result names the runtime, the model, the date, and the trial count.
- [ ] A rewrite is compared against the result from before it.

Owner: [05 — Evaluations](./05-evaluations.md)
