# 08 — Gates

Every rule this framework states is either checked by a hook or declared unenforced. This chapter
holds the wiring and the honest list of what no command can decide.

## Principle

- A rule that no command can check MUST be listed as unenforced rather than presented as gated.

[03 — Rules](./03-rules.md) requires the `Verify:` line itself; this chapter wires what it names. A
rule presented as binding but never checked teaches readers that specs describe intentions rather than
reality, and after that they stop reading them.

A check whose file set is empty exits zero, so a renamed directory or a drifted `files:` pattern turns
every gate below into a green light over nothing. Assert the set before checking it, here and for
`_docs/decisions/ADR-*.md`.

```bash
set -- _docs/specs/SPEC-*.md
[ -e "$1" ] || { echo 'FAIL no specs matched; the pattern or the layout moved'; exit 1; }
```

## Heading shapes

`MD043 required-headings` holds the fixed heading lists. It takes one `headings` array, so each shape
needs its own config file and hook entry. First remove any mention of `MD043` from the project's
`.markdownlint-cli2.jsonc`, including `"MD043": false`: that file merges over the `--config` base and
would silently disable every shape below while the hooks keep reporting success.

Each config is `{"config": {"MD043": {"headings": [...]}}}` with one array. For a spec that array is
`["?", "## Purpose", "## Requirements", "+"]`; for a record it is `"?"` followed by the five section
headings in order, with no trailing wildcard.

MD043 checks every heading level, so the array must cover the requirement and scenario headings, not
only the `##` sections. Tokens are `?` for exactly one heading, `+` for one or more, `*` for zero or
more; `?` matches the varying title, and `+` fails an empty spec.

```yaml
- id: markdownlint-cli2
  alias: md-spec
  name: markdownlint (spec heading shape)
  files: '^_docs/specs/SPEC-[a-z0-9-]+\.md$'
  args: ['--config', '.markdownlint/spec.markdownlint-cli2.jsonc']

- id: markdownlint-cli2
  alias: md-adr
  name: markdownlint (decision record heading shape)
  files: '^_docs/decisions/(ADR-.*|TEMPLATE-adr)\.md$'
  args: ['--config', '.markdownlint/adr.markdownlint-cli2.jsonc']
```

Reuse the id of the project's existing markdownlint hook so its settings carry over.

## Filenames

Scope the hook to the directory, not to the prefix. A `files:` pattern of `^_docs/decisions/ADR-`
never sees the file someone named `0001-use-postgres.md`, so the one filename the rule exists to
reject is the one filename the gate is blind to.

```yaml
- id: adr-filename-shape
  name: decision records are ADR-<slug>.md with no digit
  language: system
  files: '^_docs/decisions/.*\.md$'
  entry: sh -c 'for f; do case "${f##*/}" in
      TEMPLATE-adr.md) ;;
      ADR-*[0-9]*) echo "FAIL $f: a record filename carries no digit"; exit 1 ;;
      ADR-?*.md) ;;
      *) echo "FAIL $f: a record filename starts with ADR-"; exit 1 ;;
    esac; done' --

- id: ki-filename-shape
  name: known-issue records are KI-<slug>.md with no counter
  language: system
  files: '^_docs/reference/known-issues/.*\.md$'
  entry: sh -c 'for f; do case "${f##*/}" in
      KI-[0-9]*) echo "FAIL $f: a case id is a slug, not a counter"; exit 1 ;;
      KI-?*.md) ;;
      *) echo "FAIL $f: a known-issue filename starts with KI-"; exit 1 ;;
    esac; done' --
```

## Case ids

A suppression names a case, and the name is worth nothing if it resolves to no record. This is the
coverage grep again, over a different pair of sets: every `KI-` token cited anywhere outside the docs
root, against the records that exist.

```bash
rg -o '\bKI-[a-z0-9-]+' --glob '!<root>/**' . | sed 's/.*://' | sort -u > /tmp/cited
ls <root>/reference/known-issues/ | sed 's/\.md$//' | sort -u > /tmp/recorded
comm -13 /tmp/recorded /tmp/cited | grep . && exit 1 || exit 0
```

The check runs one way only. A cited case with no record is a fabrication and fails; a record no
suppression cites is ordinary, because the workaround may live in a config or a dependency pin rather
than in a marked test.

Wire it as `always_run`, not behind a `files:` filter. The commit this check exists to catch is the
one that deletes a record while a suppression still cites it, and pre-commit selects staged files
with `--diff-filter=ACMRTUXB`, which omits deletions. A filter would hand the hook an empty list on
exactly that commit. The same reasoning binds every gate that compares two sets rather than
inspecting the files it is given.

