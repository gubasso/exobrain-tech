#!/usr/bin/env sh
# Every markdown suppression names the case that justifies it, and that case
# resolves to a record.
#
# A suppression with no case behind it becomes permanent by default: the next
# reader takes it for a design choice and nothing says what would retire it.
#
# Scoped to the HTML-comment form, which is the suppression this repository's
# own content can carry. Prose about a suppression — a chapter teaching
# `# shellcheck disable` — is content, not a suppression, and the shell scripts
# the buckets ship are gated by their own story.
set -u

sup=$(git grep -nE '<!-- *(dprint-ignore|markdownlint-disable)' -- \
  ':(exclude)_docs/reference/known-issues' |
  grep -v -e dprint-ignore-end -e markdownlint-enable) || true

bad=$(printf '%s\n' "$sup" | grep '<!--' | grep -v 'KI-[a-z0-9-]') || true
[ -z "$bad" ] || {
  echo "FAIL known-issues:a-suppression-names-its-case"
  echo "$bad"
  exit 1
}

for c in $(printf '%s\n' "$sup" | grep -oE 'KI-[a-z0-9-]+' | sort -u); do
  [ -f "_docs/reference/known-issues/$c.md" ] || {
    echo "FAIL known-issues:a-suppression-names-its-case: $c resolves to no record"
    exit 1
  }
done
