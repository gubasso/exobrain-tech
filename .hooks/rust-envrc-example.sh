#!/usr/bin/env bash
# Hold chapter 03 of project-bootstrap and its Rust `.envrc` example to what
# the chapter says (project-platform:a-rust-project-builds-through-the-host-compile-cache).
#
# Two checks run:
#
# 1. Every relative link of the chapter names a file that exists, so the link
#    to the example cannot go stale.
# 2. The example runs in a fixture Rust project. Stub `exobrain` and `kache`
#    commands stand first on PATH, and a stub of the direnv `use` function
#    records its call, so the check needs neither direnv nor kache and runs on
#    a CI runner. The check sources the example with bash and finds
#    `RUSTC_WRAPPER=kache` exported and the calls in the order of the chapter.
#
# The optional argument is the root of the tree to judge. .hooks/test-gates.sh
# passes a throwaway tree to see the check fail.
set -eu

rule=project-platform:a-rust-project-builds-through-the-host-compile-cache
root=${1:-$(cd "$(dirname "$0")/.." && pwd)}
book=$root/programming/project-bootstrap
chapter=$book/03-local-dev-environment.md
example=$book/examples/rust.envrc
status=0

fail() {
  echo "FAIL $rule: $1"
  status=1
}

[ -f "$chapter" ] || fail "missing $chapter"
[ -f "$example" ] || fail "missing $example"
[ "$status" -eq 0 ] || exit 1

# Check 1: the relative link targets of the chapter. A target with a scheme
# leaves the repository, and an anchor alone stays in the chapter.
links=$(grep -oE '\]\([^)[:space:]]+\)' "$chapter" | sed -e 's/^](//' -e 's/)$//') || true
for target in $links; do
  case $target in
    *:* | '#'*) continue ;;
  esac
  path=${target%%#*}
  [ -e "$book/$path" ] || fail "chapter 03 links $target, which does not exist"
done

# Check 2: the example in a fixture project.
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/bin" "$work/project/src"
cp "$example" "$work/project/.envrc"
printf '[package]\nname = "fixture"\nversion = "0.1.0"\nedition = "2021"\n' \
  > "$work/project/Cargo.toml"
printf 'fn main() {}\n' > "$work/project/src/main.rs"

log=$work/calls.log
for tool in exobrain kache; do
  printf '#!/usr/bin/env bash\nprintf "%%s %%s\\n" %s "$*" >> "%s"\n' "$tool" "$log" \
    > "$work/bin/$tool"
  chmod +x "$work/bin/$tool"
done

(
  cd "$work/project"
  PATH="$work/bin:$PATH" LOG="$log" bash -c '
    use() { printf "use %s\n" "$*" >> "$LOG"; }
    unset RUSTC_WRAPPER
    # shellcheck disable=SC1091
    . ./.envrc
    printf "child %s\n" "$(bash -c "printf %s \"\${RUSTC_WRAPPER:-}\"")" >> "$LOG"
    printf "wrapper %s\n" "$(command -v "${RUSTC_WRAPPER:-none}" || echo none)" >> "$LOG"
  '
) || fail "the example exits non-zero in a fixture project"

want=$(printf '%s\n' \
  'use flake' \
  'exobrain outdated' \
  'exobrain store-guard --daily' \
  'child kache' \
  "wrapper $work/bin/kache")
got=$(cat "$log" 2> /dev/null || true)
if [ "$got" != "$want" ]; then
  fail "the example did not run as chapter 03 says"
  printf 'want:\n%s\ngot:\n%s\n' "$want" "$got"
fi

exit "$status"
