# 03 — Local dev environment

A reproducible, per-project development environment so every contributor (and every CI runner) gets
the same toolchain, and a cross-editor formatting baseline.

## Nix devShell + `.envrc`

A per-project Nix flake devShell pins the toolchain and dev tools declaratively, so the environment
is reproducible rather than "whatever is on your machine". Add an `.envrc` containing `use flake` so
[direnv](https://direnv.net/) loads the shell automatically on `cd`, and reuse the same flake in CI
so local and CI environments cannot drift apart.

The flake shape (a `devShells.default` built with `mkShell`), the direnv wiring, and per-language
starting templates are owned by
[`tools/nix/02 — Per-project devShell`](../../tools/nix/02-per-project-devshell.md) — follow it for
the template; the language binding names the toolchain that goes inside.

A recipe or a script reaches the flake of its own project through `.` or a `git+file://` reference,
never through a `path:` reference, which copies the ignored files of the tree into the store each
time a file changes. [Nix gotcha 1](../../languages/nix/05-tooling-gotchas-resources.md#gotchas-that-bite-newcomers)
explains the fetcher types.

## Who owns each dependency

Each dependency of a project has exactly one owner: the most specific file that declares that kind
of dependency natively. What uses the dependency picks the owner. To place a dependency, ask which
is the most specific file that can declare it natively. The owners run from the most specific to the
most general:

1. The manifest of the stack holds what the code of the project uses. A Python project declares its
   packages, and the dev tools that the project itself runs, such as pytest for its test suite, in
   `pyproject.toml`, and `uv.lock` locks them. A single Python script declares its packages in a
   PEP 723 block. A Rust crate uses `Cargo.toml`, a Node project uses `package.json` and its lock
   file, and a Go module uses `go.mod`.
2. The hook configuration holds what a hook uses. `.pre-commit-config.yaml` pins each hook tool for
   which pre-commit can build an environment, with `rev` and `additional_dependencies`. A tool that
   only a hook uses belongs here, even when it reads the code of the project, such as mypy,
   pyright, or ruff: pre-commit is a guardrail from outside the project, so it owns what it brings.
   A manifest holds such a tool only when the project itself uses it too, or when the hook comment
   states a reason specific to the repository.
3. The devshell holds the major tools: the language toolchains and their managers, such as cargo
   and nodejs, then `pre-commit`, the task runner, and the project tools. It also
   holds each tool that no owner above can declare, such as statix, bats with its libraries, or a
   cargo subcommand, the system libraries that a native build links, and the runtime that
   pre-commit needs to build a hook environment on a clean machine.
4. The host supplies what the project does not own: `git`, `wt`, `bash`, `awk`, `sed`, `grep`, the
   coreutils, the tools that build environments, `nix`, `direnv`, and `uv`, the tools that deploy the
   host, the standard ELF loader, and the
   tools that carry credentials, such as `gh`, `glab`, `ssh`, and `aws`.

One tool has one owner. A tool that one level declares is not declared again at another level. A
runtime that the devshell gives to an owner below it, such as the nodejs of a node hook or the go
of a golang hook, is not a second owner. A manager that installs its own runtime natively owns it:
uv installs the interpreter of a uv project, so neither the devshell nor the host supplies a Python
to it (`project-platform:the-most-specific-native-owner-holds-each-dependency`). The uv convention,
with the interpreter pin and the activation for each environment manager, is
[uv, the interpreter, and the environment manager](../../languages/python/project-bootstrap-spec/00-toolchain-and-layout.md#uv-the-interpreter-and-the-environment-manager).

A recipe reaches a hook tool through `pre-commit run <id>`, and a tool of the manifest through the
runner of the manifest, such as `uv run`, `cargo`, or `npm run`.

A flake never holds a language library. A Python, Node, or Perl package goes to the manifest of its
stack. The devshell keeps the toolchain and its manager, except where the host supplies the manager,
as it supplies uv.

The host supplies the standard ELF loader, `/lib64/ld-linux-x86-64.so.2`. A hook environment can
hold a prebuilt binary that links it, such as the typos wheel. A conventional distribution carries
the loader, and a NixOS host gets it from nix-ld.

When pre-commit cannot build the environment of a hook from a clean environment, the devshell
supplies the missing dependency, and pre-commit still manages the packages of the hook. A project
with `node` hooks puts `nodejs` in its devshell, because the generic node build that pre-commit
downloads does not run on a host without the standard glibc loader, such as a Nix host. A `rust`
hook needs `curl` from the devshell, because `rustup-init` downloads the toolchain with it.

A hook that keeps `language: system` is an exception with a recorded owner and class, as
[04 — Quality gates](./04-quality-gates.md#git-hooks) says.

A Nix flake is the example here, not the rule. The Nix form, with the package list, is in
[`tools/nix/02 — Per-project devShell`](../../tools/nix/02-per-project-devshell.md#git-hooks-pre-commit).

## `.editorconfig`

`.editorconfig` sets whitespace, charset, and final-newline rules that every editor honors, so
formatting is consistent regardless of individual editor config. Keep it aligned with the language
formatter (chapter [04](./04-quality-gates.md)) so the two never fight. The formatter is
authoritative for code, and `.editorconfig` covers the authored files that no formatter owns. A
generated file, such as a lock file, follows its generator, and editorconfig-checker excludes it
([04 — Generated files](./04-quality-gates.md#generated-files)).
