# A Tool Repository Runs Its Previous Release

A tool that lands files into projects also has a repository of its own, and that repository needs the same landing: hooks, a workflow, and skills. This chapter states how the repository takes that landing without letting the tree under development manage itself.

## The repository is a consumer

The repository of a tool that lands files is a consumer of the tool's previous release. It pins a released version, as every project does. That version manages the repository's landing, its hooks, and its workflow.

The repository then lives under the same rules as every consumer. [A version gate on every command](./a-version-gate-on-every-command.md) judges it, and its skills follow its pin, as [Skills follow the project pin](./skills-follow-the-project-pin.md) states. A special path for the repository is a path that no consumer tests.

## The built binary runs only under test

The binary built from the tree runs only where the tree is under test:

- the test suite
- a scratch landing in a throwaway target
- a check that names the built binary by its path

Nothing else in the repository calls the built binary. A hook, a recipe, or a workflow step that needs the tool takes the pinned release.

## One check of the tree against itself

One check can run the built binary against the repository's own documents or files, as a test of the tree. The check is read-only. While the record lags behind the tree, the gate warns and lets the check pass, as the read-only column of [the gate's table](./a-version-gate-on-every-command.md#the-table-for-read-only-and-writing) states.

## A payload source is not a landing destination

A payload source never doubles as a destination of the repository's own landing. The payload is what the tool lands into a consumer. Its sources live in folders of their own, apart from the files that the landing writes. The pinned release can then land over the repository and never touch a source.

release-kit keeps the sources of the snippets that it renders in a `snippets/` folder. That folder stays apart from the files that its landing writes. [Reference implementations](./reference-implementations.md#release-kit) maps the rest of that tool.

The description of the payload is a release artifact with its own record. That record stays apart from the repository's own landing record. One record says what the tree ships, and the other says what the pinned release landed.

## A release moves the pin

After a release publishes, one change moves the repository's pin to that release and runs `upgrade`. Every consumer does the same, so the repository is the first consumer to take the release.

## The cost and the benefit

A fix to the tool's workflow verbs reaches its own repository one release later. The repository runs the broken verb until the fix ships and the pin moves. That cost is a consequence of this convention.

The benefit is that a broken build cannot break the repository that must fix it. The tree can fail to build, or build a binary that lands wrong files. The hooks and the workflow still run on the pinned release.

Compilers that build themselves offer an analogy for the pinned earlier build, not a source for the cost. rustc builds itself with a stage0 compiler, by default a recent beta, and its repository names that compiler in the file `src/stage0`. Go builds itself with an earlier Go toolchain, which `GOROOT_BOOTSTRAP` can name. Neither project documents a delay like the one above, so this chapter does not cite them for it.

## The first conversion

A repository that does not yet follow this rule converts once. That first conversion runs with the binary built from the tree, because no earlier release can land the new layout. From the first pin move after that release, only the pinned release writes the repository's landing.

## See also

- [A version gate on every command](./a-version-gate-on-every-command.md): the gate that the repository runs under.
- [Skills follow the project pin](./skills-follow-the-project-pin.md): the skills that the pinned release lands into the repository.
- [Candidate artifacts and receipts](./candidate-artifacts-and-receipts.md): the record of the landing.

## Sources

- rustc dev guide, what bootstrapping does: "The stage0 compiler is by default the very recent beta `rustc` compiler." Read 2026-11-13 at <https://rustc-dev-guide.rust-lang.org/building/bootstrapping/what-bootstrapping-does.html>.
- The rustc repository, `src/stage0`: the file names the bootstrap compiler, and a tool regenerates it to update that compiler. Read 2026-11-13 at <https://github.com/rust-lang/rust/blob/main/src/stage0>.
- Go, installing from source: "The Go toolchain is written in Go. To build it, you need a Go compiler installed", and `$GOROOT_BOOTSTRAP` can name it. Read 2026-11-13 at <https://go.dev/doc/install/source>.
