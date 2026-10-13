# Design: A reliability page generated from the baseline

## Context

The evaluation suite (crate `codev-evals`, `evals/`) scores a run into
`report.json` and `report.md` (`report::json`, `report::markdown`), and
the committed baseline `evals/baseline/` holds the 40 observations, the
`run.json` written when the run started, and both reports. The test
`the_baseline_agrees_with_the_scoring` re-scores the observations and
compares the bytes of both reports, so a change to the scoring cannot
land without its baseline.

The current baseline (2026-10-13, Claude Code 2.1.289, default model,
codev 0.6.1, 40 cases, 15.18 USD) reads:

| Role | Seeded/control | Found/expected | False positives | Evidence | Severity |
|---|---|---|---|---|---|
| accessibility | 3/2 | 5/5 | 4 | 1.00 | 5/5 |
| migration | 3/2 | 5/5 | 5 | 1.00 | 4/5 |
| newcomer | 2/1 | 2/2 | 6 | 1.00 | 0/2 |
| ops | 3/2 | 6/6 | 11 | 1.00 | 3/6 |
| performance | 3/2 | 4/4 | 5 | 1.00 | 1/4 |
| privacy | 4/2 | 7/7 | 8 | 1.00 | 4/7 |
| security | 4/2 | 6/6 | 4 | 1.00 | 6/6 |
| skeptic | 2/1 | 3/3 | 9 | 1.00 | 2/3 |
| tracer | 1/1 | 1/1 | 0 | — | 1/1 |

Recall is 1.00 for every role; 52 false positives over 40 cases; eight
roles of nine exceed the threshold of two false positives; evidence
validity is 1.00 wherever a finding was checked.

