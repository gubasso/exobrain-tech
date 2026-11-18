#!/usr/bin/env sh
# Hold every source of exobrain-tech that a skill names to a path of this
# checkout.
#
# A skill under crates/exobrain/skills/ names each set and chapter that it
# reads as `$EXOBRAIN_TECH/<path>`, and a project reads that path at the tag
# it pins. A renamed chapter or a removed set would leave the skill pointing at
# nothing in the next release, so the gate fails on each path that does not
# exist here. It also fails when no skill names a source, because a gate that
# finds nothing to check proves nothing.
#
# The optional argument is the root to check, so .hooks/test-gates.sh can run
# the gate on a throwaway tree.
set -eu

root=${1:-$(cd "$(dirname "$0")/.." && pwd)}
skills="$root/crates/exobrain/skills"

# The dollar sign of the marker is literal, so the quotes stay single.
# shellcheck disable=SC2016
paths=$(grep -rhoE '\$EXOBRAIN_TECH/[A-Za-z0-9._/-]+' "$skills" |
  sed -e 's|^\$EXOBRAIN_TECH/||' -e 's|[.]*$||' | sort -u)

if [ -z "$paths" ]; then
  echo "skill-sources: no skill under $skills names a source" >&2
  exit 1
fi

status=0
for path in $paths; do
  if [ ! -e "$root/$path" ]; then
    echo "skill-sources: a skill names \$EXOBRAIN_TECH/$path, which does not exist" >&2
    status=1
  fi
done
exit "$status"
