# Review record: reliability-report

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 The run's metadata is read from one file, its scores from another, with nothing tying them

```yaml
severity: medium
confidence: high
location: "design.md#Decision: A pure page builder in `codev-evals`, a `docs` subcommand around it"
evidence: |
  reads only the fields the page shows — `thresholds`, `costUsd`, the
recommendation: >-
  `report.json` already records its `run`; reading the date, model and
  versions from `run.json` and the scores from `report.json` lets a
  baseline whose report was scored from another directory print one
  run's scores under another run's date and model. Refuse the pair when
  they disagree, naming both files, and say so in the spec with a
  scenario.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: design.md, the parser refuses a report whose `run` differs from `run.json`; spec scenario "A report of another run"; task 1.1 tests it with `a_report_of_another_run_is_refused`
  - status: fixed
    by: skeptic
    note: verified in the design, the spec and the task
```

### accessibility

#### R-accessibility-1 The status icon of the table is read aloud before the words

```yaml
severity: low
confidence: high
location: "design.md#Decision: Forms — figures, a ranked bar chart, a meter grid, a table"
evidence: |
  column with an icon and words (`✓ Met` or `▲ Failed: false positives`).
recommendation: >-
  A screen reader announces `▲` as "black up-pointing triangle" before
  "Fails", in every row. The words already carry the status (WCAG 1.1.1
  Non-text Content: a decorative glyph is hidden): mark the icon
  `aria-hidden="true"`.
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: design.md, SVG decision, "In the table, the icon is `aria-hidden=\"true\"`"
  - status: fixed
    by: accessibility
    note: verified
```

#### R-accessibility-2 The dark meter track is too close to its fill

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Inline SVG, styled by the theme's tokens, with fallbacks"
evidence: |
  meter tracks (lighter step of the series hue)
recommendation: >-
  The dark track first chosen, `#2e5263`, is 2.8:1 against the dark fill
  `#3a9ec8`: the end of a meter, which is the value, falls under the 3:1
  of WCAG 1.4.11 Non-text Contrast. Step the track down to clear 3:1
  against the fill while staying visible on the surface, and record the
  ratios.
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: design.md, track `#234758` in dark, 3.3:1 against the fill and 1.25:1 against the surface, with the text inks' ratios
  - status: fixed
    by: accessibility
    note: verified with the contrast helper
```

#### R-accessibility-3 A mark's details appear on mouse hover only

```yaml
severity: low
confidence: high
location: "design.md#Decision: Inline SVG, styled by the theme's tokens, with fallbacks"
evidence: |
  positives, over the threshold of 2"), the browser's native tooltip,
recommendation: >-
  A keyboard user cannot bring up a mark's `<title>` (WCAG 2.1.1
  Keyboard). Make each mark focusable, or make sure nothing is reachable
  only that way.
history:
  - status: open
    by: accessibility
  - status: disputed
    by: architect
    note: >-
      Nothing is gated by the tooltip: each bar ends in its value, each
      meter shows its fraction, the `<desc>` lists every value, and the
      table, always present, holds every number. Focusable marks inside
      an `role="img"` would add eighteen tab stops with no accessible
      name of their own.
  - status: rejected
    by: accessibility
    note: accepted; the tooltip enhances, the labels, the description and the table carry every value
```

## code.review

### accessibility

#### R-accessibility-4 Chart labels are small on a phone

```yaml
severity: low
confidence: high
location: "evals/src/reliability.rs:1064"
evidence: |
  let (bx, bw) = (282.0, 60.0);
recommendation: >-
  The charts are drawn in a 400-unit view scaled to the column: on a
  390 px screen, the 12-unit labels render at about 10.5 px. Consider a
  layout that keeps labels at the body size on narrow screens.
history:
  - status: open
    by: accessibility
  - status: disputed
    by: developer
    note: >-
      Every label and value is also in the table, at the body size, and in
      each chart's description; the SVG scales with the browser's zoom
      (WCAG 1.4.4 Resize Text is met), and the screenshots at 390 px show
      no collision. A second, narrow layout would double the drawing code
      for figures the table already carries.
  - status: rejected
    by: accessibility
    note: accepted; the table and the zoom keep every value readable
```

### newcomer

#### R-newcomer-1 A tuple whose fourth field is an unnamed bool

```yaml
severity: low
confidence: high
location: "evals/src/reliability.rs:422"
evidence: |
  for Tile {
recommendation: >-
  The headline figures were built as `(label, value, note, bool)` tuples,
  and the loop destructured them as `(label, value, note, over)`: at the
  definition, nothing said what `false` or `!failing.is_empty()` meant.
  Use a small struct with named fields.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: evals/src/reliability.rs, `struct Tile { label, value, note, flagged }`
  - status: fixed
    by: newcomer
    note: verified
```

#### R-newcomer-2 Chart coordinates without a word on the layout

```yaml
severity: low
confidence: high
location: "evals/src/reliability.rs:892"
evidence: |
  const VIEW_WIDTH: f64 = 400.0;
recommendation: >-
  The charts place labels, plots and status icons at literal x positions
  (100, 282, 340, 210…); a reader changing one cannot tell which others
  depend on it, nor why the gaps are that wide. Describe the columns once,
  next to the shared constants.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: evals/src/reliability.rs, comment above `VIEW_WIDTH` describing each column and why the gaps fit the French words
  - status: fixed
    by: newcomer
    note: verified
```
