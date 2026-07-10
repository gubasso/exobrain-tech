---
digest-of: tools/git
last-synced: 2026-07-10
source-files:
  - README.md
  - cmds-examples.md
  - diffs.md
  - feature-lifecycle-git-commands.md
  - feature-lifecycle.md
  - git-commit-signing-with-ssh-git-commit-s-cheatsheet.md
  - git-qa.md
  - github.md
  - gitolite.md
  - glab-auth.md
  - rebase-workflow.md
token-estimate: 300
---

# AGENTS

## Scope

Git command references and workflow notes. Top-level material covers day-to-day commands, branching,
diffs, and repo administration; subtrees cover branch protection and workflow-specific runbooks.

## Key Points

- **Commands**: Practical command snippets for checkout, branching, merge handling, and conflict
  recovery.
- **Branching**: Feature-lifecycle notes and related command sequences for local and remote branch
  work.
- **Administration**: GitHub, Gitolite, and GitLab repository management notes.
- **Authentication**: `gh`/`glab` auth runbooks — check status, keyring token storage, and HTTPS git
  credential-helper setup.
- **Workflows**: Rebase and origin-state runbooks for repeatable branch operations.
- **Comparison material**: Diffs and command examples complement the workflow guides.

## Source Map

| Topic                                  | File / Subtree                                                        |
| -------------------------------------- | --------------------------------------------------------------------- |
| Command examples and conflict handling | `cmds-examples.md`, `diffs.md`                                        |
| Feature lifecycle and branch commands  | `feature-lifecycle*.md`                                               |
| Commit signing and QA notes            | `git-commit-signing-with-ssh-git-commit-s-cheatsheet.md`, `git-qa.md` |
| GitHub, Gitolite setup                 | `github.md`, `gitolite.md`                                            |
| `gh` / `glab` authentication           | `gh-auth.md`, `glab-auth.md`                                          |
| Rebase workflow reference              | `rebase-workflow.md`                                                  |
| Branch protection workflows            | `branch-protection/`                                                  |
| Workflow runbooks                      | `workflows/`                                                          |

## Maintenance Notes

- Branch protection has its own AGENTS digest.
- Workflow runbooks are summarized here because they do not have a separate digest.
