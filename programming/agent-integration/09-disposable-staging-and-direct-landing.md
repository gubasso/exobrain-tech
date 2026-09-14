# 09 — Disposable Staging and Direct Landing

A person or an agent about to let a tool rewrite part of a project wants to see what it would write
before it writes. This chapter states how to give them that without turning the preview into an
input, which is the design mistake the whole pattern exists to avoid.

## The order

Eight steps, and the order is the content.

1. Gather project evidence: the working tree, the receipt, and the history around the destinations.
2. Render the candidate to a stage. Nothing in the project changes.
3. Investigate. Compare the stage with the working tree, read the omissions, the conflicts, and the
   version-matched guidance.
4. Prepare the project. Resolve what the comparison found, on the project's own terms.
5. Render the candidate again, into production. This is a fresh render, not a copy.
6. Verify with the project's own checks.
7. Compare against the stage, which is still there.
8. Clean the stage, under explicit authorization.

## Production never reads the stage

This is the invariant everything else rests on. Production accepts no stage as an input. It reads no
staged byte, no staged decision, and no staged record. Remove the stage between steps 4 and 5 and
step 5 produces exactly what it would have produced anyway.

Staging and production share a projection, not a result. Inside an implementation, one pure function
computes the candidate from the three inputs, and both steps call it. What they do not share is
serialized state passed from the first to the second.

The reason is what the alternative costs. A production step that consumes staged bytes has to answer
a new question every time: is this stage still valid. The project may have moved, the configuration
may have changed, the tool may have been upgraded. Answering it means fingerprints, validity windows,
and revalidation rules, and an implementation that gets any of them slightly wrong writes stale
content with full confidence. Rendering again asks none of those questions, and it costs one
projection.

The second reason is evidential. An agent comparing a stage against a production run learns what
actually happened. An agent watching staged bytes get copied learns nothing it did not already know.

## What the stage holds

The stage is knowledge and evidence, and it is a directory nobody has to trust.

- The candidate artifact tree, as the tool would write it.
- A receipt explaining the stage: what produced it, from which inputs, and when.
- The omissions and the conflicts: every destination the candidate did not produce, and every one it
  could not claim.
- The changelog and the migration guidance for the installed version.
- The version-matched reference material an agent reads to understand any of the above, per
  [06 — Documentation and discovery](./06-documentation-and-discovery.md).

Project documentation the projection selected is an artifact and sits in the tree. The broader
reference corpus is reference material and stays reference material: the stage exposes it so an
agent can read it, and nothing lands it.

The stage persists through step 7. It is evidence during verification, which is exactly when a
person most wants to know what was supposed to happen.

## Landing by ownership

Direct landing writes into the project, and every write is decided by the ownership classes in
[08 — Candidate artifacts and receipts](./08-candidate-artifacts-and-receipts.md).

Validate first. Resolve every destination path, check it stays inside the target, and classify every
collision, before anything is written. A run that discovers a refusal halfway through has already
made half its changes.

Then, per destination:

| Destination state                         | The write                                      |
| ----------------------------------------- | ---------------------------------------------- |
| Absent                                    | Create it                                      |
| Attributed generated file, digest matches | Replace it                                     |
| Attributed marked region                  | Replace the region, leave the rest of the file |
| Seeded or state file the project owns     | Preserve it                                    |
| Occupied, unattributed                    | Refuse, and write nothing there                |
| Attributed, absent from the candidate     | Leave it, and release the attribution          |

The central receipt is written last. Until it is, the project holds new files the record does not
claim, which is recoverable. The reverse, a record claiming files that were never written, is not.

## What the failure behavior actually is

A partial failure is possible and the chapter says so rather than promising it away.

One lock per target prevents two landings overlapping. Replacing a file within its own directory
gives the local filesystems an implementation supports a whole-file boundary: a reader sees the old
content or the new one, never a half-written file. That is the guarantee, and it is the only one.

The set of writes is not transactional. There is no rollback and no all-or-nothing. A failure part
way through can leave new whole files on disk beside a receipt that predates them. Where a rename
may have completed on a filesystem that cannot confirm it, which is the case on some remote
filesystems, the implementation re-observes the destination rather than assuming either outcome. The
command reports the paths it observed completed, so the state is legible rather than guessed.

Recovery is version control. The destinations are files in a repository, the run is visible as a
diff, and reverting is the project's own operation rather than the tool's. Rerunning after a fix is
supported, and it is the normal response: the candidate is computed again from the same three
inputs, and the destinations already correct are already correct.

## Cleanup is its own destructive command

Removing a directory tree is the most dangerous thing in this lifecycle, so it is a separate command
with its own authorization, never a flag on the landing step.

Before removing anything it establishes that the target is a stage this tool made: it reads the
identifying receipt in the directory, and it resolves the root to a real path. It refuses a symlink,
a filesystem root, a home directory, the project root, any ancestor of the project root, and any
directory it cannot identify unambiguously. It holds the validated boundary through the removal, so
a path that escapes mid-walk does not take the removal with it.

It removes exactly one stage, and only after the person or the agent authorized that stage by name.

## Why the stage is never applied

Applying the stage looks like a saving and is not.

It creates a stale-input problem, which creates a validity protocol, which creates fingerprints and
revalidation and a compatibility floor between the version that staged and the version that applies.
None of that exists in a design that renders twice.

And it removes the evidence that made staging worth doing. The comparison in step 7 is between what
was predicted and what happened. Copy the prediction into place and there is nothing left to
compare.

## Portable properties, platform mechanisms

A rule here states a property: bounded access to the stage, no link followed during a walk, a
whole-file boundary on replacement. Those hold anywhere, and an implementation states which
mechanism it uses and on which targets it proved it.

An implementation that sets owner-only modes claims that for the targets it supports and for no
others. A chapter that turned one implementation's platform into a universal requirement would be
stating a mechanism as if it were the property, which is the confusion this section exists to
prevent.

## See also

- [06 — Documentation and discovery](./06-documentation-and-discovery.md) — what the stage exposes
  and what it does not land.
- [08 — Candidate artifacts and receipts](./08-candidate-artifacts-and-receipts.md) — the candidate
  both renders compute, and the record the last write updates.