## Companion directories

A companion directory with no spec beside it is an orphan; an empty one is a scaffold nobody filled.

```bash
for d in _docs/specs/*/; do
  n=$(basename "$d")
  [ -f "_docs/specs/$n.md" ] || { echo "FAIL orphan companion: $d"; exit 1; }
  [ -n "$(ls -A "$d")" ]     || { echo "FAIL empty companion: $d"; exit 1; }
done
```

## Requirement parts

Uniqueness across the corpus, then one identifier and one verification per requirement. Because the ID
now lives on the heading, one pattern checks the heading shape and the ID's presence together.

```bash
rg -o --no-filename '^### `([a-z0-9-]+:[a-z0-9-]+)`' -r '$1' _docs/specs \
  | sort | uniq -d | grep . && exit 1

for f in _docs/specs/SPEC-*.md; do
  reqs=$(rg -c '^### `[a-z0-9-]+:[a-z0-9-]+` — .' "$f" || echo 0)
  [ "$reqs" = "$(rg -c '^### ' "$f" || echo 0)" ] \
    && [ "$reqs" = "$(rg -c '^Verify: ' "$f" || echo 0)" ] \
    || { echo "FAIL $f: heading shape or verify count"; exit 1; }
done
```

## Statement grammar

A full EARS parser is not worth building. Check the two properties that catch most breaches: a
requirement statement carries an RFC 2119 keyword, and where it is conditional it opens with one of
the four conditional keywords.

```bash
rg -U '^### Requirement: .*\n\n`[a-z0-9-]+:[a-z0-9-]+`\n\n(.+)' -r '$1' _docs/specs \
  | rg -v '\b(MUST|MUST NOT|SHALL|SHALL NOT|SHOULD|SHOULD NOT|MAY|REQUIRED)\b' \
  | grep . && exit 1 || exit 0
```

Whether the sentence names an actor that can act is a judgment call and stays with the reviewer.

## Prohibition cap

```bash
for f in _docs/specs/SPEC-*.md; do
  n=$(rg -c '\b(MUST NOT|SHALL NOT)\b' "$f" || echo 0)
  [ "$n" -le 5 ] || { echo "FAIL $f: $n prohibitions, cap is 5"; exit 1; }
done
```

## Sizes

```bash
[ "$(wc -l < AGENTS.md)" -le 100 ] || { echo 'FAIL AGENTS.md over 100 lines'; exit 1; }

for f in _docs/specs/SPEC-*.md; do
  n=$(sed '/^<!--TOC-->$/,/^<!--TOC-->$/d' "$f" | wc -l)   # authored lines only
  [ "$n" -le 300 ] || { echo "FAIL $f: $n authored lines, cap is 300"; exit 1; }
  [ "$n" -le 100 ] || rg -q '<!--TOC-->' "$f" || { echo "FAIL $f: over 100 lines, no TOC"; exit 1; }
done

for f in _docs/decisions/ADR-*.md; do
  w=$(wc -w < "$f")
  [ "$w" -le 350 ] || { echo "FAIL $f: $w words, cap is 350"; exit 1; }
done

for f in <shelf>/[0-9][0-9]-*.md; do
  n=$(wc -l < "$f")
  case "$f" in *-gates.md | *-checklist.md) cap=300 ;; *) cap=200 ;; esac
  [ "$n" -le "$cap" ] || { echo "FAIL $f: $n lines, cap is $cap"; exit 1; }
done
```

The chapter loop is what stops a shelf from absorbing a subject by growing. A catalog takes the
larger number for the reason [06 — Format](./06-format.md) states, and is matched by name rather than
by inspecting the file, because no command can tell an argument from an inventory.

## Tables of contents

`md-toc` owns the TOC: generated, never written, and gated rather than trusted. Depth 3 stops at the
requirement headings; the default of 6 adds an entry per scenario, which nobody navigates to.

```yaml
- repo: https://github.com/frnmst/md-toc
  rev: 9.0.0
  hooks:
    - id: md-toc
      args: [-p, -c, --skip-lines, '1', github, -l, '3']
```

The depth flag belongs to the parser subcommand, so it follows `github`; at the top level `-l` means
`--no-links` and `-l 3` is an error. In CI, check instead of write:

```bash
md_toc -d -c -s 1 github -l 3 _docs/specs/SPEC-*.md   # 0 fresh, 128 stale
```

