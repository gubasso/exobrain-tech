# Prior Art

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
[Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md) refuses, and a
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
[Skills](./skills.md).

The specification that defines the portable package is at <https://agentskills.io/specification>,
read 2026-09-14, and the rule that makes portability work is that an unrecognized frontmatter field
is ignored rather than rejected.

## Shipping skills with the version of the tool

Prisma ships its skills inside the packages that a project installs, "so they always describe the version in use". Its sync writes one copy into each agent root of the project, and the copies "are ordinary files that git tracks". It refuses a folder that is not its own copy. Its notice about stale skills "never changes the exit code", and the page lists the conditions that suppress it. Read 2026-11-13 at <https://www.prisma.io/docs/cli/skills>. The sync code copies file by file and removes the `*` ignore file that older versions wrote into each copy. Read 2026-11-13 at <https://github.com/prisma/prisma-cli/blob/main/packages/cli/src/lib/skills/sync.ts>.

The GitHub CLI installs a skill with `--scope project` by default, and `--pin` names a tag or a commit. Read 2026-11-13 at <https://cli.github.com/manual/gh_skill_install>. The Vercel `skills` installer also defaults to project scope, and it records a computed hash for each project skill in a committed `skills-lock.json`. Read 2026-11-13 at <https://github.com/vercel-labs/skills>.

antfu's `skills-npm` moved to committed relative links with a lock file. The links stay links, and the change notes say that they are broken on a fresh clone until the first install. Read 2026-11-13 at <https://github.com/antfu/skills-npm/pull/45>. Next.js ships version-matched documentation inside its package and asks a project to commit the block that it manages in `AGENTS.md`. Read 2026-11-13 at <https://nextjs.org/docs/app/guides/ai-agents>.

The Agent Skills specification defines no top-level `version` field. Read 2026-11-13 at <https://agentskills.io/specification>. Claude Code resolves a shared name "Enterprise over personal, and personal over project". Read 2026-11-13 at <https://code.claude.com/docs/en/skills>. Codex does not merge two skills with one name, and "both can appear in skill selectors". Read 2026-11-13 at <https://learn.chatgpt.com/docs/build-skills>.

Inference, not stated by any source: a host copy of a tool's skills describes one version while the projects pin several. [Skills follow the project pin](./skills-follow-the-project-pin.md) draws its rules from that.

## Gating a command on a recorded version

cargo-dist records `cargo-dist-version` in the project. A binary of another version refuses, with the help "Rerun 'dist init' to update to this version." Read 2026-11-13 at <https://github.com/axodotdev/cargo-dist/blob/main/cargo-dist/src/errors.rs>. Its updating page says to install the new version and run `dist init`, which updates incrementally. Read 2026-11-13 at <https://axodotdev.github.io/cargo-dist/book/updating.html>.

When the running version does not meet `required_version`, "Terraform prints an error and exits without taking actions." Read 2026-11-13 at <https://developer.hashicorp.com/terraform/language/block/terraform>. An older Terraform also gives an error on a state file version that it does not support. That is a check of the state format, not of version equality. Read 2026-11-13 at <https://developer.hashicorp.com/terraform/tutorials/configuration-language/versions>.

uv and ruff each exit with an error when the running version does not meet `required-version`. Read 2026-11-13 at <https://docs.astral.sh/uv/reference/settings/#required-version> and <https://docs.astral.sh/ruff/settings/#required-version>. prek exits with an error when it is older than `minimum_prek_version`. Read 2026-11-13 at <https://prek.j178.dev/reference/configuration/>.

The Gradle wrapper "invokes a declared version of Gradle, downloading it beforehand if necessary." Read 2026-11-13 at <https://docs.gradle.org/current/userguide/gradle_wrapper.html>. Bundler's `version` setting defaults to `lockfile`, the version that `Gemfile.lock` records. Read 2026-11-13 at <https://guides.rubygems.org/command-reference/bundle-config/>. Since Bundler 2.3, a missing locked version is installed and Bundler restarts with it. Read 2026-11-13 at <https://blog.rubygems.org/2022/01/23/bundler-v2-3>.

Two parts of [A version gate on every command](./a-version-gate-on-every-command.md) are this library's own policy, stated by no source: the exact-equality gate, and its split between a writing command that refuses and a read-only command that warns.

## A tool built by its previous release

The stage0 rustc "is by default the very recent beta `rustc` compiler." Read 2026-11-13 at <https://rustc-dev-guide.rust-lang.org/building/bootstrapping/what-bootstrapping-does.html>. The file `src/stage0` of the rustc repository names that compiler, and a tool regenerates it. Read 2026-11-13 at <https://github.com/rust-lang/rust/blob/main/src/stage0>. Go needs an installed Go compiler to build its toolchain, and `$GOROOT_BOOTSTRAP` can name it. Read 2026-11-13 at <https://go.dev/doc/install/source>.

Inference, not stated by either source: these are analogies for a pinned earlier build. Neither documents the delay that [A tool repository runs its previous release](./a-tool-repository-runs-its-previous-release.md) accepts for a fix to a tool's workflow verbs.

## See also

- [Reference implementations](./reference-implementations.md) — the concrete tools, mapped
  to the concepts above.
