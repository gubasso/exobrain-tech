# 01 — Repository foundation

The three root files every repository needs before anything else: an ignore file, a license, and a
readme. These are language-agnostic; the language binding adds ecosystem-specific ignore patterns
and metadata.

## `.gitignore`

Ignore build output, caches, secrets, and environment-specific files so they never enter history.
A generated file that the project keeps, such as a lock file or a changelog, is committed, and the
hooks exclude it ([04 — Generated files](./04-quality-gates.md#generated-files)). Ignore:

- **OS / editor cruft** — `.DS_Store`, `Thumbs.db`, swap files, editor-local directories.
- **Secrets / environment** — `.env`, credential files, anything holding a token or key.
- **Build artifacts** — compiled output, caches, dependency directories.
- **Nix artifacts** — `result`, `result-*` symlinks from `nix build`.

Start from the language template at [github/gitignore](https://github.com/github/gitignore) and
prune to what the project actually produces. Syntax reference:
[gitignore](https://git-scm.com/docs/gitignore).

## `LICENSE`

Pick an [SPDX](https://spdx.org/licenses/) identifier and commit the matching license text at the
repo root. A public repo without a license grants no reuse rights, so this is not optional for
anything you intend others to use.

For a repo that mixes prose and code, dual-licensing is a clean pattern — this very repository, for
example, licenses documentation under CC-BY-4.0 and scripts/config under MIT, with each file class
pointing at its license. Choose the model deliberately and state it in the `README`.

## `README` skeleton

The `README.md` is the project index. At a minimum it states: what the project is (one line), how to
build/run it, how to set up the dev environment, and where the deeper docs live. Keep it an index
that routes to detail, not a place that duplicates it.

The dev environment section links `host-requirements.toml`, which lists each tool that the host
supplies and the project calls, and repeats none of it. It names only what no single command
covers, such as the coreutils and the ELF loader
([03 — Host requirements](./03-local-dev-environment.md#host-requirements)).

```markdown
## Host requirements

The devshell supplies every other tool. [`host-requirements.toml`](./host-requirements.toml) lists
each host tool that a recipe, a hook, or a script calls.
```
