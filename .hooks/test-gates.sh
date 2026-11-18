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

# A named `.envrc` example is a shell file of the library too. An unquoted
# expansion in one fails the gate. A bare `.envrc` of a template folder carries
# no shell directive, and the gate leaves it alone.
mkdir -p programming/examples tools/templates
printf '# shellcheck shell=bash\nuse flake\n' > tools/templates/.envrc
git add -A
accept "shipped-scripts" "$shipped"
printf '# shellcheck shell=bash\nuse flake\ncat $1\n' > programming/examples/bad.envrc
git add -A
reject "shipped-scripts" "$shipped"
git rm -q --cached programming/examples/bad.envrc
rm programming/examples/bad.envrc

# --- rust-envrc-example -------------------------------------------------------
#
# The gate judges the tree that its argument names. A copy of the chapter and
# the example passes. An example that drops the wrapper line fails, and so does
# a chapter that links a file that does not exist.
rust_envrc=$root/.hooks/rust-envrc-example.sh
r="$work/rust-envrc"
rm -rf "$r"
mkdir -p "$r/programming/project-bootstrap/examples"
cp "$root/programming/project-bootstrap/03-local-dev-environment.md" \
  "$r/programming/project-bootstrap/"
cp "$root/programming/project-bootstrap/examples/rust.envrc" \
  "$r/programming/project-bootstrap/examples/"
# Each other relative link of the chapter gets an empty file, so a new link in
# the chapter needs no edit here.
grep -oE '\]\([^)[:space:]]+\)' "$root/programming/project-bootstrap/03-local-dev-environment.md" \
  | sed -e 's/^](//' -e 's/)$//' -e 's/#.*//' | grep -v -e ':' -e '^$' \
  | while read -r target; do
    f="$r/programming/project-bootstrap/$target"
    [ -e "$f" ] && continue
    mkdir -p "$(dirname "$f")"
    : > "$f"
  done
accept "rust-envrc-example" "$rust_envrc" "$r"
grep -v RUSTC_WRAPPER "$root/programming/project-bootstrap/examples/rust.envrc" \
  > "$r/programming/project-bootstrap/examples/rust.envrc"
reject "rust-envrc-example" "$rust_envrc" "$r"
cp "$root/programming/project-bootstrap/examples/rust.envrc" \
  "$r/programming/project-bootstrap/examples/"
rm "$r/programming/project-bootstrap/04-quality-gates.md"
reject "rust-envrc-example" "$rust_envrc" "$r"

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

# --- host-requirements ---------------------------------------------------------
#
# Published and run through a local twin. The twin runs the exobrain of this
# checkout through cargo, so its entry equals the published one once the cargo
# prefix becomes `exobrain`. The case builds the binary and puts it first on
# PATH for the published hook. Each malformed file sits in its own tree, so each
# one fails alone, and the example of the convention passes.
req_hook=host-requirements
published=$(field_of "$req_hook" files "$root/.pre-commit-hooks.yaml")
twin=$(field_of "$req_hook" files "$root/.pre-commit-config.yaml")
[ "$published" = "$twin" ] || fail "$req_hook: the published files and the local twin differ"
published=$(field_of "$req_hook" entry "$root/.pre-commit-hooks.yaml")
twin=$(field_of "$req_hook" entry "$root/.pre-commit-config.yaml")
[ -n "$published" ] || fail "$req_hook has no published entry"
[ "exobrain ${twin#cargo run --quiet -p exobrain -- }" = "$published" ] ||
  fail "$req_hook: the local twin does not run the published entry through cargo"
(cd "$root" && cargo build -q -p exobrain) || fail "$req_hook: cargo build failed"
PATH="$root/target/debug:$PATH"
export PATH
n=0
must_fail_req() {
  n=$((n + 1))
  d="$work/host-requirements-fail-$n"
  mkdir -p "$d"
  git -C "$d" init -q
  printf '%b' "$1" > "$d/host-requirements.toml"
  git -C "$d" add -A
}
must_fail_req 'schema = 1\n[tools.gh]\nversion = "2.57.0"\n'
must_fail_req 'schema = 1\n[tools.gh]\nversion = "2.*"\n'
must_fail_req 'schema = 1\n[tools.gh]\nversion = ">=banana"\n'
must_fail_req 'schema = 1\n[tools.gh]\nflavor = "x"\n'
must_fail_req 'schema = 1\nscope = "global"\n'
must_fail_req 'schema = 2\n'
must_fail_req '[tools.git]\n'
i=0
while [ "$i" -lt "$n" ]; do
  i=$((i + 1))
  cd "$work/host-requirements-fail-$i"
  if out=$(try_hook "$req_hook" host-requirements.toml); then
    fail "$req_hook accepted must-fail file $i"
  fi
  printf '%s\n' "$out" | grep -q "^- hook id: $req_hook\$" ||
    fail "$req_hook did not run on must-fail file $i: $out"
