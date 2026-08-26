#!/usr/bin/env sh
# A guide is a recipe: prerequisites, then an ordered list of steps.
#
# The three things a command can decide about a recipe are the three a reader
# loses first: a step section that opens with prose instead of step one, a
# recipe that assumes state it never names, and a subtask promoted to a
# sibling of the step it belongs to. What the command cannot decide -- whether
# a step's precondition is actually met, whether the prose earns its place --
# is held at review, and `SPEC-guides.md` says so.
#
# A heading is read only outside a fence. A template ships the shape it seeds
# inside a fenced block, and a scanner blind to fences would fail the template
# for demonstrating the very form it teaches.
set -u

status=0

for file in "$@"; do
  awk -v file="$file" '
    match($0, /^[ \t]*(`+|~+)/) {
      marker = substr($0, RSTART, RLENGTH)
      gsub(/[ \t]/, "", marker)
      if (length(marker) >= 3) {
        if (!open) { open = marker; next }
        if (substr(open, 1, 1) == substr(marker, 1, 1) && length(marker) >= length(open)) open = ""
        next
      }
    }
    open { next }

    /^## / {
      if (insteps) insteps = 0
      if ($0 ~ /^## Prerequisites/) prereq = FNR
      if ($0 ~ /^## Steps/) {
        steps = FNR
        heading = $0
        insteps = 1
        wantfirst = 1
      }
      next
    }

    insteps && wantfirst && NF {
      wantfirst = 0
      if ($0 !~ /^1\. /) opener = FNR
    }

    insteps && /^[-*+][ \t]/ { if (!bullet) bullet = FNR }

    END {
      if (!steps) exit 0
      if (heading != "## Steps")
        report("the step section is headed \"" heading "\", not \"## Steps\"")
      if (!prereq || prereq > steps)
        report("no \"## Prerequisites\" section precedes the steps")
      if (opener)
        report("line " opener ": the step section opens with prose, not \"1. \"")
      if (bullet)
        report("line " bullet ": a top-level bullet; a subtask indents under its step")
      exit bad
    }

    function report(msg) {
      if (!bad) print "FAIL guides:a-guide-is-an-ordered-recipe"
      bad = 1
      print "  " file ": " msg
    }
  ' "$file" || status=1
done

exit "$status"
