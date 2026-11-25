## ADDED Requirements

### Requirement: The documentation exists in English and French with the same structure

The documentation SHALL be an mdBook per language, under `docs/en/` and
`docs/fr/`. Each language's `src/SUMMARY.md` MUST list the same chapter
paths in the same order, and each chapter MUST carry exactly one top-level
`# ` title. The French edition MUST translate the prose; commands, command
output, file paths and configuration keys stay identical to the English
edition.

#### Scenario: Both languages list the same chapters

- **GIVEN** the repository's `docs/en/src/SUMMARY.md` and
  `docs/fr/src/SUMMARY.md`
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails if the two tables of contents do not list the same
  chapter paths in the same order

### Requirement: `codev docs --lang` selects the documentation language

`codev docs` SHALL accept `--lang <en|fr>`, defaulting to `en`, in all three
forms (open, `--write`, `--print`). The selected language's chapters MUST
be embedded in the binary in the order of its `SUMMARY.md` and concatenated
into a single document; links between chapters MUST be rewritten to anchors
within that document, and the HTML MUST declare the language in
`<html lang>`. When opened by default, a non-English page is written to
`codev-docs-<version>-<lang>.html`.

#### Scenario: French documentation printed

- **GIVEN** the current binary
- **WHEN** the user runs `codev docs --print --lang fr`
- **THEN** stdout carries the French documentation, starting with `# codev`
- **AND** no file is written

#### Scenario: Cross-chapter links work on the single page

- **GIVEN** a chapter that links to `concepts.md#delta`
- **WHEN** the user runs `codev docs --write out.html`
- **THEN** the link points to `#delta`
- **AND** an element with the id `delta` exists in `out.html`
