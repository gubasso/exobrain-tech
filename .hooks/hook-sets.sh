#!/usr/bin/env sh
# Hold the reference hook sets of programming/project-bootstrap/hook-sets/ to
# their rules, offline (hook-sets:reference-hook-sets-hold-their-evidence).
#
# 1. prek validates each set as a configuration.
# 2. A repository with a rev appears in one set only, so two sets never expect
#    two revisions of it. No set holds a repository that a release-kit block
#    or an sdd block renders, and no set states the exobrain-tech entry,
#    whose rev selects the sets.
# 3. Each set pins exactly the repositories and revs of its evidence file,
#    and each hook id it takes is one that the evidence lists. The evidence
#    comes from .hooks/hook-sets-evidence.sh, the one step that reads the
#    network.
# 4. Each prek built-in of a set runs with the arguments of the set on a
#    fixture that its upstream hook refuses, and fails. A built-in can judge
#    less than its upstream hook, so this proves that the set arguments keep
#    the upstream behavior.
#
# The hook-sets hook and `just test-hook-sets` run this script.
set -u

# `git commit` exports these into every hook process, and the fixtures below
# run git in throwaway repositories of their own.
unset GIT_DIR GIT_INDEX_FILE GIT_WORK_TREE GIT_OBJECT_DIRECTORY \
  GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_COMMON_DIR GIT_PREFIX

rule=hook-sets:reference-hook-sets-hold-their-evidence
root=$(cd "$(dirname "$0")/.." && pwd)
dir="$root/programming/project-bootstrap/hook-sets"
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
status=0

fail() {
  echo "hook-sets: $1 ($rule)" >&2
  status=1
}

sets=
for f in "$dir"/*.yaml; do
  [ -f "$f" ] || continue
  n=${f##*/}
  sets="$sets ${n%.yaml}"
done
[ -n "$sets" ] || {
  fail "no set file in $dir"
  exit 1
}

# 1. Each set is a valid prek configuration.
for s in $sets; do
  prek validate-config "$dir/$s.yaml" > /dev/null 2>&1 ||
    fail "$s.yaml is not a valid prek configuration"
done

# 2. One repository, one set. No rendered repository, no exobrain-tech entry.
for s in $sets; do
  yq -r '.repos[] | select(has("rev")) | .repo' "$dir/$s.yaml" |
    sed "s|\$| $s|"
done > "$work/repos"
sort -k1,1 "$work/repos" | awk '
  $1 == prev { print $1 " is in " prevset " and " $2 }
  { prev = $1; prevset = $2 }' > "$work/dups"
while read -r line; do
  fail "$line"
done < "$work/dups"
grep -Ei '/(conventional-pre-commit|pre-commit-hooks|markdownlint-cli2|exobrain-tech) ' "$work/repos" > "$work/owned"
while read -r repo s; do
  fail "$s.yaml holds $repo, whose rev another owner states"
done < "$work/owned"

# 3. Each set matches its evidence.
for s in $sets; do
  ev="$dir/evidence/$s.yaml"
  if [ ! -f "$ev" ]; then
    fail "$s.yaml has no evidence file evidence/$s.yaml"
    continue
  fi
  yq -r '.repos[] | select(has("rev")) | .repo + " " + .rev' "$dir/$s.yaml" | sort > "$work/pins"
  yq -r '.repos[] | .repo + " " + .rev' "$ev" | sort > "$work/evpins"
  cmp -s "$work/pins" "$work/evpins" ||
    fail "$s.yaml and evidence/$s.yaml pin different repositories or revs: run .hooks/hook-sets-evidence.sh $s"
  yq -r '.repos[] | select(has("rev")) | .repo as $r | .hooks[] | $r + " " + .id' "$dir/$s.yaml" > "$work/used"
  while read -r repo id; do
    r="$repo" i="$id" yq -e '.repos[] | select(.repo == strenv(r)) | .ids[] | select(. == strenv(i))' "$ev" > /dev/null 2>&1 ||
      fail "$s.yaml takes $id from $repo, which its evidence does not list"
  done < "$work/used"
done

# 4. Each built-in fails on a fixture that its upstream hook refuses.
gitq() { git -C "$repo" -c user.name=hook-sets -c user.email=hook-sets@invalid "$@"; }

