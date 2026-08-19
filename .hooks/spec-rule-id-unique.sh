#!/usr/bin/env sh
# A rule ID resolves to exactly one requirement across the whole shelf.
#
# A commit citing a duplicated ID names two rules at once, so the citation stops
# being an address. Corpus-wide by construction: the duplicate is only visible
# when every spec is read together.
set -eu

set -- _docs/specs/SPEC-?*.md
[ -e "$1" ] || { echo "FAIL no specs matched; the layout moved"; exit 1; }

dupes=$(grep -hoE '^### `[a-z0-9-]+:[a-z0-9-]+`' "$@" | sort | uniq -d)
[ -z "$dupes" ] || {
  echo "FAIL docs-specs:a-rule-id-is-unique"
  echo "$dupes"
  exit 1
}
