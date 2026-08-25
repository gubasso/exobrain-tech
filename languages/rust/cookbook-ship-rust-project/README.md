# Ship a Rust project — cookbook (TLDR)

One-file, top-to-bottom runbook to take a Rust crate from _repo + remote_ to _published,
branch-protected, CI-gated_. Copy-paste commands; each section footnotes the canonical spec that
owns the _why_.

> **Assumes:** a crate you can already `cargo build`, a git repo, and a GitHub remote (GitLab notes
> at the end). Replace `<owner>/<repo>` and `<crate>` throughout.
>
> **Cookbook:** steps + snippets only; the _why_ lives in the footnoted specs. Inlined snippets are a
> sanctioned exception to single-source-of-truth[^rule] — if one disagrees with its spec, **the spec wins**.

**See also:** general recipes — project-bootstrap[^gen-bootstrap], release-workflow[^gen-release];
Rust specs distilled here — project-bootstrap-spec[^rs-bootstrap],
release-workflow-spec[^rs-release], cli-spec[^cli]; platform enforcement — branch-protection[^bp].

---

## 0. Prerequisites

```bash
cargo --version         # rustc + cargo installed
gh auth status          # GitHub CLI authenticated  (https://cli.github.com)
git remote -v           # origin points at your GitHub repo
```

## 1. Scaffold the crate[^rs-bootstrap]

Skip if the crate already exists.

```bash
cargo new <crate>            # binary  -> src/main.rs
# cargo new --lib <crate>    # library -> src/lib.rs
# cargo init                 # in an existing directory
```

Pin the toolchain so bare `cargo`, CI, and editors never drift — `rust-toolchain.toml` at the repo
root:[^nix-toolchain]

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
# targets = ["wasm32-unknown-unknown"]
```

## 2. Crate metadata — the publish gate[^crate-meta]

crates.io **rejects** a publish without `description` and a license — set them now.[^crate-meta]

```toml
[package]
name = "<crate>"
version = "0.1.0"
edition = "2021"
description = "One-sentence summary of what the crate does."
license = "MIT OR Apache-2.0"
repository = "https://github.com/<owner>/<repo>"
readme = "README.md"
keywords = ["cli", "example", "tooling"]     # <=5, each <=20 chars
categories = ["command-line-utilities"]      # must match canonical slugs exactly
rust-version = "1.74"                         # MSRV

# Keep the .crate tarball lean (denylist; leading / anchors to package root).
exclude = [
    "/docs", "/.github", "/scripts",
    "/deny.toml", "/release-plz.toml", "/dist-workspace.toml",
    "/justfile", "/flake.nix", "/flake.lock", "/.pre-commit-config.yaml",
]
```

Ship the license files too (`LICENSE-MIT` + `LICENSE-APACHE`) — the SPDX field doesn't include them.[^crate-meta]

## 3. Quality gates[^rs-gates]

Lints in `Cargo.toml`:[^cli-quality]

```toml
[lints.rust]
unsafe_code = "forbid"          # drop if you genuinely need unsafe

[lints.clippy]
pedantic         = { level = "warn", priority = -1 }
nursery          = { level = "warn", priority = -1 }
unwrap_used      = "warn"
expect_used      = "warn"
# restriction — catch scaffolding left behind
todo             = "deny"
dbg_macro        = "deny"
unimplemented    = "deny"
panic            = "deny"
wildcard_imports = "deny"
```

Dependency + license policy — `deny.toml` at the crate root:[^cli-quality]

```toml
[graph]
targets = []
all-features = true

[advisories]
# These take a scope, not a severity: all | workspace | transitive | none.
# An advisory in scope is an error. `vulnerability`, `notice` and
# `severity-threshold` were removed and now hard-error if named; vulnerability
# advisories are always denied.
unmaintained = "workspace"
unsound = "workspace"
yanked = "warn"

[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Zlib",
    "Unicode-3.0",
    "Unicode-DFS-2016",
]

[bans]
multiple-versions = "warn"
deny = [
    { name = "openssl-sys" },  # prefer rustls for static builds
]

