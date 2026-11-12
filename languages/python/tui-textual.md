# Live terminal dashboards with Textual

How a Python program builds a live dashboard on [Textual](https://github.com/Textualize/textual):
the split between the app, its screens, its widgets, and its stylesheet, the worker that feeds the
data, the themes, and the tests. The worked example is the vitals project
([gubasso/vitals](https://github.com/gubasso/vitals), v0.1.0, Textual 8.2.8), which draws the CPU,
the GPU, the memory, the disks, and a temperature chart. Textual is the choice for a live, refreshing
view; a printed report or a one-shot chart takes the tools of
[13 — Terminal output toolbox](../../programming/cli-design/13-terminal-output-toolbox.md).

## The split

| Piece                 | Holds                                                                               |
| --------------------- | ----------------------------------------------------------------------------------- |
| `App` subclass        | The view registry, the bindings, the sampling timer, and the latest data            |
| One `Screen` per view | The panels of the view, and one method that draws a new snapshot on them            |
| Widgets               | A labeled gauge, a titled panel, a chart: each draws one value, with no system call |
| `.tcss` stylesheet    | Every layout rule and every color, as tokens of the active theme                    |

The UI holds no system call. Typed collectors build an immutable snapshot, and the widgets draw it,
so a test passes a fake snapshot and a failed source shows as a value
([Typing a Python project](./typing.md)).

## Panels, titles, and labels

A panel is a container with `border_title` set, so every panel names its subject on its border. A
gauge composes a label, a `ProgressBar`, and the value with its unit, and sets a CSS class at a
threshold, so the stylesheet colors it. A panel whose source failed adds a class that hides its body
and shows the error kind:

```css
.panel.-failed > .panel-body { display: none; }
.panel.-failed > .panel-error { display: block; }
```

Panels of one grid row share one height, or the shorter one leaves a blank row under it. A label
column takes the width of its longest entry, set from Python on the label styles, so long mount
points stay whole and the bars line up.

## The worker that feeds the data

Sampling runs off the event loop, so a slow source never blocks a key:

```python
@work(thread=True, exclusive=True, group="sampling")
def sample(self) -> None:
    self.post_message(SnapshotReady(collect_snapshot(self._sources)))
```

`on_mount` calls `sample()` once and `set_interval(interval, self.sample)` after. The handler of
`SnapshotReady` keeps the latest snapshot and draws the active screen. A screen that opens later
shows nothing until the next sample, so the action that opens a view draws the latest snapshot on
it at once.

## Views, bindings, and the palette

One registry maps a name and a key to a screen factory, and `on_mount` adds each entry with
`add_mode`. A binding per entry runs an app action that switches the mode and draws, so a new view is
one registry entry. `q` quits, `?` toggles the built-in help panel with `action_show_help_panel` and
`action_hide_help_panel`, and `ctrl+p` opens the command palette, which also switches the theme.

## Themes and colors

Every color in the stylesheet is a token of the theme, such as `$primary`, `$warning`, or `$error`,
so a theme that the palette picks recolors every panel. A widget that takes a color in Python, such
as a chart series, reads `app.theme_variables`. An ANSI theme names its colors `ansi_blue`, which
Rich does not parse: strip the `ansi_` prefix before passing a token to a Rich style, or the theme
`ansi-dark` crashes with `MissingStyle`.

## Charts

[textual-plot](https://github.com/davidfokkema/textual-plot) 0.10.1 is typed end to end and draws
braille lines, round ticks, and a boxed legend. textual-plotext inherits the untyped plotext figure
and fixes its own colors. Set `can_focus = False` on a read-only chart, or the footer lists its zoom
and pan keys. Pad the y limits, because the automatic limits end exactly at the extreme values, and a
series at the lowest value disappears into the axis.

## Tests

- Pilot tests drive the bindings: `async with app.run_test(size=(120, 40)) as pilot`, then
  `pilot.press("2")`, and assert the active screen. Wait for the sampling worker with
  `await app.workers.wait_for_complete()` before asserting a drawn value.
- [pytest-textual-snapshot](https://github.com/Textualize/pytest-textual-snapshot) records each view
  as an SVG at a fixed terminal size, with a fixed fake snapshot and a fixed history, and fails on a
  changed screen. Record one snapshot per view, per failed source, and per terminal size, and update
  them only after a person judged the look.
- An async test needs `pytest-asyncio` with `asyncio_mode = "auto"`.
