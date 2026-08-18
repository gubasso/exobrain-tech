# A resolved external bug leaves a diagnostic, not an archive

## Context and Problem Statement

A bug in a system this project does not own accumulates working material while it is live: a
reproduction, an escalation, a workaround and the code sites carrying it. The existing rule deletes
the record when the bug is fixed, which is right about the constraint and wrong about the symptom —
a symptom that recurs a year later gets re-investigated from nothing. The alternative on offer was an
archive directory of resolved cases, which is a store kept because the cases existed.

## Considered Options

- Delete the record on resolution and keep nothing.
- Collapse each resolved case into a summary under a `resolved/` directory.
- Delete the record, and where the symptom could recur and be misread, leave a diagnostic entry.

## Decision Outcome

Chosen option: delete the record and leave a diagnostic where recurrence is plausible. The durable
value of a closed case is recognition, and that is the shape `11-operational.md` already specifies:
symptom, signal, expected result, interpretation. An archive would be a second document class holding
the same half, plus the halves the log already holds.

`07-lifecycle.md` also gains the state vocabulary the working phase needs. The pair that earns it is
`mitigated` against `masked`: a permanent guard and a temporary workaround look alike in a diff and
retire oppositely, so a record that cannot tell them apart cannot say what happens on the fix.
`09-spec-to-code.md` owns the code side: every suppression names its case and its exit, and an
expected failure is strict so the upstream fix turns the suite red.

## Consequences

- Good: nothing is kept because it existed. What survives survives on recognition value.
- Good: a strict expected failure makes the upstream fix announce itself, so masks stop accumulating.
- Bad: whether a symptom could recur and mislead is a judgment, made once, with no command behind it.
- Bad: the investigation trail lives only in the log, so recovering it means reading history.

## Status

Accepted.
