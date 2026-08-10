---
digest-of: programming/release-workflow
last-synced: 2026-07-10
token-estimate: 650
---

# AGENTS

## Scope

Language-agnostic general principles for development + release workflow: the `develop`/`master`
branch model, the automated release-PR pattern, Trusted Publishing / OIDC, and CI-driven promotion.
The source of truth that the per-language `languages/*/release-workflow-spec/` bindings
implement (rust + bash full; python + JS stubs). Mirrors the `cli-design/` → `cli-spec/` pattern.

## Key Points

- **Branch model:** `develop` integrates (feature branches merge via reviewed PRs, green CI) and is
  the release trigger; `master` mirrors the latest published release and is **written only by CI**
  (`main` is the common alias). Promotion is fast-forward-only onto the release tag, guarded by an
  ancestry check (the tag must be reachable from `develop`).
- **Release-PR invariant:** change intent (Conventional Commits _or_ changeset files) → a bot opens
  and maintains a release PR (version bump + changelog) → **merging it is the release gate** → tag +
  registry publish → CI promotes `master`. SemVer + Keep-a-Changelog; published versions are
  immutable (fix forward, never overwrite).
- **Trusted Publishing / OIDC** is the cross-ecosystem auth convergence (crates.io, PyPI, npm):
  short-lived keyless tokens minted at job time, matched on repo + workflow filename (+ environment)
  and **branch-agnostic**. The first publish is manual (TP attaches to an existing package). Enable
  enforcement (e.g. crates.io "require trusted publishing") once OIDC works. crates.io TP now covers
  GitHub Actions **and GitLab.com** (self-hosted GitLab not yet).
- **Tooling:** rust=release-plz, python=release-please / python-semantic-release, JS=Changesets,
  cross-language=release-please (tags only, bring your own publish), Go=GoReleaser (no registry —
  tagging is publishing).
- **Binary/artifact distribution is separate from registry publishing** (source vs prebuilt
  binaries): rust=cargo-dist, Go=GoReleaser, bash=install.sh/AUR/OBS, python/JS=the published
  artifacts are the dist. When both a publish workflow and a binary-dist workflow exist, keep them
  in **separate files** and register only the _publish_ filename with the trusted publisher —
  cargo-dist's default `release.yml` must not be the registered file.
- **Promotion pattern (official):** promote `master` onto the release **tag**, not a SHA. When a bot
  creates the tag with `GITHUB_TOKEN` (no workflow retrigger), run promote as a `needs:` job in the
  same run reading the tool's output; when a human pushes the tag, use a separate `on: push: tags`
  release-promote workflow.

## Maintenance Notes

- Bindings live at `languages/{rust,bash,python,javascript}/release-workflow-spec/`, each with
  its own digest (or, for stubs, covered by the parent language digest). Platform enforcement
  runbooks/assets live at `tools/git/branch-protection/`; git mechanics at
  `tools/git/{rebase-workflow.md,feature-lifecycle.md}`.
- External auth model is perishable: re-verify crates.io / PyPI / npm Trusted Publishing status and
  the release-plz / release-please / Changesets behavior against upstream on a cadence.
- Standardize on `develop`/`master` — flag and fix any reintroduced `main`/`devel` in the bindings.
