# 99 — Checklist

The review gate for planning and work-execution changes. Each check is derived from the owning chapter
or executable artifact; the owner wins if wording differs.

## Plan and slices

Owners: [01 — Appetite and Scope](./01-appetite-and-scope.md) and
[02 — Plan and Slices](./02-plan-and-slices.md).

- [ ] The unit of work names an appetite fixed before design, in the project's one chosen unit.
- [ ] It declares a non-negotiable core apart from an ordered, negotiable remainder, leaving room to
      cut inside the appetite.
- [ ] It is one directory under `<project>/docs/plan/slices/`, entered through `README.md`, committed
      before work starts, and using the fixed heading list.
- [ ] `tasks.md` exists only for a context reset and restates nothing; `requirements.md` exists only
      for many-to-many acceptance mapping; no `design.md` exists.
- [ ] `Governed by` names individual governing specs and ADRs, never a directory, zone, or "the docs".
- [ ] Acceptance lines are EARS-phrased and name tests that a hook verifies, or test naming was dropped.
- [ ] Every rabbit hole carries a pre-authorized escape.
- [ ] Changes to `Goal`, `Core`, `Appetite`, or `Acceptance` after work starts add a `Revisions` line;
      appetite changes name both satisfied conditions and what was cut first.
- [ ] Scope was cut before budget moved, and no test, review step, security control, or other quality
      constraint was dropped to fit the appetite.
- [ ] Plan status has one owner, uses its closed vocabulary, and open questions name what they block.
- [ ] The slice and milestones heading shapes are wired to hooks, and shape changes amend their arrays.

## Lane record

Owner: [plan artifacts](./plan/README.md).

- [ ] All five lane files exist and every entry satisfies `plan-lane.schema.json`.
- [ ] Every lane entry and slice directory has a matching counterpart, with identical appetite values.
- [ ] Slice ids are unique, dependencies exist and are acyclic, and work in `doing` has no open need.
- [ ] Closed reshaped work names its successor, and open questions do not block only closed work.
- [ ] The schema, example, linter, and cross-file self-test hooks select the plan artifact paths.

## Integration

- [ ] Governing specs and ADRs are named rather than copied into the slice.
- [ ] Finished durable design, decisions, and exact values migrate to documentation-design zones while
      the plan keeps pointers.

## Verification

- [ ] Relative links resolve and markdown checks pass.
- [ ] The plan schema, example lanes, shell linter, and self-test all pass.
