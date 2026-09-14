# 12 — Prior Art

What already exists for the problems this shelf's landing chapters describe, read from primary
sources on the date each entry records. An entry states what its source says. Where a conclusion is
this library's reading rather than the source's statement, the entry says so.

## Replacing a file without a reader seeing half of it

POSIX specifies `rename()` so that the destination name stays visible throughout: it refers either
to the old file or to the new one, never to nothing. That is the boundary the landing chapter
relies on. Read 2026-09-14 at
<https://pubs.opengroup.org/onlinepubs/9799919799/functions/rename.html>.

Two limits come from the same page. On failure for any reason other than an input or output error,
the destination is unaffected; after an input or output error the standard promises nothing about
either file. And a cross-device rename may fail, which is why the boundary holds for a replacement
within one directory and is not a general file-moving guarantee.

Inference, not stated by the source: a set of renames is not a transaction. The standard describes
one call. Nothing composes several into an all-or-nothing group, which is why the landing chapter
declines to promise one.

## The same boundary over a network filesystem

Network filesystems weaken the guarantee in two ways that implementers report and no specification
promises away.

A rename is not idempotent, so a lost reply followed by a retransmission can report failure for an
operation that succeeded. Clients carry a duplicate-request cache for exactly this. And a client
that has to preserve open-file semantics may move the destination aside before renaming the source
onto it, which opens a window where the destination does not exist.

Read 2026-09-14 at <https://www.mail-archive.com/tech@openbsd.org/msg17349.html>. This is a
developer mailing list rather than a specification, so treat it as a report of implementation
behavior. The consequence the landing chapter takes from it is narrow: an implementation re-observes
a destination rather than trusting a reported failure, and it scopes its replacement guarantee to
the local filesystems it supports.

## Resolving a path that cannot escape

Linux exposes path resolution flags that make containment a property of the call rather than of the
caller's checks. `RESOLVE_BENEATH` refuses any component that is not a descendant of the given
directory. `RESOLVE_IN_ROOT` treats that directory as the root, so an absolute symlink and a `..`
both stop there. `RESOLVE_NO_SYMLINKS` refuses every symlink in the path, which is wider than
`O_NOFOLLOW`, since that flag governs the final component alone. `RESOLVE_NO_XDEV` refuses to cross
a mount point. Read 2026-09-14 at <https://man7.org/linux/man-pages/man2/openat2.2.html>.

This is the mechanism behind the property the cleanup section states. The property is that a walk
holds its boundary. The mechanism is one platform's, and an implementation claims it for the targets
where it proved it.

## Updating a project from a template

Copier records the answers given at generation, then re-renders the template at the version the
project was generated from, diffs that against the project to recover the local changes, applies the
newer template, and replays the diff. Conflicts surface as markers in the file or as a rejected-diff
file beside it. Read 2026-09-14 at <https://copier.readthedocs.io/en/stable/updating/>.

This is the design the landing chapters decline, and the reason is visible in the mechanism rather
than in its quality. Re-rendering the version a project was generated from requires the running tool
to reach and interpret an older release. That is the cross-release dependency
[08 — Candidate artifacts and receipts](./08-candidate-artifacts-and-receipts.md) refuses, and a
tool that embeds its own sources cannot do it at all without fetching a second copy of itself.

## Recording what a tool created

Terraform keeps a state file binding declared resources to the objects it created in a remote
system, because inspecting the world on every operation is neither fast nor reliable. A resource
removed from the configuration without being destroyed leaves an object the person must then delete
or re-import themselves. The documentation says not to edit the file directly, and offers commands
instead. Read 2026-09-14 at <https://developer.hashicorp.com/terraform/language/state>.

Two of those are the receipt's own shape. A record of what this tool created answers a question
inspection cannot, and a destination that leaves the desired set becomes a decision rather than a
deletion. The third, a record a person is told not to edit, is a consequence of a record that
carries more than provenance.

## Discovering an agent's instructions and skills

Four runtimes state their own discovery roots, precedence, frontmatter, activation, and permission
models, and the differences are real enough that no portable rule covers them. Those facts are dated
in each runtime's adapter under `tools/`, and the portable baseline they deviate from is
[03 — Skills](./03-skills.md).

The specification that defines the portable package is at <https://agentskills.io/specification>,
read 2026-09-14, and the rule that makes portability work is that an unrecognized frontmatter field
is ignored rather than rejected.

## See also

- [13 — Reference implementations](./13-reference-implementations.md) — the concrete tools, mapped
  to the concepts above.
