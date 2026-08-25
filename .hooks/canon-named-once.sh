#!/usr/bin/env sh
# The upstream canon is named in one place, and that name carries no version.
#
# This repository is an instance of the canon, not a sibling of it. Every
# reference it makes is therefore an upstream dependency reference, and a
# dependency is declared once: the root `AGENTS.md` names the project, and
# `.spec-driven-docs/manifest.json` records the version implemented. A second
# mention is a second source of truth, and a version written into prose lags
# the manifest the moment an upgrade lands.
#
# The upstream URL lives in `.hooks/upstream-canon.txt` rather than inline, for
# the reason `no-named-method.sh` keeps its names in a file: a URL spelled here
# would make the gate fail on its own definition.
#
# The projection is not a reference. The vendored payload is excluded whole --
# it is canon-written provenance rather than prose this repository authors --
# and the forms that name it from outside are filtered from the residue: the
# `.`-prefixed path, the `-verify` hook id, and the managed-block markers.
# `_docs/decisions/` is exempt because a record states its own moment and is
# never edited to describe the present.
#
# `git grep`, not `grep -r`: the check is about what the repository carries.
set -u

url=$(head -n 1 .hooks/upstream-canon.txt)
name=${url##*/}
rule=knowledge-base-boundary:the-canon-is-named-once
home=AGENTS.md
status=0

stray=$(git grep -In -F -e "$name" -- \
  ":(exclude)$home" ':(exclude)_docs/decisions' ':(exclude).hooks/upstream-canon.txt' \
  ":(exclude).$name" |
  sed -e "s/\.$name/PROJECTION/g" \
    -e "s/$name-verify/PROJECTION/g" \
    -e "s/BEGIN $name managed/PROJECTION/g" \
    -e "s/END $name managed/PROJECTION/g" |
  grep -F -e "$name") || true
[ -z "$stray" ] || {
  echo "FAIL $rule: the upstream canon is named outside $home"
  echo "$stray"
  status=1
}

pinned=$(git grep -In -F -e "$url/" -- \
  ':(exclude)_docs/decisions' ':(exclude).hooks/upstream-canon.txt' \
  ":(exclude).$name") || true
[ -z "$pinned" ] || {
  echo "FAIL $rule: an upstream link carries a version or a deep path"
  echo "$pinned"
  status=1
}

# The home names it once and names it as the link. A second mention inside the
# one allowed file is still a second statement of the same fact, and a bare name
# is a statement a reader cannot follow.
named=$(git grep -h -F -e "$name" -- "$home" |
  sed -e "s/\.$name/PROJECTION/g" |
  grep -o -F -e "$name" | wc -l)
linked=$(git grep -o -F -e "$url" -- "$home" | wc -l)
[ "$named" = 1 ] && [ "$linked" = 1 ] || {
  echo "FAIL $rule: $home names the upstream canon $named times and links it $linked times, expected once each"
  status=1
}

exit "$status"
