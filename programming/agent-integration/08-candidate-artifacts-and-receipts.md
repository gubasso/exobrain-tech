# 08 — Candidate Artifacts and Receipts

A tool that writes files into a project needs two facts at once: what the project should hold now,
and what this tool put there last time. This chapter names both, and keeps them apart, because a
design that conflates them grows a protocol between releases that nobody wanted to write.

## The candidate

A candidate is the set of target-specific artifacts the installed executable projects from its own
sources, for one invocation, against one target's configuration.

Three things it is not.

- It is not another release's bundle. The running executable projects from what it carries. It does
  not decode content produced by a different version.
- It is not a stored plan. Nothing is serialized for a later run to execute. The candidate is
  computed, used, and discarded, and the next invocation computes it again.
- It is not installed state. It is desired state, and the difference between the two is the whole
  subject of the comparison a person or an agent performs.

The candidate depends on three inputs and nothing else: the executable's own sources, the target's
resolved configuration, and the target's observed capabilities. Same three inputs, same candidate.

## The receipt

The receipt is the record of attributed installed state, held centrally in the target. It answers
one question: which files in this project did this tool put here, and in what form.

| Field             | What it records                                             |
| ----------------- | ----------------------------------------------------------- |
| Producer identity | Which tool wrote the entry                                  |
| Producer version  | Which version of that tool wrote it                         |
| Resolved inputs   | The configuration the candidate was computed from           |
| Destination       | The path in the target                                      |
| Ownership class   | Which of the three classes below the destination falls into |
| Placement         | For a marked region, where inside the file the region sits  |
| Digest            | The content digest at the moment of writing                 |

The digest is what turns "this tool wrote the file" into "this tool wrote the file and nobody has
touched it since". An entry whose digest no longer matches the file on disk is an edited file, which
is a fact the tool reports rather than a conflict it resolves.

Reading a receipt an older version wrote is a bounded concern: the record has a declared schema, and
a reader handles the schemas it knows and says so when it meets one it does not. That is not the
same problem as interpreting another release's content, and keeping the two apart is what stops a
record format from becoming a protocol.

## Three ownership classes

The classes are semantic. An implementation may name them differently, and the names below are the
meanings that have to survive.

| Class          | Who owns it after creation | What the tool does on a later run               |
| -------------- | -------------------------- | ----------------------------------------------- |
| Generated file | The tool                   | Replaces it, where the digest still matches     |
| Seeded file    | The project                | Creates it when absent, never rewrites it       |
| Marked region  | The tool, inside the file  | Replaces the region, leaving the rest untouched |

A generated file has no content the project is expected to edit. A seeded file is a starting point
the project takes over the moment it exists, which covers configuration a project tunes and state a
project accumulates. A marked region is the tool's content living inside a file the project owns,
and the markers say where the region begins and ends.

## Provenance lives in the receipt

A generated file carries no watermark. No version comment, no "do not edit" header bearing a release
number, no embedded hash. The receipt owns provenance, and a second copy inside the file is a second
copy that goes stale and that a reader cannot distinguish from content.

Markers are the one exception, and they are not provenance. A marker says where a region starts and
ends, so the tool can replace the region without reading the rest. It names no version.

## Missing provenance reduces certainty

A receipt can be absent, unreadable, or incomplete. Each of those makes the tool know less. None of
them makes the tool allowed to do more.

The rule that follows is the sharp one: an unattributed whole file at a candidate's destination is
never overwritten. Not when the content looks generated. Not when the path is one the tool would
have chosen. The tool reports the collision and stops there, because the alternative is destroying
work whose author it cannot identify.

Confidence can be recovered from outside the receipt. Version control history shows when a file
arrived and in what shape. An agent reads that history, the project's own documentation, and the
file itself, and reaches a judgment a person can review. What the tool does not do is fetch an older
release of itself to manufacture the record it is missing.

## Retirement releases a destination

A destination the receipt attributes, and the current candidate no longer produces, is retired. The
tool reports it and drops the attribution, which leaves the file in the project under the project's
ownership.

It does not delete the file. Automatic deletion needs a stronger contract than attribution alone:
the receipt says the tool put the file there, and says nothing about whether anything has come to
depend on it since. Reporting the retirement gives a person or an agent the decision with the
evidence attached.

## The cases that decide the design

Each of these is a state the tool meets in practice, and the behavior is what the chapter above
produces.

| State                                         | What the tool does                                          |
| --------------------------------------------- | ----------------------------------------------------------- |
| Generated file, digest matches                | Replaces it                                                 |
| Generated file, digest differs                | Reports the edit and leaves the file                        |
| Seeded file, present and changed              | Leaves it, because the project owns it                      |
| Attributed file, absent from disk             | Creates it again                                            |
| Destination occupied, no attribution          | Reports the collision and writes nothing                    |
| Marked region, markers absent or malformed    | Reports the file as unresolvable and leaves it              |
| Attributed destination, absent from candidate | Reports the retirement and releases the file to the project |

Two of these rows are the ones that carry the design. An unattributed collision is a refusal rather
than a heuristic, and a retirement is a report rather than a deletion. Both trade a little
convenience for the property that the tool never destroys something it cannot account for.

## See also

- [00 — The four layers](./00-model.md) — where a tool's mechanism sits among what a project carries.
- [06 — Documentation and discovery](./06-documentation-and-discovery.md) — the distinction between
  project artifacts and tool references that decides what a candidate contains.
- [09 — Disposable staging and direct landing](./09-disposable-staging-and-direct-landing.md) — the
  lifecycle that renders a candidate twice and writes the receipt last.
