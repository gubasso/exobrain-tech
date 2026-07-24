---
digest-of: tools/git/branch-protection
last-synced: 2026-07-24
source-files:
  - README.md
  - github-web-ui.md
  - gitlab-web-ui.md
  - workflow.md
  - first-run-enablement.md
  - github-app-token.md
  - master-promotion.md
token-estimate: 600
---

# AGENTS

## Scope

Branch-protection enforcement for the `develop` → tag → CI → `master` path: manual host runbooks,
the canonical strategy doc, first-run host toggles, and copy-into-project CI templates.

## Key Points

- **Workflow**: `workflow.md` describes the branch-protection strategy and rationale (the abstract
  branch/release _model_ lives in `programming/release-workflow/`; this subtree is the platform
  _enforcement_ layer).
- **Apply it**: `github-web-ui.md` and `gitlab-web-ui.md` are the one-run-per-project host runbooks.
  Each covers branch/tag protection, the `develop` default branch, and verification.
- **Required status checks are host inputs, not hardcodes**: in GitHub's ruleset UI, enter contexts
  matching the job names the project's CI emits. The check names are owned by the project's
  CI/language spec, not by this layer. GitLab gates on the pipeline
  (`only_allow_merge_if_pipeline_succeeds`), so it uses no named checks.
- **First-run enablement**: `first-run-enablement.md` — after the first push, enable Actions/CI and
  set write permissions (GitHub: Actions → General → Read and write + allow Actions to create PRs;
  GitLab: enable CI/CD, let CI push to protected `master`, OIDC `id_tokens`). OIDC needs no
  repo-level switch on GitHub — `id-token: write` at the job level is enough.
- **Bypass actor**: `master` is written only by CI. Standard model = the **installed GitHub App** is the
  bypass actor and the promote workflow pushes `master` under the App token. On a **personal account**
  the global `github-actions[bot]` app cannot be a bypass actor at all (HTTP 422); on an **organization**
  it can, so `github-actions[bot]` + a default-token push is an org-only shortcut. GitLab Premium allow-lists a Project
  Access Token bot user; Free relies on the 17.2+ job-token push toggle.
- **GitHub App token**: `github-app-token.md` owns why a GitHub App (not a PAT/deploy key) is the CI
  actor whose tag push retriggers workflows (the default `GITHUB_TOKEN` does not), plus the
  field-by-field App registration. Consumed by the language release spec's release-plz workflow.
- **Master promotion**: `master-promotion.md` owns how CI fast-forwards `master` onto each release tag —
  standalone tag-triggered workflow vs inline `needs:` job (chosen by who pushes the tag), the
  ancestry-check guard, and the App token that both _triggers_ it (the tag push) and _pushes_ `master`
  as its own bypass actor.
- **Layout**: `github/` holds the release-promote workflow template; `gitlab/` holds the
  release-promote CI template. Manual host configuration lives in `github-web-ui.md` and
  `gitlab-web-ui.md`.

## Source Map

| Topic                              | File / Subtree                            |
| ---------------------------------- | ----------------------------------------- |
| Canonical workflow strategy        | `workflow.md`                             |
| Enable Actions/CI + write perms    | `first-run-enablement.md`                 |
| GitHub App token (why + setup)     | `github-app-token.md`                     |
| Master promotion (fast-forward)    | `master-promotion.md`                     |
| Point-and-click runbooks           | `github-web-ui.md`, `gitlab-web-ui.md`    |
| GitHub release-promote workflow    | `github/workflows/release-promote.yml`    |
| GitLab release-promote CI template | `gitlab/ci/release-promote.gitlab-ci.yml` |

## Maintenance Notes

- The parent `tools/git/` digest points here for the branch-protection subtree.
- Keep the README index and Usage block aligned with the manual host runbooks and copy-into-project
  CI template subtrees.
- The abstract branch/release model (the `develop` → tag → CI-promote-`master` strategy) is owned by
  the general shelf `programming/release-workflow/`; this subtree is the platform _enforcement_
  layer (GitHub Rulesets / GitLab protected-branch runbooks and the tag-triggered release-promote CI
  templates). Standardized on `develop`/`master`.
- Status-check context names are intentionally not hardcoded: they must match each project's actual
  CI job names (per its language `release-workflow-spec`) and are entered in the host UI.
