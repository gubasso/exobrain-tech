#!/usr/bin/env sh
# A record filed upstream carries the body it was filed with.
#
# Once `upstream:` names one issue rather than a tracker, the report exists in
# two places, and only one of them is under review here. A `## Report` section
# holding the filed text in the tracker's own markup keeps the repository
# holding what was actually said upstream, and makes the next filing a paste.
#
# What the gate can see is the pairing: a specific upstream reference and the
# section. That the section is the filed text, in the tracker's markup, is held
# by review — as is the body written ahead of filing, which no frontmatter
# announces.
set -eu

set -- _docs/reference/known-issues/KI-?*.md
[ -e "$1" ] || { echo "FAIL no known-issue records matched; the layout moved"; exit 1; }

# A specific item: a URL ending in /<digits>, or a #<digits> / bsc#<digits> id.
filed='^upstream:.*([/#][0-9]+/?[[:space:]]*)$'

bad=""
for f; do
  grep -qE "$filed" "$f" || continue
  grep -qE '^## Report$' "$f" || bad="$bad $f"
done
[ -z "$bad" ] || {
  echo "FAIL known-issues:a-filed-record-carries-its-report:$bad"
  exit 1
}