[sources]
unknown-registry = "deny"
unknown-git = "deny"
```

Run the gates locally:[^rs-gates]

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run
cargo deny check
```

Pre-commit hooks — `.pre-commit-config.yaml`, then `pre-commit install`:[^gen-gates]

```yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt --check
        entry: cargo fmt --check
        language: system
        types: [rust]
        pass_filenames: false
      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy --all-targets --all-features -- -D warnings
        language: system
        types: [rust]
        pass_filenames: false
```

One command surface — `justfile`:[^gen-gates]

```just
default:
    @just --list

fmt:
    cargo fmt

lint:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo nextest run

check: fmt lint test
    cargo deny check
```

## 4. Local dev shell (Nix)[^nix-devshell]

A flake that reads `rust-toolchain.toml`, so local + CI share one toolchain and can't drift.
`flake.nix`:

```nix
{
  description = "rust dev shell (toolchain from rust-toolchain.toml)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            toolchain
            pkgs.cargo-nextest
            pkgs.cargo-deny
            pkgs.cargo-audit
            pkgs.just
            pkgs.pre-commit
          ];
          shellHook = ''echo "rust dev shell ready (toolchain from rust-toolchain.toml)"'';
        };
      }
    );
}
```

`.envrc` (direnv auto-loads on `cd`), then lock + enter:

```bash
echo 'use flake' > .envrc
nix flake lock            # generate flake.lock — commit it
direnv allow              # or a one-off: nix develop
```

## 5. First CI workflow[^gen-ci]

`.github/workflows/ci.yml` reuses the flake and runs the gates on every push/PR. The **job name is
the status-check context** branch protection will require (§7) — name it deliberately.

```yaml
name: ci

on:
  push:
    branches: [develop, master]
  pull_request:

permissions:
  contents: read

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: DeterminateSystems/nix-installer-action@main
      - run: |
          nix develop -c bash -c '
            cargo fmt --check &&
            cargo clippy --all-targets --all-features -- -D warnings &&
            cargo nextest run --profile ci &&
            cargo deny check
          '
```

Optional CI test profile — `.config/nextest.toml` (a non-default profile is inert unless invoked
with `--profile ci`):[^cli-test]

```toml
[profile.ci]
fail-fast = false
retries = { backoff = "exponential", count = 2, delay = "1s", max-delay = "10s", jitter = true }
status-level = "fail"
final-status-level = "fail"
success-output = "never"
failure-output = "immediate-final"
```

## 6. GitHub App — the CI bot identity[^app-token]

Register **one** GitHub App per account and reuse it across every repo. It is the bot identity that:

- lets release-plz's tag push **retrigger** the tag workflows — cargo-dist's `release.yml` (§9) and the
  promote (§7); the default `GITHUB_TOKEN` would not;
- is `master`'s **ruleset bypass actor** (§7); and
- **pushes `master`** in the promote workflow (§7), under its token.

Its `RELEASE_PLZ_APP_ID` + `RELEASE_PLZ_APP_PRIVATE_KEY` secrets are consumed by branch security (§7)
and by `release-plz.yml` (§8). Use the numeric App ID as the bypass actor in the ruleset UI.
Register once (steps 1–2); **install + store secrets per repo** (steps 3–4).

1. **Register the App** (once per account) — Settings → Developer settings → GitHub Apps → **New**.
   Fill only these; leave everything else at its default:
   - **Name** — unique, ≤34 chars; prefer a neutral, reusable name like `<owner>-ci-bot` (not
     `<owner>-release-plz-bot`).
   - **Homepage URL** — your profile `https://github.com/<owner>` (required but cosmetic).
   - **Webhook → Active** — **uncheck**.
   - **Repository permissions** — **Contents: Read and write** and **Pull requests: Read and write**.
   - **Where can this be installed?** — **Only on this account**.

   Then click **Create GitHub App**.
2. **Generate a private key** (once) — on the App's page → **Private keys** → **Generate a private
   key**. A `.pem` downloads; store it securely (GitHub keeps only the public half).