done
d="$work/host-requirements-pass"
mkdir -p "$d"
git -C "$d" init -q
printf '%s\n' 'schema = 1' '' '[tools.git]' 'version = ">=2.36.0"' '' \
  '[tools.gh]' 'version = ">=2.57.0"' '' '[tools.prek]' 'version = "*"' \
  > "$d/host-requirements.toml"
printf '%s\n' 'not a requirements file' > "$d/other.toml"
git -C "$d" add -A
cd "$d"
out=$(try_hook "$req_hook" host-requirements.toml) ||
  fail "$req_hook rejected the example: $out"
printf '%s\n' "$out" | grep -q 'parses\.*Passed$' ||
  fail "$req_hook did not pass the example: $out"
out=$(try_hook "$req_hook" other.toml) ||
  fail "$req_hook read a file outside its scope: $out"
printf '%s\n' "$out" | grep -q 'parses\.*(no files to check)Skipped$' ||
  fail "$req_hook did not skip a file outside its scope: $out"

# --- hook-revisions -----------------------------------------------------------
#
# Published and run through a local twin, held equal past the cargo prefix as
# host-requirements is, and the published hook runs the binary that the case
# above put first on PATH. `prek try-repo` takes no hook `args`, and the hook
# needs the `--set` arguments of a project, so each case runs `prek run` on a
# config that pins a throwaway hook repository, with the entry and arguments
# that a project uses. That repository holds the published manifest and the
# sets at two tags: v1.0.0 as this tree holds them, and v1.1.0 with typos
# moved. The project config that the hook reads names the exobrain-tech entry
# and typos, as a project config does. prek keeps its clones under a PREK_HOME
# of this run, so no throwaway tag reaches the store of the host.
rev_hook=hook-revisions
for key in entry files; do
  published=$(field_of "$rev_hook" "$key" "$root/.pre-commit-hooks.yaml")
  twin=$(field_of "$rev_hook" "$key" "$root/.pre-commit-config.yaml")
  [ -n "$published" ] || fail "$rev_hook has no published $key"
  [ "$key" = files ] || twin="exobrain ${twin#cargo run --quiet -p exobrain -- }"
  [ "$published" = "$twin" ] ||
    fail "$rev_hook: the published $key and the local twin differ"
done
rgit() {
  git -c core.hooksPath=/dev/null -c commit.gpgSign=false -c tag.gpgSign=false \
    -c user.name=test-gates -c user.email=test-gates@example.invalid "$@"
}
hr="$work/hook-revisions-repo"
hr_common="$hr/programming/project-bootstrap/hook-sets/common.yaml"
mkdir -p "$hr/programming/project-bootstrap"
cp "$root/.pre-commit-hooks.yaml" "$hr/.pre-commit-hooks.yaml"
cp -R "$root/programming/project-bootstrap/hook-sets" "$hr/programming/project-bootstrap/hook-sets"
rgit -C "$hr" init -q
rgit -C "$hr" add -A
rgit -C "$hr" commit -qm "test: hold the sets"
rgit -C "$hr" tag v1.0.0
typos_rev=$(yq '.repos[] | select(.repo == "https://github.com/crate-ci/typos") | .rev' "$hr_common")
[ -n "$typos_rev" ] || fail "$rev_hook: the common set pins no typos"
yq -i '(.repos[] | select(.repo == "https://github.com/crate-ci/typos")).rev = "v9.0.0"' "$hr_common"
rgit -C "$hr" commit -qam "test: move typos"
rgit -C "$hr" tag v1.1.0
# rev_case <name> <tag> <typos rev> builds a project at the tag that pins typos
# at the rev, and runs the published hook on its config.
rev_case() {
  p="$work/hook-revisions-$1"
  mkdir -p "$p"
  rgit -C "$p" init -q
  cat > "$p/.pre-commit-config.yaml" << EOF
repos:
  - repo: https://github.com/gubasso/exobrain-tech
    rev: $2
    hooks:
      - id: hook-revisions
        args: [--set, common, --set, rust]
  - repo: https://github.com/crate-ci/typos
    rev: $3
    hooks:
      - id: typos
        args: [--write-changes]
EOF
  cat > "$p/run.yaml" << EOF
repos:
  - repo: $hr
    rev: $2
    hooks:
      - id: hook-revisions
        args: [--set, common, --set, rust]
EOF
  rgit -C "$p" add -A
  cd "$p"
  PREK_HOME="$work/prek-home" prek run -c run.yaml "$rev_hook" --files .pre-commit-config.yaml 2>&1
}
out=$(rev_case match v1.0.0 "$typos_rev") ||
  fail "$rev_hook rejected a pin that matches its set: $out"
printf '%s\n' "$out" | grep -q 'exobrain-tech tag\.*Passed$' ||
  fail "$rev_hook did not pass a pin that matches its set: $out"
# The rev of the newer tag, at the older tag: the tag of the entry selects the sets.
if out=$(rev_case drift v1.0.0 v9.0.0); then
  fail "$rev_hook accepted a drifted pin: $out"