Put `md-toc` in the devshell as well as the hooks. A tool that exists only inside pre-commit cannot be
run against a path the hook misses, or tested at all.

## Fence languages

A closing fence is always bare, so matching every bare fence reports one false failure per correctly
closed block. Track the open state and check the opening fence only.

````bash
awk '/^```/{ if(!inf){ inf=1; if($0=="```"){ printf "%s:%d: bare opening fence\n", FILENAME, FNR; bad=1 } }
             else inf=0; next }
     END{ exit bad }' "$@"
````

## Prose checks

Two rules match on prose, and both must strip fenced blocks and inline code first: a document stating
either rule necessarily quotes the words it forbids, and an unstripped match reports the definition as
a breach.

````bash
strip() { sed '/^```/,/^```/d' "$1" | sed 's/`[^`]*`//g'; }

strip "$f" | grep -niE 'formerly|used to be|this replaces|inherited from' && exit 1   # self-narration
strip "$f" | grep -nE '\*\*[^*]+\*\*' && exit 1                                       # emphasis
````

Give the emphasis check an `<!-- allow-emphasis: <reason> -->` escape hatch, so a genuine exception
costs a sentence in the diff rather than becoming a habit.

## Comment citations

Two checks, and the second is the one that matters. Scope both to the code, excluding the docs root:
a chapter stating either rule necessarily writes the strings it forbids.

```bash
rg -n --glob '!<root>/**' -e '^[[:space:]]*(#|//|--|\*)[[:space:]].*\bADR-' . \
  && { echo 'FAIL a comment names a decision record'; exit 1; } || true

rg -o '(SATISFIES|VERIFIES) ([a-z0-9-]+:[a-z0-9-]+)' -r '$2' --glob '!<root>/**' \
  | sort -u > /tmp/cited
rg -o '^### `([a-z0-9-]+:[a-z0-9-]+)`' -r '$1' <root>/specs | sort -u > /tmp/agreed
comm -13 /tmp/agreed /tmp/cited | grep . \
  && { echo 'FAIL a comment cites a rule no spec defines'; exit 1; } || true
```

The second is the orphan check requirements tracers run: a citation that resolves to nothing is a
fabrication, and catching it is what makes the other three citation sites worth trusting.

## Clarification markers

The cap and the form are one loop. Whether the marker blocks enactment is a set intersection against
the enacted list from [09 — Spec to Code](./09-spec-to-code.md).

```bash
for f in <root>/specs/SPEC-*.md; do
  n=$(rg -c '\[NEEDS CLARIFICATION' "$f" || echo 0)
  [ "$n" -le 3 ] || { echo "FAIL $f: $n markers, cap is 3"; exit 1; }
  rg -q '\[NEEDS CLARIFICATION:?[[:space:]]*\]' "$f" \
    && { echo "FAIL $f: a marker carries no question"; exit 1; }
done

awk '/^### `/{id=$2} /\[NEEDS CLARIFICATION/{print id}' <root>/specs/SPEC-*.md \
  | tr -d '`' | sort -u > /tmp/marked
comm -12 /tmp/marked /tmp/enacted | grep . \
  && { echo 'FAIL work enacts a rule carrying an open question'; exit 1; } || true
```

The `awk` tracks the requirement heading it is inside, so a marker is attributed to the rule above it
rather than to whatever heading happens to fall in a fixed context window.

## Unenforced

These rules are real and no command decides them. A reviewer does.

| Rule                                                     | Why no command                            |
| -------------------------------------------------------- | ----------------------------------------- |
| A requirement names a subject that can act               | requires reading the sentence             |
| A scenario names the contested case, not a restatement   | requires knowing the ambiguity            |
| A deferral's reopening condition is checkable            | requires domain knowledge                 |
| Prose is spent only on a decision, hazard, or constraint | requires judging necessity                |
| One term for one concept                                 | requires knowing which terms are synonyms |
| A fact has exactly one owner                             | requires knowing what the fact is         |
| A spec change is declared as a typed clause              | a command cannot see an omitted clause    |
| A typed clause's type matches the diff                   | requires reading both sides               |
| A step is one action, and an unprinted outcome is a step | requires reading the step                 |
| An artifact token names one artifact, never a step       | requires judging the name                 |
| A comment holds only what the code cannot express        | requires reading the code beside it       |
| An operational document carries every part of its shape  | requires knowing which shape it is        |
| A destructive step shows its dry run and its loss        | requires knowing the tool's forms         |

[99 — Checklist](./99-checklist.md) is where these are asked at review time.
