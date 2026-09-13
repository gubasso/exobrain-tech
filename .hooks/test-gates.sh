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
#
# The gates the documentation canon delivers are not tested here. `sdd` serves
# them from its own binary and holds each one to its own failing case, so a copy
# of those cases would test a program this repository does not author.
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

# A throwaway tree carrying the layout the two corpus gates read, tracked by
# git so `git grep` sees it.
newtree() {
  t="$work/$1"
  rm -rf "$t"
  mkdir -p "$t/_docs/decisions" "$t/_docs/reference" "$t/.hooks"
  cp "$root/.hooks/unnamed-methods.txt" "$t/.hooks/unnamed-methods.txt"
  cp "$root/.hooks/upstream-canon.txt" "$t/.hooks/upstream-canon.txt"
  printf 'A digest naming the upstream [canon](%s).\n' \
    "$(head -n 1 "$root/.hooks/upstream-canon.txt")" > "$t/AGENTS.md"

  git -C "$t" init -q
  git -C "$t" add -A
  cd "$t"
}

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

md() { markdownlint-cli2 --config "$root/.spec-driven-docs/markdownlint/$1" "$2"; }
accept "md-adr" md adr.markdownlint-cli2.jsonc ADR-ok.md
reject "md-adr" md adr.markdownlint-cli2.jsonc ADR-case.md
reject "md-adr" md adr.markdownlint-cli2.jsonc ADR-sixth.md
accept "md-spec" md spec.markdownlint-cli2.jsonc SPEC-ok.md
reject "md-spec" md spec.markdownlint-cli2.jsonc SPEC-case.md

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

# --- canon-named-once -------------------------------------------------------
#
# The URL is read from the pattern file rather than written out, for the reason
# the gate keeps it in a file at all: spelled here it would itself be a second
# place naming the upstream, and this suite would fail the gate it is testing.
newtree canon-named-once
canon_gate=$root/.hooks/canon-named-once.sh
canon=$(head -n 1 "$root/.hooks/upstream-canon.txt")
accept "canon-named-once" "$canon_gate"

# A second mention, outside the one digest allowed to carry it.
printf 'See %s for the method.\n' "$canon" > _docs/reference/upstream.md
git add -A
reject "canon-named-once" "$canon_gate"
git rm -q --cached _docs/reference/upstream.md
rm _docs/reference/upstream.md

# A record names it and passes: the decision log states its own moment.
printf '# A record\n\nAdopted %s.\n' "$canon" > _docs/decisions/ADR-a-canon-was-adopted.md
git add -A
accept "canon-named-once" "$canon_gate"
git rm -q --cached _docs/decisions/ADR-a-canon-was-adopted.md
rm _docs/decisions/ADR-a-canon-was-adopted.md

# The vendored payload names itself, which is provenance and not a reference.
mkdir -p ".${canon##*/}"
printf '{"canon_source": "%s"}\n' "$canon" > ".${canon##*/}/manifest.json"
git add -A
accept "canon-named-once" "$canon_gate"

# A version written into the link, in the one place allowed to name it.
printf 'A digest naming the upstream [canon](%s/tree/v0.1.5).\n' "$canon" > AGENTS.md
git add -A
reject "canon-named-once" "$canon_gate"

# A digest that names no upstream at all: the pointer has to exist, not merely
# be unique. `at most one` is satisfied by zero, and zero is the state where a
# reader has nowhere to look.
printf 'No upstream is named here.\n' > AGENTS.md
git add -A
reject "canon-named-once" "$canon_gate"

# A second mention inside the one allowed file, alongside the link. One place
# means one statement, not one file free to repeat the fact.
printf 'The upstream [canon](%s) supplies the %s method.\n' \
  "$canon" "${canon##*/}" > AGENTS.md
git add -A
reject "canon-named-once" "$canon_gate"

# --- guide-recipe-shape -----------------------------------------------------
#
# The fenced cases are the quiet ones. A template ships the shape it seeds
# inside a fence, and a scanner reading headings without fence state fails the
# template for demonstrating the form it teaches -- while a page that only
# quotes a recipe would be held to a shape it never claimed. The nested case
# goes further: a template whose steps carry their own fences is wrapped in a
# longer marker, and a toggle blind to marker length reopens at the inner fence
# and reads the rest of the block as prose.
g="$work/guides"
mkdir -p "$g"
gate=$root/.hooks/guide-recipe-shape.sh

recipe() {
  printf '# Task\n\n## Prerequisites\n\n- A checkout.\n\n## Steps\n\n1. Do the thing.\n\n   ```sh\n   run it\n   ```\n\n%b2. Verify.\n\n   ```sh\n   check it\n   ```\n' "$1"
}
recipe '' > "$g/ok.md"
recipe '   - a nested part\n   - another\n\n' > "$g/nested.md"
printf '# A page\n\nProse only, no steps section.\n' > "$g/not-a-guide.md"
accept "guide-recipe-shape" "$gate" "$g/ok.md" "$g/nested.md" "$g/not-a-guide.md"

printf '# Task\n\n## Steps\n\n1. Do the thing.\n' > "$g/no-prereq.md"
reject "guide-recipe-shape" "$gate" "$g/no-prereq.md"
printf '# Task\n\n## Steps\n\n1. Do the thing.\n\n## Prerequisites\n\n- Too late.\n' > "$g/late-prereq.md"
reject "guide-recipe-shape" "$gate" "$g/late-prereq.md"
printf '# Task\n\n## Prerequisites\n\n- A checkout.\n\n## Steps\n\nFirst, some background prose.\n\n1. Do the thing.\n' \
  > "$g/prose-opener.md"
reject "guide-recipe-shape" "$gate" "$g/prose-opener.md"
printf '# Task\n\n## Prerequisites\n\n- A checkout.\n\n## Steps\n\n1. Do the thing.\n\n- a promoted subtask\n' \
  > "$g/flat-bullet.md"
reject "guide-recipe-shape" "$gate" "$g/flat-bullet.md"
printf '# Task\n\n## Prerequisites\n\n- A checkout.\n\n## Steps from scratch\n\n1. Do the thing.\n' \
  > "$g/renamed-section.md"
reject "guide-recipe-shape" "$gate" "$g/renamed-section.md"

# One violation is enough: a gate reporting the first file and exiting leaves
# the rest of the argument list unread, and pre-commit hands it many files.
accept "guide-recipe-shape" "$gate" "$g/ok.md"
reject "guide-recipe-shape" "$gate" "$g/ok.md" "$g/no-prereq.md"

# The shape quoted inside a fence is an illustration, not a claim.
printf '# A page about guides\n\nThe shape:\n\n```markdown\n## Steps\n\nprose, not a step\n```\n' \
  > "$g/quoted.md"
accept "guide-recipe-shape" "$gate" "$g/quoted.md"
printf '# Template\n\n````markdown\n## Prerequisites\n\n- A checkout.\n\n## Steps\n\n1. Do it.\n\n   ```sh\n   run it\n   ```\n\n- a bullet that would fail if the fence reopened\n````\n' \
  > "$g/template.md"
accept "guide-recipe-shape" "$gate" "$g/template.md"

echo "test-gates: every gate failed when it should"
