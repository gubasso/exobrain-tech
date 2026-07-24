# Master promotion — fast-forward `master` onto each release tag

In the `develop` → tag → `master` model, `master` is never written by hand: it **mirrors releases**
with linear history. When the release tool tags `vX.Y.Z` on `develop`, a small CI job advances
`master` to that exact tag — after checking the tag is really reachable from `develop`. This page owns
the _why_ and the _how_ of that promotion job; the branch-protection strategy that surrounds it lives
in [workflow.md](./workflow.md), and the actor whose tag push can retrigger it is the
[GitHub App token](./github-app-token.md).

## What it does

Promotion targets the release **tag** — the canonical release marker — not an arbitrary branch SHA. It:

1. Resolves the tagged commit (`refs/tags/<tag>^{commit}`).
2. Verifies that commit is an ancestor of `develop` (refuses otherwise — a tag off `develop` must never
   move `master`).
3. Fast-forwards `master` to the tagged commit (`git merge --ff-only`), or **creates** `master` at the
   tag on the very first release.
4. Pushes `master`.

Because it is fast-forward-only, `master` keeps a linear history and can only ever move to a commit that
already exists on `develop` — it never introduces a merge commit or a rewrite.

## Where it runs — two placements

The same git logic can live in either of two shapes. Pick by **who pushes the tag**.

- **(A) Standalone tag-triggered workflow** (`on: push: tags: ['v*']`). Decoupled from the release tool;
  the cleanest separation. But a tag-triggered workflow only fires if the tag push comes from an actor
  whose events **retrigger** workflows — a **GitHub App token**, a **PAT**, or a **human** push. A tag
  pushed with the default `GITHUB_TOKEN` is invisible to triggers (GitHub's anti-recursion rule), so a
  standalone promote would simply never run. See [github-app-token.md](./github-app-token.md) for that
  rule.
- **(B) Inline `needs:` job** in the release workflow. The promote job depends on the release job and
  runs in the **same workflow run**, reading the release tag from the tool's outputs. It has **no
  retrigger dependency**, so it works even when the tag is pushed with the default `GITHUB_TOKEN`, and
  it is atomic with the release.

**Decision rule.** If your release tool already pushes the tag under an App token (e.g. so a
tag-triggered binary build can fire), use **(A)** — the standalone workflow costs nothing extra and
keeps promotion independent. If the tag is pushed with the default `GITHUB_TOKEN`, or you want the
promotion to be atomic with the release, use **(B)**.

## Mechanics (the canonical standalone workflow)

`github/workflows/release-promote.yml` is the template this shelf ships:

```yaml
name: release-promote
on:
  push:
    tags: ['v*']

permissions:
  contents: write

jobs:
  promote:
    runs-on: ubuntu-latest
    steps:
      # Mint the App token so the push to master is attributed to the App —
      # master's bypass actor (see "Token & bypass" below).
      - uses: actions/create-github-app-token@v3
        id: app-token
        with:
          app-id: ${{ secrets.RELEASE_PLZ_APP_ID }}
          private-key: ${{ secrets.RELEASE_PLZ_APP_PRIVATE_KEY }}

      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
          ref: ${{ github.ref }}
          token: ${{ steps.app-token.outputs.token }}

      - name: Verify tag is on develop
        run: |
          git fetch origin develop
          tag_sha="$(git rev-parse "refs/tags/${GITHUB_REF_NAME}^{commit}")"
          if ! git merge-base --is-ancestor "$tag_sha" origin/develop; then
            echo "Tag $GITHUB_REF_NAME is not on develop. Refusing to promote." >&2
            exit 1
          fi

      - name: Fast-forward master to the release tag
        run: |
          git config user.name  "github-actions[bot]"
          git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
          tag_sha="$(git rev-parse "refs/tags/${GITHUB_REF_NAME}^{commit}")"
          if git fetch origin master; then
            git checkout -B master origin/master
            git merge --ff-only "$tag_sha"
          else
            # master does not exist yet: create it at the release tag.
            git checkout -B master "$tag_sha"
          fi
          git push origin master

      - name: Create GitHub Release
        env:
          GH_TOKEN: ${{ steps.app-token.outputs.token }}
        run: gh release create "$GITHUB_REF_NAME" --generate-notes
```

The **ancestry check** is load-bearing: it is the guard that stops a tag placed on a stray commit from
advancing the release branch. Never remove it.

## Token & bypass

`master` is a protected branch, so the promote push must come from an actor its ruleset lets bypass.
Use the **installed GitHub App** for both halves: it is `master`'s **bypass actor**, and the job
**pushes `master` under the App token** (minted with `create-github-app-token`, wired into `checkout`
via `token:` so `git push` is attributed to the App). Push identity = bypass actor → the push is
accepted. Set the ruleset's bypass `actor_id` to the App's ID (`actor_type: Integration`).

Why the App and not the default token: a push made with the default `GITHUB_TOKEN` is attributed to
`github-actions[bot]`, which is **not** the App. On a **personal account** `github-actions[bot]` can
never be added as a ruleset bypass actor at all — GitHub rejects it with _"Actor GitHub Actions
integration must be part of the ruleset source or owner organization"_ (HTTP 422) — so a default-token
push has no way through. The App-token push is uniform: it works the same on personal and organization
accounts.

> **Org-only shortcut.** On an **organization** repo you _may_ instead keep the default `GITHUB_TOKEN`
> push and add the global **`github-actions[bot]`** app to the bypass list — there the global
> GitHub Actions app _is_ an eligible bypass actor. This shelf standardizes on the App to keep one model
> everywhere; reach for the shortcut only if you deliberately want no App on an org repo.

Because the push is made with the App token, it **does** retrigger workflows (App-token events are not
subject to the anti-recursion rule), so any `on: push: [master]` CI runs on the promoted commit — a
default-token push would not. Two token concerns, both now the App's: **triggering** this workflow needs
the App-token tag push (the release tool's job — see [github-app-token.md](./github-app-token.md)), and
**pushing** `master` uses the App token as its own bypass actor (this page's job).

## Don't double-create the release

The template's final step runs `gh release create`, which suits a shelf that ships without a separate
binary pipeline. If another tag-triggered workflow **already creates the GitHub Release** — for example
cargo-dist's `release.yml` — then the promote job must **drop the `Create GitHub Release` step**, or the
second `gh release create <tag>` fails because the release already exists. In that setup, promotion only
fast-forwards `master`; release creation belongs to the other workflow.

## Reference

- [workflow.md](./workflow.md) — the surrounding branch-protection strategy and verification checklist.
- [github-app-token.md](./github-app-token.md) — the App token whose tag push retriggers this workflow,
  and the `GITHUB_TOKEN` anti-recursion rule.
- [github/workflows/release-promote.yml](./github/workflows/release-promote.yml) — the template applied
  per project (copied into each project during manual setup).
- [GitHub Actions — triggering a workflow](https://docs.github.com/en/actions/using-workflows/triggering-a-workflow)
  (the default-`GITHUB_TOKEN` no-retrigger rule and its exceptions).
