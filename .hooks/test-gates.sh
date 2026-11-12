#!/usr/bin/env sh
# Prove that every gate this repository adds can be made to fail.
#
# A gate that has never been observed failing has not been observed at all, and
# the failure modes worth catching are the quiet ones: a matcher whose exit code
# is inverted names the clean files and passes every breach, and a `grep -L`
# over a key accepts that key with no value. Both report success.
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

# --- shipped-scripts --------------------------------------------------------
#
# The gate discovers its set from the index, so the control needs a tracked
# tree with a bucket in it. The two halves fail for different reasons and are
# exercised apart: a linter finding is a defect in what the script does, and a
# formatting finding is a defect in how it reads. A suite that broke only one
# would leave the other free to be inverted.
#
# The unformatted case is written with the wrong indent rather than the wrong
# flags, because `.editorconfig` is what decides the indent and a gate reading
# it from somewhere else would still pass this file.
shipped=$root/.hooks/shipped-scripts.sh
s="$work/shipped"
rm -rf "$s"
mkdir -p "$s/infra/scripts"
cp "$root/.editorconfig" "$s/.editorconfig"
git -C "$s" init -q
cd "$s"

# An empty set is not a failure. The library is allowed to ship no script.
git add -A
accept "shipped-scripts" "$shipped"

printf '#!/usr/bin/env bash\nset -eu\n\nmain() {\n  echo "$1"\n}\n\nmain "$@"\n' \
  > infra/scripts/ok.sh
git add -A
accept "shipped-scripts" "$shipped"

# SC2086: an unquoted expansion, which splits on the first path with a space.
printf '#!/usr/bin/env bash\nset -eu\n\ncat $1\n' > infra/scripts/unquoted.sh
git add -A
reject "shipped-scripts" "$shipped"
git rm -q --cached infra/scripts/unquoted.sh
rm infra/scripts/unquoted.sh

# Four-space bodies, against the two the shared .editorconfig block declares.
printf '#!/usr/bin/env bash\nset -eu\n\nmain() {\n    echo "$1"\n}\n\nmain "$@"\n' \
  > infra/scripts/wide.sh
git add -A
reject "shipped-scripts" "$shipped"

# --fix settles the formatting half and leaves the tree clean.
"$shipped" --fix > /dev/null 2>&1 || fail "shipped-scripts --fix failed"
accept "shipped-scripts" "$shipped"

# A script outside the eight buckets is this repository's own tooling, not a
# drop-in, and the gate leaves it alone. A pattern that lost its bucket anchor
# would start judging `.hooks/` and pass this check by accident, so the file is
# one the gate would refuse if it saw it.
mkdir -p .hooks
printf '#!/usr/bin/env bash\ncat $1\n' > .hooks/not-shipped.sh
git add -A
accept "shipped-scripts" "$shipped"