fi
printf '%s\n' "$out" | grep -q "^- hook id: $rev_hook\$" ||
  fail "$rev_hook did not run on a drifted pin: $out"
printf '%s\n' "$out" | grep -qF 'https://github.com/crate-ci/typos' ||
  fail "$rev_hook did not name the drifted repository: $out"
out=$(rev_case newer v1.1.0 v9.0.0) ||
  fail "$rev_hook rejected a project at a newer tag: $out"
printf '%s\n' "$out" | grep -q 'exobrain-tech tag\.*Passed$' ||
  fail "$rev_hook did not pass a project at a newer tag: $out"

# --- hook-sets --------------------------------------------------------------
#
# The gate reads the set folder beside its own script, so each case copies the
# script and the sets into a throwaway tree and breaks one rule there. A case
# that breaks a set rule also updates the evidence where it must, so the case
# fails on the rule it names and not on a stale evidence file.
hs_tree() {
  t="$work/hook-sets-$1"
  rm -rf "$t"
  mkdir -p "$t/.hooks" "$t/programming/project-bootstrap"
  cp "$root/.hooks/hook-sets.sh" "$t/.hooks/hook-sets.sh"
  cp -R "$root/programming/project-bootstrap/hook-sets" "$t/programming/project-bootstrap/hook-sets"
  hs="$t/programming/project-bootstrap/hook-sets"
}
hs_tree pass
accept "hook-sets" "$t/.hooks/hook-sets.sh"

# A built-in that loses the argument that keeps its upstream behavior.
hs_tree unknown-tags
yq -i '(.repos[] | select(.repo == "builtin") | .hooks[] | select(.id == "check-yaml")) |= del(.args)' "$hs/common.yaml"
reject "hook-sets (a built-in judges less than its upstream hook)" "$t/.hooks/hook-sets.sh"

# One repository in two sets.
hs_tree two-sets
yq -i '.repos += [{"repo": "https://github.com/crate-ci/typos", "rev": "v1.0.0", "hooks": [{"id": "typos"}]}]' "$hs/rust.yaml"
yq -i '.repos += [{"repo": "https://github.com/crate-ci/typos", "rev": "v1.0.0", "ids": ["typos"]}]' "$hs/evidence/rust.yaml"
reject "hook-sets (one repository in two sets)" "$t/.hooks/hook-sets.sh"

# A repository that a rendered block owns.
hs_tree rendered
yq -i '.repos += [{"repo": "https://github.com/DavidAnson/markdownlint-cli2", "rev": "v0.24.0", "hooks": [{"id": "markdownlint-cli2"}]}]' "$hs/nix.yaml"
yq -i '.repos += [{"repo": "https://github.com/DavidAnson/markdownlint-cli2", "rev": "v0.24.0", "ids": ["markdownlint-cli2"]}]' "$hs/evidence/nix.yaml"
reject "hook-sets (a set holds a rendered repository)" "$t/.hooks/hook-sets.sh"

# A rev that moved without its evidence.
hs_tree stale-rev
yq -i '(.repos[] | select(.repo == "https://github.com/astro/deadnix")).rev = "v0.0.1"' "$hs/nix.yaml"
reject "hook-sets (a rev without its evidence)" "$t/.hooks/hook-sets.sh"

# A hook id that the manifest does not publish.
hs_tree unknown-id
yq -i '(.repos[] | select(.repo == "https://github.com/astro/deadnix") | .hooks[0].id) = "deadnix-gone"' "$hs/nix.yaml"
reject "hook-sets (a hook id the manifest does not publish)" "$t/.hooks/hook-sets.sh"

# --- skill-sources ----------------------------------------------------------
#
# The gate takes the root to check as its argument, so each case builds a
# throwaway tree with one skill that names one source.
ss_tree() {
  t="$work/skill-sources-$1"
  rm -rf "$t"
  mkdir -p "$t/crates/exobrain/skills/demo" "$t/programming/project-bootstrap"
  printf '# 04\n' > "$t/programming/project-bootstrap/04-quality-gates.md"
}
ss_tree pass
printf 'Read `$EXOBRAIN_TECH/programming/project-bootstrap/04-quality-gates.md`.\n' \
  > "$t/crates/exobrain/skills/demo/SKILL.md"
accept "skill-sources" "$root/.hooks/skill-sources.sh" "$t"

# A skill names a chapter that the tree no longer holds.
ss_tree missing
printf 'Read `$EXOBRAIN_TECH/programming/project-bootstrap/04-gates.md`.\n' \
  > "$t/crates/exobrain/skills/demo/SKILL.md"
reject "skill-sources (a skill names a missing source)" "$root/.hooks/skill-sources.sh" "$t"

# No skill names a source, so the gate has nothing to check.
ss_tree none
printf 'Read the chapters.\n' > "$t/crates/exobrain/skills/demo/SKILL.md"
reject "skill-sources (no skill names a source)" "$root/.hooks/skill-sources.sh" "$t"

echo "test-gates: every gate failed when it should"
