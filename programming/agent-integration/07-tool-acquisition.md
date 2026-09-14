# 07 — Tool Acquisition

Before a tool can land anything into a project, the project has to obtain the tool, at a version it
records, through the manager it already runs. This is the mechanism layer from the project's side:
what a tool author builds so a consumer can pin the tool, and what a consumer checks before adopting
one.

The subject is a development-time tool a project depends on. Installing an application for a person
is not the subject, and neither is resolving a library at build time.

## The two axes

Two independent facts decide how a project pins a tool.

A manager is what the project declares its development tools in. It owns a file in the repository,
and that file records a version. Four are in common use: a Nix flake input, mise, asdf, and devbox.
A shell loader such as direnv is not a manager. It loads an environment and records no version, so a
tool reports it beside the list and has nothing there to move.

A venue is where a release is published: a package registry, a flake the repository serves at every
tag, a forge release carrying prebuilt archives, or a distribution package built by a build service.

The two are independent. A manager consumes some venues and not others, and a venue is reachable
from some managers and not others. A tool that offers itself through one manager while it offers
every other project a matrix has the wrong shape, because it has assumed its consumers run what its
author runs.

## The matrix

Cross the two enumerations and classify every pair once. A pair either renders, meaning the tool can
produce the exact fragment that pair needs, or it is manual. Rendering splits again on the target's
state: where the target already owns the manager file the tool prints the fragment for the project
to place, and where the target has no such file the tool may seed it.

A manual pair carries a reason from a closed set. The set is closed because an open one becomes a
free-text field, and a free-text field becomes a guess. Four reasons cover the pairs below: no
package-set attribute is known for that manager, a source hash the tool cannot compute offline is
required, the manager has no backend for that venue, and no published plugin installs the tool.

| Manager | Registry                  | Flake                  | Forge release          |
| ------- | ------------------------- | ---------------------- | ---------------------- |
| flake   | manual, attribute unknown | renders                | manual, hash needed    |
| mise    | renders                   | manual, no backend     | renders                |
| asdf    | manual, plugin unknown    | manual, plugin unknown | manual, plugin unknown |
| devbox  | manual, attribute unknown | renders                | manual, hash needed    |

That table is one tool's matrix, not the shape of the world. What transfers is the discipline: every
pair has a verdict, and a gap says why it is a gap.

A venue joins the list only where the tool's own release publishes to it under continuous
integration, the same standard a project applies to a platform it claims support for. A venue
nothing can pin inside a repository takes no row at all. A distribution package is the example: the
host's package manager chooses the version, and no file in the project records or moves it, so the
reason is recorded and the venue stays out.

## The pin

Each manager records the version in its own form, and a sync moves what that manager records.

```text
flake input      github:<owner>/<repo>/<tag> in flake.nix, and the input's node in flake.lock
mise, registry   "cargo:<package>" = "<version>"
mise, release    "ubi:<owner>/<repo>" = { version = "<version>", exe = "<binary>" }
asdf             <tool> <version>, one line in .tool-versions
devbox           "github:<owner>/<repo>/<tag>#<output>", one entry in the packages array
```

A registry entry names the package as the registry knows it. An archive entry names the binary
inside the archive. Where a project's package name and its binary name differ, writing one where the
other belongs is the first mistake available here, and it stays invisible until an install fails.

One target runs one bump mechanism. Two mechanisms over the same files on the same trigger either
fight or silently undo each other, and there is no correct behavior to fall back on. A tool reports
a target ready only where the wire is in place and no artifact of a predecessor remains, and it
refuses to add a second manager to a target one manager already pins.

## Keeping the pin fresh

A pin a person moves by hand goes stale. The loop that moves it without a person has four properties
a consumer can check for.

One transaction. Where the manager records two facts, a tag and its locked node, both move or
neither does. A tag whose lock did not follow is a version the environment has not taken. Where the
manager records one fact, the sync moves that one fact in place, because a second file would be a
second pin.

One rate limit. The sync runs from the project's shell entry, which fires on every directory entry,
so a stamp kept per checkout holds it to at most one attempt a day.

Two registers. The caller selects how the run reports. The shell-entry caller stays silent and exits
0 on every outcome, a failed sync included, because a development shell that refuses to open over a
stale pin is worse than the stale pin. The operator caller reports every outcome against an
exit-code matrix.

