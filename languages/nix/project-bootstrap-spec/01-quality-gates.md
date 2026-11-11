# 01 — Quality gates

The Nix concretion of the general
[quality gates](../../../programming/project-bootstrap/04-quality-gates.md) chapter, for a flake
that is the deliverable. Formatting, linting, and dead-code detection over the Nix code itself, all
gated by `nix flake check`.

## Formatter — `nixpkgs-fmt` or `alejandra`

Pick one canonical formatter and expose it as `formatter.<sys>` so `nix fmt` uses it:

- **`nixpkgs-fmt`** — the conservative, widely-used default.
- **`alejandra`** — an opinionated, faster alternative.

Enforce in CI by checking formatting is clean, e.g.:

```bash
nix fmt -- --check          # via formatter.<sys>
# or directly:
alejandra --check .
nixpkgs-fmt --check .
```

For multi-language repos, `treefmt-nix` wires several formatters behind one `nix fmt`.

## Linter — `statix`

`statix` flags Nix anti-patterns and suggests idiomatic fixes:

```bash
statix check .              # report
statix fix .                # auto-apply safe suggestions
```

## Dead code — `deadnix`

`deadnix` finds unused `let` bindings and function arguments:

```bash
deadnix --fail .            # non-zero exit on findings (CI-friendly)
```

## `nix flake check` — the gate

Expose the format/lint checks as `checks.<sys>.*` derivations so a single command validates the
whole flake (outputs evaluate, checks build/pass):

```bash
nix flake check
```

This is the Nix analogue of "one command fails fast" and is what CI runs. See the output schema for
`checks` in [`03-flake-outputs.md`](../03-flake-outputs.md).

## A flake reference reads the git tree

A command, a script, or a hook that names a flake in the project tree reads its git tree:
`git+file://` with the root, `?dir=` for a flake in a subfolder, `?shallow=1` where CI clones
shallowly, or `.` from the CLI inside the repository. A `path:` reference and a bare-path
`builtins.getFlake` copy the whole folder into the store, ignored files included, as
[gotcha 1](../05-tooling-gotchas-resources.md#gotchas-that-bite-newcomers) explains.

```bash
nix flake check --no-build path:.?dir=nix   # nix-path-ok: the bad example
nix flake check --no-build .?dir=nix
```

A line that holds a legitimate `path:` use, such as a `/nix/store` path, a relative input inside a
generated flake, or a throwaway fixture folder, carries the marker `nix-path-ok: <reason>`. The
hook `no-path-flake-reference` enforces the rule line by line. Add it to `.pre-commit-config.yaml`:

```yaml
- repo: https://github.com/gubasso/exobrain-tech
  rev: vX.Y.Z
  hooks:
    - id: no-path-flake-reference
```

The hook reads each line on its own, so it misses a reference that a program assembles from parts
or that spans two lines.

## Pre-commit wiring — `git-hooks.nix`

Wire `nixpkgs-fmt`/`alejandra`, `statix`, and `deadnix` into git hooks with
[`git-hooks.nix`](https://github.com/cachix/git-hooks.nix), the same project as
`pre-commit-hooks.nix`. It
generates a `pre-commit` install step and can also expose the same hooks as a
`checks.<sys>.pre-commit` derivation, so local hooks and `nix flake check` share one definition.
This slots into the general
[04 — Quality gates](../../../programming/project-bootstrap/04-quality-gates.md) hook layer.