3. **Install the App** on the repo (per repo) — on the App's page → **Install App** (left sidebar) →
   **Install** next to your account → choose **Only select repositories** → select `<owner>/<repo>` →
   **Install**. Creation alone mints no token — an uninstalled App is inert.
4. **Store two repo secrets** (per repo → Settings → Secrets and variables → Actions → **New repository
   secret**) — same values in every repo:
   - `RELEASE_PLZ_APP_ID` — the App's numeric ID (shown under **About** on the App page).
   - `RELEASE_PLZ_APP_PRIVATE_KEY` — the full contents of the downloaded `.pem`.

## 7. Branch security[^bp]

Model: `develop` integrates (feature branches PR here); `master` mirrors releases (CI-only, linear
history); `v*` tags immutable.[^bp-model]

Use the manual host runbook for the full field-by-field path:
[github-web-ui](../../../tools/git/branch-protection/github-web-ui.md) (GitLab:
[gitlab-web-ui](../../../tools/git/branch-protection/gitlab-web-ui.md)).

1. **Create `develop`** (if it doesn't exist yet) and push it:

   ```bash
   git switch -c develop && git push -u origin develop
   ```

2. **Create the `master-protection` ruleset** in GitHub's ruleset UI.[^bp] Target the default branch
   (`master` until step 5), add your installed GitHub App as the bypass actor using its numeric App
   ID (§6), block deletion and force-push, require linear history, require a pull request with one
   approval, require resolved conversations, and require the `test` status check from §5.

3. **Create the `develop-protection` ruleset.** Target `refs/heads/develop`, block deletion and
   force-push, require a pull request with one approval, require resolved conversations, and require
   the `test` status check from §5.

4. **Create the `release-tags` ruleset.** Target `refs/tags/v*`; block delete, update, and
   non-fast-forward changes.

5. **Set the default branch** to `develop` in Settings → General.

6. **Enable Actions write**:[^bp-firstrun] **Settings → Actions → General →** allow Actions;
   **Workflow permissions → Read and write**; tick **Allow GitHub Actions to create and approve pull
   requests**.

7. **Add the promote workflow** — copy `release-promote.yml` into `.github/workflows/` so CI
   fast-forwards `master` onto each release tag. It pushes `master` **under the App token** (the bypass
   actor), so the App must be installed with its secrets stored (§6).[^promote]

On a **personal account** `github-actions[bot]` cannot be the bypass actor (422); use your installed
App instead.[^app-token] Applies: `master` — no human writes, linear history, App bypass actor, one
review; `develop` — one review, no force-push/deletion; `v*` tags — no delete/update. _(Solo project?
Set the review count to `0` — see [^bp].)_[^bebash]

## 8. Release + publish[^rs-release]

Once per project, in order:[^rs-runbook]

1. **Validate metadata (no token):**

   ```bash
   cargo publish --dry-run
   cargo package --list
   ```

2. **First publish is manual** — Trusted Publishing attaches to a crate that already exists.[^oidc]
   Create a scoped token[^tokens] at <https://crates.io/settings/tokens> (endpoint `publish-new`,
   exact crate scope, shortest expiry), then:

   ```bash
   cargo login          # paste the token
   cargo publish
   ```

3. **Register the trusted publisher** at `https://crates.io/crates/<crate>/settings` → Trusted
   Publishing → Add:[^oidc]
   - Repository: `<owner>/<repo>`
   - **Workflow filename: `release-plz.yml`** — the **publish** workflow, never the CI
     (`ci.yml`) or the binary (`release.yml`) workflow.
   - Environment: blank — only needed if the release-plz job declares an `environment:`.

4. **Revoke the bootstrap token** at <https://crates.io/settings/tokens>. CI mints short-lived OIDC
   tokens from here on.[^tokens]

