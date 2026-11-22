#!/usr/bin/env sh
# Refresh the evidence of the reference hook sets: for each set, the hook ids
# that the manifest of each upstream repository publishes at the pinned rev
# (hook-sets:reference-hook-sets-hold-their-evidence).
#
# This is the one step of the maintenance of the sets that reads the network.
# It runs after `prek update -c <set>.yaml`, and writes
# programming/project-bootstrap/hook-sets/evidence/<set>.yaml. The offline gate
# .hooks/hook-sets.sh then holds each set to that file, so no test clones.
#
# With set names as arguments it refreshes only those sets.
set -eu

unset GIT_DIR GIT_INDEX_FILE GIT_WORK_TREE GIT_OBJECT_DIRECTORY \
  GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_COMMON_DIR GIT_PREFIX

root=$(cd "$(dirname "$0")/.." && pwd)
dir="$root/programming/project-bootstrap/hook-sets"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

if [ "$#" -eq 0 ]; then
  for f in "$dir"/*.yaml; do
    n=${f##*/}
    set -- "$@" "${n%.yaml}"
  done
fi

mkdir -p "$dir/evidence"
for set in "$@"; do
  conf="$dir/$set.yaml"
  [ -f "$conf" ] || {
    echo "hook-sets-evidence: no set $set" >&2
    exit 1
  }
  out="$work/$set.yaml"
  {
    echo "# The hook ids that each upstream manifest publishes at the rev that"
    echo "# $set.yaml pins. .hooks/hook-sets-evidence.sh writes this file, and"
    echo "# .hooks/hook-sets.sh holds the set to it offline."
    echo "repos:"
  } > "$out"
  # Every repository with a rev: `builtin`, `local`, and `meta` have none.
  yq -r '.repos[] | select(has("rev")) | .repo + " " + .rev' "$conf" |
    while read -r repo rev; do
      clone="$work/clone"
      rm -rf "$clone"
      git -c advice.detachedHead=false clone --quiet --depth 1 --branch "$rev" "$repo" "$clone"
      ids=$(yq -o=json -I0 '[.[].id]' "$clone/.pre-commit-hooks.yaml")
      {
        echo "  - repo: $repo"
        echo "    rev: $rev"
        echo "    ids: $ids"
      } >> "$out"
    done
  mv "$out" "$dir/evidence/$set.yaml"
done
