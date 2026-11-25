# Docs Specification

## Purpose

Deliver complete Compagnon documentation in a **single, shareable**
form: a self-contained HTML file (embedded CSS, no external assets)
that the `compagnon` binary can generate and open by itself, to
complement the CLI help and serve as a point of sharing with third
parties.

## Requirements

### Requirement: `compagnon docs` generates a self-contained HTML file and opens it in the browser

The `compagnon` binary SHALL expose a `docs` subcommand which, without
arguments, generates a self-contained HTML file and opens it in the
default system browser.

The HTML MUST:

- carry all of its CSS **inline** in a `<style>` element in the
  head — no reference to a CDN, no external image, no required
  runtime script;
- correctly render the six common markdown elements — headings,
  paragraphs, lists, tables, code blocks, blockquotes;
- carry the name `compagnon` and the version of the binary that
  generated it at the top;
- open in any recent browser, including **offline**.

The default output path MUST be in the system temporary directory
(`std::env::temp_dir()`) and carry the version in its name:
`compagnon-docs-<version>.html`.

#### Scenario: `compagnon docs` without arguments opens the HTML

- **GIVEN** a `compagnon` binary at version 0.1.0
- **WHEN** the user runs `compagnon docs`
- **THEN** a file `compagnon-docs-0.1.0.html` is written to
  `std::env::temp_dir()`
- **AND** a system call opens that file in the default browser
  (macOS `open`, Linux `xdg-open`, Windows `start`)
- **AND** the exit code is 0
- **AND** the HTML carries the name `compagnon` and the version `0.1.0` at the top

#### Scenario: The HTML is self-contained

- **GIVEN** the file generated in the previous step
- **WHEN** it is opened in a browser without an internet connection
- **THEN** the page renders correctly (inline CSS, no external
  request)
- **AND** no `<link>` element points to an `http(s):` URL

### Requirement: `compagnon docs --print` prints the markdown source to stdout

`compagnon docs --print` MUST print the **markdown source** of the
documentation to stdout, without conversion. The output is useful
for piping into `less`, `bat`, or a tool that consumes markdown
(an LLM, for example).

This form MUST NOT open a browser, MUST NOT write any file, and
MUST return exit code 0.

#### Scenario: `--print` outputs non-empty markdown

- **GIVEN** the current binary
- **WHEN** the user runs `compagnon docs --print`
- **THEN** stdout carries non-empty content (> 1000 bytes — the docs
  have several sections)
- **AND** the content starts with a level-1 markdown heading
  (`# compagnon`)
- **AND** no file is written

### Requirement: `compagnon docs --write <PATH>` writes without opening

`compagnon docs --write <PATH>` MUST write the self-contained HTML to
the given path and **open nothing**. This is the "targeted sharing"
mode — placing the docs in a shared drive, a Confluence folder, a
repository.

The path is used as is: the command MUST create missing parent
directories **only** if the given path has a parent that already
exists (the command does not invent a directory tree — it rejects
a path with several missing levels).

#### Scenario: `--write` produces the file at the given path

- **GIVEN** the current binary
- **AND** a working directory where `./out/` exists
- **WHEN** the user runs `compagnon docs --write ./out/manual.html`
- **THEN** the file `./out/manual.html` exists and carries the
  self-contained HTML
- **AND** no browser is opened
- **AND** the exit code is 0

### Requirement: `compagnon docs` does not depend on an initialized repository

`compagnon docs` MUST NOT read `_compagnon/config.yaml` or any other
project state. The command works from any directory, even without
a Compagnon repository, even without `.git`.

#### Scenario: Works outside a Compagnon repository

- **GIVEN** a user in a directory that has no `_compagnon/`
- **WHEN** they run `compagnon docs --print`
- **THEN** the markdown is printed normally
- **AND** no error message mentions `_compagnon`

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
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if the two tables of contents do not list the same
  chapter paths in the same order

### Requirement: `compagnon docs --lang` selects the documentation language

`compagnon docs` SHALL accept `--lang <en|fr>`, defaulting to `en`, in all three
forms (open, `--write`, `--print`). The selected language's chapters MUST
be embedded in the binary in the order of its `SUMMARY.md` and concatenated
into a single document; links between chapters MUST be rewritten to anchors
within that document, and the HTML MUST declare the language in
`<html lang>`. When opened by default, a non-English page is written to
`compagnon-docs-<version>-<lang>.html`.