# expect_fail <set> <hook id>: the hook, with the set's arguments, fails on
# the fixture repository. It runs on every file, because prek drops a named
# file that does not exist, such as a broken symlink.
expect_fail() {
  s=$1 id=$2
  shift 2
  gitq add -A
  if (cd "$repo" && prek run --config .pre-commit-config.yaml "$id" --all-files) > "$work/out" 2>&1; then
    fail "$s.yaml: built-in $id passed a fixture that its upstream hook refuses"
  fi
}

expect_pass() {
  s=$1 id=$2
  shift 2
  gitq add -A
  (cd "$repo" && prek run --config .pre-commit-config.yaml "$id" --all-files) > "$work/out" 2>&1 ||
    fail "$s.yaml: built-in $id refused a file that it must accept"
}

for s in $sets; do
  yq -r '.repos[] | select(.repo == "builtin") | .hooks[].id' "$dir/$s.yaml" > "$work/builtins"
  while read -r id; do
    repo="$work/fixture-$s-$id"
    rm -rf "$repo"
    mkdir -p "$repo"
    git init -q "$repo"
    # The configuration keeps the set's top-level keys and its built-ins only,
    # so prek clones nothing.
    yq '.repos |= map(select(.repo == "builtin"))' "$dir/$s.yaml" > "$repo/.pre-commit-config.yaml"
    case $id in
      trailing-whitespace)
        printf 'a \n' > "$repo/t.txt"
        expect_fail "$s" "$id"
        ;;
      end-of-file-fixer)
        printf 'a' > "$repo/t.txt"
        expect_fail "$s" "$id"
        ;;
      mixed-line-ending)
        # Only CRLF: the default `auto` keeps it, and the upstream `--fix=lf`
        # refuses it.
        printf 'a\r\nb\r\n' > "$repo/t.txt"
        expect_fail "$s" "$id"
        ;;
      check-added-large-files)
        kb=$(i="$id" yq -r '.repos[] | select(.repo == "builtin") | .hooks[] | select(.id == strenv(i)) | .args // [] | .[]' "$dir/$s.yaml" | sed -n 's/^--maxkb=//p')
        head -c $(((${kb:-500} + 1) * 1024)) /dev/zero > "$repo/big.bin"
        expect_fail "$s" "$id"
        ;;
      check-case-conflict)
        printf 'a\n' > "$repo/a.txt"
        printf 'a\n' > "$repo/A.txt"
        expect_fail "$s" "$id"
        ;;
      check-merge-conflict)
        printf '%s\n' '<<<<<<< HEAD' a '=======' b '>>>>>>> other' > "$repo/t.txt"
        expect_fail "$s" "$id"
        ;;
      check-symlinks)
        ln -s missing "$repo/broken"
        expect_fail "$s" "$id"
        ;;
      destroyed-symlinks)
        printf 'a\n' > "$repo/target.txt"
        ln -s target.txt "$repo/link"
        gitq add -A
        gitq commit -qm "test: add the symlink fixture"
        rm "$repo/link"
        printf 'target.txt' > "$repo/link"
        expect_fail "$s" "$id"
        ;;
      check-executables-have-shebangs)
        printf 'echo a\n' > "$repo/run"
        chmod +x "$repo/run"
        expect_fail "$s" "$id"
        ;;
      check-shebang-scripts-are-executable)
        printf '#!/bin/sh\necho a\n' > "$repo/run"
        chmod -x "$repo/run"
        expect_fail "$s" "$id"
        ;;
      check-yaml)
        printf 'a: !unknown b\n' > "$repo/t.yaml"
        expect_fail "$s" "$id"
        ;;
      check-toml)
        printf 'a =\n' > "$repo/t.toml"
        expect_fail "$s" "$id"
        ;;
      check-xml)
        printf '<a>\n' > "$repo/t.xml"
        expect_fail "$s" "$id"
        ;;
      check-json5)
        # A syntax error in a .json file fails, and a commented .json file
        # passes, as the upstream hook selected json and parsed JSON5.
        printf '{"a": }\n' > "$repo/bad.json"
        expect_fail "$s" "$id"
        rm "$repo/bad.json"
        printf '{\n  // a comment\n  "a": 1,\n}\n' > "$repo/ok.json"
        expect_pass "$s" "$id"
        ;;
      detect-private-key)
        # Split, so this script holds no key header for a secret scan.
        printf '%s%s\n' '-----BEGIN RSA PRIV' 'ATE KEY-----' > "$repo/key.txt"
        expect_fail "$s" "$id"
        ;;
      *)
        fail "$s.yaml: built-in $id has no fixture in .hooks/hook-sets.sh"
        ;;
    esac
  done < "$work/builtins"
done

exit "$status"