No write the consumer did not review. What the loop leaves behind is a diff over the manager file
and its lock. It never commits and never pushes.

```text
directory entry -> shell loader -> sync, shell-entry caller
                                     stamp   at most one attempt today
                                     fence   both files move or neither
                                     result  a diff waiting for review
```

Two conditions switch the loop off rather than complicate it: a continuous-integration environment,
where the pin is whatever the checkout carries, and one variable the operator sets. Where the shell
loader watches the files it loads from, a moved pair reloads the environment on the next prompt with
no further step.

## What a tool owes its consumers

A tool distributed as another project's development dependency owes its consumers this loop. It is
an obligation of the tool, not an extra the consumer earns by asking.

Three things discharge it.

- The tool ships the sync verb.
- Its status verb reports, offline, whether a target carries the wire, and exits 0 for every state
  it reports, because a report is not a verdict.
- Its setup path raises a missing wire as a decision the operator answers, instead of a verb the
  operator has to discover.

The failure this prevents is a consumer pinned at a release nobody moves. It is invisible by
construction. Everything works, at the version that was current on the day of the landing. Nothing
fails, so nothing reports it, and the gap surfaces when somebody reads a manifest and compares it to
a release list.

The alternative, where the consumer reads a manual and wires the loop by hand, is refused. A
capability nobody offers reaches only the operator who already knew it existed, which is usually the
tool's own author.

Where no manager is wired, there is no pin to keep fresh and the tool says nothing. The obligation is
to offer the loop where one is possible, never to press it on a project that pins nothing.

## Serving against editing

A tool prints the fragment with its anchor and its placement, and edits no manifest the project
owns. It seeds a file only where the project has none.

The reason is ownership. A manager file belongs to the project. A tool that edits it is a second
writer into a file with one owner, and a lexical scan of another project's configuration language is
not the judgment a write into it needs. Printed text also costs the automated path nothing, because
an agent applies a fragment as readily as a person does.

The freshness loop is the one bounded exception, and the consumer authorized it by landing the line
that calls it. The four properties above are what bound it.

## Growing the lists

A new manager is one variant plus its rows. A new venue is the same. That is what a matrix buys over
a list of supported pairs: the list of gaps grows with the list of features, and neither can be
extended quietly.

Two habits keep the lists honest. State the list once, in the tool, and restate it in no skill and
no chapter, because a prose copy drifts at the first addition and nothing fails when it does. And
carry a dated citation from each venue's and each manager's own documentation, so the form the tool
renders can be checked against the source it came from.

## What this is not

It is not release automation. Nothing here publishes, tags, or cuts a release; the subject begins
once a release exists. It is not a dependency resolver either. The project pins one tool at one
version, and the manager installs it.

What a tool may write into a project's own directories when it does write is
[11 — XDG scaffolding](../cli-design/11-xdg-scaffolding.md). Who selects the version, and what an
agent owes an operator when moving between two of them needs judgment, is
[10 — Versions and migration](./10-operator-selected-versions-and-agent-guided-migration.md).

The same two axes serve a third-party dependency a project takes, rather than the tool itself, with
one difference. There the tool that served the fragment is not the tool that moves the pin, so it
adds no mover and the manager that took the pin keeps its own freshness verb. The obligation above
falls on whoever published that dependency.

## Sources

Read on 2026-09-14.

- Nix flake references and what a lock file records for an input:
  <https://nix.dev/manual/nix/latest/command-ref/new-cli/nix3-flake>
- The mise cargo backend, its key form, and its fallback to a source build:
  <https://mise.jdx.dev/dev-tools/backends/cargo.html>
- The mise ubi backend, its key form, and the `exe` option for a binary the repository does not name:
  <https://mise.jdx.dev/dev-tools/backends/ubi.html>
- The asdf `.tool-versions` format and its per-tool plugin requirement:
  <https://asdf-vm.com/manage/configuration.html>
- Flake references in a devbox package list, and output selection:
  <https://www.jetify.com/docs/devbox/configuration/>
- The direnv stdlib, on loading a flake environment and on watching a file:
  <https://direnv.net/man/direnv-stdlib.1.html>
