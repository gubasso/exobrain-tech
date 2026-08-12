# 13 — Terminal output toolbox

What draws a chart, a board, or a graph in a terminal, what it costs, and where a shell script does
the job without help? This chapter answers it once so a program picks from a catalogue instead of
repeating the survey.

Everything here was measured on 2026-08-11 against nixpkgs unstable, by running the tool rather than
reading its README. Versions are dated evidence, not a package index; the verdicts outlive them.
Revalidate a row before adopting the tool it names, by running it rather than by reading a changelog.

The subset a renderer actually needs — the padding trap, the glyph tiers, the ladder, and the adopted
set — is restated inside
[plan-xp](https://github.com/gubasso/plan-xp/blob/develop/docs/explanation/terminal-rendering.md),
which carries its own copy so it stays self-contained. This chapter is the full survey.

## Method

Three tests decided every row, and the second one is the interesting one.

**Closure size**, via `nix path-info -S` on a `buildEnv` of the candidate set, not on each tool
alone. Individual figures double-count shared dependencies badly: `gum` and `datamash` measure 51 MiB
each, and together they measure 71 MiB, because most of each is glibc.

**Pipe safety**: does the tool emit ANSI when stdout is not a terminal, and does it honour
[`NO_COLOR`](https://no-color.org/)? A tool that colours unconditionally cannot be redirected into a
file, pasted into a fenced code block, or diffed. This is not visible from a version number and it is
not usually documented — it has to be run.

```bash
count_escapes() { grep -c $'\033' || true; }
printf '...' | some-tool | count_escapes        # want 0
printf '...' | NO_COLOR=1 some-tool | count_escapes
```

**Output quality on a real graph**, because layout algorithms differ far more than feature lists
suggest. Two tools that both claim "ASCII diagrams" can be a page apart in readability.

## What a shell already does

Reach for a dependency after this section, not before. Both of these are complete, and both were
checked against the packaged alternative.

### Sparklines

A sparkline is a [dataword](https://www.edwardtufte.com/notebook/sparkline-theory-and-practice-edward-tufte/):
word-sized, frameless, no tick marks, a data-ink ratio of 1.0. Fifteen lines of `awk`:

```awk
BEGIN { split("▁ ▂ ▃ ▄ ▅ ▆ ▇ █", T, " ") }
{ for (i = 1; i <= NF; i++) v[++n] = $i }
END {
  if (n == 0) exit
  min = max = v[1]
  for (i = 1; i <= n; i++) { if (v[i] < min) min = v[i]; if (v[i] > max) max = v[i] }
  range = max - min
  for (i = 1; i <= n; i++) {
    lvl = (range == 0) ? 4 : int((v[i] - min) / range * 7 + 1.5)
    if (lvl < 1) lvl = 1; if (lvl > 8) lvl = 8
    out = out T[lvl]
  }
  print out
}
```

```console
$ printf '4 5 2 7 3 6 1 5 8 4 4 6\n' | awk -f sparkline.awk
▄▅▂▇▃▆▁▅█▄▄▆
```

`python3Packages.sparklines` 0.7.0 produced byte-identical output on the same input. A Python
closure buys nothing here.

One judgement call is embedded above: a flat series renders mid-height. A degenerate range has no
shape, and rendering it empty or full would assert one the data does not support.

### Bars at eighth-cell resolution

[Block Elements](https://www.unicode.org/charts/PDF/U2580.pdf) (U+2580–U+259F) carry left-anchored
eighths `▏▎▍▌▋▊▉` and full `█`. Using them gives eight times the precision of a whole-cell bar, which
is the difference between rendering 3/11 honestly and rounding it into a lie.

```console
001 one-home-for-kb-support-tooling   ███████████████▎········   7/11
002 the-plan-record-answers-questions ███▍····················   3/21
```

### Half-block, for double vertical resolution

`▀` with a foreground colour for the upper half-cell and a background colour for the lower gives two
data rows per text row — a twelve-week stacked area chart in fourteen rows. It works only with
colour, and the consequence is severe enough to be its own rule: strip the escapes and every row is
an identical `▀`. All of the meaning was in the colour. Reserve the technique for panels read as a
continuous field, never for panels whose rows are read one at a time.

### Graphs, as a ladder

A layered drawing is a hard layout problem and a tool's job. A **ladder** is not: one row per node in
topological order, one column per edge currently in flight, corners in
[Box Drawing](https://www.unicode.org/charts/PDF/U2500.pdf) characters. A column opens when a node is
drawn and closes on its last child, so a column is reused the moment it frees, and the width is the
number of edges in flight rather than the number of nodes.

```console
│ │ ●           006  migrate-the-plan-linter
┌─┼─┤
● │ │           008  share-the-record-parser
│ │ ├─┐
│ │ │ ●         015  the-config-file-is-required
```

The same twenty-node graph measured 13 columns as a ladder and 77 as a ranked layout, and the ladder
had room for the names beside the nodes while the layered drawing had to move them to a legend. It
is the shape `git log --graph` uses, and it wins for the same reason: a row per node is a line of
text you can put a label on.

What it gives up is honest — it routes nothing and minimises no crossings, so it says which edges
exist rather than showing their shape, and it cannot nest. About sixty lines of `awk`: rank each
node, allocate a lane per open edge, and pick each junction glyph from four bits of up, down, left,
right.

## Glyph tiers

| Tier           | Range         | Portability                                                                                                                                                          |
| -------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ASCII          | —             | Universal. The fallback every renderer needs.                                                                                                                        |
| Block Elements | U+2580–U+259F | Near-universal, all display-width 1. The default.                                                                                                                    |
| Braille        | U+2800–U+28FF | 2×4 dots per cell, so an 80×24 terminal is a 160×96 dot grid — the only way to a real scatterplot. Weakest font coverage, and screen readers announce it as braille. |

### The padding trap

`awk`'s `printf "%-Ns"` pads by **character count**, not display width, and only in a UTF-8 locale:

```console
UTF-8:      |▇▇▇       |     ← correct: 3 chars, 3 columns
            |日本語       |     ← wrong:   3 chars, 6 columns
LC_ALL=C:   |▇▇▇ |          ← wrong:   pads by bytes
```

So shell-native padding is correct exactly when labels are ASCII and glyphs are Block Elements —
which is the common case — and requires asserting the locale. Anything wider needs a real layout
engine. `column -t` from util-linux is display-width aware and free.

## The catalogue

### Adopted

| Tool                                                     | Version | Draws                                          | Closure | Pipe-safe |
| -------------------------------------------------------- | ------- | ---------------------------------------------- | ------- | --------- |
| [`gum`](https://github.com/charmbracelet/gum)            | 0.17.0  | Column joins of variable-height blocks, tables | 51 MiB  | yes       |
| [`datamash`](https://www.gnu.org/software/datamash/)     | 1.9     | Group-by sums and counts                       | 51 MiB  | yes       |
| [`asciigraph`](https://github.com/guptarohit/asciigraph) | 0.7.3   | Line and trend charts                          | 4 MiB   | yes       |

Combined as one environment: 71 MiB for all three, because most of each is glibc.

`gum join --horizontal` is the specific reason to take a dependency for layout: joining
variable-height blocks side by side without an off-by-one is the bug every hand-rolled board has.

### Documented, install when a panel needs one

| Tool               | Version         | Draws                                                        | Note                                                                                                            |
| ------------------ | --------------- | ------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------- |
| `d2`               | 0.7.1           | Ranked-layer DAGs, nested containers                         | 2154 MiB. Best ASCII layout measured, and the section below is what it does and does not do.                    |
| `graph-easy`       | 0.76            | DAGs                                                         | 105 MiB — the low-closure `d2` substitute. Needs `--as_boxart` and `rankdir=LR`; the defaults are unreadable.   |
| `termgraph`        | 0.7.6           | Stacked bars, calendar heatmaps                              | The only off-the-shelf stacked-bar renderer. **Fails pipe safety**: emits ANSI when piped and under `NO_COLOR`. |
| `chafa`            | 1.18.2          | Images via the kitty graphics protocol or cell approximation | 157 MiB. Verified emitting `\033_Ga=T,f=32,…`.                                                                  |
| `plotille`         | 5.0.0           | Braille scatterplots                                         | A library, not a CLI, so adopting it adds Python. Axis ticks render as `36.6666667`.                            |
| `youplot`          | 0.4.6           | Labeled bars, scatter, density, boxplot                      | 42 MiB of Ruby. Unique value is the statistical plots.                                                          |
| `ansifilter`       | 2.22            | —                                                            | Strips ANSI; the escape hatch for a tool that fails pipe safety.                                                |
| `dateutils`        | 0.4.11          | —                                                            | `datediff` for age and duration arithmetic.                                                                     |
| `column`           | util-linux 2.42 | Aligned tables                                               | Display-width aware, effectively free.                                                                          |
| `termshot` / `aha` | 0.6.1 / 0.5.1   | —                                                            | Terminal output to PNG or HTML, when a panel must be shared as a link.                                          |

#### What `d2` is worth reaching for

Its ASCII mode is narrower than its SVG mode, so the feature list overstates it. Measured against one
real record of twenty nodes:

| Panel                      | `d2`                               | Verdict                                                              |
| -------------------------- | ---------------------------------- | -------------------------------------------------------------------- |
| Dependency DAG             | 77 × 75, ranked layers             | Good, and beaten here by a ladder at 13 columns wide.                |
| The same DAG grouped       | 102 × 93, nested containers        | **Only `d2` can do this.** Containment has no shell-native analogue. |
| Kanban board (grid layout) | 168 columns for six labelled cards | Loses badly. Cards lay out horizontally inside a lane by default.    |
| Any quantitative chart     | —                                  | Not applicable. `d2` draws no bars, lines, or distributions at all.  |

So it is worth its closure for exactly one thing a shell cannot reach: **a graph whose nodes nest**.
Containers, subgraphs, a system diagram with services inside boundaries, a schema with tables inside
databases. Reach for it there and nowhere else, and expect the drawing to exceed 100 columns as soon
as it nests, which is the practical ceiling on the technique rather than on the tool.

```bash
d2 --stdout-format ascii --ascii-mode extended graph.d2 -   # 0.7 or later; --ascii-mode is new
```

Two traps, both measured: it defaults to SVG even with `--ascii-mode`, so `--stdout-format ascii`
is not optional, and it writes a `success:` banner to stdout that a pipeline has to strip.

### Rejected

| Tool                             | Reason                                                                                                                                                                                                                                                    |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `gnuplot`                        | Passes pipe safety, but drags in cairo, pango, gdk-pixbuf, and fontconfig, emits `Fontconfig error` even for `set terminal dumb`, and produces `+---+` frames with `A` overplot markers — worse output than `asciigraph` at a thousand times the closure. |
| `mermaid-cli`                    | 11.12.0 does clear the 11.4 floor for kanban diagrams, but needs Chromium and emits SVG or PNG. `d2` does the job natively.                                                                                                                               |
| `visidata`, `textual`, `wtfutil` | Interactive TUIs. Their payoff is mutating the data; a read-only TUI is a worse pager.                                                                                                                                                                    |
| `plantuml`                       | `-ttxt` works but needs a JVM.                                                                                                                                                                                                                            |
| `python3Packages.sparklines`     | Identical output to fifteen lines of `awk`.                                                                                                                                                                                                               |

### Rasterising diagrams does not work

`graphviz -Tpng` piped through `chafa --format symbols` renders node labels as unreadable grey
smudge — text does not survive downsampling to character cells. In kitty's graphics protocol the
image passes through at full resolution and is crisp, but it is then an image: not selectable, not
greppable, gone when redirected to a file. **Native ASCII wins for anything text-bearing; images are
for continuous data.**

### Naming traps

Two nixpkgs attributes mean something other than what a reader expects, and both would waste an
afternoon:

- **`spark` is Apache Spark**, not the sparkline shell script. The sparkline tool is not packaged.
- **`freeze` is "Payload toolkit for bypassing EDRs"**, not the charmbracelet code-screenshot tool.

Not packaged at all: `sparkline`, `diagon`, `mermaid-ascii`, `xsv`, `asciichartpy`.

## When the terminal is known

A program targeting a specific modern terminal can assume more. In kitty, verified: 24-bit colour via
`COLORTERM=truecolor`, the graphics protocol for inline images, and — the one that matters for
charts — box-drawing and block characters drawn by the terminal itself rather than taken from a font,
so half-block charts have no seams between rows.

None of that removes the need for a fallback tier. Colour vision is not a terminal capability, and a
panel that is pasted or piped has left the terminal entirely.
