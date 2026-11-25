# Proposal: A reliability page generated from the baseline

## Why

The role evaluation suite measures what each built-in role finds, misses
and invents, but its results live in `evals/baseline/report.md`, a
maintainer's report that a reader of the documentation never sees. Someone
deciding whether to trust a role's findings needs the scores, their
limits and the roles that fail their own thresholds — today eight of nine
roles report more than two false positives — in the documentation, and
kept true each time the baseline is recorded again.

## What Changes

- **A new command, `codev-evals docs`**, writes the chapter
  `reliability.md` of both books (`docs/en/src/`, `docs/fr/src/`) from
  the committed baseline's `report.json` and `run.json`. The pages are
  never written by hand.
- **The chapter**, in English and French: what the bench is and what it
  is not; the run (date, model, Claude Code and codev versions, cases,
  cost); headline figures (recall, evidence validity, false positives,
  roles within thresholds); a chart of false positives per role with the
  threshold drawn and the roles over it marked by an icon and a label; a
  chart of recall and severity agreement per role; the full data table;
  the thresholds and which roles fail them; the limits of the measure;
  how to run it; a link to `evals/baseline` on GitHub. The charts are
  inline SVG, with no script, readable in light and dark themes, on the
  website and in `codev docs`.
- **A test fails when the committed pages differ from what the command
  produces**, so a new baseline cannot be committed without its page.
- The chapter is listed in both `SUMMARY.md` under "Using codev", after
  "Roles"; the Roles chapter links to it; the contributing guides say to
  regenerate it with the baseline; the website's landing page links to it.

## Capabilities

### New Capabilities

### Modified Capabilities

- `role-evaluation`

### Removed Capabilities

## Impact

- `evals/src/` (a pure module that builds the page, the `docs`
  subcommand), `evals/tests/` (the pages follow the baseline, the guides
  name the command).
- `docs/en/src/reliability.md`, `docs/fr/src/reliability.md` (new,
  generated), both `SUMMARY.md`, both `roles.md` (a link).
- `docs/theme/mairistem.css` and `crates/codev-cli/assets/docs.css`: the
  chart and figure styles, in light and dark.
- `docs/site/index.html`: a link to the chapter.
- `CONTRIBUTING.md`, `CONTRIBUTING.fr.md`: regenerate the page with the
  baseline.
- `codev docs` embeds the new chapter through `SUMMARY.md`, unchanged in
  code. The roles, the skills and the scoring are unchanged.

## Domains

- [ ] security — the page is static text and SVG built from a committed JSON file; nothing is fetched or run in the reader's browser
- [ ] personal-data — the report holds role ids, counts and versions only
- [x] user-interface
- [ ] performance — a page of a few kilobytes, built once per baseline
- [ ] operations — no runtime, no log; the generator runs by hand
- [ ] migration — `report.json` is read as it is written today, unknown fields ignored; no format changes

## Release note

- Added: A "Reliability of the roles" chapter shows how the built-in roles
  score on the evaluation suite — what they find, what they invent, and
  which fail their thresholds — generated from the committed baseline, in
  English and French.
