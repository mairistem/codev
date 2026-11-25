## Purpose

Ship complete codev documentation in a **single, shareable** form: a
standalone HTML file (embedded CSS, no external assets) that the
`codev` binary can generate and open by itself, to complement the
CLI help and serve as a point of sharing with third parties.

## ADDED Requirements

### Requirement: `codev docs` generates a standalone HTML file and opens it in the browser

The `codev` binary SHALL expose a `docs` subcommand that, without
arguments, generates a standalone HTML file and opens it in the
default system browser.

The HTML MUST:

- carry all its CSS **inline** in a `<style>` in the head — no
  reference to a CDN, no external image, no mandatory runtime
  script;
- correctly render the six common markdown elements — headings,
  paragraphs, lists, tables, code blocks, blockquotes;
- carry at the top the name `codev` and the version of the binary
  that generated it;
- open in any recent browser, including **offline**.

The default output path MUST be in the system temporary directory
(`std::env::temp_dir()`) and carry the version in its name:
`codev-docs-<version>.html`.

#### Scenario: `codev docs` without arguments opens the HTML

- **GIVEN** a `codev` binary at version 0.1.0
- **WHEN** the user runs `codev docs`
- **THEN** a `codev-docs-0.1.0.html` file is written in
  `std::env::temp_dir()`
- **AND** a system call opens that file in the default browser
  (macOS `open`, Linux `xdg-open`, Windows `start`)
- **AND** the exit code is 0
- **AND** the HTML carries the name `codev` and the version `0.1.0`
  at the top

#### Scenario: The HTML is standalone

- **GIVEN** the file generated in the previous step
- **WHEN** it is opened in a browser without an internet connection
- **THEN** the page renders correctly (inline CSS, no external
  request)
- **AND** no `<link>` element points to an `http(s):` URL

### Requirement: `codev docs --print` prints the markdown source to stdout

`codev docs --print` MUST print the documentation's **markdown
source** to stdout, without conversion. The output is useful for
piping into `less`, `bat`, or a tool that consumes markdown (an LLM,
for example).

This form MUST NOT open a browser, MUST NOT write a file, and MUST
return exit code 0.

#### Scenario: `--print` outputs non-empty markdown

- **GIVEN** the current binary
- **WHEN** the user runs `codev docs --print`
- **THEN** stdout carries non-empty content (> 1000 bytes — the
  documentation has several sections)
- **AND** the content starts with a level-1 markdown heading
  (`# codev`)
- **AND** no file is written

### Requirement: `codev docs --write <PATH>` writes without opening

`codev docs --write <PATH>` MUST write the standalone HTML to the
given path and **open nothing**. This is the "targeted distribution"
mode — placing the documentation in a shared drive, a Confluence
folder, a repository.

The path is used as is: the command MUST create missing parent
folders **only** if the given path has a parent that already exists
(the command does not invent a directory tree — it refuses a path
with several missing levels).

#### Scenario: `--write` produces the file at the given path

- **GIVEN** the current binary
- **AND** a working directory where `./out/` exists
- **WHEN** the user runs `codev docs --write ./out/manuel.html`
- **THEN** the file `./out/manuel.html` exists and carries the
  standalone HTML
- **AND** no browser is opened
- **AND** the exit code is 0

### Requirement: `codev docs` is independent of an initialized repository

`codev docs` MUST NOT read `_codev/config.yaml` nor any other
project state. The command works in any directory, even without a
codev repository, even without `.git`.

#### Scenario: Works outside a codev repository

- **GIVEN** a user in a directory that has no `_codev/`
- **WHEN** they run `codev docs --print`
- **THEN** the markdown is printed normally
- **AND** no error message mentions `_codev`
