#!/usr/bin/env sh
# Every known-issue record walks its mechanism step by step.
#
# A record that only names the defect is unfalsifiable to everyone but its
# author, and the reader who arrives from a suppression's case id knows nothing
# about the bug. The heading is what the gate can see; the walkthrough being a
# run rather than a restatement is held by review.
#
# The heading is matched anchored and case-sensitively, so a record that buries
# the phrase in prose does not clear the gate.
set -eu

set -- _docs/reference/known-issues/KI-?*.md
[ -e "$1" ] || { echo "FAIL no known-issue records matched; the layout moved"; exit 1; }

bad=""
for f; do
  grep -qE '^## How it works$' "$f" || bad="$bad $f"
done
[ -z "$bad" ] || {
  echo "FAIL known-issues:a-record-walks-the-mechanism:$bad"
  exit 1
}
