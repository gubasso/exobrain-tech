# 11 — Router and Gates

The lifecycle in [09 — Disposable staging and direct landing](./09-disposable-staging-and-direct-landing.md)
has mechanical steps and judgment steps, and they want different owners. This chapter states the
skill that drives the sequence, and the boundary between a check a program can make and a decision
only a person or an agent can.

## One router skill

A setup domain gets one skill, not one per verb. It drives the whole sequence and owns none of it.

What the router does:

- Discovers the installed documentation, by the three-step route in
  [06 — Documentation and discovery](./06-documentation-and-discovery.md).
- Gathers the project's evidence, in the order
  [10 — Versions and migration](./10-operator-selected-versions-and-agent-guided-migration.md) sets.
- Writes an explicit plan, in prose, before it changes anything. A person reads the plan and can
  refuse it.
- Respects the authorization boundary. An action nobody asked for does not happen because a step
  followed from the previous one.
- Invokes the mechanical surfaces. The staging command, the landing command, the cleanup command,
  and the project's own checks.
- Escalates a semantic decision it cannot make with the evidence it has.

What the router does not do: restate the projection, the ownership classes, the landing order, or
the failure behavior. Those have chapters, and a skill that copies them has made a second copy that
drifts. It links, the way [03 — Skills](./03-skills.md) requires of any skill's body.

One skill rather than several also keeps the trigger surface legible. Three skills for staging,
landing, and cleanup compete for one request, and the description that would disambiguate them is
longer than the boundary clause a single skill needs.

## Gates split by judgment

A gate is a check that fails a change. Which checks belong in a program is decided by one question:
is the answer a fact or an opinion.

| A program decides                                     | A person or an agent decides                         |
| ----------------------------------------------------- | ---------------------------------------------------- |
| A destination path resolves inside the target         | Whether a configuration the project tuned still fits |
| A record matches its declared schema                  | Whether a semantic migration preserved intent        |
| An ownership record covers every written file         | Which documentation a project wants landed           |
| A marked region's markers are present and well formed | Who owns a file the record does not attribute        |
| A link resolves                                       | Whether a retired file still earns its place         |
| A document is inside its budget                       |                                                      |

The left column is deterministic: the same inputs give the same verdict, and a failure names the
rule it broke. The right column is not, and a program that renders an opinion as a verdict has made
a decision nobody reviewed.

The boundary matters most where it is tempting to cross. Ownership of an unattributed file looks
decidable, because the content often matches what the tool would generate. It is not decidable, and
[08 — Candidate artifacts and receipts](./08-candidate-artifacts-and-receipts.md) refuses rather
than guesses.

## A gate proves its claim, or narrows it

A check states a claim, and the mechanism has to be capable of the claim. The failure mode is a
proxy that resembles the property closely enough that nobody notices it is not the property.

The clearest case is a declared build target. The claim "this target builds" is proved by invoking
the compiler for that target, before the release decision. Nothing else proves it. A search over the
source, a parser reading the manifest, or a rule about which modules may name a platform can enforce
a narrower policy about how the source is written, and that policy can be worth having. It cannot
stand in for compilation, because source that looks right is not source that compiles.

The same shape recurs. A text scan over a configuration file is not the configuration's evaluated
value. A count of matching lines is not a schema check. A heuristic over content is not a record.

So a check states what it holds and what it does not, and where the full claim needs a second
mechanism, the two are named and the specification says which holds which half. A check that
overstates its reach is worse than no check, because the reach is what people rely on.

## See also

- [03 — Skills](./03-skills.md) — the package the router is an instance of.
- [09 — Disposable staging and direct landing](./09-disposable-staging-and-direct-landing.md) — the
  sequence the router drives.
- [10 — Versions and migration](./10-operator-selected-versions-and-agent-guided-migration.md) — the
  judgment the router escalates.
- [cli-design/09 — Testing and quality](../cli-design/09-testing-and-quality/README.md) — what a
  project's own checks are made of.
