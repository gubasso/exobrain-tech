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

## Who supplies each tool

Three owners supply the tools of a project, and each tool has exactly one:

- The devshell holds the major tools: the language toolchain, `pre-commit`, the task runner, and
  every tool that a task-runner recipe, a `language: system` hook, or a `language: script` hook
  calls from `PATH`.
- pre-commit manages the tools of a hook whose language gets an environment that pre-commit builds,
  such as `python`, `golang`, `node`, or `rust`.
- The host supplies only the ubiquitous tools: `git`, `wt`, `bash`, `awk`, `sed`, `grep`, and the
  coreutils. `nix` and `direnv` are host tools too, because they enter the devshell, and so are the
  tools that deploy the host itself, such as `nixos-rebuild` and `home-manager`.

A hook or recipe that calls a host tool from `PATH` follows the rule. A project tool that is
missing goes into the devshell, never onto the host. A hook added without its provider in the
devshell passes only on a machine that happens to carry the tool, and fails on a clean machine and
in CI.

When pre-commit cannot build the environment of a hook from a clean environment, the devshell
supplies the missing dependency, and pre-commit still manages the packages of the hook. A project
with `node` hooks puts `nodejs` in its devshell, because pre-commit's node installer cannot fetch
node from a clean environment. A `rust` hook needs `curl` from the devshell, because `rustup-init`
downloads the toolchain with it.

A Nix flake is the example here, not the rule. The Nix form, with the package list, is in
[`tools/nix/02 — Per-project devShell`](../../tools/nix/02-per-project-devshell.md#git-hooks-pre-commit).

## `.editorconfig`

`.editorconfig` sets whitespace, charset, and final-newline rules that every editor honors, so
formatting is consistent regardless of individual editor config. Keep it aligned with the language
formatter (chapter [04](./04-quality-gates.md)) so the two never fight. The formatter is
authoritative for code, and `.editorconfig` covers the authored files that no formatter owns. A
generated file, such as a lock file, follows its generator, and editorconfig-checker excludes it
([04 — Generated files](./04-quality-gates.md#generated-files)).

## Automation

`bootstrap-nix` scaffolds the flake devShell + `.envrc`; `bootstrap-lint` writes the
`.editorconfig` and aligns it with the detected formatter/linter. The setup above is the SoT; see
[07 — Automation with cog](./07-automation-with-cog.md).
