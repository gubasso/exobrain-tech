#!/usr/bin/env sh
# Every shell script the library ships is linted and formatted.
#
# A bucket may ship a drop-in a reader is expected to copy and run, and a
# drop-in this repository never ran is asserted rather than proven. Two tools
# decide it: `shellcheck` for what the script does, and `shfmt` for how it
# reads.
#
# The set is discovered from the index rather than listed, because the
# filesystem owns what exists and a listed set stops growing the day someone
# adds the sixth script. Reading the index also keeps the gate judging what the
# repository carries rather than what the working tree happens to hold.
#
# The eight content buckets are the product. `_docs/` is metadata, and `.hooks/`
# holds this repository's own gates -- neither ships anything a reader copies,
# so neither is judged here.
#
# The style is not spelled in the flags. `.editorconfig` carries it, `shfmt`
# reads that file when it is called with no formatting flags, and
# editorconfig-checker reads the same block, so one source decides the layout
# both tools see.
#
# The list crosses to each tool NUL-separated through `xargs -0 -r`, so a path
# carrying a space reaches the tool whole and an empty set runs nothing. A
# library that is mostly prose is allowed to ship no script at all.
#
# `--fix` writes the formatting instead of reporting it, and is what `just fmt`
# calls. The two modes share one definition of the set, so the recipe that
# formats and the gate that judges can never disagree about which files they
# mean.
set -u

rule=knowledge-base-boundary:a-shipped-script-is-gated
status=0
fix=

case ${1:-} in
  --fix) fix=1 ;;
  '') ;;
  *)
    echo "usage: $0 [--fix]" >&2
    exit 2
    ;;
esac

# The pathspecs stay quoted so the shell hands them to git untouched. Git reads
# them with its own wildmatch, where `*` crosses a `/`, so one pattern per
# bucket covers the whole subtree.
shipped() {
  git ls-files -z -- \
    'data/*.sh' 'infra/*.sh' 'languages/*.sh' 'platforms/*.sh' \
    'programming/*.sh' 'systems/*.sh' 'tools/*.sh' 'workflows/*.sh'
}

if [ -n "$fix" ]; then
  shipped | xargs -0 -r shfmt -w
  exit $?
fi

# `-S style` is the severity this library's own Bash chapters call the
# non-negotiable floor, so the repository is held to what it publishes.
shipped | xargs -0 -r shellcheck -S style || {
  echo "FAIL $rule: a shipped shell script does not pass shellcheck"
  status=1
}

diff=$(shipped | xargs -0 -r shfmt -d) || {
  echo "FAIL $rule: a shipped shell script is not formatted; run just fmt"
  echo "$diff"
  status=1
}

exit "$status"
