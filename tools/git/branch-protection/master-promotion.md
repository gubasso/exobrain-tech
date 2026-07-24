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
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
          ref: ${{ github.ref }}

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
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: gh release create "$GITHUB_REF_NAME" --generate-notes
```

The **ancestry check** is load-bearing: it is the guard that stops a tag placed on a stray commit from
advancing the release branch. Never remove it.

## Token & bypass

The promote **push moves a branch pointer and retriggers nothing downstream**, so it uses the default
`GITHUB_TOKEN` — no App token is needed _for the push itself_. But `master` is a protected branch, so
its ruleset must let the push through: add **`github-actions[bot]`** (the default-token actor, GitHub
App id **15368**) to the bypass list. Without that bypass entry, the protected-branch rule rejects the
promote push and `master` never advances.

Two distinct token concerns, kept separate:

- **Triggering** the standalone workflow needs an App-token (or PAT/human) tag push — the release tool's
  concern. See [github-app-token.md](./github-app-token.md).
- **Pushing** `master` inside the job uses the default `GITHUB_TOKEN` + the `github-actions[bot]` bypass
  entry — this page's concern.

The one exception: if `master` has its **own** CI that must run _after_ promotion, the default-token push
will not retrigger it (same anti-recursion rule), so promote under an App token instead.

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
  per project (copied in by `github/setup.sh`).
- [GitHub Actions — triggering a workflow](https://docs.github.com/en/actions/using-workflows/triggering-a-workflow)
  (the default-`GITHUB_TOKEN` no-retrigger rule and its exceptions).
