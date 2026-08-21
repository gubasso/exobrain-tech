#!/usr/bin/env sh
# A filed report body fits in 79 columns.
#
# A tracker renders a comment in a fixed-width box and reflows nothing, so a
# line past that width wraps where the tracker chooses. Prose survives that; the
# aligned table and the annotated source excerpt a report is made of do not, and
# those are the parts carrying the argument. 79 is the width terminals, diff
# views, and quoted replies already agree on, and it leaves room for a `> `.
#
# The body sits in a fence, which is what stops a markdown formatter reflowing
# it and what makes the width checkable here rather than after the paste. Body
# text outside the fence fails for the same reason: dprint owns the wrapping of
# everything unfenced, at a width it chooses rather than the tracker's.
#
# The heading match is guarded by the fence state. A body in a tracker markup
# that spells a heading `## ` -- or quotes one -- would otherwise close the
# section from inside itself and take the rest of the body out of the check.
#
# A report kept as a standalone file rather than a fenced section is the same
# rule through .editorconfig: `max_line_length = 79` scoped to the ticket
# bodies, read by the editorconfig-checker hook.
set -eu

set -- _docs/reference/known-issues/KI-?*.md
[ -e "$1" ] || { echo "FAIL no known-issue records matched; the layout moved"; exit 1; }

awk '
  FNR == 1         { report = 0; fence = 0 }
  /^## Report$/    { report = 1; next }
  !fence && /^## / { report = 0 }
  report && /^```/ { fence = !fence; next }
  report && fence && length($0) > 79 {
    printf "FAIL known-issues:a-report-body-fits-in-79-columns %s:%d: %d columns\n",
      FILENAME, FNR, length($0); bad = 1 }
  report && !fence && $0 != "" {
    printf "FAIL known-issues:a-report-body-fits-in-79-columns %s:%d: body outside a fence\n",
      FILENAME, FNR; bad = 1 }
  END { exit bad }
' "$@"
