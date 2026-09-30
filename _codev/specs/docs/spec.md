# Docs Specification

## Purpose

Deliver complete codev documentation in a **single, shareable**
form: a self-contained HTML file (embedded CSS, no external assets)
that the `codev` binary can generate and open by itself, to
complement the CLI help and serve as a point of sharing with third
parties.

## Requirements

### Requirement: `codev docs` generates a self-contained HTML file and opens it in the browser

The `codev` binary SHALL expose a `docs` subcommand which, without
arguments, generates a self-contained HTML file and opens it in the
default system browser.

The HTML MUST:

- carry all of its CSS **inline** in a `<style>` element in the
  head — no reference to a CDN, no external image, no required
  runtime script;
- correctly render the six common markdown elements — headings,
  paragraphs, lists, tables, code blocks, blockquotes;
- carry the name `codev` and the version of the binary that
  generated it at the top;
- open in any recent browser, including **offline**.

The default output path MUST be in the system temporary directory
(`std::env::temp_dir()`) and carry the version in its name:
`codev-docs-<version>.html`.

#### Scenario: `codev docs` without arguments opens the HTML

- **GIVEN** a `codev` binary at version 0.1.0
- **WHEN** the user runs `codev docs`
- **THEN** a file `codev-docs-0.1.0.html` is written to
  `std::env::temp_dir()`
- **AND** a system call opens that file in the default browser
  (macOS `open`, Linux `xdg-open`, Windows `start`)
- **AND** the exit code is 0
- **AND** the HTML carries the name `codev` and the version `0.1.0` at the top

#### Scenario: The HTML is self-contained

- **GIVEN** the file generated in the previous step
- **WHEN** it is opened in a browser without an internet connection
- **THEN** the page renders correctly (inline CSS, no external
  request)
- **AND** no `<link>` element points to an `http(s):` URL

### Requirement: `codev docs --print` prints the markdown source to stdout

`codev docs --print` MUST print the **markdown source** of the
documentation to stdout, without conversion. The output is useful
for piping into `less`, `bat`, or a tool that consumes markdown
(an LLM, for example).

This form MUST NOT open a browser, MUST NOT write any file, and
MUST return exit code 0.

#### Scenario: `--print` outputs non-empty markdown

- **GIVEN** the current binary
- **WHEN** the user runs `codev docs --print`
- **THEN** stdout carries non-empty content (> 1000 bytes — the docs
  have several sections)
- **AND** the content starts with a level-1 markdown heading
  (`# codev`)
- **AND** no file is written

### Requirement: `codev docs --write <PATH>` writes without opening

`codev docs --write <PATH>` MUST write the self-contained HTML to
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
- **WHEN** the user runs `codev docs --write ./out/manual.html`
- **THEN** the file `./out/manual.html` exists and carries the
  self-contained HTML
- **AND** no browser is opened
- **AND** the exit code is 0

### Requirement: `codev docs` does not depend on an initialized repository

`codev docs` MUST NOT read `_codev/config.yaml` or any other
project state. The command works from any directory, even without
a codev repository, even without `.git`.

#### Scenario: Works outside a codev repository

- **GIVEN** a user in a directory that has no `_codev/`
- **WHEN** they run `codev docs --print`
- **THEN** the markdown is printed normally
- **AND** no error message mentions `_codev`

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
