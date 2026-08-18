# 09 — Spec to Code

A spec may exist before the code it binds. This chapter owns the seam between the two: how a
requirement written first becomes work, how the work declares what it changed, and how coverage is
derived rather than stored. It states the contract any planning tool can satisfy; it names none.

## A failing verification is an unimplemented rule

Every requirement carries a `Verify:` command that exits non-zero on violation. Before the behavior
exists, the command fails — and that failure is not a defect in the spec. It is the definition of
"not yet built."

- An author MAY write a requirement whose verification command does not yet pass.
- A unit of work enacting a requirement MUST leave its verification command passing.

This is what makes the spec a legitimate greenfield artifact. The requirement states the agreement,
the failing command states the distance, and the work closes it. Writing the check before the
behavior is the same discipline as writing the failing test first, applied to documentation.

## Requirement state is derived, never stored

Run the verification commands of a domain and the failures are its unimplemented requirements. That
is the whole status system.

- A specification MUST NOT carry a status marker on a requirement.

A stored status (`status: implemented`, a checkbox, a phase column) is a second copy of a fact the
command already decides, and the copy drifts the first time behavior changes without the marker.
Derived state cannot disagree with the code, because it is recomputed from the code on every ask.

## Precedence is phase-dependent

[00 — Model](./00-model.md) owns precedence and states both directions. The marker that selects the
direction lives here: a unit of work is in flight for a rule while an open entry document cites that
rule's ID. While it is, the spec states the agreement and divergent code is the defect. When no work
cites the rule, the code is the observed truth and a divergent spec is the defect.

## The entry document enacts rules by ID

[05 — Agent Context](./05-agent-context.md) gives each unit of work one entry document that names
its sources by path. When the work changes agreed behavior, path-level naming is not enough: the
entry document also names the rules, so enactment is greppable.

- An entry document that changes agreed behavior MUST cite the affected rule IDs.
- An entry document citing a spec change MUST type it as `ADDED`, `MODIFIED`, or `REMOVED`.

The three types are the three operations of [07 — Lifecycle](./07-lifecycle.md), stated from the
work's side. One clause per affected rule, on the line that names the owning spec:

```markdown
- `_docs/specs/SPEC-auth.md` — ADDED `auth:token-expiry-is-bounded`
- `_docs/specs/SPEC-auth.md` — MODIFIED `auth:refresh-requires-reauth`
```

The clause grammar is fixed so a command can check the shape: the type in capitals, then the rule ID
in inline code, matching `[a-z0-9-]+:[a-z0-9-]+`. A typed clause whose ID token is malformed is a
gate failure; whether a story that changed a spec declared the clause at all is a review question,
because no command can see the omission.

## Coverage is a grep

The rule ID is one string in three record sets: the spec defines it, a decision record argues for
it, an entry document enacts it. Traceability is therefore derived on demand, in both directions,
from the records that already exist.

```bash
rg -o '^### `([a-z0-9-]+:[a-z0-9-]+)`' -r '$1' _docs/specs | sort -u > /tmp/agreed
rg -oe '(ADDED|MODIFIED|REMOVED) `[a-z0-9-]+:[a-z0-9-]+`' -r '$0' <plan-zone> \
  | rg -o '[a-z0-9-]+:[a-z0-9-]+' | sort -u > /tmp/enacted
comm -23 /tmp/agreed /tmp/enacted
```

The third command prints the agreed rules no work has enacted: the spec-first backlog, computed from
two record sets and stored in neither.

- A project MUST NOT maintain a stored coverage artifact.

A traceability matrix, a rules-to-stories index, or a backlog file restates what the greps derive,
and each is the filesystem-index shape [00 — Model](./00-model.md) forbids: a copy kept because the
records exist, drifting on the next change to either side.

## What the planning tool owes

This framework does not name a planning tool. Any tool serves whose work record satisfies the
contract the rules above already state: one entry document per unit of work, sources named by path,
spec changes cited by typed rule ID, and the record readable by the greps in this chapter. The
inverse dependency is also bounded: the specs never name the tool, so replacing it edits the plan
zone and nothing under `specs/` or `decisions/`.

## Unenforced

Two rules in this chapter no command can decide: that a unit of work which changed a spec declared
the typed clause at all, and that the cited type matches the diff. A gate checks every declared
clause and cannot see an omitted or mistyped one; the reviewer compares the spec diff against the
entry document. [08 — Gates](./08-gates.md) carries both in the unenforced list.

## Sources

- GitHub Spec Kit, on tests written first and confirmed to fail before implementation:
  <https://github.com/github/spec-kit/blob/main/spec-driven.md>
- OpenSpec, for the `ADDED` / `MODIFIED` / `REMOVED` delta typing:
  <https://github.com/Fission-AI/OpenSpec/blob/main/docs/concepts.md>
- AWS Kiro, on tasks tracing to requirement identifiers:
  <https://kiro.dev/docs/specs/>
