#!/usr/bin/env sh
# Prove that every gate this repository adds can be made to fail.
#
# A gate that has never been observed failing has not been observed at all, and
# the failure modes worth catching are the quiet ones: a matcher whose exit code
# is inverted names the clean files and passes every breach, an `MD043` array
# that lost `match_case` accepts a heading in the wrong case, and a `grep -L`
# over a key accepts that key with no value. All three report success.
#
# Every check runs the gate that ships, never a copy of it. The corpus gates
# read paths relative to the working directory and two of them run `git grep`,
# so each is exercised against a throwaway tree with the layout they expect —
# which is why they are scripts under `.hooks/` and not shell bodies inlined
# into the hook config.
set -eu

# `git commit` exports GIT_DIR, GIT_INDEX_FILE, and GIT_WORK_TREE into every
# hook process, and this script runs as a hook. The throwaway trees below call
# `git init` and `git add -A`; inheriting those variables would point that work
# at the committing repository's index, corrupting the commit in progress and
# making every later hook report against the wrong file set.
unset GIT_DIR GIT_INDEX_FILE GIT_WORK_TREE GIT_OBJECT_DIRECTORY \
  GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_COMMON_DIR GIT_PREFIX

root=$(cd "$(dirname "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

fail() {
  echo "test-gates: $1" >&2
  exit 1
}

# accept/reject take a label and the command under test. `reject` is the half
# that matters: a gate nobody has seen refuse anything is a green light.
accept() {
  label=$1
  shift
  "$@" > /dev/null 2>&1 || fail "$label rejected a conforming artifact"
}

reject() {
  label=$1
  shift
  if "$@" > /dev/null 2>&1; then
    fail "$label accepted a violation"
  fi
}

# A throwaway tree carrying one conforming artifact of every kind the corpus
# gates read, tracked by git so `git grep` sees it.
newtree() {
  t="$work/$1"
  rm -rf "$t"
  mkdir -p "$t/_docs/specs" "$t/_docs/decisions" "$t/_docs/reference/known-issues" "$t/.hooks"
  cp "$root/.hooks/unnamed-methods.txt" "$t/.hooks/unnamed-methods.txt"

  cat > "$t/_docs/specs/SPEC-sample.md" << 'EOF'
# Sample Specification

## Purpose

What a sample is.

## Requirements

### `sample:a-rule-names-its-gate` — A rule names its gate

A rule MUST name the gate that enforces it.

#### Scenario: A rule is stated with no verification

- GIVEN a requirement written as a MUST
- WHEN it carries no verification
- THEN the commit fails

Verify: `pre-commit run sample-gate --all-files`
EOF

  cat > "$t/.pre-commit-config.yaml" << 'EOF'
repos:
  - repo: local
    hooks:
      - id: sample-gate
        name: the sample gate
EOF

  printf '# A sample case\n\nretire_when: upstream ships the fix\n\n## How it works\n\nStep one leaves the marker set; step two reads it.\n' \
    > "$t/_docs/reference/known-issues/KI-sample-bug.md"
  printf 'A page with a suppression.\n\n<!-- dprint-ignore-start KI-sample-bug -->\n\ntext\n\n<!-- dprint-ignore-end -->\n' \
    > "$t/_docs/reference/sample.md"

  git -C "$t" init -q
  git -C "$t" add -A
  cd "$t"
}

# --- no-self-narration: the prose gate --------------------------------------
#
# The fence cases are the ones a column-zero matcher gets wrong: a fence inside
# a list item is indented, and a tilde fence is a fence.
n="$work/narration"
mkdir -p "$n"
printf 'This formerly lived in the other shelf.\n' > "$n/bad.md"
printf 'Write no `formerly` here.\n\n```text\nformerly\n```\n' > "$n/inline.md"
printf 'Item:\n\n  ```text\n  formerly\n  ```\n' > "$n/indented.md"
printf 'Item:\n\n~~~text\nformerly\n~~~\n' > "$n/tilde.md"

printf '> Quoted:\n>\n> ```text\n> formerly\n> ```\n' > "$n/quoted.md"
printf '1. Step:\n\n     ```text\n     formerly\n     ```\n' > "$n/nested.md"
printf 'Write no ``formerly`` here.\n' > "$n/wide-span.md"
printf '> This formerly lived elsewhere.\n' > "$n/quoted-prose.md"
printf '> ```text\n> code\n\nThis formerly lived elsewhere.\n' > "$n/quote-escape.md"
printf '1. Step:\n\n     ```text\n     code\n\nThis formerly lived here.\n' > "$n/list-escape.md"
printf 'This `formerly`` lived elsewhere.\n' > "$n/uneven-span.md"
printf '1. Step:\n\n     ```text\n     code\n\n> This formerly lived elsewhere.\n' > "$n/list-to-quote.md"
printf 'This `formerly\nlived` elsewhere.\n' > "$n/multiline-span.md"
printf 'Line one.\nThis `formerly lived elsewhere.\nLine three.\n' > "$n/unclosed-span.md"
printf 'Opening `\n\nThis formerly lived elsewhere.\n\nClosing `\n' > "$n/cross-paragraph.md"

narrate() { awk -f "$root/.hooks/no-self-narration.awk" "$1"; }
reject "no-self-narration" narrate "$n/bad.md"
accept "no-self-narration" narrate "$n/inline.md"
accept "no-self-narration" narrate "$n/indented.md"
accept "no-self-narration" narrate "$n/tilde.md"
accept "no-self-narration" narrate "$n/quoted.md"
accept "no-self-narration" narrate "$n/nested.md"
accept "no-self-narration" narrate "$n/wide-span.md"
reject "no-self-narration" narrate "$n/quoted-prose.md"
# A fence left open ends with the container that held it; prose after it is
# prose, and a scanner holding fence state open silences the rest of the file.
reject "no-self-narration" narrate "$n/quote-escape.md"
reject "no-self-narration" narrate "$n/list-escape.md"
# Backtick runs of unequal length open no code span, so what sits between them
# is prose.
reject "no-self-narration" narrate "$n/uneven-span.md"
# Stepping out of a list item and into a block quote leaves the container the
# fence opened in, even though the quote depth went up rather than down.
reject "no-self-narration" narrate "$n/list-to-quote.md"
# A code span may cross a line ending; a run that never closes is literal text.
accept "no-self-narration" narrate "$n/multiline-span.md"
reject "no-self-narration" narrate "$n/unclosed-span.md"
# A span never crosses a blank line, so two stray backticks in separate
# paragraphs pair with nothing and the prose between them is prose.
reject "no-self-narration" narrate "$n/cross-paragraph.md"

# --- Heading shapes: md-adr and md-spec -------------------------------------
mkdir -p "$work/shapes"
cd "$work/shapes"
adr() {
  printf '# T\n\n## Context and Problem Statement\n\nW.\n\n## Considered Options\n\n- One.\n\n## Decision Outcome\n\nOne.\n\n## Consequences\n\n- Good.\n\n%s\n\nAccepted\n' "$1"
}
adr '## Status' > ADR-ok.md
adr '## status' > ADR-case.md
{
  adr '## Status'
  printf '\n## Sixth\n\nNo.\n'
} > ADR-sixth.md
printf '# T\n\n## Purpose\n\nW.\n\n## Requirements\n\n### A\n\nB.\n' > SPEC-ok.md
printf '# T\n\n## purpose\n\nW.\n\n## Requirements\n\n### A\n\nB.\n' > SPEC-case.md

md() { markdownlint-cli2 --config "$root/.markdownlint/$1" "$2"; }
accept "md-adr" md adr.markdownlint-cli2.jsonc ADR-ok.md
reject "md-adr" md adr.markdownlint-cli2.jsonc ADR-case.md
reject "md-adr" md adr.markdownlint-cli2.jsonc ADR-sixth.md
accept "md-spec" md spec.markdownlint-cli2.jsonc SPEC-ok.md
reject "md-spec" md spec.markdownlint-cli2.jsonc SPEC-case.md

# --- Filenames: adr-filename-shape and ki-filename-shape --------------------
newtree filenames
: > _docs/decisions/TEMPLATE-adr.md
: > _docs/decisions/ADR-a-good-slug.md
accept "adr-filename-shape" "$root/.hooks/adr-filename-shape.sh" \
  _docs/decisions/TEMPLATE-adr.md _docs/decisions/ADR-a-good-slug.md
reject "adr-filename-shape" "$root/.hooks/adr-filename-shape.sh" _docs/decisions/ADR-0001-thing.md
reject "adr-filename-shape" "$root/.hooks/adr-filename-shape.sh" _docs/decisions/ADR-Bad_Name.md
reject "adr-filename-shape" "$root/.hooks/adr-filename-shape.sh" _docs/decisions/0001-use-postgres.md

accept "ki-filename-shape" "$root/.hooks/ki-filename-shape.sh" \
  _docs/reference/known-issues/KI-sample-bug.md _docs/reference/known-issues/KI-md043-quirk.md
reject "ki-filename-shape" "$root/.hooks/ki-filename-shape.sh" _docs/reference/known-issues/KI-0007.md
reject "ki-filename-shape" "$root/.hooks/ki-filename-shape.sh" _docs/reference/known-issues/KI-Bad_Name.md
reject "ki-filename-shape" "$root/.hooks/ki-filename-shape.sh" _docs/reference/known-issues/case.md

# --- spec-verify-hooks-exist ------------------------------------------------
newtree verify-hooks
accept "spec-verify-hooks-exist" "$root/.hooks/spec-verify-hooks-exist.sh"
sed -i 's/sample-gate/renamed-gate/' .pre-commit-config.yaml
reject "spec-verify-hooks-exist" "$root/.hooks/spec-verify-hooks-exist.sh"

# --- spec-requirement-parts -------------------------------------------------
newtree requirement-parts
accept "spec-requirement-parts" "$root/.hooks/spec-requirement-parts.sh" _docs/specs/SPEC-sample.md
printf '# T\n\n## Purpose\n\nP.\n\n## Requirements\n\n### A plain heading\n\nMUST.\n\nVerify: `pre-commit run sample-gate --all-files`\n' \
  > _docs/specs/SPEC-noid.md
reject "spec-requirement-parts" "$root/.hooks/spec-requirement-parts.sh" _docs/specs/SPEC-noid.md
printf '# T\n\n## Purpose\n\nP.\n\n## Requirements\n\n### `a:b` — X\n\nMUST.\n' > _docs/specs/SPEC-noverify.md
reject "spec-requirement-parts" "$root/.hooks/spec-requirement-parts.sh" _docs/specs/SPEC-noverify.md
# The totals balance and the second requirement still owns no verification. A
# gate counting rule ids against `Verify:` lines over the whole file passes this.
printf '# T\n\n## Purpose\n\nP.\n\n## Requirements\n\n### `a:b` — X\n\nMUST.\n\nVerify: `pre-commit run sample-gate --all-files`\n\nVerify: `pre-commit run sample-gate --all-files`\n\n### `c:d` — Y\n\nMUST.\n' \
  > _docs/specs/SPEC-lopsided.md
reject "spec-requirement-parts" "$root/.hooks/spec-requirement-parts.sh" _docs/specs/SPEC-lopsided.md

# --- spec-rule-id-unique ----------------------------------------------------
newtree rule-id
accept "spec-rule-id-unique" "$root/.hooks/spec-rule-id-unique.sh"
sed 's/^# Sample Specification/# Second Specification/' _docs/specs/SPEC-sample.md \
  > _docs/specs/SPEC-second.md
reject "spec-rule-id-unique" "$root/.hooks/spec-rule-id-unique.sh"

# --- spec-size-cap ----------------------------------------------------------
newtree size-cap
accept "spec-size-cap" "$root/.hooks/spec-size-cap.sh"
{
  printf '# Long\n\n## Purpose\n\nP.\n\n## Requirements\n\n'
  i=0
  while [ "$i" -lt 320 ]; do
    printf 'padding line\n'
    i=$((i + 1))
  done
} > _docs/specs/SPEC-long.md
reject "spec-size-cap" "$root/.hooks/spec-size-cap.sh"
rm _docs/specs/SPEC-long.md
{
  printf '# Middling\n\n## Purpose\n\nP.\n\n## Requirements\n\n'
  i=0
  while [ "$i" -lt 120 ]; do
    printf 'padding line\n'
    i=$((i + 1))
  done
} > _docs/specs/SPEC-middling.md
reject "spec-size-cap" "$root/.hooks/spec-size-cap.sh"
rm _docs/specs/SPEC-middling.md
# One opening marker and nothing to close it: the TOC exclusion would otherwise
# delete every line after it and report an over-budget spec as fitting.
{
  printf '# Unclosed\n\n<!--TOC-->\n\n## Purpose\n\nP.\n\n## Requirements\n\n'
  i=0
  while [ "$i" -lt 320 ]; do
    printf 'padding line\n'
    i=$((i + 1))
  done
} > _docs/specs/SPEC-unclosed.md
reject "spec-size-cap" "$root/.hooks/spec-size-cap.sh"

# --- no-named-method --------------------------------------------------------
#
# The violation is built from the pattern file rather than written out, for the
# reason the gate keeps its patterns in a file at all: a name spelled here would
# make this suite fail the gate it is testing. Reading the list also keeps the
# control honest when a name is added to it.
newtree named-method
named=$(head -n 1 "$root/.hooks/unnamed-methods.txt")
accept "no-named-method" "$root/.hooks/no-named-method.sh"
printf 'The method is %s.\n' "$named" > _docs/reference/method.md
git add -A
reject "no-named-method" "$root/.hooks/no-named-method.sh"
git rm -q --cached _docs/reference/method.md
rm _docs/reference/method.md
printf '# A record\n\nThe method was %s.\n' "$named" > _docs/decisions/ADR-a-method-was-chosen.md
git add -A
accept "no-named-method" "$root/.hooks/no-named-method.sh"

# --- suppression-names-its-case ---------------------------------------------
newtree suppression
accept "suppression-names-its-case" "$root/.hooks/suppression-names-its-case.sh"
printf 'A page.\n\n<!-- dprint-ignore-start -->\n\ntext\n\n<!-- dprint-ignore-end -->\n' \
  > _docs/reference/naked.md
git add -A
reject "suppression-names-its-case" "$root/.hooks/suppression-names-its-case.sh"
rm _docs/reference/naked.md
printf 'A page.\n\n<!-- dprint-ignore-start KI-no-such-case -->\n\ntext\n\n<!-- dprint-ignore-end -->\n' \
  > _docs/reference/dangling.md
git add -A
reject "suppression-names-its-case" "$root/.hooks/suppression-names-its-case.sh"

# --- ki-retire-when ---------------------------------------------------------
newtree retire-when
accept "ki-retire-when" "$root/.hooks/ki-retire-when.sh"
printf '# Empty\n\nretire_when:\n' > _docs/reference/known-issues/KI-empty-condition.md
reject "ki-retire-when" "$root/.hooks/ki-retire-when.sh"
rm _docs/reference/known-issues/KI-empty-condition.md
printf '# Missing\n\nNo condition here.\n' > _docs/reference/known-issues/KI-no-condition.md
reject "ki-retire-when" "$root/.hooks/ki-retire-when.sh"

# --- ki-mechanism-walkthrough -----------------------------------------------
#
# The heading is matched anchored, so a record naming the phrase in prose is the
# case that separates a real walkthrough from a mention of one.
newtree mechanism-walkthrough
accept "ki-mechanism-walkthrough" "$root/.hooks/ki-mechanism-walkthrough.sh"
printf '# No walkthrough\n\nretire_when: never\n' > _docs/reference/known-issues/KI-names-the-defect-once.md
reject "ki-mechanism-walkthrough" "$root/.hooks/ki-mechanism-walkthrough.sh"
rm _docs/reference/known-issues/KI-names-the-defect-once.md
printf '# Mentioned\n\nretire_when: never\n\nThis is ## How it works in prose.\n' \
  > _docs/reference/known-issues/KI-mentions-the-heading.md
reject "ki-mechanism-walkthrough" "$root/.hooks/ki-mechanism-walkthrough.sh"

# --- ki-report-body ---------------------------------------------------------
#
# The gate keys on a specific upstream reference, so the tracker-only URL is the
# case that separates a filed record from one that names where it would go.
newtree report-body
printf '# Filed\n\nupstream: https://github.com/org/repo/issues/1234\nretire_when: never\n\n## How it works\n\nStep one.\n\n## Report\n\nThe filed body.\n' \
  > _docs/reference/known-issues/KI-filed-with-report.md
printf '# Not filed yet\n\nupstream: https://github.com/org/repo/issues\nretire_when: never\n\n## How it works\n\nStep one.\n' \
  > _docs/reference/known-issues/KI-tracker-only.md
accept "ki-report-body" "$root/.hooks/ki-report-body.sh"
rm _docs/reference/known-issues/KI-filed-with-report.md
printf '# Filed\n\nupstream: https://github.com/org/repo/issues/1234\nretire_when: never\n\n## How it works\n\nStep one.\n' \
  > _docs/reference/known-issues/KI-filed-no-report.md
reject "ki-report-body" "$root/.hooks/ki-report-body.sh"
rm _docs/reference/known-issues/KI-filed-no-report.md
printf '# Filed\n\nupstream: bsc#1234567\nretire_when: never\n\n## How it works\n\nStep one.\n\nSee the ## Report in the tracker.\n' \
  > _docs/reference/known-issues/KI-filed-mentions-report.md
reject "ki-report-body" "$root/.hooks/ki-report-body.sh"

# --- The worked example, exercised against violations ----------------------
#
# The shelf ships `pre-commit-additions.yaml` for a reader to copy, and its
# hooks are separate implementations from the scripts above. `just
# test-spec-shelf` runs them over a conforming tree, which cannot tell a working
# gate from one that passes everything — the direction that matters is the one
# a positive run never takes.
example=$root/programming/spec-driven-docs/worked-example

newexample() {
  t="$work/example-$1"
  rm -rf "$t"
  cp -r "$example/." "$t"
  cd "$t"
  git init -q
  git add -A
}

example_hook() {
  git add -A
  pre-commit run "$1" --all-files --config pre-commit-additions.yaml
}

# The shelf ships the scanner as a file, so the copy a reader takes home is the
# program that was exercised above rather than a paraphrase of it.
cmp -s "$root/.hooks/no-self-narration.awk" "$example/.hooks/no-self-narration.awk" ||
  fail "the worked example's narration scanner has drifted from the one under test"

newexample baseline
accept "worked-example" example_hook adr-filename-shape
accept "worked-example" example_hook ki-filename-shape
accept "worked-example" example_hook spec-requirement-parts
accept "worked-example" example_hook spec-size-cap
accept "worked-example" example_hook no-self-narration

newexample adr-name
cp _docs/decisions/ADR-use-slugs-as-decision-record-ids.md _docs/decisions/ADR-Bad_Name.md
reject "worked-example adr-filename-shape" example_hook adr-filename-shape

newexample ki-name
cp _docs/reference/known-issues/KI-vendor-replays-idempotency-key.md \
  _docs/reference/known-issues/KI-Bad_Name.md
reject "worked-example ki-filename-shape" example_hook ki-filename-shape

newexample lopsided
printf '# T\n\n## Purpose\n\nP.\n\n## Requirements\n\n### `a:b` — X\n\nMUST.\n\nVerify: `pre-commit run x --all-files`\n\nVerify: `pre-commit run x --all-files`\n\n### `c:d` — Y\n\nMUST.\n' \
  > _docs/specs/SPEC-lopsided.md
reject "worked-example spec-requirement-parts" example_hook spec-requirement-parts

newexample unclosed-toc
{
  printf '# Unclosed\n\n<!--TOC-->\n\n## Purpose\n\nP.\n\n## Requirements\n\n'
  i=0
  while [ "$i" -lt 320 ]; do
    printf 'padding line\n'
    i=$((i + 1))
  done
} > _docs/specs/SPEC-unclosed.md
reject "worked-example spec-size-cap" example_hook spec-size-cap

newexample narration
printf '# A page\n\nThis formerly lived in the other shelf.\n' > _docs/reference/narrating.md
reject "worked-example no-self-narration" example_hook no-self-narration

echo "test-gates: every gate failed when it should"
