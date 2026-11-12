# pre-commit hooks

The rule for the two hook stages, and for what CI repeats, is in
[project-bootstrap/04 — Quality gates](../project-bootstrap/04-quality-gates.md#git-hooks). This
page holds the configuration forms.

To run in just a subdir or selected files...

```yaml
---
files: ^my-subdif/
# The host supplies prek; this is the oldest prek the configuration accepts.
minimum_prek_version: "0.5.7"
# Install the push stage too: the costly checks run there.
default_install_hook_types: [pre-commit, commit-msg, pre-push]
# Covers only a hook with no `stages:` of its own. A remote hook's manifest
# wins over it: pre-commit-hooks declares [pre-commit, pre-push, manual], so
# give each such hook an explicit `stages:`.
default_stages: [pre-commit]

repos:
  # Pre-commit hooks provided by the pre-commit project
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.6.0
    hooks:
      # Check if Python files are syntactically valid
```

## Excluding generated files

The rule is in
[project-bootstrap/04 — Generated files](../project-bootstrap/04-quality-gates.md#generated-files):
every hook that rewrites or judges a generated file excludes it, and the reading hooks keep it. Use
the tool's own configuration first, because it also governs a run outside pre-commit:

| Tool                            | Where the exclusion goes                          |
| ------------------------------- | ------------------------------------------------- |
| dprint                          | `excludes` in `dprint.json`                       |
| markdownlint-cli2               | `ignores` in `.markdownlint-cli2.jsonc`           |
| typos                           | `extend-exclude` under `[files]` in `_typos.toml` |
| editorconfig-checker            | `Exclude` in `.editorconfig-checker.json`         |
| prettier                        | `.prettierignore`                                 |
| pre-commit fixers, pygrep rules | the hook's own `exclude:`                         |

```yaml
- repo: https://github.com/pre-commit/pre-commit-hooks
  rev: v6.0.0
  hooks:
    # flake.lock and CHANGELOG.md are generated (nix, release-plz): see
    # project-bootstrap/04, "Generated files".
    - id: trailing-whitespace
      exclude: '^(flake\.lock|CHANGELOG\.md)$'
```

- Never use the top-level `exclude:` of `.pre-commit-config.yaml`. It also drops the reading hooks:
  the secret scan, the large-file check, and the merge-conflict check.
- markdownlint-cli2 with `"fix": true` rewrites files, so it counts as a rewriting hook.
- typos with no `args` inherits the upstream `--write-changes`, so it rewrites too. typos reads only
  the name of a lock-type file (`*.lock`, `package-lock.json`), never its content. Keep
  `--force-exclude` in its `args`, so that `extend-exclude` applies to the files pre-commit passes.
- taplo and editorconfig-checker skip `Cargo.lock` on their own, and editorconfig-checker also skips
  `*.snap`.
- dprint fails with "No files found" when a commit stages only excluded files, such as a lock file
  after an update. Run its hook as `dprint fmt --allow-no-files`.
- One comment beside each exclusion names the generator, so the next reader knows why it is there.
- Ask the tools, not the patterns, whether an exclusion holds: `dprint output-file-paths`,
  `typos --files`, and `editorconfig-checker -dry-run` list what each tool selects.
