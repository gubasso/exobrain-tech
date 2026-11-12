# 01 — Repository foundation

Every repository needs three root files before anything else:

- an ignore file
- a license
- a readme

These files are the same in every language. The language binding adds the ignore patterns and the
metadata of its ecosystem.

## `.gitignore`

The `.gitignore` keeps build output, caches, secrets, and environment-specific files out of the
history. The project commits a generated file that it keeps, such as a lock file or a changelog,
and the hooks exclude it ([04 — Generated files](./04-quality-gates.md#generated-files)). Ignore
these classes of file:

- OS and editor cruft, such as `.DS_Store`, `Thumbs.db`, swap files, and editor-local directories.
- Secrets and environment files, such as `.env`, credential files, and anything that holds a token
  or a key.
- Build artifacts, such as compiled output, caches, and dependency directories.
- Nix artifacts, the `result` and `result-*` symlinks that `nix build` creates.

Start from the language template at [github/gitignore](https://github.com/github/gitignore). Prune
it to what the project produces. The [gitignore](https://git-scm.com/docs/gitignore) reference
documents the syntax.

## `LICENSE`

Pick an [SPDX](https://spdx.org/licenses/) identifier. Commit the matching license text at the
repository root. A public repository without a license grants no reuse rights. Anything that you
intend others to use needs a license.

Dual licensing suits a repository that mixes prose and code. This repository, for example, licenses
documentation under CC-BY-4.0 and scripts and configuration under MIT. Each file class points at its
license. Choose the model deliberately. State it in the `README`.

## `README` skeleton

The `README.md` is the project index. At a minimum, it states these facts:

- what the project is, in one line
- how to build and run it
- how to set up the dev environment
- where the deeper docs live

Keep the `README.md` an index that routes to detail. Do not duplicate the detail in it.

The dev environment section links `host-requirements.toml`, which lists each tool that the host
supplies and the project calls. The section repeats none of the file. It names only what no single
command covers, such as the coreutils and the ELF loader
([03 — Host requirements](./03-local-dev-environment.md#host-requirements)).

```markdown
## Host requirements

The devshell supplies every other tool. [`host-requirements.toml`](./host-requirements.toml) lists
each host tool that a recipe, a hook, or a script calls.
```
