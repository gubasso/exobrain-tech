#!/usr/bin/env bash
# Branch-protection setup for a GitHub repo: apply the master/develop/tag
# rulesets, set develop as default, and verify. Run once per new project.
#
# Spec, strategy, prerequisites, caveats: ../workflow.md
#
# Usage:
#   OWNER_REPO=owner/repo ./setup.sh
#   OWNER_REPO=owner/repo REQUIRED_CHECKS="ci/build,ci/test" ./setup.sh
#   OWNER_REPO=owner/repo ./setup.sh --lookup     # just print bypass-actor ids
#
# Environment:
#   OWNER_REPO       owner/repo (required)
#   REQUIRED_CHECKS  comma-separated CI status-check contexts to require on
#                    master + develop. MUST match the job names your CI emits
#                    (see your language's release-workflow-spec). If unset, no
#                    status-check rule is added — nothing to block PRs on.
#   BYPASS_ACTOR_ID  master bypass actor id. Default: github-actions app 15368,
#                    which is a valid bypass actor only on ORGANIZATION repos. On a
#                    PERSONAL account it is rejected (HTTP 422) — pass your installed
#                    GitHub App's ID (Integration actor_type is correct for an App ID).
#   DEFAULT_BRANCH   default branch to set (default: develop).
#
# Requires: gh (authenticated), jq.

set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
rulesets="${here}/rulesets"

: "${OWNER_REPO:?set OWNER_REPO=owner/repo}"
DEFAULT_BRANCH="${DEFAULT_BRANCH:-develop}"

# --- optional: list common bypass-actor ids and exit ------------------------
if [[ "${1:-}" == "--lookup" ]]; then
  # GitHub Actions app: a fixed global id, but a valid bypass actor only on
  # organization repos (rejected on personal accounts).
  printf 'github-actions\t15368\tIntegration\t(org repos only)\n'
  # Your installed GitHub App — the bypass actor to use on a personal account.
  if app_id=$(gh api "/repos/${OWNER_REPO}/installation" --jq '.app_id' 2>/dev/null); then
    printf 'installed-app\t%s\tIntegration\t(pass as BYPASS_ACTOR_ID)\n' "$app_id"
  fi
  exit 0
fi

# --- build the required_status_checks rule from REQUIRED_CHECKS -------------
# Emits a jq filter fragment; empty when no checks requested (rule omitted, so
# the ruleset never blocks on a check that never reports).
checks_filter='.'
if [[ -n "${REQUIRED_CHECKS:-}" ]]; then
  checks_json=$(printf '%s' "$REQUIRED_CHECKS" \
    | jq -Rc 'split(",") | map(gsub("^\\s+|\\s+$";"")) | map(select(length>0)) | map({context: .})')
  checks_filter='.rules += [{"type":"required_status_checks","parameters":{"required_status_checks":$checks,"strict_required_status_checks_policy":true}}]'
fi

apply_ruleset() {
  # $1 = payload file, $2 = "checks" to append status checks (branch rulesets only)
  local payload="$1" add_checks="${2:-}"
  local filter='.'
  local -a jq_args=()

  if [[ -n "${BYPASS_ACTOR_ID:-}" ]] && jq -e '.bypass_actors[0]' "$payload" >/dev/null 2>&1; then
    filter='(.bypass_actors[0].actor_id) = ($id | tonumber)'
    jq_args+=(--arg id "$BYPASS_ACTOR_ID")
  fi
  if [[ "$add_checks" == "checks" && -n "${REQUIRED_CHECKS:-}" ]]; then
    filter="${filter} | ${checks_filter}"
    jq_args+=(--argjson checks "$checks_json")
  fi

  jq "${jq_args[@]}" "$filter" "$payload" \
    | gh api -X POST "/repos/${OWNER_REPO}/rulesets" --input -
}

# --- guard: the master bypass actor must be valid for this repo's owner type ---
# On a personal (User-owned) account the default github-actions app (15368) cannot
# be a ruleset bypass actor, so creating master-protection would fail with HTTP 422.
# Require an explicit BYPASS_ACTOR_ID (your installed GitHub App's ID) there.
if [[ -z "${BYPASS_ACTOR_ID:-}" ]]; then
  owner_type=$(gh api "/repos/${OWNER_REPO}" --jq '.owner.type' 2>/dev/null || echo "")
  if [[ "$owner_type" == "User" ]]; then
    echo "error: ${OWNER_REPO} is a personal (User) repo; the default bypass actor" >&2
    echo "       (github-actions app 15368) is rejected there (HTTP 422). Set" >&2
    echo "       BYPASS_ACTOR_ID to your installed GitHub App's ID and re-run." >&2
    echo "       Find it with: OWNER_REPO=${OWNER_REPO} $0 --lookup" >&2
    exit 1
  fi
fi

echo "== applying master-protection =="
apply_ruleset "${rulesets}/master.json" checks

echo "== applying develop-protection =="
apply_ruleset "${rulesets}/develop.json" checks

echo "== applying release-tags =="
apply_ruleset "${rulesets}/tags.json"

echo "== setting default branch -> ${DEFAULT_BRANCH} =="
gh repo edit "$OWNER_REPO" --default-branch "$DEFAULT_BRANCH"

echo
echo "== verify =="
gh ruleset list -R "$OWNER_REPO"
echo "-- applicable to master --"
gh ruleset check master -R "$OWNER_REPO" || true
echo "-- applicable to develop --"
gh ruleset check develop -R "$OWNER_REPO" || true

cat <<EOF

Done. Manual steps this script cannot do via the API:
  1. Copy workflows/release-promote.yml into the repo at
    .github/workflows/release-promote.yml (commit on a feature branch, PR into develop).
  2. Turn Actions on and grant write — see ../first-run-enablement.md
    (Settings -> Actions -> General: Read and write + allow Actions to create PRs).
EOF
