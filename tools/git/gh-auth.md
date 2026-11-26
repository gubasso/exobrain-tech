# gh: Authentication Setup & Fixes

> TLDR runbook. Check `gh` auth, log in (token goes to the system keyring by default), wire the HTTPS git credential helper, and verify — top to bottom. This is the source of truth for `gh` authentication. For GitLab's `glab`, see [glab-auth.md](./glab-auth.md).

## TLDR

```bash
gh auth status                                    # 1. where do I stand?
env | grep -E '^(GH_TOKEN|GITHUB_TOKEN)='         # 2. stale overrides?
gh auth login                                     # 3. login (keyring is default)
gh auth setup-git                                 # 4. wire the git credential helper
printf 'protocol=https\nhost=github.com\n\n' | git credential fill  # 5. verify
```

## 1. Check auth

```bash
gh auth status
```

Expected when healthy:

```text
✓ Logged in to github.com account <user>
✓ Git operations for github.com configured to use https protocol
✓ Token scopes: 'gist', 'read:org', 'repo', 'workflow'
```

`You are not logged into any GitHub hosts` → continue below.

## 2. Clear stale env overrides

Environment variables **override** the stored token, so a stale one masks a good login. Check, then unset for the current shell if present:

```bash
env | grep -E '^(GH_TOKEN|GITHUB_TOKEN|GH_HOST)='

unset GH_TOKEN GITHUB_TOKEN
```

> If one is set permanently (shell rc, CI secrets, secrets manager), remove it there too — otherwise it keeps shadowing the keyring token every session.

## 3. Log in (token → system keyring by default)

`gh` stores the token in your OS keyring (secure storage) **by default** — there is **no** `--use-keyring` flag; keyring is automatic when a Secret Service backend is available.

```bash
gh auth login
```

When prompted, choose **GitHub.com** → **HTTPS** → authenticate via browser or a Personal Access Token.

- **Opt out of the keyring** with `--insecure-storage`, which writes the token in cleartext to `~/.config/gh/hosts.yaml`:

  ```bash
  gh auth login --insecure-storage   # NOT recommended
  ```

- **Headless / container fallback.** When no keyring is reachable, `gh` falls back to writing the token to `~/.config/gh/hosts.yaml` — the same as `--insecure-storage`. Confirm where the token landed:

  ```bash
  grep -A3 'github.com' ~/.config/gh/hosts.yaml   # a token line here == plaintext fallback
  gh auth status
  ```

## 4. Wire the git credential helper

`gh` has a dedicated command for this (the equivalent `glab` lacks):

```bash
gh auth setup-git                       # all authenticated hosts
gh auth setup-git --hostname github.com # or scope to one host
```

Confirm exactly one helper is registered:

```bash
git config --global --get-all credential.https://github.com.helper
# Expected: a single line — !/usr/bin/gh auth git-credential
```

If more than one line appears, reset to a single entry:

```bash
git config --global --unset-all credential.https://github.com.helper
gh auth setup-git --hostname github.com
```

## 5. Verify end-to-end

```bash
# Helper resolves credentials with no prompt (do not paste the output anywhere)
printf 'protocol=https\nhost=github.com\n\n' | git credential fill
# Expected: prints protocol/host/username/password=<token>

# A real HTTPS git operation succeeds without prompting
git fetch
```

## Troubleshooting

**HTTPS clone still invokes an askpass helper** (e.g. `unable to read askpass response from '.../ssh-askpass-rofi'`) — git isn't getting credentials from `gh`. Clear any askpass override so git uses the credential helper:

```bash
git config --global --get core.askPass   # inspect
echo "$GIT_ASKPASS"; echo "$SSH_ASKPASS"

git config --global --unset core.askPass  # if set globally
unset GIT_ASKPASS SSH_ASKPASS             # for this session
```

Then re-check step 4 (single helper) and step 5.

## How it works

