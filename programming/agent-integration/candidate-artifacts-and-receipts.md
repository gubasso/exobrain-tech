# Candidate Artifacts and Receipts

A tool that writes files into a project needs two facts at once: what the project should hold now, and what this tool put there last time. This chapter names both, and keeps them apart, because a design that conflates them grows a protocol between releases that nobody wanted to write.

## The candidate

A candidate is the set of target-specific artifacts the installed executable projects from its own sources, for one invocation, against one target's configuration.

Three things it is not.

- It is not another release's bundle. The running executable projects from what it carries. It does not decode content produced by a different version.
- It is not a stored plan. Nothing is serialized for a later run to execute. The candidate is computed, used, and discarded, and the next invocation computes it again.
- It is not installed state. It is desired state, and the difference between the two is the whole subject of the comparison a person or an agent performs.

The candidate depends on three inputs and nothing else: the executable's own sources, the target's resolved configuration, and the target's observed capabilities. Same three inputs, same candidate.

## The receipt

The receipt is the record of attributed installed state, held centrally in the target. It answers one question: which files in this project did this tool put here, and in what form.

| Field             | What it records                                             |
| ----------------- | ----------------------------------------------------------- |
| Producer identity | Which tool wrote the entry                                  |
| Producer version  | Which version of that tool wrote it                         |
| Resolved inputs   | The configuration the candidate was computed from           |
| Destination       | The path in the target                                      |
| Ownership class   | Which of the three classes below the destination falls into |
| Placement         | For a marked region, where inside the file the region sits  |
| Digest            | The content digest at the moment of writing                 |

The digest is what turns "this tool wrote the file" into "this tool wrote the file and nobody has touched it since". An entry whose digest no longer matches the file on disk is an edited file, which is a fact the tool reports rather than a conflict it resolves.

Reading a receipt an older version wrote is a bounded concern: the record has a declared schema, and a reader handles the schemas it knows and says so when it meets one it does not. That is not the same problem as interpreting another release's content, and keeping the two apart is what stops a record format from becoming a protocol.

## Three ownership classes

The classes are semantic. An implementation may name them differently, and the names below are the meanings that have to survive.

| Class          | Who owns it after creation | What the tool does on a later run               |
| -------------- | -------------------------- | ----------------------------------------------- |
| Generated file | The tool                   | Replaces it, where the digest still matches     |
| Seeded file    | The project                | Creates it when absent, never rewrites it       |
| Marked region  | The tool, inside the file  | Replaces the region, leaving the rest untouched |

A generated file has no content the project is expected to edit. A seeded file is a starting point the project takes over the moment it exists, which covers configuration a project tunes and state a project accumulates. A marked region is the tool's content living inside a file the project owns, and the markers say where the region begins and ends. The project's side of the generated class, which hooks exclude it, is in [project-bootstrap/04 — Generated files](../project-bootstrap/04-quality-gates.md#generated-files).

## Provenance lives in the receipt

A generated file carries no watermark. No version comment, no "do not edit" header bearing a release number, no embedded hash. The receipt owns provenance, and a second copy inside the file is a second copy that goes stale and that a reader cannot distinguish from content.

The skills that a tool lands follow the same rule. The record holds their version, and the skill files carry no stamp, as [Skills follow the project pin](./skills-follow-the-project-pin.md#the-record-holds-the-version) states.

Markers are the one exception, and they are not provenance. A marker says where a region starts and ends, so the tool can replace the region without reading the rest. It names no version.

## Missing provenance reduces certainty

A receipt can be absent, unreadable, or incomplete. Each of those makes the tool know less. None of them makes the tool allowed to do more.

The rule that follows is the sharp one: an unattributed whole file at a candidate's destination is never overwritten. Not when the content looks generated. Not when the path is one the tool would have chosen. The tool reports the collision and stops there, because the alternative is destroying work whose author it cannot identify.

Confidence can be recovered from outside the receipt. Version control history shows when a file arrived and in what shape. An agent reads that history, the project's own documentation, and the file itself, and reaches a judgment a person can review. What the tool does not do is fetch an older release of itself to manufacture the record it is missing.

## A destination the candidate stops producing

A destination the receipt attributes, and the current candidate no longer produces, leaves the receipt. What happens on disk depends on whether the bytes are still the tool's own.

A whole generated file whose bytes match the receipt digest is retired. The landing deletes it. The preview and the stage name each retired file before the landing runs.

Every other such destination is dropped: an edited generated file, a seeded or state file, a marked region, and a file the project's state withholds. A dropped destination stays on disk, and the project owns it from that landing. A check reports a dropped file that still acts, such as an edited release workflow.

The digest is the argument for the deletion. It proves that the bytes are the tool's own, and the earlier release reproduces them, so a deletion loses nothing. A file left in place can keep acting. A release workflow that a new release engine replaced still runs on every push, beside the engine.

## The cases that decide the design

Each of these is a state the tool meets in practice, and the behavior is what the chapter above produces.

| State                                                | What the tool does                             |
| ---------------------------------------------------- | ---------------------------------------------- |
| Generated file, digest matches                       | Replaces it                                    |
| Generated file, digest differs                       | Reports the edit and leaves the file           |
| Seeded file, present and changed                     | Leaves it, because the project owns it         |
| Attributed file, absent from disk                    | Creates it again                               |
| Destination occupied, no attribution                 | Reports the collision and writes nothing       |
| Marked region, markers absent or malformed           | Reports the file as unresolvable and leaves it |
| Generated file absent from candidate, digest matches | Deletes it, which retires it                   |
| Other attributed destination, absent from candidate  | Leaves it to the project, which drops it       |

Three of these rows carry the design. An unattributed collision is a refusal rather than a heuristic. A retirement deletes only bytes the receipt vouches for, and anything else is dropped. Each row keeps the property that the tool never destroys something it cannot account for.

## See also

- [The four layers](./model.md): where a tool's mechanism sits among what a project carries.
- [Documentation and discovery](./documentation-and-discovery.md): the distinction between project artifacts and tool references that decides what a candidate contains.
- [Disposable staging and direct landing](./disposable-staging-and-direct-landing.md): the lifecycle that renders a candidate twice and writes the receipt last.
- [Versions and migration](./operator-selected-versions-and-agent-guided-migration.md): the confidence classes a missing or partial record produces, and what none of them authorizes.
