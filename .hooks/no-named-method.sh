#!/usr/bin/env sh
# A name this repository retired does not come back.
#
# The universal rule — that no planning, project-management, or workflow method
# is named here — is reviewer work, because no command knows every method name.
# What a command can hold is the list of names already removed, so a removal
# cannot be undone one file at a time.
#
# The names live in `.hooks/unnamed-methods.txt` rather than inline, because a
# pattern written into the gate would match itself and the gate would fail on
# its own definition. Two paths are exempt: `_docs/decisions/`, where a record
# states its own moment and is never edited to describe the present, and the
# vendored payload's `manifest.json`, machine-written provenance rather than
# prose this repository authors.
#
# `git grep`, not `grep -r`: the check is about what the repository carries, so
# a gitignored link cache or build output is not in scope and would otherwise
# fail a commit it has nothing to do with.
set -u

bad=$(git grep -In -f .hooks/unnamed-methods.txt -- \
  ':(exclude)_docs/decisions' ':(exclude).hooks/unnamed-methods.txt' \
  ':(exclude).spec-driven-docs/manifest.json') || true
[ -z "$bad" ] || {
  echo "FAIL knowledge-base-boundary:a-retired-name-does-not-return"
  echo "$bad"
  exit 1
}