#### Scenario: French documentation printed

- **GIVEN** the current binary
- **WHEN** the user runs `compagnon docs --print --lang fr`
- **THEN** stdout carries the French documentation, starting with `# compagnon`
- **AND** no file is written

#### Scenario: Cross-chapter links work on the single page

- **GIVEN** a chapter that links to `concepts.md#delta`
- **WHEN** the user runs `compagnon docs --write out.html`
- **THEN** the link points to `#delta`
- **AND** an element with the id `delta` exists in `out.html`

### Requirement: Warning callouts stand out from notes in the offline manual

In the HTML that `compagnon docs` produces, a blockquote that opens on a bold
warning title — `**Warning**` in English, `**Attention**` in French — MUST
be rendered as a warning callout, visually distinct from a note (a coral
rule instead of the green one). Other blockquotes keep the note style.

#### Scenario: Warning and note callouts differ

- **GIVEN** a chapter containing `> **Warning**` and `> **Note**` callouts
- **WHEN** the user runs `compagnon docs --write out.html`
- **THEN** the warning blockquote carries a distinct style class
- **AND** the note blockquote does not

### Requirement: `compagnon docs --print` stops silently when its reader exits

When the output of `compagnon docs --print` is closed before everything is
written (for example `compagnon docs --print | head`), the command MUST stop
without printing an error and exit with code 0. Any other write error is
reported and exits with a non-zero code.

#### Scenario: Piping into `head`

- **WHEN** the user runs `compagnon docs --print | head -1`
- **THEN** the first line of the manual is printed
- **AND** nothing is printed on stderr

### Requirement: The Roles chapter lists exactly the catalog's roles

The Roles chapter of each language SHALL hold a table of the built-in
roles with one row per role of the catalog — the built-in step roles, the
experts and the artifact roles of the default `spec-driven` schema —
giving its id, its display name in that language, and its step or steps.
The table MUST NOT list any other role. The chapter SHALL also describe
how to configure roles: the short form, extra criteria, the per-change
override, and which roles cannot be disabled; and the experts: the domain
of each, the `## Domains` section that activates them, the `on_domain`
activation, and the sections of a role's knowledge.

#### Scenario: The catalog and the documentation agree

- **GIVEN** the role catalog and the `roles.md` chapters of `docs/en` and
  `docs/fr`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if a role of the catalog is missing from either table,
  or listed without its display name in that language

#### Scenario: A role that does not exist

- **GIVEN** a `roles.md` whose table of built-in roles lists an id the
  catalog does not hold
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails and names that id

#### Scenario: Every expert is documented with its domain

- **GIVEN** the role catalog and the `roles.md` chapters of `docs/en` and
  `docs/fr`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if an expert's row does not name its domain

### Requirement: The review record is documented in both languages

The documentation of each language SHALL hold a "Review record" guide —
the file `guides/review-record.md`, listed in its table of contents — that
describes the record's layout, a finding's fields, the statuses and who
may set them, the answer loop, the checks, the archive gate and what to do
for a change started with an earlier Compagnon. The Roles chapter of each
language SHALL give the severity and confidence scales; the English
chapter MUST give each level's definition exactly as Compagnon's skills
receive it.

#### Scenario: The guide exists in both languages

- **GIVEN** the repository's `docs/en` and `docs/fr`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if either table of contents does not list
  `guides/review-record.md`

#### Scenario: The scale in the docs is the scale in the skills

- **GIVEN** the English `roles.md` and the severity and confidence scales
  the skills receive
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if a level's definition is missing from the chapter,
  or if a level is missing from the French chapter

### Requirement: The Roles chapter documents triggers, suggestions and the stack filter

The Roles chapter of each language SHALL describe the three kinds of
trigger (`paths`, `manifests`, `patterns`), the built-in triggers of
each expert in outline, the change's diff and what it leaves out, the
states of a suggestion and how to justify an unticked domain, the
warning `domain_suggested`, the project's `triggers:` key, and the stack
filter with `--all-stacks`. The configuration reference of each language
SHALL document the `triggers:` key.