`gh auth setup-git` registers `gh auth git-credential` as a [git credential helper](https://git-scm.com/docs/gitcredentials). When git needs to authenticate an HTTPS remote, it calls the helper, which returns the token from `gh`'s store (keyring or `~/.config/gh/hosts.yaml`) — no prompt, no askpass. GitLab's `glab` uses the same mechanism, minus the `setup-git` convenience; see [glab-auth.md](./glab-auth.md).

## The identity for agents

The runbook above logs in the operator. The agent identity never uses it: an agent holds no token and never runs `gh auth login`. A program on the host mints the token of the agent identity. [The authorization boundary](../../programming/agent-integration/authorization-boundary.md#the-forge-bounds-the-agent-identity) states why each purpose gets its own identity.

The facts in this section come from the GitHub documentation and gh 2.105.0, read on 2026-11-25. One line names a probe of the same date, and one point is unproven. Both carry their label.

### A GitHub App, not a personal token

The documentation states that a fine-grained personal access token does not support a repository where the user is an outside or repository collaborator ([managing your personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)). A bot account that is a collaborator therefore gets no fine-grained token for that repository. The agent identity is a GitHub App instead.

The App asks for the minimum permissions ([best practices for creating a GitHub App](https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/best-practices-for-creating-a-github-app)):

- Contents: read and write
- Pull requests: read and write
- Metadata: read
- no Workflows, Administration, or Secrets permission

Install the App on only select repositories, never on all repositories of the owner.

### The installation token

The documentation states that an installation token expires after one hour ([generating an installation access token](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-an-installation-access-token-for-a-github-app)). A host program mints a new token before the old one expires. It signs an App JWT with `openssl` and sends it with `curl` as `Authorization: Bearer <jwt>` ([generating a JWT for a GitHub App](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-json-web-token-jwt-for-a-github-app)).

The gh manual and the source of gh 2.105.0 show that default gh authentication sends the stored token. Send the JWT with `curl`, or with `gh api` and an explicit `Authorization` header.

The documentation states that an App private key does not expire, that an App holds up to 25 keys, and that only the browser generates and deletes a key ([managing private keys for GitHub Apps](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/managing-private-keys-for-github-apps)). Rotate the key by overlap:

1. Generate the new key in the browser.
2. Give the new key to the host program.
3. Mint a token with the new key, and make sure that it works.
4. Delete the old key in the browser.

[The security baseline](../../programming/project-bootstrap/06-security-baseline.md#secrets-hygiene) states the rotation period and the rotation by overlap for every long-lived key.

### Protection needs a paid plan on a private repository

The documentation states which plans carry protected branches and rulesets on a private repository:

- for a personal account, GitHub Pro ([GitHub's plans](https://docs.github.com/en/get-started/learning-about-github/githubs-plans))
- for an organization, Team and Enterprise Cloud ([available rules for rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets), [about protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches))

The probe of 2026-11-25 showed one case. For a private repository of a personal account on GitHub Free, both the rulesets API and the branch protection API answered:

```text
403 Upgrade to GitHub Pro or make this repository public to enable this feature.
```

A repository with no protection stays out of the reach of the agent identity.

### The trunk ruleset

The documentation states that the rule "Restrict updates" lets only bypass actors push to a matching branch or tag ([available rules for rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)). A trunk ruleset with "Restrict updates" and the operator alone as the bypass keeps the agent identity off the trunk.

The page names a push. That a pull request merge counts as an update rests only on community reports ([discussion #205590](https://github.com/orgs/community/discussions/205590)). This point is unproven.

A ruleset with 0 approvals and only the required checks lets any identity with write access merge its own pull request. Such a ruleset does not keep the agent identity off the trunk.

### The release refs

The release refs need their own rulesets, beside the trunk ruleset:

- One ruleset sets "Restrict creations" on every tag that matches the release tag pattern, such as `v*`.
- One ruleset sets "Restrict creations" and "Restrict updates" on `release/**`.

The repository admin role and the release bot App bypass both rulesets.

A rule against the update and the deletion of a tag does not stop its creation. Without "Restrict creations", the agent identity can create a release tag, and a workflow that runs on a pushed tag then starts a release. No test on 2026-11-25 showed a non-bypass identity refused at a release tag or a `release/**` branch.