5. **Commit release automation.** `release-plz.toml`:[^rs-plz]

   ```toml
   # See https://release-plz.dev/docs/config for all options.
   [workspace]
   changelog_update = true   # maintain CHANGELOG.md from conventional commits
   release_always   = false  # release only when there is something to release
   publish          = true   # publish to crates.io on release-PR merge
   semver_check     = true   # gate public-API compatibility (libraries)
   ```

   `release_always = false` gates **publish**, not the release PR.[^rs-plz]

   `.github/workflows/release-plz.yml` — runs on `develop`, OIDC auth (**no
   `CARGO_REGISTRY_TOKEN`**), under the GitHub App token from §6 so its tag push retriggers cargo-dist
   (§9):[^rs-plz]

   ```yaml
   name: release-plz

   on:
     push:
       branches: [develop]

   permissions:
     contents: write
     pull-requests: write
     id-token: write

   jobs:
     release-plz:
       if: ${{ github.repository_owner == '<owner>' }}
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v4
           with:
             fetch-depth: 0
             persist-credentials: false
         - uses: actions/create-github-app-token@v3
           id: app-token
           with:
             app-id: ${{ secrets.RELEASE_PLZ_APP_ID }}
             private-key: ${{ secrets.RELEASE_PLZ_APP_PRIVATE_KEY }}
         # `command:` accepts only `release-pr` or `release`; leaving it
         # unset runs both, which is the whole loop this job wants.
         - uses: release-plz/action@v0.5
           id: release-plz
           env:
             GITHUB_TOKEN: ${{ steps.app-token.outputs.token }}
   ```

   The App token comes from the App set up in §6; `release-plz.yml` reads its `RELEASE_PLZ_APP_ID` /
   `RELEASE_PLZ_APP_PRIVATE_KEY` secrets. `master` is promoted onto each release tag by the promote
   workflow (§7), which pushes under that same App token.[^promote]

6. **(Optional) Enable "require trusted publishing"** on the crate once an OIDC release has
   succeeded — it rejects all token publishes.[^oidc]

**The everyday loop after setup:** merge a `feat:`/`fix:` to `develop` → release-plz opens a release
PR → merge it → release-plz tags `vX.Y.Z` and publishes over OIDC.[^branch-model]

## 9. (Optional) Prebuilt binaries — cargo-dist[^cargo-dist]

```bash
cargo install cargo-dist
dist init          # writes dist-workspace.toml + .github/workflows/release.yml
dist generate      # regenerate release.yml after editing dist-workspace.toml
```

`release.yml` (binaries) is a **separate** file from `release-plz.yml` — never merge or register it
with the trusted publisher. It is **tag-triggered**, firing on the tag release-plz pushes with §6's
App token.[^cargo-dist] Then `cargo binstall <crate>` works for free.

## 10. Day-2 — semver / yank / rollback[^semver]

```bash
cargo semver-checks check-release      # libraries: catch API breaks (release-plz runs this)
cargo yank --version 1.2.3             # stop new selections of a bad version
cargo yank --version 1.2.3 --undo      # reverse it
```

Published versions are **immutable** — rollback = fix forward (patch on `develop`, cut a new PATCH),
optionally yank.[^semver]

## GitLab notes

Branch model, release-plz/OIDC, and metadata are identical. Differences:

- **Branch protection:** follow [gitlab-web-ui.md](../../../tools/git/branch-protection/gitlab-web-ui.md):
  protect `master`, `develop`, and `v*` tags; set `develop` as default; enable pipeline-only merge
  gates and CI promotion permissions.[^bp]
- **Enable CI/CD**, let the pipeline write to `master`, set default branch `develop`.[^bp-firstrun]
- **OIDC:** crates.io Trusted Publishing supports gitlab.com (not self-hosted). The job requests a
  GitLab `id_token`, exchanges it via `CRATES_IO_ID_TOKEN`, then `cargo publish`; register the
  publisher for GitLab (project path + CI config filename).[^oidc]

## Footnotes

[^rule]: Cookbook duplication is a sanctioned exception to the repo's SoT/DRY rule — see
    the repo's `AGENTS.md` and
    [ADR-cookbook-duplication-exception](../../../_docs/decisions/ADR-cookbook-duplication-exception.md).

