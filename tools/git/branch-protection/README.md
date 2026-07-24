# branch-protection

Manual runbooks and CI templates for the `develop` → tag → CI → `master` workflow. Use them once per
new project.

Part of the once-per-project setup — see
[project bootstrap](../../../programming/project-bootstrap/runbook.md).

- [workflow](./workflow.md) — strategy, prerequisites, caveats, verification checklist.
- [first-run-enablement](./first-run-enablement.md) — turning Actions/CI on and to write, after the
  first push.
- [github-app-token](./github-app-token.md) — why a GitHub App (not a PAT/deploy key) is the CI
  bypass/retrigger actor, and the field-by-field setup.
- [master-promotion](./master-promotion.md) — how CI fast-forwards `master` onto each release tag
  (standalone vs inline, the ancestry check, and the token/bypass split).
- [github-web-ui](./github-web-ui.md) / [gitlab-web-ui](./gitlab-web-ui.md) — point-and-click
  branch and tag protection runbooks.

## Usage

Choose the host runbook for the new project:

- GitHub — [github-web-ui.md](./github-web-ui.md): create the `master`, `develop`, and `v*` tag
  rulesets; set `develop` as default; verify.
- GitLab — [gitlab-web-ui.md](./gitlab-web-ui.md): protect `master`, `develop`, and release tags;
  set `develop` as default; verify.

For GitHub, the required status checks entered in the ruleset UI must match the job names your CI
actually emits. See the project's language `release-workflow-spec` for those names. GitLab gates on
the pipeline itself (`only_allow_merge_if_pipeline_succeeds`), so it needs no check names.

**Solo project.** The `master` and `develop` rulesets require one approving review. Working alone,
set the review count to `0` in the GitHub ruleset UI (or merge via the CI bypass actor); every other
rule applies unchanged.

The former automated scripting path is retired. This shelf now keeps the self-contained manual
runbooks plus the workflow templates that each project copies during setup.

## Layout

- `github-web-ui.md` — GitHub rulesets runbook.
- `gitlab-web-ui.md` — GitLab protected branches/tags runbook.
- `github/workflows/release-promote.yml` — copied into the target repo's `.github/workflows/`.
- `gitlab/ci/release-promote.gitlab-ci.yml` — copied into the target project's CI configuration.
