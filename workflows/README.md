# Workflows

Cross-cutting workflow notes that are not tied to one language or tool.

- [development-tools-workflow.md](./development-tools-workflow.md) — the three-role project
  convention (Nix/devShell environment + dependency manager + task runner).
- [mise.md](./mise.md) — the `mise` version manager.
- [claude-self-debug-loop.md](./claude-self-debug-loop.md) — a Claude self-debugging loop.

## See also

Release and rebase workflows are owned elsewhere:

- [tech/programming/release-workflow/](../programming/release-workflow/) — the **general release
  workflow**: branch model, release-PR pattern, Trusted Publishing. Per-language bindings sit under
  `tech/languages/*/release-workflow-spec/`.
- [tech/languages/bash/release-workflow-spec/](../languages/bash/release-workflow-spec/) — releasing
  a **bash program**.
- [tech/tools/git/rebase-workflow.md](../tools/git/rebase-workflow.md) — the canonical **git
  rebase** reference.
