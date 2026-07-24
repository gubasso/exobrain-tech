# GitHub App token — a bot identity whose pushes retrigger CI

Release automation needs a git identity whose **tag push retriggers other workflows**. The default
`GITHUB_TOKEN` cannot do this. The fix is a small, purpose-built **GitHub App**: CI mints a
short-lived installation token from it, and the release bot runs under that token. Set it up once per
project. This page owns the _why_ and the field-by-field _how_; the workflow that consumes it lives in
the language release spec (for Rust,
[release-plz config](../../../languages/rust/release-workflow-spec/04-release-plz-config.md) and
[binary distribution](../../../languages/rust/release-workflow-spec/05-binary-distribution-cargo-dist.md)).

## Why — the retrigger problem

GitHub Actions is deliberately anti-recursive: **events triggered by a workflow using the default
`GITHUB_TOKEN` do not start new workflow runs** (the only exceptions are `workflow_dispatch` and
`repository_dispatch`). This prevents a workflow from pushing a commit that triggers itself forever.

That safeguard breaks the release pipeline: when release-plz tags `vX.Y.Z` under `GITHUB_TOKEN`, the
tag push is invisible to triggers, so a tag-triggered binary-build workflow (cargo-dist's
`release.yml`) never fires — no binaries build. The same limitation applies to a protected-`master`
promotion push that must itself run CI. The tag/commit has to come from an actor whose events **do**
retrigger workflows. A GitHub App installation token is that actor.

## What it does at runtime

You store the App's **ID + private key** as secrets — never a token. CI mints a fresh installation
token per run and the bot runs under it:

```yaml
- uses: actions/create-github-app-token@v3
  id: app-token
  with:
    app-id: ${{ secrets.RELEASE_PLZ_APP_ID }}
    private-key: ${{ secrets.RELEASE_PLZ_APP_PRIVATE_KEY }}
- uses: release-plz/action@v0.5
  env:
    GITHUB_TOKEN: ${{ steps.app-token.outputs.token }}
```

The minted **installation token expires after ~1 hour** and is revoked when the job ends. Because the
push is now attributed to the App (not `github-actions[bot]`/`GITHUB_TOKEN`), it retriggers
tag-triggered workflows. (`actions/create-github-app-token` also accepts a newer `client-id` input in
place of `app-id`; either works — this shelf uses `app-id` for consistency.)

## Why not the alternatives

Three actors can push a tag that retriggers workflows: a **GitHub App** installation token, a
**personal access token (PAT)**, or an **SSH deploy key**. The App wins for this job:

- **PAT** — bound to a person (dies when they leave; every release is authored under their name),
  coarse-grained, and long-lived (a standing credential you must rotate).
- **SSH deploy key** — git-over-SSH only. It has **no API token**, so it cannot open release-plz's
  release PR (which needs Pull requests write). It also can't be the actor for API-driven steps.
- **GitHub App** — an independent bot identity, scoped to **exactly** the permissions and repos you
  grant, with a **short-lived** per-run token, higher rate limits, and a clean audit trail. This is
  GitHub's own recommendation for automation and long-lived integrations.

## Register the App (field by field)

**Settings → Developer settings → GitHub Apps → New GitHub App.** Fill only what a minimal CI token
needs; leave everything else at its default:

- **GitHub App name** — any **globally unique** name, **≤34 characters** (e.g. `<owner>-release-bot`).
- **Homepage URL** — **required**; the repo URL (`https://github.com/<owner>/<repo>`) is fine.
- **Identifying and authorizing users** — leave **Callback URL** blank; leave _Expire user
  authorization tokens_, _Request user authorization (OAuth) during installation_, and _Enable Device
  Flow_ **unchecked** (there is no user-login flow).
- **Post installation** — leave **Setup URL** blank and _Redirect on update_ **unchecked**.
- **Webhook** — **uncheck _Active_**. The App is called via API by CI; it receives nothing. Leave
  **Webhook URL** and **Secret** blank.
- **Permissions → Repository permissions** — set exactly two:
  - **Contents: Read and write** — push commits and tags.
  - **Pull requests: Read and write** — open/update the release PR.

  (_Metadata: Read-only_ is granted automatically. Grant nothing else — least privilege.)
- **Subscribe to events** — none (no webhook, nothing to subscribe to).
- **Where can this GitHub App be installed?** — **Only on this account**.

Click **Create GitHub App**.

## After creation (both steps are mandatory)

1. **Generate a private key.** On the App's page → **Private keys** → **Generate a private key**. A
   `.pem` downloads; GitHub keeps only the public half, so store the file securely (you can hold up to
   25 keys; they don't expire but can be revoked).
2. **Install the App on the repo.** **Install App** → install on your account → grant access to
   `<owner>/<repo>` (**Only select repositories**). _Creating the App is not enough — an uninstalled
   App mints no token._
3. **Store two repository secrets** (repo → Settings → Secrets and variables → Actions):
   - `RELEASE_PLZ_APP_ID` — the App's **numeric App ID** (shown under _About_ on the App page).
   - `RELEASE_PLZ_APP_PRIVATE_KEY` — the **full contents of the `.pem`**.

The workflow's `create-github-app-token` step then mints tokens from these on every run.

## Reference

- [GITHUB_TOKEN — automatic token authentication](https://docs.github.com/en/actions/concepts/security/github_token)
  (the no-retrigger rule and its exceptions)
- [actions/create-github-app-token](https://github.com/actions/create-github-app-token) — the token-minting action
- [Registering a GitHub App](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app)
  · [Choosing permissions for a GitHub App](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/choosing-permissions-for-a-github-app)
- [Managing private keys for GitHub Apps](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/managing-private-keys-for-github-apps)
  · [Installing your own GitHub App](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app)
- [Deciding when to build a GitHub App](https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/deciding-when-to-build-a-github-app)
  · [Managing deploy keys](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/managing-deploy-keys)
  (why an App over a PAT / deploy key)