#### Scenario: Both languages document them

- **GIVEN** the `roles.md` and `reference/configuration.md` chapters of
  `docs/en` and `docs/fr`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if a `roles.md` does not name `paths`, `manifests`,
  `patterns`, `domain_suggested` and `--all-stacks`, or if a
  `configuration.md` has no `triggers` section

#### Scenario: A chapter that forgets the flag

- **GIVEN** a French `roles.md` that does not name `--all-stacks`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails and names the French chapter

### Requirement: Isolated review is documented in both languages

The Roles chapter of each language SHALL explain the two review modes,
what a reviewer receives in isolated mode, the fallback to inline, and
the meeting — how findings are merged, credited and ranked. The
configuration reference of each language SHALL document `review.mode`,
its values, its default and the per-change override, how to switch
modes, and that a project using the key raises `compagnon_version` to the
release that reads it. The skills chapter of each language SHALL say which skills
request the sub-agent tool and when. No error case applies: the
requirement is on the repository's documentation, checked by its tests.

#### Scenario: Both languages document the review modes

- **GIVEN** the repository's `docs/en` and `docs/fr`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if the Roles chapter or the configuration reference
  of either language does not name `review.mode`, `inline` and
  `isolated`, if the configuration reference's `review` section does not
  name `compagnon_version`, or if the skills chapter of either language does
  not name the `Agent` tool

### Requirement: Compagnon's mark identifies the product in every documentation surface

The repository SHALL hold the Compagnon mark as SVG files under
`docs/assets/`: a color version for light backgrounds and a version for
dark backgrounds, drawn only in the Mairistem palette (`#ff5331`,
`#ffab03`, `#004c66`, `#00c39b`, plus black or white). The mark MUST be
shown:

- at the top of `README.md` and `README.fr.md`, with the variant matching
  the reader's color scheme;
- in the header of the mdBook documentation, in both languages, next to
  the Mairistem logo;
- in the header of the landing page `docs/site/index.html`, next to the
  Mairistem logo;
- as the favicon of the documentation and of the landing page;
- in the self-contained HTML file written by `compagnon docs`, inline, with
  no external asset.

A mark that stands alone carries a text alternative naming Compagnon: `alt`
on an image, `role="img"` with `aria-label` on inline SVG — a `<title>`
inside the SVG file is not enough. A mark placed next to the visible word
"Compagnon", or inside a link that has its own accessible name, is
decorative (`aria-hidden="true"`). The brand links name Compagnon
("Compagnon, by Mairistem — home"; "Accueil du site Compagnon" in French).

#### Scenario: The offline manual shows the mark without fetching anything

- **WHEN** the user runs `compagnon docs --write manual.html`
- **THEN** `manual.html` holds the Compagnon mark as inline SVG with a text
  alternative naming Compagnon
- **AND** it references no external image

#### Scenario: Both READMEs show the mark

- **GIVEN** the repository's `README.md` and `README.fr.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails unless both reference the light and dark variants of
  the mark under `docs/assets/`, with a non-empty `alt` naming Compagnon

#### Scenario: A mark outside the palette is rejected

- **GIVEN** a mark file under `docs/assets/` using a fill outside the
  Mairistem palette, black and white
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming the file and the color

### Requirement: Moving from codev to Compagnon is documented in both languages

The documentation SHALL hold, in `docs/en` and `docs/fr`, a guide to
moving a project from codev to Compagnon: what is renamed (binary, planning
root, lock file, configuration key, skills, environment variables, JSON
codes, CI workflow), how to install `compagnon`, how to run
`compagnon migrate --dry-run` then `compagnon migrate`, what to do with a
shared source repository still laid out for codev, and what a hand-edited
`codev-*` skill becomes. The CLI reference SHALL document
`compagnon migrate`.

#### Scenario: The migration guide exists in both editions

- **GIVEN** the repository's `docs/en/src/SUMMARY.md` and
  `docs/fr/src/SUMMARY.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** both list the same migration guide chapter
- **AND** both chapters name `compagnon migrate --dry-run`

#### Scenario: A guide missing from one edition is caught

- **GIVEN** the migration guide listed in `docs/en/src/SUMMARY.md` only
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming the missing chapter
