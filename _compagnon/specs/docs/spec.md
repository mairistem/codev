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
documentation to stdout, without conversion, except for diagrams: each
`<svg>…</svg>` element is left out, and the `<figcaption>` that follows
it stays, so that a terminal reader sees the caption instead of drawing
instructions. The output is useful
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

#### Scenario: `--print` leaves out the drawings and keeps the captions

- **GIVEN** the current binary, whose introduction holds the workflow
  diagram
- **WHEN** the user runs `compagnon docs --print`
- **THEN** stdout holds no `<svg`
- **AND** it holds the diagram's `<figcaption>` text

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
`#ffab03`, `#004c66`, `#00c39b`, plus black or white). It SHALL also hold
the "Compagnon by Mairistem" logo — the mark, the word "Compagnon" and
the line "BY MAIRISTEM" — in the same two versions. The logo MUST be
shown:

- at the top of `README.md` and `README.fr.md`, with the variant matching
  the reader's color scheme;
- in the header of the mdBook documentation, in both languages, in place
  of the Mairistem logo and the word "Compagnon" beside it;
- in the header of the landing pages `docs/site/index.html` and
  `docs/site/fr.html`, in place of the Mairistem logo and the word
  "Compagnon" beside it.

The Mairistem logo stays in the footer of the landing pages. The mark
MUST be shown:

- as the favicon of the documentation and of the landing pages;
- in the self-contained HTML file written by `compagnon docs`, inline, with
  no external asset.