[^gen-bootstrap]: [General project-bootstrap](../../../programming/project-bootstrap/README.md) —
    the language-agnostic once-per-project recipe.

[^gen-release]: [General release-workflow](../../../programming/release-workflow/README.md) — the
    language-agnostic release principles.

[^rs-bootstrap]: [Rust project-bootstrap-spec](../project-bootstrap-spec/README.md) ·
    [toolchain & layout](../project-bootstrap-spec/00-toolchain-and-layout.md).

[^rs-release]: [Rust release-workflow-spec](../release-workflow-spec/README.md) — the full Rust
    release & publishing shelf.

[^cli]: [Rust cli-spec](../cli-spec/README.md) — detailed CLI crate structure, testing, and quality.

[^bp]: [branch-protection](../../../tools/git/branch-protection/README.md) — the manual host runbooks
    and copy-into-project workflow templates.

[^bebash]: Optional automation — the same rulesets can be applied from the command line with
    [git-branch-protection](https://github.com/gubasso/bebash), a public bebash tool. Further reading
    only; the manual GUI steps above are the authoritative path.

[^nix-toolchain]: [nix/03 — Rust toolchain in a devShell](../../../tools/nix/03-rust-toolchain.md).

[^crate-meta]: [release-workflow-spec/01 — Crate metadata](../release-workflow-spec/01-crate-metadata.md).

[^rs-gates]: [Rust project-bootstrap-spec/01 — Quality gates](../project-bootstrap-spec/01-quality-gates.md).

[^cli-quality]: [cli-spec/06 — Code quality](../cli-spec/06-testing-and-quality/code-quality.md) —
    the `deny.toml` and clippy restriction templates.

[^gen-gates]: [General project-bootstrap/04 — Quality gates](../../../programming/project-bootstrap/04-quality-gates.md).

[^nix-devshell]: [nix/02 — Per-project devShell](../../../tools/nix/02-per-project-devshell.md).

[^gen-ci]: [General project-bootstrap/05 — CI & release-readiness](../../../programming/project-bootstrap/05-ci-and-release-readiness.md).

[^cli-test]: [cli-spec/06 — Testing](../cli-spec/06-testing-and-quality/testing.md) — nextest
    profiles.

[^bp-model]: [Branch model & release-plz](../release-workflow-spec/00-branch-model-and-release-plz.md)
    · general [branch model](../../../programming/release-workflow/00-branch-model.md).

[^bp-firstrun]: [branch protection first-run enablement](../../../tools/git/branch-protection/first-run-enablement.md)

[^promote]: [branch-protection/master-promotion](../../../tools/git/branch-protection/master-promotion.md) —
    how CI fast-forwards `master` onto each release tag (standalone vs inline, the ancestry guard,
    token/bypass).

[^rs-runbook]: [release-workflow-spec/runbook](../release-workflow-spec/runbook.md) — the canonical
    ordered setup sequence.

[^oidc]: [release-workflow-spec/03 — Trusted Publishing / OIDC](../release-workflow-spec/03-trusted-publishing-oidc.md).

[^tokens]: [release-workflow-spec/02 — API tokens and scopes](../release-workflow-spec/02-api-tokens-and-scopes.md).

[^rs-plz]: [release-workflow-spec/04 — release-plz config & CI](../release-workflow-spec/04-release-plz-config.md).

[^branch-model]: [release-workflow-spec/00 — Promoting `master` onto the release tag](../release-workflow-spec/00-branch-model-and-release-plz.md#promoting-master-onto-the-release-tag-the-official-way).

[^app-token]: [branch-protection/github-app-token](../../../tools/git/branch-protection/github-app-token.md) —
    why a GitHub App (not a PAT/deploy key) and the field-by-field App registration.

[^cargo-dist]: [release-workflow-spec/05 — Binary distribution (cargo-dist)](../release-workflow-spec/05-binary-distribution-cargo-dist.md).

[^semver]: [release-workflow-spec/07 — SemVer, yank, rollback](../release-workflow-spec/07-semver-yank-rollback.md).
