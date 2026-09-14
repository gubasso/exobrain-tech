#!/usr/bin/env bash
# Hold the flake's evaluated output set to the one system this project supports.
#
# The claim is about what the flake exposes, so the check evaluates it. A text
# scan over flake.nix cannot make the same claim: Nix spells one attribute tree
# several ways, and a nested set declares an output with no dotted family
# before its key.
#
# See _docs/specs/SPEC-project-platform.md, rule
# project-platform:one-supported-system.
set -euo pipefail

supported="x86_64-linux"
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
reference="$(cd "${1:-${root}}" && pwd)"

found="$(
  nix eval --impure --json \
    --expr "import \"${root}/.hooks/one-system.nix\" \"${reference}\"" |
    jq -r 'unique | join(" ")'
)"

if [ "${found}" != "${supported}" ]; then
  echo "the flake exposes [${found}], and ${supported} is the only supported system" >&2
  echo "project-platform:one-supported-system" >&2
  exit 1
fi

echo "the flake exposes ${supported} and nothing else"