A mark or logo that stands alone carries a text alternative naming
Compagnon: `alt` on an image, `role="img"` with `aria-label` on inline SVG
— a `<title>` inside the SVG file is not enough. A mark placed next to the
visible word "Compagnon", or a logo inside a link that has its own
accessible name, is decorative (`aria-hidden="true"`). The brand links
name Compagnon ("Compagnon, by Mairistem — home"; "Accueil du site
Compagnon" in French). The header logo switches to its dark-background
version in the dark themes.

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

#### Scenario: The headers show the Compagnon by Mairistem logo

- **GIVEN** `docs/theme/index.hbs` (the sidebar brand and the menu bar
  brand), `docs/site/index.html` and `docs/site/fr.html`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each header brand link holds the "Compagnon by Mairistem"
  logo, and no Mairistem logo
- **AND** each brand link keeps its accessible name naming Compagnon

#### Scenario: A header still showing the Mairistem logo is caught

- **GIVEN** a home page whose header brand link uses the Mairistem logo
- **WHEN** the check runs on it
- **THEN** it fails naming the page

### Requirement: The first steps stay short and plain

Not counting fenced code blocks, "Under the hood" blocks (`<details>`)
nor figures (`<figure>`, a diagram and its caption),
the introduction SHALL hold at most 450 words in English and 560 in
French, and the quickstart at most 500 and 625. The introduction MUST
hold a section titled "Who decides" ("Qui décide") saying, in plain words,
that the reader decides, that Claude proposes and carries out, and that
nothing is applied without the reader's approval. The quickstart MUST walk
through the Claude Code path, with one section per step in this order:
`/compagnon-propose`, reading the plan, `/compagnon-apply`,
`/compagnon-archive`. A fenced shell block (`bash`, `sh`, `shell`,
`console`, `powershell`, `pwsh`) outside an "Under the hood" block MAY only
install Compagnon or run `compagnon init`.

#### Scenario: The first steps stay under their word caps

- **GIVEN** the introduction and the quickstart of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming the chapter, the edition and the count when the
  introduction or the quickstart exceeds its cap

#### Scenario: Under the hood blocks, code and figures are not counted

- **GIVEN** a chapter of 480 words of prose, a 400-word `<details>` block,
  a 100-word fenced code block and a 200-word `<figure>`
- **WHEN** its words are counted
- **THEN** the count is 480

#### Scenario: The introduction says who decides

- **GIVEN** the introduction of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds the "Who decides" / "Qui décide" section

#### Scenario: The quickstart walks the four skills in order

- **GIVEN** the quickstart of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** `/compagnon-propose`, `/compagnon-apply` and `/compagnon-archive`
  first appear in that order, each in a section heading of its own

#### Scenario: A terminal command outside an Under the hood block is caught

- **GIVEN** a chapter with a fenced `bash` block running
  `compagnon validate` outside any `<details>` block
- **WHEN** the check runs on it
- **THEN** it fails naming `compagnon validate`

#### Scenario: Installing and initializing are allowed outside the blocks

- **GIVEN** a chapter whose only fenced shell blocks outside `<details>`
  install Compagnon and run `compagnon init`
- **WHEN** the check runs on it
- **THEN** it passes

### Requirement: The documentation shows how to ask in plain words

Claude Code starts a skill when a request matches its description; the
`/compagnon-…` command is a shortcut, not a requirement. Each step of the
quickstart SHALL show first the sentence to say to Claude in plain words
(in the edition's language), then the skill's command as an alternative.
The Skills chapter SHALL hold a section "How to ask" ("Comment demander")
with, for each installed skill, at least two example requests in plain
words and the cue that makes Claude pick that skill.

#### Scenario: Each quickstart step leads with a plain request

- **GIVEN** the quickstart of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** in each step section, a quoted plain-words request comes before
  the skill's `/compagnon-…` command

#### Scenario: Every skill has examples

- **GIVEN** the Skills chapter of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** its "How to ask" section names each of the eight skills with at
  least two quoted example requests

#### Scenario: A skill without examples is caught

- **GIVEN** a "How to ask" section that gives `compagnon-sync` one example
- **WHEN** the check runs on it
- **THEN** it fails naming `compagnon-sync`

### Requirement: The glossary defines every term of the first steps

`glossary.md` SHALL hold one `## <Term>` section per term, defining, in
plain words, at least: agent, Claude Code, skill, change, plan, proposal,
spec, requirement, scenario, delta, capability, design, tasks, archive,
decision, role, review, MCP, artifact, schema, workflow, frontmatter,
kebab-case — under the French names the French edition uses, following
the lexicon of the French edition (Modification, Compétence,
Spécification, Déroulé, Relecture…). Each definition MUST hold at most 60 words in English and 75 in
French.

#### Scenario: Every required term is defined in both editions

- **GIVEN** `docs/en/src/glossary.md` and `docs/fr/src/glossary.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds a section for every required term, each under its
  cap

#### Scenario: A missing or overlong entry is caught

- **GIVEN** a glossary without a "Delta" section and with a 90-word
  definition of "Spec"
- **WHEN** the check runs on it
- **THEN** it fails naming "Delta" and "Spec"

### Requirement: Non-developers can read and approve a plan without the terminal

`guides/review-a-plan.md` SHALL explain, in both languages, for a reader
who does not code: what a plan holds (proposal, specs, design, tasks),
what to check in each, how to read a modified rule — the added,
modified, removed and renamed sections of the specs, the rule before and
after — and how to ask Claude for changes before approving.
It MUST hold no fenced block in a shell language (`bash`, `sh`, `shell`,
`console`, `powershell`, `pwsh`); quoting part of a plan in a `markdown`
or `text` block is allowed.

#### Scenario: The guide covers the plan without a terminal

- **GIVEN** both editions of `guides/review-a-plan.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each names the proposal, the specs, the design and the tasks
- **AND** each explains the added, modified, removed and renamed sections
  under the keywords its edition's plans show: `ADDED`, `MODIFIED`,
  `REMOVED`, `RENAMED` in English; `AJOUTÉES`, `MODIFIÉES`, `SUPPRIMÉES`,
  `RENOMMÉES` in French
- **AND** each holds no fenced shell block

#### Scenario: A shell block in the guide is caught

- **GIVEN** a guide holding a fenced `bash` block
- **WHEN** the check runs on it
- **THEN** it fails

### Requirement: Under the hood blocks fold away in every rendering

A collapsible "Under the hood" block SHALL be an HTML `<details>` element,
closed by default (no `open` attribute), whose `<summary>` starts with
"Under the hood" ("Sous le capot" in French) and then names what it
explains, so that two blocks never share a name in a chapter. Its content
is Markdown, separated from `<summary>` by a blank line, and MUST be
rendered both on the website and in the offline manual written by
`compagnon docs`. The block is the native disclosure element: no script,
and the browser makes it operable with the keyboard.

#### Scenario: The offline manual renders a folded block

- **GIVEN** a chapter holding a `<details>` block whose content has a
  fenced code block
- **WHEN** the manual is rendered
- **THEN** it holds the `<details>` element, without an `open` attribute,
  with its `<summary>` and the code block rendered as HTML

#### Scenario: A malformed block is caught

- **GIVEN** a French chapter with a `<details open>` block, and one whose
  summary reads "Details"
- **WHEN** the check runs on it
- **THEN** it fails naming both blocks

#### Scenario: Two blocks with the same name are caught

- **GIVEN** a chapter with two blocks whose summary reads "Under the hood"
- **WHEN** the check runs on it
- **THEN** it fails naming the repeated summary

### Requirement: The README is a short entry point

`README.md` and `README.fr.md` SHALL hold, after the mark: the promise in
at most two sentences, a "Getting started" section ("Démarrer" in French)
in three numbered steps — install, `compagnon init`, `/compagnon-propose` —
showing the `curl … | sh` and `iwr … | iex` one-liners, and a link to
the first page of the tunnel, `introduction.html` of its edition on the
documentation site. Each MUST hold at
most 600 words in English and 750 in French, code blocks excluded.

#### Scenario: Both READMEs stay short, keep the one-liners and link the tunnel

- **GIVEN** the repository's `README.md` and `README.fr.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if either exceeds its cap
- **AND** it fails unless each "Getting started" section shows both
  one-liners, `compagnon init` and `/compagnon-propose`
- **AND** it fails unless each links the introduction of its edition

#### Scenario: A README without the tunnel's start is caught

- **GIVEN** a README that never links the introduction
- **WHEN** the check runs on it
- **THEN** it fails naming the README

### Requirement: The everyday pages explain the technical terms they use

In the chapters of the first three steps of the tunnel — Discover, Get
started and Every day — the first use of a listed technical term SHALL be a link to its
glossary entry; the later uses of that term in the chapter need no link.
The listed terms are, in English, delta, artifact, capability, schema,
workflow, frontmatter and kebab-case, and in French, delta, artefact,
capacité, schéma, déroulé, frontmatter and kebab-case. The phrase "main
specs" ("specs principales") MUST NOT appear in these chapters: they say
"the project's specs" ("les spécifications du projet"). Headings, inline code,
fenced code blocks and "Under the hood" blocks are not concerned: a
configuration key such as `workflows:` stays as it is.

#### Scenario: Every everyday chapter explains its terms

- **GIVEN** the chapters of the first three steps of the tunnel of both
  editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** in each, the first use of every listed term is a link to the
  glossary
- **AND** none holds "main specs" or "specs principales"

#### Scenario: A term used without a link is caught

- **GIVEN** a chapter whose first prose use of "delta" is plain text, and
  whose second use is a link to the glossary
- **WHEN** the check runs on it
- **THEN** it fails naming "delta"

#### Scenario: A link to the top of the glossary is caught

- **GIVEN** a chapter whose first use of "delta" links to `glossary.md`
  without an anchor, and another whose link names an anchor no glossary
  heading produces
- **WHEN** the check runs on them
- **THEN** each fails naming "delta"

#### Scenario: A term in code or under the hood is not concerned

- **GIVEN** a chapter that uses "schema" only in inline code, in a fenced
  block and in a `<details>` block
- **WHEN** the check runs on it
- **THEN** it passes

#### Scenario: The phrase "main specs" is caught

- **GIVEN** a chapter that says "merges the deltas into the main specs"
- **WHEN** the check runs on it
- **THEN** it fails naming "main specs"

### Requirement: The French edition speaks French

Every French chapter listed in `docs/fr/src/SUMMARY.md`, in all its parts,
and `README.fr.md`, SHALL use the French word wherever one exists,
following this lexicon: change → modification, skill → compétence, spec →
spécification, workflow → déroulé, review → relecture, commit →
enregistrement, pull request and merge request → demande de fusion,
design → conception, template → modèle, and, checked by hand because
« issue » is also French, issue → ticket. The English word
MAY appear only between parentheses, placed after its French equivalent so
that a reader can match the screen ("une compétence (skill)"), and stays
untouched in inline code, fenced code, "Under the hood" blocks, command
names, flags, configuration keys, JSON fields, error codes and paths
(`/compagnon-apply`, `_compagnon/changes/`), and in headings that name an
identifier — a file name, a configuration key, a `compagnon` command.
« artefact » and « delta », defined in the French glossary, are French
terms of the edition; Compagnon's review is « relecture », not « revue ». The
example requests the French edition gives to say to Claude follow the same
lexicon (« réalise la modification », not « applique le change »).

#### Scenario: The French book holds no anglicism of the lexicon

- **GIVEN** every French chapter listed in `docs/fr/src/SUMMARY.md` and
  `README.fr.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** none holds, in prose or headings and outside parentheses, the words
  change, changes, skill, skills, spec, specs, workflow, workflows, review,
  reviews, commit, commits, pull request, merge request, design, template
  or templates

#### Scenario: A merge request in prose is caught

- **GIVEN** a French chapter saying « Ouvrez une merge request. »
- **WHEN** the check runs on it
- **THEN** it fails naming « merge request »

#### Scenario: An anglicism in prose is caught

- **GIVEN** a French chapter saying « Relisez le change avant de
  l'archiver »
- **WHEN** the check runs on it
- **THEN** it fails naming « change »

#### Scenario: A change counted after a quantifier or in a heading is caught

- **GIVEN** a French chapter saying « aucun change n'est actif » and
  holding the heading `## Changes`
- **WHEN** the check runs on it
- **THEN** it fails naming « change » and « changes »

#### Scenario: A heading naming an identifier is not prose

- **GIVEN** a French chapter with the headings `## review`, `## design.md`
  and `### compagnon review`
- **WHEN** the check runs on it
- **THEN** it passes

#### Scenario: An anglicism of a technical page is caught

- **GIVEN** a French chapter saying « le design et le template du schéma »
- **WHEN** the check runs on it
- **THEN** it fails naming « design » and « template »

#### Scenario: The French verb « changer » is not an anglicism

- **GIVEN** a French chapter saying « voici ce qui change »
- **WHEN** the check runs on it
- **THEN** it passes

#### Scenario: The English word between parentheses, in code or in a path is allowed

- **GIVEN** a French chapter saying « une compétence (skill) », and
  naming `/compagnon-apply` and `_compagnon/changes/`
- **WHEN** the check runs on it
- **THEN** it passes

#### Scenario: A renamed heading keeps its links

- **GIVEN** a French heading renamed from « Écrire un template » to
  « Écrire un modèle », and a chapter linking to its old anchor
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** the check that every link of the offline manual lands on a
  heading fails until the link names the new anchor

### Requirement: The home page shows the tunnel first, in both languages

The site SHALL have a home page in English, `docs/site/index.html`, and in
French, `docs/site/fr.html` (`lang="fr"`), each linking the other with the
other's `lang` and `hreflang`; the English page's links to the French
edition lead to `fr.html`. Each MUST show, before
any installation command or technical section, the five steps of the
tunnel in its language, each a card linking the first chapter of its step
in its edition (`introduction.html`, `installation.html`, `skills.html`,
`workflow.html`, `guides/existing-project.html`), and the link for
contributors; these links are plain `<a href>` elements, which work
without script. The home pages show no reader profile.

#### Scenario: Both home pages lead with the five steps

- **GIVEN** `docs/site/index.html` and `docs/site/fr.html`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds the five step titles of its language, each linking
  the first chapter of its step under `en/` or `fr/`
- **AND** the first step comes before the first installation command
- **AND** each links `origins.html` of its edition for contributors
- **AND** `fr.html` declares `lang="fr"`, and each page links the other
  with the other's `lang` and `hreflang`

#### Scenario: A home page that asks after installing is caught

- **GIVEN** a home page whose first step comes after its `curl … | sh`
  command
- **WHEN** the check runs on it
- **THEN** it fails naming the page

### Requirement: Developer pages do not gloss everyday developer words

The workflow, skills, concepts and FAQ chapters, read mostly by
developers, SHALL NOT explain in parentheses what a terminal or a test is
("the window where you type commands", "a small program that checks…";
in French « la fenêtre où l'on tape des commandes », « un petit programme
qui vérifie »); the glossary link on first use stays.

#### Scenario: The developer chapters hold no gloss

- **GIVEN** the workflow, skills, concepts and FAQ chapters of both
  editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** none holds one of the glosses of the requirement

#### Scenario: A gloss for developers is caught

- **GIVEN** a chapter saying "a terminal (the window where you type
  commands)"
- **WHEN** the check runs on it
- **THEN** it fails naming the gloss

### Requirement: The example plan is complete and annotated

`guides/example-plan.md` SHALL show one complete plan — its proposal, its
specs with at least one requirement and scenario, its design and its
tasks — each part under a page heading of its own, its text in a quoted
`markdown` block, followed by a "What to check here" heading ("Ce qu'on
vérifie ici") holding the note addressed to a reviewer. The French edition's plan is
written as Compagnon writes a plan in French, its structural keywords in
French.

#### Scenario: Every part of the example is annotated

- **GIVEN** both editions of `guides/example-plan.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each shows the four parts, each followed by its "What to
  check here" / « Ce qu'on vérifie ici » note
- **AND** the plan's text is inside fenced `markdown` blocks, none of its
  headings rendered as a page heading
- **AND** the French plan uses the French keywords (`### Exigence :`,
  `#### Scénario :`)
- **AND** each edition's spec block parses with Compagnon's delta parser
  without issue

#### Scenario: A part of the example without its note is caught

- **GIVEN** an example plan whose design part has no "What to check here"
  heading after it
- **WHEN** the check runs on it
- **THEN** it fails naming the design part

### Requirement: Pages for people who do not code ask for no command

`guides/approve-a-plan.md` SHALL explain, without Claude Code or a
terminal, where a plan arrives — among the request's changed files, in
`_compagnon/changes/<name>/`, on GitHub — where its points to discuss are, and how
to comment, ask for changes and approve on GitHub; it MUST
hold no fenced shell block. The "Starting from nothing" section of
`installation.md` (« Partir de rien » in French) SHALL say what to get
first — Claude Code and a folder for the project — and the chapter's
fenced shell blocks outside "Under the hood" blocks MAY only install
Compagnon or run `compagnon init`.

#### Scenario: The two pages keep to what a non-developer can do

- **GIVEN** both editions of `guides/approve-a-plan.md` and
  `installation.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** the approving page holds no fenced shell block, names GitHub,
  and names `_compagnon/changes/` as where the plan is
- **AND** in French, each quoted button label stands between parentheses
  after its action, never as inline code
- **AND** the installation chapter's "Starting from nothing" section
  names Claude Code, and the chapter runs nothing but installing and
  `compagnon init` outside "Under the hood" blocks

#### Scenario: A command in a page for non-developers is caught

- **GIVEN** an approving page holding a fenced `bash` block, and an
  installation chapter running `compagnon validate`
- **WHEN** the check runs on them
- **THEN** each fails, naming the block or the command

### Requirement: Every entry point says that a plan can start from a Jira ticket

Both home pages (`docs/site/index.html`, `docs/site/fr.html`), both READMEs,
and both editions of `introduction.md` and `quickstart.md` SHALL say, in
plain words, that naming a Jira ticket in a request — the example
"PROJ-123" — makes Claude read the ticket and cite it at the top of the
plan, and SHALL link the Jira guide of their language (`guides/mcp.md`,
or its published `guides/mcp.html` from the home pages and the READMEs).
On the home pages, this mention MUST come after the five step cards, so
that the tunnel still comes first. In the tunnel, the Jira guide is
the step after Asking Claude.

#### Scenario: Every entry point mentions Jira and links the guide

- **GIVEN** both home pages, both READMEs, and both editions of
  `introduction.md` and `quickstart.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds "Jira" and a link to the Jira guide of its language
- **AND** on each home page, the last of the five step titles comes
  before the first mention of Jira

#### Scenario: An entry point without Jira is caught

- **GIVEN** an introduction that never names Jira, and a home page naming
  Jira between its first and its last step
- **WHEN** the check runs on them
- **THEN** each fails naming the page

### Requirement: The Jira guide opens with what it changes for the reader

The Jira guide SHALL be titled "Start from a Jira ticket" in English and
« Partir d'un ticket Jira » in French — its `# ` heading and its entry in
`SUMMARY.md` — and SHALL be listed in the "Every day" step of the tunnel, keeping
its file `guides/mcp.md` so that existing links still resolve. Before its
first configuration section, it MUST show what the reader says (a request
naming a ticket), what Claude does (reads the ticket, once, read-only)
and what the reader gets (the plan citing the ticket), with the diagram
"From a Jira ticket to a plan". The configuration, the read-only rules
and the section on other MCP servers follow.

#### Scenario: The guide is in the Every day step and leads with the benefit

- **GIVEN** both editions of `SUMMARY.md` and `guides/mcp.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** the guide's entry is in the "Every day" part, titled
  "Start from a Jira ticket" / « Partir d'un ticket Jira »
- **AND** the guide holds a diagram before its first `## ` heading

#### Scenario: A guide that configures before it shows is caught

- **GIVEN** a Jira guide whose first `## ` heading comes before its
  diagram
- **WHEN** the check runs on it
- **THEN** it fails naming the guide

### Requirement: The documentation's diagrams are accessible in every rendering

The documentation SHALL hold these sixteen diagrams, in both editions:

| Diagram | Where |
|---|---|
| The workflow (ask, plan, approve, build, file away) | `introduction.md`, `workflow.md`, home pages |
| Installing: install, set up the project, first request | `installation.md` |
| Asking Claude: your words, the skill Claude picks, what it does | `skills.md` |
| Building and filing away: tasks done one by one, review, archive | `guides/build-and-file.md` |
| From a Jira ticket to a plan | `guides/mcp.md`, home pages |
| The reading tunnel (its five steps) | `introduction.md` |
| Inside a plan (proposal, specs, design, tasks) | `guides/review-a-plan.md`, home pages |
| A change's folder, from start to archive | `quickstart.md` |
| How a plan is built (the steps of `/compagnon-propose`) | `workflow.md` |
| The project's records (specs, decisions, changes) and a delta merging | `concepts.md` |
| The life of a decision | `guides/decisions.md` |
| The life of a review finding | `guides/review-record.md` |
| Who reviews when (steps and roles) | `roles.md` |
| Approving a plan on GitHub | `guides/approve-a-plan.md` |
| Inherited sources | `guides/inherited-sources.md` |
| The crate graph | `architecture.md` |

Each diagram, in
a chapter or on a home page, MUST be an inline `<svg>` inside a
`<figure>` with `role="img"` and an `aria-label` in the page's language
naming every step or node in reading order (for example "Five steps: you
ask, Claude plans, you approve, Claude builds, the change is filed
away"), followed by a `<figcaption>`; it MUST reference no external file, draw
its text as text (no outlined letters), and take its colors from the
Mairistem palette through CSS custom properties, so that it reads in the
light and dark themes of the books, of the home pages and of the offline
manual. In both themes, its text MUST reach a contrast of at least 4.5:1
against the fill it sits on, and its lines and arrows at least 3:1
against the page background. Every word of the French diagrams is
French.

#### Scenario: Every diagram has a text alternative and no external file

- **GIVEN** every chapter of both editions and both home pages
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each diagram `<svg>` sits in a `<figure>`, carries
  `role="img"` and a non-empty `aria-label`, and is followed by a
  `<figcaption>`
- **AND** none holds `href` or `src` to another file

#### Scenario: A diagram without alternative is caught

- **GIVEN** a chapter holding an `<svg>` with no `aria-label`, and a
  home page whose diagram has no `aria-label`
- **WHEN** the check runs on them
- **THEN** each fails naming its page

#### Scenario: Every diagram color pair is contrasted enough

- **GIVEN** the diagram colors defined for the light and dark themes of
  the books, of the home pages and of the offline manual
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** every text color reaches 4.5:1 against the fill it is drawn
  on, and every line color 3:1 against the page background
- **AND** a pair below its threshold fails naming the theme and the two
  colors

#### Scenario: The offline manual draws the diagrams in both themes

- **WHEN** the user runs `compagnon docs --write manual.html`
- **THEN** `manual.html` holds the workflow diagram's `<svg role="img"`
  inline, unescaped
- **AND** its styles define the diagram colors for the light scheme and
  for `prefers-color-scheme: dark`

### Requirement: The design is approved on a mockup before it is applied

The new look of the home pages and of the books SHALL first be shown to
the user as a mockup — a home page and a book chapter, in light and
dark — and applied to `docs/site/` and `docs/theme/` only once the user
has approved it. The applied look MUST keep: the Mairistem palette, the
skip link, visible keyboard focus, text contrast of at least 4.5:1 in
both themes, a layout with no horizontal scroll at 320 px wide, and
every behavior the existing requirements give the home pages and the
books. The home pages follow the reader's color scheme
(`prefers-color-scheme`), with no switch of their own; the books keep
mdBook's theme menu. Nothing on the home pages moves on its own: the
example exchange with Claude is static text, present in the page from
the start.

#### Scenario: The applied look keeps the page usable

- **GIVEN** the published home pages and a book chapter
- **WHEN** they are opened at 320 px wide, in light and in dark
- **THEN** no page scrolls horizontally
- **AND** the skip link and the keyboard focus are visible

#### Scenario: A page wider than the screen is not approved

- **GIVEN** a restyled home page that scrolls horizontally at 320 px
- **WHEN** it is checked before the restyle is committed
- **THEN** the restyle is not committed until the layout is fixed

### Requirement: Error codes and decisions have their own chapters

`reference/errors.md` ("Error codes and fixes" / « Codes d'erreur et
solutions ») SHALL hold one `### ` section per error or warning code the
FAQ explained before this change, each headed by the code in inline code
and saying what it means and how to fix it; `faq.md` SHALL hold no such
section and link this chapter. `guides/decisions.md` ("Decisions" /
« Décisions ») SHALL explain the life of an architecture decision —
proposed, accepted and sealed, superseded, deviated from, promoted —
with its diagram; `concepts.md` keeps a short `## Decision` section
linking it.

#### Scenario: Error codes moved out of the FAQ

- **GIVEN** both editions of `faq.md` and `reference/errors.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** every code the FAQ held before this change is a `### `
  heading of `reference/errors.md`
- **AND** `faq.md` holds no heading made of a code, and links
  `reference/errors.md`

#### Scenario: A code left in the FAQ is caught

- **GIVEN** a FAQ with a section headed `` `decision_seal_mismatch` ``
- **WHEN** the check runs on it
- **THEN** it fails naming the code

### Requirement: The documentation reads as one tunnel in five steps

Each language's `SUMMARY.md` SHALL list, in this order, five parts — the
steps of the tunnel — then the appendices and Contributing, with these
chapters in this order:

| Part (EN / FR) | Chapters |
|---|---|
| Discover / Découvrir | `introduction.md` |
| Get started / Démarrer | `installation.md`, `quickstart.md` |
| Every day / Au quotidien | `skills.md`, `guides/mcp.md`, `guides/review-a-plan.md`, `guides/approve-a-plan.md`, `guides/build-and-file.md` |
| Understand / Comprendre | `workflow.md`, `concepts.md`, `guides/decisions.md`, `roles.md`, `guides/review-record.md` |
| For your team / Pour l'équipe | `guides/existing-project.md`, `guides/team-rollout.md`, `guides/inherited-sources.md`, `guides/artifact-language.md`, `guides/release-notes.md`, `guides/custom-schemas.md` |
| Appendices / Annexes | `reference/cli.md`, `reference/configuration.md`, `reference/file-formats.md`, `reference/json-output.md`, `reference/errors.md`, `glossary.md`, `faq.md`, `guides/example-plan.md` |
| Contributing / Contribuer | `origins.md`, `contributing.md`, `architecture.md`, `reliability.md` |

Each chapter's `# ` heading SHALL equal its `SUMMARY.md` entry. The new
chapter `guides/build-and-file.md` is titled "Building and filing away"
(« Réaliser et ranger »), and `guides/approve-a-plan.md` "Approving a plan
on GitHub" (« Approuver un plan sur GitHub »). No chapter is nested under
another.

#### Scenario: Both editions list the tunnel, then the appendices

- **GIVEN** both editions of `SUMMARY.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each lists exactly the parts and chapters above, in that order
- **AND** each chapter's `# ` heading equals its entry's title

#### Scenario: A chapter out of its step is caught

- **GIVEN** a `SUMMARY.md` listing `glossary.md` under Discover
- **WHEN** the check runs on it
- **THEN** it fails naming `glossary.md`

### Requirement: Each tunnel page leads to the next

Every chapter of the five tunnel steps SHALL end with a section titled
"Next step" (« Étape suivante ») whose first link opens the next chapter
of the tunnel, its text being that chapter's title; the last chapter of
the tunnel leads to the first appendix, `reference/cli.md`. A chapter that
some readers can skip MAY open, right under its title, with a skip button:
one link, alone in a `<div class="skip-step">` block placed before the
chapter's first paragraph and drawn as a button by the theme, whose text
says in plain words who can skip and ends with the title of the chapter
it opens — "<who>? Skip to <title>" (« <qui> ? Passez à <titre> »), for
example "No Jira? Skip to Reviewing a plan" — and which opens a later
chapter of the tunnel. There is no other notion of reader profile.
The chapters of the first three steps SHALL hold at most 1 200 words in
English and 1 500 in French, not counting fenced code blocks, "Under the
hood" blocks and figures; the introduction and the quickstart keep their
own caps.

#### Scenario: The tunnel reads from page to page

- **GIVEN** the tunnel chapters of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each ends with "Next step" / « Étape suivante », whose first
  link opens the next chapter of the tunnel and is titled as it
- **AND** each chapter of the first three steps is under its cap

#### Scenario: A page leading elsewhere is caught

- **GIVEN** a Concepts chapter whose "Next step" links Roles, while
  Decisions comes next
- **WHEN** the check runs on it
- **THEN** it fails naming `concepts.md` and `guides/decisions.md`

#### Scenario: A skip button leading backwards is caught

- **GIVEN** a Roles chapter opening with the skip button "Not for you?
  Skip to Concepts", linking `concepts.md`
- **WHEN** the check runs on it
- **THEN** it fails: the skip button opens a chapter before it

#### Scenario: A skip button out of place or misnamed is caught

- **GIVEN** a Jira guide whose skip link sits in its second section, and a
  Roles chapter whose skip button reads "Skip ahead" while it opens
  Review record
- **WHEN** the check runs on them
- **THEN** each fails: the first because the button is not before the
  chapter's first paragraph in a `skip-step` block, the second because
  its text does not end with "Review record"

### Requirement: Removed pages leave nothing behind

The pages this change removes — `choose-your-path.md`, `paths/*.md`,
`guides/start-from-nothing.md`, `guides/repository-integration.md` and
`guides/moving-from-codev.md` — SHALL no longer exist in either edition,
and nothing SHALL point to them: no chapter, README, home page or CHANGELOG
entry links them, and the books hold no redirect for them.

#### Scenario: No trace of a removed page

- **GIVEN** both editions, both `book.toml`, both READMEs and both home
  pages
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** none of the removed files exists
- **AND** nothing links one of them, the CHANGELOG included, and no
  `book.toml` holds an `[output.html.redirect]` entry

#### Scenario: A link to a removed page is caught

- **GIVEN** a chapter linking `paths/review.md`, and a `book.toml` holding
  a redirect for `choose-your-path.html`
- **WHEN** the check runs on them
- **THEN** each fails naming the removed page

### Requirement: The documentation names GitHub only

No chapter of the tunnel, glossary, FAQ, example plan, README or home
page SHALL name GitLab: approving a plan, continuous integration and every
example are described on GitHub. The command-line and JSON-output
reference chapters MAY name it only to describe, literally, what
`compagnon init` prints, as long as the command does.

#### Scenario: GitLab is not named

- **GIVEN** every chapter of both editions but `reference/cli.md` and
  `reference/json-output.md`, both READMEs and both home pages
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** none holds "GitLab"

#### Scenario: A chapter naming GitLab is caught

- **GIVEN** an approval guide that says "on GitHub or GitLab"
- **WHEN** the check runs on it
- **THEN** it fails naming the guide

### Requirement: The tunnel is approved on a mockup before the rewrite

The new order SHALL first be shown to the owner as a clickable mockup — the
tunnel's table of contents, each page's one-line purpose, its skip button
if any and its "Next step" — and the chapters SHALL be rewritten only once
the owner has approved it. The approved mockup stays in the change folder.

#### Scenario: The rewrite waits for the approval

- **GIVEN** the change's tasks
- **WHEN** the mockup is published
- **THEN** no chapter is moved or rewritten until the owner approves the
  mockup

#### Scenario: Changes asked on the mockup are shown again

- **GIVEN** an owner who asks to move a chapter on the mockup
- **WHEN** the mockup is changed
- **THEN** it is published again and the rewrite still waits for the
  owner's approval

### Requirement: The documentation's look follows the Mairistem design system

The books' theme (`docs/theme/mairistem.css`, `mairistem.js`), the offline
manual's stylesheet (`crates/compagnon-cli/assets/docs.css`) and the home
pages (`docs/site/index.html`, `docs/site/fr.html`) SHALL follow the
Mairistem design system as the approved mockup shows it: the four brand
colors (coral, deep blue, green, yellow) with black, white and the
system's greys; no gradient and no blur; IBM Plex Sans as the only text
font, a monospace font only for code. Each tunnel chapter of the books
SHALL show, above its title, an eyebrow naming its place — "Step <n> of
5 · <step>" (« Étape <n> sur 5 · <étape> ») — and a five-segment progress
bar; its "Next step" section SHALL be drawn as a dark card; its "Under the
hood" blocks as accordions marked + and −; its example block as a capture
of a Claude Code session in a white frame with a 1 px border. The home
pages SHALL open with the design system's header and product hero, and
show the five steps as cards with a colored top rule.

#### Scenario: The styles hold no gradient, blur or other font

- **GIVEN** `mairistem.css`, `docs.css` and both home pages
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** none holds `gradient(`, `blur(` or `backdrop-filter`
- **AND** every `font-family` names IBM Plex Sans, a monospace font, or
  inherits

#### Scenario: A gradient is caught

- **GIVEN** a stylesheet whose hero uses `linear-gradient(…)`
- **WHEN** the check runs on it
- **THEN** it fails naming the file and the gradient

### Requirement: The documentation holds no trace of codev

No chapter of either edition, README or home page SHALL name codev
(in any case): the command-line reference has no `compagnon migrate`
section, no error code or chapter describes the former layout, and the
introduction and the origins tell Compagnon's story without its former
name. The CHANGELOG keeps its dated history but no note pointing readers
to a codev migration.

#### Scenario: codev is not named

- **GIVEN** every chapter of both editions, both READMEs and both home
  pages
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** none holds "codev", whatever its case

#### Scenario: A chapter naming codev is caught

- **GIVEN** an origins chapter saying "formerly codev"
- **WHEN** the check runs on it
- **THEN** it fails naming the chapter

### Requirement: Beginners read a simple path, experts open the detail

In the chapters of the first three steps of the tunnel, the text shown
by default SHALL be written for a beginner: outside "Under the hood"
blocks (`<details>`, whose summary starts "Under the hood:" / « Sous le
capot : »), a fenced block MAY only be a shell block that installs
Compagnon or runs `compagnon init`, or a `markdown` or `text` block
quoting a request, an answer or an excerpt of a plan; configuration,
options and internals live in "Under the hood" blocks or in the
Understand and For your team steps and the appendices.

#### Scenario: The first steps show only what a beginner needs

- **GIVEN** the chapters of the first three steps of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** no fenced block outside "Under the hood" blocks is in another
  language than a shell install or init, `markdown` or `text`

#### Scenario: Configuration shown to a beginner is caught

- **GIVEN** a Jira guide showing a `yaml` block outside any "Under the
  hood" block
- **WHEN** the check runs on it
- **THEN** it fails naming the guide and the `yaml` block

### Requirement: Every tunnel page shows a diagram and a concrete example

Every chapter of the first four steps of the tunnel SHALL hold at least
one diagram (a `<figure class="diagram">`), and every chapter of the first
three steps at least one concrete example: a `<div class="example">`
block, titled "Example" (« Exemple »), showing a real request and what
comes of it — Claude's answer, a plan's excerpt, a pull request — with
real names and values, never placeholders alone. The request and Claude's
answer are written as a Claude Code session: a `text` block whose request
line starts with `>` and whose answer lines start with `⏺`.

#### Scenario: Each page draws and shows

- **GIVEN** the tunnel chapters of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each chapter of the first four steps holds a diagram
- **AND** each chapter of the first three steps holds an example block

#### Scenario: A page without diagram or example is caught

- **GIVEN** an Asking Claude chapter with no figure, and an installation
  chapter with no example block
- **WHEN** the check runs on them
- **THEN** each fails naming the chapter and what it lacks
