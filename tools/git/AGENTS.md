---
digest-of: tools/git
last-synced: 2026-07-10
token-estimate: 300
---

# AGENTS

## Scope

Git command references and workflow notes. Top-level material covers day-to-day commands, branching,
diffs, and repo administration; subtrees cover workflow-specific runbooks.

## Key Points

- **Commands**: Practical command snippets for checkout, branching, merge handling, and conflict
  recovery.
- **Branching**: Feature-lifecycle notes and related command sequences for local and remote branch
  work. The example vocabulary is shared across the bucket: `master` is the trunk and `origin` the
  remote, and a branch a worked example names takes one of the two short-lived forms,
  `<type>/<slug>` or `<issue-id>-<slug>`. A command whose subject is the mechanic rather than the
  scenario writes its branch as an angle-bracket slot, so no reader copies a placeholder as a name.
  The model itself lives in
  [trunk-based development](../../workflows/trunk-based-development.md).
- **Administration**: GitHub, Gitolite, and GitLab repository management notes.
- **Authentication**: `gh`/`glab` auth runbooks — check status, keyring token storage, and HTTPS git
  credential-helper setup.
- **Workflows**: Rebase and origin-state runbooks for repeatable branch operations.
- **Comparison material**: Diffs and command examples complement the workflow guides.

## Maintenance Notes

- Workflow runbooks are summarized here because they do not have a separate digest.
