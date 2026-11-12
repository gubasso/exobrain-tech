# Development Tools / Workflows

Default project-tooling convention, language-agnostic. Three roles, kept separate — do not collapse
them into one tool.

## The three roles

| Role                    | Canonical choice                                           | Owns                                                      |
| ----------------------- | ---------------------------------------------------------- | --------------------------------------------------------- |
| Environment / toolchain | **Nix flake devShell**                                     | language runtime(s) + system tools, pinned + reproducible |
| Dependency resolver     | the language's native package manager                      | the dependency graph + lockfile                           |
| Task runner             | project-local (`just`, `make`, npm scripts, cargo aliases) | repeatable dev commands                                   |

## Environment manager: Nix flake devShell (canonical)

Per-project environments default to a **Nix flake devShell** for any language. The flake pins the
runtime(s) and system tooling; `flake.lock` makes it reproducible; `.envrc` (`use flake`)
auto-enters it via direnv/nix-direnv. See [nix](../tools/nix/README.md). It is the default for any
language, ahead of ad-hoc version managers ([mise](./mise.md), asdf, pyenv, nvm,
rustup-as-installer).

## Dependency manager: language-native, inside the devShell

The devShell provides the package-manager binary; the package manager still owns dependency
resolution and the lockfile. Nix does **not** replace it by default.

| Language | Dependency manager (runs inside the devShell)                                                         |
| -------- | ----------------------------------------------------------------------------------------------------- |
| Python   | uv, see [toolchain and layout](../languages/python/project-bootstrap-spec/00-toolchain-and-layout.md) |
| JS / TS  | npm / pnpm / yarn                                                                                     |
| Rust     | Cargo                                                                                                 |
| Go       | Go modules                                                                                            |

Each dependency has one owner, the most specific file that declares it. The rule and its order are
in [who owns each dependency](../programming/project-bootstrap/03-local-dev-environment.md#who-owns-each-dependency).

Typical Python flow (first-time setup):

```bash
direnv allow           # once per project: enters the devShell, which syncs .venv from uv.lock
uv sync                # lock and install after a change of pyproject.toml
```

The host supplies uv, uv installs the interpreter of `.python-version`, and `layout uv` in `.envrc`
syncs `.venv` and puts `.venv/bin` on `PATH`, so a `cd` into the project activates the venv with no
manual step. That layering is in
[nix: Python and uv in a devshell](../tools/nix/02-per-project-devshell.md#python-and-uv-in-a-devshell).
Direnv itself is in
[nix: automatic activation](../tools/nix/02-per-project-devshell.md#automatic-activation-direnv).

Poetry, PDM, and pip-tools are alternatives for a project that already uses one of them. Each
project keeps one manager. Poetry is in [python-poetry](../languages/python/python-poetry.md).

Reach for `uv2nix`, `poetry2nix`, or `cargo2nix` only when Nix itself builds the app from the
lockfile. That is a bigger change than managing the environment, and it creates a second source of
truth beside the native lockfile.
