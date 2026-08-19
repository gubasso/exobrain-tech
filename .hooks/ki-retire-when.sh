#!/usr/bin/env sh
# Every known-issue record carries the condition under which it is removed.
#
# A record whose workaround has no exit becomes permanent by default, and the
# next reader takes it for a design choice. The key must carry a value: an empty
# `retire_when:` states no condition and would otherwise report success.
set -eu

set -- _docs/reference/known-issues/KI-?*.md
[ -e "$1" ] || { echo "FAIL no known-issue records matched; the layout moved"; exit 1; }

bad=""
for f; do
  grep -qE '^retire_when:[[:space:]]*[^[:space:]]' "$f" || bad="$bad $f"
done
[ -z "$bad" ] || {
  echo "FAIL known-issues:a-record-carries-its-retirement-condition:$bad"
  exit 1
}