# --- no-path-flake-reference ------------------------------------------------
#
# The hook is published in .pre-commit-hooks.yaml and runs here through a local
# twin, so the two entries must stay one string: a twin that drifted would gate
# this repository with an expression no consumer runs. The comparison drops the
# trailing marker comment, which is YAML and not part of the value.
# The same holds for every key that decides what the hook matches.
field_of() {
  awk -v id="$1" -v key="$2" '
    $0 ~ "id: " id "$" { found = 1; next }
    found && /id: / { exit }
    found && $0 ~ "^[[:space:]]*" key ":" {
      sub("^[[:space:]]*" key ":[[:space:]]*", "")
      sub(/[[:space:]]+#.*$/, "")
      print
      exit
    }' "$3"
}
same_as_twin() {
  for key in entry files; do
    published=$(field_of "$1" "$key" "$root/.pre-commit-hooks.yaml")
    twin=$(field_of "$1" "$key" "$root/.pre-commit-config.yaml")
    [ "$published" = "$twin" ] ||
      fail "$1: the published $key and the local twin differ"
  done
  [ -n "$(field_of "$1" entry "$root/.pre-commit-hooks.yaml")" ] ||
    fail "$1 has no published entry"
}
same_as_twin no-path-flake-reference

# The negative control runs the published hook, through `prek try-repo`,
# with no local exclusion. Each must-fail line fails alone, so one hit cannot
# hide a case that stopped matching, and the must-pass file passes whole.
corpus=$root/.hooks/flake-reference-corpus
c="$work/flake-reference"
rm -rf "$c"
mkdir -p "$c"
git -C "$c" init -q
n=0
while IFS= read -r line; do
  n=$((n + 1))
  printf '%s\n' "$line" > "$c/case-$n.txt"
done < "$corpus/must-fail.txt"
cp "$corpus/must-pass.txt" "$c/must-pass.txt"
git -C "$c" add -A
cd "$c"
# A try-repo that cannot load the manifest also exits non-zero, so a bare exit
# code would pass every must-fail case over a broken hook. Each verdict reads
# the result line that prek prints for the hook itself. try-repo reads the
# working tree of $root, uncommitted changes included, through a temporary copy.
try_hook() {
  prek try-repo "$root" "$1" --files "$2" 2>&1
}
i=0
while [ "$i" -lt "$n" ]; do
  i=$((i + 1))
  if out=$(try_hook no-path-flake-reference "case-$i.txt"); then
    fail "no-path-flake-reference accepted must-fail line $i"
  fi
  printf '%s\n' "$out" | grep -q '^- hook id: no-path-flake-reference$' ||
    fail "no-path-flake-reference did not run on must-fail line $i: $out"
done
out=$(try_hook no-path-flake-reference must-pass.txt) ||
  fail "no-path-flake-reference rejected the must-pass file: $out"
printf '%s\n' "$out" | grep -q 'not a path copy\.*Passed$' ||
  fail "no-path-flake-reference did not pass the must-pass file: $out"

# --- no-hook-runner-in-flake -----------------------------------------------
#
# Published and run through a local twin, held equal as above. The fixtures are
# built here rather than kept in a corpus file, so no tracked file names a
# runner in a devshell and a search of the tree for one stays empty.
same_as_twin no-hook-runner-in-flake
runner_hook=no-hook-runner-in-flake
pc=pre-commit
pk=prek
c="$work/hook-runner"
rm -rf "$c"
mkdir -p "$c"
git -C "$c" init -q
# One case per directory, each the only line of its file, so every case fails
# alone. The last case sits under nix/, which the hook reads beside flake.nix.
n=0
must_fail() {
  n=$((n + 1))
  f="$c/fail-$n/${2:-flake.nix}"
  mkdir -p "$(dirname "$f")"
  printf '%s\n' "$1" > "$f"
}
must_fail "          pkgs.$pc              # per-project git hooks"
must_fail "  packages = [ pkgs.$pk ];"
must_fail "    $pc = pkgs.$pc.override { };"
must_fail "  shellHook = \"\${pkgs.$pk}/bin/$pk install\";"
must_fail "            $pc"
must_fail "            $pk # the runner"
must_fail "  packages = with pkgs; [ just $pc ];"
must_fail "  packages = with pkgs; [ $pk ];"
must_fail "  packages = [ pkgs.\"$pc\" ];"
must_fail "  shellHook = \"echo \$HOME \${pkgs.$pc}/bin/$pc\";"
must_fail "  x = \"a\"; packages = [ pkgs.$pk ];"
must_fail "  packages = [ pkgs.\"$pk\" ];" nix/devshell.nix
# A string with an unrelated interpolation does not hide a later reference.
must_fail "  { label = \"\${name}\"; packages = [ pkgs.$pk ]; }"
must_fail "  { label = \"\${name}\"; packages = [ pkgs.\"$pc\" ]; }"
must_fail "  { shellHook = \"echo \${name} \${pkgs.$pk}/bin/$pk\"; }"
must_fail "  { label = \"\${name}\"; packages = [ pkgs.$pc ]; }" nix/devshell.nix
must_fail "  { label = \"\${name}\"; packages = [ pkgs.\"$pk\" ]; }" nix/devshell.nix
must_fail "  { shellHook = \"echo \${name} \${pkgs.$pc}\"; }" nix/devshell.nix
must_fail "  label = \"\${a}\${b}\"; packages = with pkgs; [ $pk ];"
must_fail "  packages = [ pkgs.$pk ];" nix/devshell.nix
# The must-pass flake holds every near miss in one file.
{
  printf '%s\n' "# pkgs.$pc is a host tool, so this devshell lists none."
  printf '%s\n' "  home.packages = [ pkgs.$pk ]; # hook-runner-ok: the host profile installs the runner"
  printf '%s\n' "  packages = [ pkgs.$pc-hook-ensure-sops pkgs.just pkgs.uv ];"
  printf '%s\n' "  packages = with pkgs; [ just uv ]; # not $pk"
  printf '%s\n' "  shellHook = \"$pk install\";"
  printf '%s\n' "  description = \"runs $pc on the host\";"
  # A string literal that only names the runner declares nothing.
  printf '%s\n' "  description = \"pkgs.$pk\";"
  printf '%s\n' "  shellHook = \"echo pkgs.$pc\";"
  printf '%s\n' "  description = \"with pkgs; [ $pk ]\";"
  printf '%s\n' "  description = \"a \\\" pkgs.$pk\";"
  printf '%s\n' "  url = \"github:o/r#pkgs.$pk\";"
  printf '%s\n' "  label = \"\${name}\"; description = \"pkgs.$pk\";"
  printf '%s\n' "  shellHook = \"echo \${name} pkgs.$pc\";"
} > "$c/flake.nix"
# The same near misses under nix/, the other file form the hook reads.
mkdir -p "$c/nix"
cp "$c/flake.nix" "$c/nix/devshell.nix"
# A file that is neither a flake nor under nix/ is not read at all.
mkdir -p "$c/modules"
printf '%s\n' "  home.packages = [ pkgs.$pk ];" > "$c/modules/runner.nix"
git -C "$c" add -A
cd "$c"
i=0
while [ "$i" -lt "$n" ]; do
  i=$((i + 1))
  f=$(git ls-files "fail-$i")
  if out=$(try_hook "$runner_hook" "$f"); then
    fail "$runner_hook accepted must-fail case $i ($f)"
  fi
  printf '%s\n' "$out" | grep -q "^- hook id: $runner_hook\$" ||
    fail "$runner_hook did not run on must-fail case $i: $out"
done
for f in flake.nix nix/devshell.nix; do
  out=$(try_hook "$runner_hook" "$f") ||
    fail "$runner_hook rejected the must-pass file $f: $out"
  printf '%s\n' "$out" | grep -q 'hook runner\.*Passed$' ||
    fail "$runner_hook did not pass the must-pass file $f: $out"
done
out=$(try_hook "$runner_hook" modules/runner.nix) ||
  fail "$runner_hook read a file outside its scope: $out"
printf '%s\n' "$out" | grep -q 'hook runner\.*(no files to check)Skipped$' ||
  fail "$runner_hook did not skip a file outside its scope: $out"

echo "test-gates: every gate failed when it should"