The documentation is two mdBooks (`docs/en`, `docs/fr`) sharing the
Mairistem theme (`docs/theme/mairistem.css`: tokens under `.light` and
`.navy`, mdBook's class on `<html>`). `codev docs` embeds the chapters
listed in each `SUMMARY.md` (`crates/codev-cli/build.rs`) and renders
them with pulldown-cmark into one standalone page styled by
`crates/codev-cli/assets/docs.css` (light, and dark through
`prefers-color-scheme`). Both renderers pass an HTML block through as it
is, up to the first blank line, and process no markdown inside it.

Decisions that constrain the approach: **0001** (functional core,
imperative shell) — the page is computed by a pure function, the command
only reads and writes; **0002** (the crate graph is the dependency rule)
— the generator lives in `codev-evals`, which no crate depends on, and
the `codev` binary keeps embedding plain markdown files without knowing
where they come from.

## Goals / Non-Goals

**Goals:**

- A reader sees, without running anything, how each role scores, which
  roles fail and why the numbers are limited.
- The page cannot drift from the baseline: it is generated, and a test
  compares the bytes.
- Charts that obey the dataviz method: the right form, validated colors,
  icon and text for status, a table as the accessible twin, light and
  dark selected rather than flipped.

**Non-Goals:**

- Changing the scoring, the cases, the thresholds or the roles
  (precision is the next change's work, against this same baseline).
- History across baselines, or variance across runs: one run, one page.
- A script, a chart library or an image file: the page stays text.

## Decisions

### Decision: A pure page builder in `codev-evals`, a `docs` subcommand around it

New module `evals/src/reliability.rs`:

- `Baseline::parse(report_json, run_json) -> Result<Baseline, String>`
  reads only the fields the page shows — `thresholds`, `costUsd`, the
  length of `cases`, and per role the counts, recall, evidence validity
  and severity agreement — from `report.json`, and the run's metadata
  from `run.json`, and refuses the pair when the `run` recorded in
  `report.json` differs from `run.json` — a report scored from another
  run would put one run's scores under another's date and model. Unknown
  fields are ignored: `report.json` only ever
  gains fields. The format marker of each file goes through the same
  check as `observed.json` (a newer format is refused, naming the file
  and the format).
- `page(&Baseline, Lang) -> String` returns the markdown of a chapter;
  `Lang` is `En` or `Fr`. No I/O, no clock: the same baseline gives the
  same bytes.

`main.rs` gains `docs [--baseline <dir>]` (default `evals/baseline` of
the repository): it reads both files, builds both pages, and writes them
only once both are built, to `docs/en/src/reliability.md` and
`docs/fr/src/reliability.md`. A missing or unreadable file exits with 2
before anything is written.

**Alternatives considered**: a `codev` subcommand (the binary must never
know about the evaluation suite, decision 0002); an mdBook preprocessor
reading the JSON at build time (a second toolchain, and `codev docs`
would need the same code in its `build.rs`); re-scoring the observations
instead of reading `report.json` (the report is already checked against
the scoring by `the_baseline_agrees_with_the_scoring`; reading it keeps
the page a view of the committed report).

### Decision: The committed pages are compared byte for byte

Test `the_reliability_pages_follow_the_baseline` (in
`evals/tests/suite.rs`) parses `evals/baseline/report.json` and
`run.json`, builds both pages and compares them with the committed
files, with a message naming the file and the command to run — the same
mechanism as the report's. A new baseline therefore fails the tests until
`cargo run -p codev-evals -- docs` is run.

**Alternatives considered**: a `--check` flag run by CI (a second gate
to wire; `cargo test` already runs everywhere); generating the pages in
`codev-cli`'s `build.rs` (the binary would read evaluation files).

### Decision: The page's sections and its honest sentences come from the data

Chapter `reliability.md`, title "Reliability of the roles" /
"Fiabilité des rôles", listed after "Roles" under "Using codev". It
opens with an HTML comment saying it is generated and by which command.
Sections, in order: what the bench is; what it is not; the run; at a
glance (the figures, then the verdict); false positives per role (chart);
recall and severity agreement per role (chart); every role (table);
thresholds; limits; run it yourself (commands, link to `evals/baseline`
on GitHub).

The sentences that judge the run are computed, so they stay true:

- the verdict names how many roles fail and on which measure ("8 of 9
  roles report more than 2 false positives"), or says every role meets
  the thresholds;
- when any role fails on false positives, a fixed sentence says that
  precision is the roles' weak point and that reducing false positives
  without losing what they find is the next work on the roles;
- when every role with a seeded case has recall 1.00, a sentence says
  recall does not tell the roles apart on this model, and the
  differences show in false positives and severity agreement.

French follows the documentation's conventions: vouvoiement, a
non-breaking space before `:` `;` `?` `!` in prose, decimal comma
(`1,00`, `15,18 USD`); identifiers, commands and paths stay as they are.

**Alternatives considered**: hand-written prose around generated tables
(the prose would go stale with the next baseline — the opposite of the
goal); generating only English (the books have the same chapters,
`both_languages_have_the_same_chapters`).

### Decision: Forms — figures, a ranked bar chart, a meter grid, a table

Following the dataviz form heuristic, each view does one job:

- **At a glance: four stat tiles** — recall (found/expected over every
  seeded defect), evidence validity (valid/checked), false positives
  (total, and per case), roles within thresholds (`1 of 9`). A handful
  of headline numbers is a KPI row, not a chart; the values use
  proportional figures.
- **False positives per role: horizontal bars**, one series in one hue,
  sorted by count (descending, ties by role id) because the job is to
  compare magnitudes; a dashed vertical reference line at the threshold,
  labeled "Threshold: 2"; each role over it carries, in a fixed column
  after the values, a warning icon and the label "over". Each bar ends
  in its value; the axis has round ticks.
- **Recall and severity agreement: a grid of meters**, two panels side
  by side sharing the role rows (role id order), each 0 to 1: recall
  (found/expected) with the recall threshold drawn, severity agreement
  (agreed/found). Recall is 1.00 for every role today: a recall bar
  chart would be nine identical full bars, the "one number" anti-pattern
  repeated nine times. The headline recall goes to a stat tile, and per
  role, a meter — a ratio against a limit — keeps the value readable
  with its fraction (`6/6`) next to it and puts it beside severity
  agreement, the measure that does vary. Two panels on the same 0–1
  scale are small multiples, not a dual axis. When recall varies, the
  meters show it, and a role under the threshold carries an icon and
  "under". A measure without value (`—`) draws no fill.
- **Every role: the full table**, always present, the accessible twin
  of both charts: role, cases (seeded/control), found/expected, misses,
  false positives, evidence validity, severity agreement, and a status
  column with an icon and words (`✓ Met` or `▲ Failed: false positives`).

**Alternatives considered**: one grouped bar chart of every measure (a
count and three ratios on one axis: a dual scale in disguise); a recall
bar chart (nine full bars carry no information); coloring each bar by
pass/fail (status by color alone, and status reused as a series color).

### Decision: Inline SVG, styled by the theme's tokens, with fallbacks

Each chart is a `<figure>` holding one `<svg role="img"
aria-labelledby="…">` with a `<title>` (the chart's title) and a
`<desc>` (every value, in reading order), followed by a `<figcaption>`.
Every mark sits in a `<g>` with its own `<title>` ("ops: 11 false
positives, over the threshold of 2"), the browser's native tooltip,
and a transparent hit rectangle covering the whole row, larger than the
bar. No script.

The whole figure is written without a blank line so that both renderers
keep it as one HTML block. The SVG has a fixed `viewBox` 400 units wide
and scales to the column (`width: 100%`, `max-width: 520px` — not `rem`,
which mdBook sets to 10 px), so text of 12 units stays between about
10.5 px on a 390 px phone and 15.6 px on a desktop.

Marks follow the specs: bars at most 14 units thick, 4-unit rounded end,
square at the baseline; solid hairline gridlines; text in text tokens,
never in the series color.

Colors are CSS custom properties defined by the theme: in
`mairistem.css` under `.light` (and the no-JS default) and `.navy`, in
`docs.css` under `:root` and `prefers-color-scheme: dark`. Each SVG
element also carries the light value as a presentation attribute, so the
chart stays legible where the stylesheet is absent (a markdown viewer);
the class rule overrides it where the theme is loaded.

| Token | Light (surface `#ffffff`) | Dark (surface `#00394d`) | Use |
|---|---|---|---|
| `--rel-series` | `#076c92` | `#3a9ec8` | bars, meter fills |
| `--rel-track` | `#dbebf3` | `#234758` | meter tracks (lighter step of the series hue) |
| `--rel-over` | `#a46e0b` | `#c48300` | the status icon of a role failing a threshold |
| grid, axis, text | theme's hairline, muted and text inks | the same in dark | chrome |

The series hue is Mairistem's deep blue (`#004c66`, OKLCH hue 230). As
given, `#004c66` fails the validator's lightness band (L 0.39 < 0.43)
and chroma floor (C 0.077 < 0.10) on the light surface: it is snapped to
the nearest passing step of the same hue, `#076c92` (L 0.50, C 0.10,
5.9:1 on white), and stepped for the dark surface to `#3a9ec8` (L 0.66,
C 0.11, 4.1:1 on `#00394d`). The status color is derived from
Mairistem's yellow (`#ffab03`, hue 74) — `#ffab03` itself is outside the
dark band (L 0.80) and at 1.8:1 on white — stepped to `#a46e0b` (light,
4.4:1) and `#c48300` (dark, 3.9:1). Coral (`#ff5331`) is reserved for
actions and not used. Validator output, series and status together as
they share the false-positive chart:

```text
#004c66, light, surface #ffffff (the brand blue as given):
  [FAIL] Lightness band   outside band: [["#004c66",0.39]]
  [FAIL] Chroma floor     below floor (reads gray): [["#004c66",0.077]]
"#076c92,#a46e0b", light, surface #ffffff:
  [PASS] Lightness band · [PASS] Chroma floor
  [PASS] CVD separation   worst adjacent ΔE 18.2 (protan) · tritan 20.9
  [PASS] Normal-vision floor  ΔE 23.1 · [PASS] Contrast vs surface
"#3a9ec8,#c48300", dark, surface #00394d:
  [PASS] Lightness band · [PASS] Chroma floor
  [PASS] CVD separation   worst adjacent ΔE 21.3 (protan) · tritan 24.0
  [PASS] Normal-vision floor  ΔE 24.4 · [PASS] Contrast vs surface
```

Status is never carried by color alone: the icon has a shape (a warning
triangle) and the word "over" or "under" follows it. In the table, the
icon is `aria-hidden="true"`, so a screen reader reads the words only.

The meter's fill against its track is a boundary the reader needs, so it
clears 3:1 (WCAG 1.4.11): 4.8:1 in light; in dark, the track is
`#234758` (OKLCH L 0.38, C 0.05, hue 230), 3.3:1 against the fill and
1.25:1 against the surface, so the empty part stays visible. Text uses
the theme's inks: muted `#5f767e` on white is 4.8:1, `#9cbac4` on
`#00394d` 6.0:1.

**Alternatives considered**: HTML/CSS bars (responsive text for free,
but no per-mark `<title>` and no single image with a description);
an image file (no dark mode, no hover, a binary in the book); a style
block inside the generated page (mdBook's theme class and `codev docs`'s
media query would both have to be guessed from the page; the theme files
already own the tokens).

### Decision: The page is linked from the Roles chapter, the guides and the landing page

Both `roles.md` link to the chapter at the end of "Measuring the roles".
`CONTRIBUTING.md` and `CONTRIBUTING.fr.md` say, under "Changing a role",
that recording the baseline again is followed by
`cargo run -p codev-evals -- docs`, which the tests check. The landing
page's Roles section gains one link, "Measured, not assumed", next to
"Read about roles": no figure is copied there, so it never goes stale.

**Alternatives considered**: a card with the headline figures on the
landing page (hard-coded numbers that the tests would not check).

### Every case, planned

| Case | Behavior |
|---|---|
| No git repository | Irrelevant: the command reads two files and writes two. |
| Offline | Nothing is fetched; the page loads no script, font or image. |
| Windows paths and CRLF | Paths are joined with `Path`; the test normalizes CRLF like the report's test; the generated text uses `\n`. |
| An interrupted run | Both pages are built before either is written; a failure writes nothing. |
| A step re-run (idempotence) | Same baseline, same bytes (test `the_page_is_identical_twice`). |
| Older binaries reading newer files | A `report.json` or `run.json` of a newer format is refused with code 2, naming the file. Fields added to `report.json` later are ignored. |
| A role without seeded case or without checked evidence | Its recall or validity reads `—`, draws no mark and fails no threshold, as in the report. |
| Every role within thresholds | No icon; the verdict says so. |
| A role disabled by configuration, inherited sources, huge diffs, monorepos | Out of scope: the page reads a report, not a project. |
| `codev docs` | Embeds the chapter through `SUMMARY.md`; the SVG passes through pulldown-cmark as an HTML block (test `the_reliability_chapter_renders_its_charts`). |

## Risks / Trade-offs

- [A chart that looks right in one renderer breaks in the other] → the
  figure holds no blank line and no markdown; a `codev-cli` test renders
  the embedded chapter and checks the `<svg` survives; both books are
  built and screenshot at 1440 px and 390 px, light and dark.
- [French typography: a non-breaking space inside generated text could
  land in a command or a path] → non-breaking spaces are written only in
  the fixed French sentences, never around values that are code.
- [The honest sentences go stale] → they are computed from the report:
  the verdict and the recall remark change with the data; the precision
  sentence appears only while a role fails on false positives.
- [Small text on a phone] → the 400-unit viewBox keeps 12-unit labels at
  about 11 px on a 390 px screen; checked on the screenshots.
- [Theme tokens missing where the page is viewed without the theme] →
  presentation attributes carry the light colors.

## Migration Plan

None: new files, plus additive styles and links. Rollback: revert the
commit.

## Open Questions

None.
