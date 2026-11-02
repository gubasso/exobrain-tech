# pre-commit hooks

The rule for the two hook stages, and for what CI repeats, is in
[project-bootstrap/04 — Quality gates](../project-bootstrap/04-quality-gates.md#git-hooks). This
page holds the configuration forms.

To run in just a subdir or selected files...

```yaml
---
files: ^my-subdif/
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
