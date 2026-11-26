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
- in the header of the landing pages `docs/site/index.html` and
  `docs/site/fr.html`, next to the Mairistem logo;
- as the favicon of the documentation and of the landing pages;
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

### Requirement: The documentation opens with a short Start here path

Each language's `SUMMARY.md` SHALL open with a part titled "Start here"
("Commencer ici" in French) listing exactly, in this order:
`introduction.md` (Compagnon in two minutes), `choose-your-path.md`, the
four path pages `paths/review.md`, `paths/build.md`, `paths/develop.md`
and `paths/team.md` (nested under Choose your path), and `glossary.md`.
The table of contents SHALL then group the other chapters in four parts,
in this order: Everyday use, Going further, Reference, Contributing ("Au
quotidien", "Aller plus loin", "Référence", "Contribuer"); Everyday use
opens with `quickstart.md`. The Contributing part MUST
hold `origins.md`, `architecture.md` and `reliability.md`. No chapter is
removed: every chapter listed before this change stays listed, under its
file path, so that existing links keep working.

#### Scenario: Both editions open with the Start here path

- **GIVEN** the repository's `docs/en/src/SUMMARY.md` and
  `docs/fr/src/SUMMARY.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each lists exactly `introduction.md`, `choose-your-path.md`,
  `paths/review.md`, `paths/build.md`, `paths/develop.md`, `paths/team.md`
  and `glossary.md`, in that order, under its first part
- **AND** its Everyday use part opens with `quickstart.md`
- **AND** its parts are, in order, Start here, Everyday use, Going further,
  Reference, Contributing (or their French titles)
- **AND** the Contributing part holds `origins.md`, `architecture.md` and
  `reliability.md`

#### Scenario: A chapter moved into the Start here path is caught

- **GIVEN** a `SUMMARY.md` that lists `concepts.md` inside the Start here
  part
- **WHEN** the check runs on it
- **THEN** it fails naming `concepts.md`

#### Scenario: A chapter missing from the Start here path is caught

- **GIVEN** a `SUMMARY.md` whose Start here part lacks `glossary.md`
- **WHEN** the check runs on it
- **THEN** it fails naming `glossary.md`

#### Scenario: No chapter is removed

- **GIVEN** the 22 chapters each `SUMMARY.md` listed before this change
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming any of them no longer listed

#### Scenario: The offline manual follows the new order

- **WHEN** the user runs `compagnon docs --print`
- **THEN** the manual starts with the introduction, then Choose your path,
  then the four path pages, then the glossary

### Requirement: The Start here path stays short and plain

Not counting fenced code blocks nor "Under the hood" blocks (`<details>`),
the introduction SHALL hold at most 450 words in English and 560 in
French, Choose your path at most 200 and 250, each path page at most 250
and 310, and the quickstart at most 500 and 625. The introduction MUST
hold a section titled "Who decides" ("Qui décide") saying, in plain words,
that the reader decides, that Claude proposes and carries out, and that
nothing is applied without the reader's approval. The quickstart MUST walk
through the Claude Code path, with one section per step in this order:
`/compagnon-propose`, reading the plan, `/compagnon-apply`,
`/compagnon-archive`. A fenced shell block (`bash`, `sh`, `shell`,
`console`, `powershell`, `pwsh`) outside an "Under the hood" block MAY only
install Compagnon or run `compagnon init`.

#### Scenario: The path stays under its word caps

- **GIVEN** the Start here chapters of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming the chapter, the edition and the count when the
  introduction, Choose your path, a path page or the quickstart exceeds its
  cap

#### Scenario: Under the hood blocks and code are not counted

- **GIVEN** a chapter of 480 words of prose, a 400-word `<details>` block
  and a 100-word fenced code block
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

### Requirement: The glossary defines every term of the Start here path

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
showing the `curl … | sh` and `iwr … | iex` one-liners, and the four
profile labels of Choose your path, each linking its path page on the
documentation site. Each MUST hold at
most 600 words in English and 750 in French, code blocks excluded.

#### Scenario: Both READMEs stay short, keep the one-liners and link the profiles

- **GIVEN** the repository's `README.md` and `README.fr.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if either exceeds its cap
- **AND** it fails unless each "Getting started" section shows both
  one-liners, `compagnon init` and `/compagnon-propose`
- **AND** it fails unless each holds the four profile labels of its
  language, each linking its path page

#### Scenario: A README missing a path page is caught

- **GIVEN** a README that links three of the four path pages
- **WHEN** the check runs on it
- **THEN** it fails naming the missing page

### Requirement: The everyday pages explain the technical terms they use

In the chapters of the "Start here" and "Everyday use" parts, glossary
excepted, the first use of a listed technical term SHALL be a link to its
glossary entry; the later uses of that term in the chapter need no link.
The listed terms are, in English, delta, artifact, capability, schema,
workflow, frontmatter and kebab-case, and in French, delta, artefact,
capacité, schéma, déroulé, frontmatter and kebab-case. The phrase "main
specs" ("specs principales") MUST NOT appear in these chapters: they say
"the project's specs" ("les spécifications du projet"). Headings, inline code,
fenced code blocks and "Under the hood" blocks are not concerned: a
configuration key such as `workflows:` stays as it is.

#### Scenario: Every everyday chapter explains its terms

- **GIVEN** the chapters of the "Start here" and "Everyday use" parts of
  both editions
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

The French chapters of the "Start here" and "Everyday use" parts, and
`README.fr.md`, SHALL use the French word wherever one exists, following
this lexicon: change → modification, skill → compétence, spec →
spécification, workflow → déroulé, review → relecture, commit →
enregistrement, pull request → demande de fusion. The English word MAY
appear only between parentheses, placed after its French equivalent so that
a reader can match the screen ("une compétence (skill)"), and stays
untouched in inline code, fenced code, "Under the hood" blocks, command
names and paths (`/compagnon-apply`, `_compagnon/changes/`). The example
requests the French edition gives to say to Claude follow the same lexicon
(« réalise la modification », not « applique le change »).

#### Scenario: The French everyday pages hold no anglicism of the lexicon

- **GIVEN** the French chapters of the "Start here" and "Everyday use"
  parts and `README.fr.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** none holds, in prose or headings and outside parentheses, the words
  change, changes, skill, skills, spec, specs, workflow, review, commit or
  pull request

#### Scenario: An anglicism in prose is caught

- **GIVEN** a French chapter saying « Relisez le change avant de
  l'archiver »
- **WHEN** the check runs on it
- **THEN** it fails naming « change »

#### Scenario: The French verb « changer » is not an anglicism

- **GIVEN** a French chapter saying « voici ce qui change »
- **WHEN** the check runs on it
- **THEN** it passes

#### Scenario: The English word between parentheses, in code or in a path is allowed

- **GIVEN** a French chapter saying « une compétence (skill) », and
  naming `/compagnon-apply` and `_compagnon/changes/`
- **WHEN** the check runs on it
- **THEN** it passes

### Requirement: Choose your path asks one question with four answers

`choose-your-path.md` SHALL ask one question, "What will you do with
Compagnon?" ("Qu'allez-vous faire avec Compagnon ?"), answered by four
profiles phrased as activities, in this order, each a list item whose
only link opens its path page:

- "I review and approve plans — I do not code" (`paths/review.md`);
- "I do not code, but I want Claude to build something for me"
  (`paths/build.md`);
- "I am a developer and I have Claude write code" (`paths/develop.md`);
- "I set Compagnon up for my team" (`paths/team.md`).

Below them, one sentence MUST point those who want to change Compagnon
itself to `origins.md`, the first chapter of the Contributing part. The same four labels, in the same words,
SHALL be used by the introduction, the READMEs and the home pages of the
site.

#### Scenario: Both editions ask the question with the four profiles

- **GIVEN** both editions of `choose-your-path.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds the four profile labels of its language, in order,
  each linking its path page and nothing else
- **AND** a link to `origins.md` follows them

#### Scenario: The introduction uses the four labels

- **GIVEN** both editions of `introduction.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds the four profile labels of its language, each
  linking its path page

#### Scenario: A profile with two links or the wrong page is caught

- **GIVEN** a Choose your path page whose first profile links both
  `paths/review.md` and `glossary.md`
- **WHEN** the check runs on it
- **THEN** it fails naming that profile

### Requirement: Each profile has a short path page

Each path page SHALL be titled with its profile's label, word for word —
its `# ` heading and its entry in `SUMMARY.md` — so that every link to it,
from any entry point, names the page it opens. It SHALL open, under that
title, with one sentence saying who it is for and how long the path takes, then list at most three numbered
steps, each holding exactly one link to a chapter of the same edition whose
text is that chapter's title, then a "Next, if you need it" ("Ensuite, si
besoin") section, and end with a link back to Choose your path. The path
for people who do not code but want Claude to build something MUST NOT
take, as a step, a chapter whose fenced shell blocks outside "Under the
hood" blocks run anything but installing Compagnon or `compagnon init`;
its "Next" section names no developer chapter.

#### Scenario: Every path page has at most three steps, titled as their pages

- **GIVEN** the four path pages of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each is titled with its profile's label, in `SUMMARY.md` too
- **AND** each has one to three numbered steps, each with one link whose
  text equals the title of the chapter it opens
- **AND** a sentence comes between the title and the first step
- **AND** each holds a "Next, if you need it" / « Ensuite, si besoin »
  section and links back to Choose your path

#### Scenario: The build path asks no command beyond installing

- **GIVEN** the build path page of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** no chapter it takes as a step holds, outside "Under the hood"
  blocks, a shell block running anything but installing or
  `compagnon init`

#### Scenario: Every link of the paths resolves

- **GIVEN** Choose your path and the four path pages of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** every link to a chapter resolves to a chapter of the table of
  contents, and every anchor it names to a heading of that chapter

#### Scenario: A path page with four steps is caught

- **GIVEN** a path page with four numbered steps
- **WHEN** the check runs on it
- **THEN** it fails naming the page

#### Scenario: A step labelled differently from its page is caught

- **GIVEN** a path page whose step reads "[Step by step](workflow.md)"
  while `workflow.md` is titled "The workflow"
- **WHEN** the check runs on it
- **THEN** it fails naming "Step by step"

### Requirement: The home page asks the question first, in both languages

The site SHALL have a home page in English, `docs/site/index.html`, and in
French, `docs/site/fr.html` (`lang="fr"`), each linking the other with the
other's `lang` and `hreflang`; the English page's links to the French
edition lead to `fr.html`. Each MUST show, before
any installation command or technical section, the question of Choose your
path and its four profiles in its language, each linking the path page of
its edition, and the link for contributors; these links are plain
`<a href>` elements, which work without script.

#### Scenario: Both home pages lead with the four profiles

- **GIVEN** `docs/site/index.html` and `docs/site/fr.html`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds the four labels of its language, each linking
  `en/paths/…` or `fr/paths/…`
- **AND** the first label comes before the first installation command
- **AND** each links `origins.html` of its edition for contributors
- **AND** `fr.html` declares `lang="fr"`, and each page links the other
  with the other's `lang` and `hreflang`

#### Scenario: A home page that asks after installing is caught

- **GIVEN** a home page whose first profile label comes after its
  `curl … | sh` command
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

### Requirement: Each profile finds the pages it asked for

Each edition SHALL hold these pages, listed in its `SUMMARY.md`:
`guides/example-plan.md` (an annotated example plan),
`guides/approve-a-plan.md` (approving a plan on GitHub or GitLab) and
`guides/start-from-nothing.md` (starting from nothing) in the Everyday use
part; `guides/existing-project.md` (adopting Compagnon on an existing
project) and `guides/team-rollout.md` (deploying for a team) in the Going
further part; `contributing.md` in the Contributing part. The path pages
SHALL lead to them: the build path takes starting from nothing as a
step and its "Next" names the example plan; the review path's "Next"
names the example plan and approving; the developer path's "Next" names
adopting on an existing project; the team path takes deploying for a team
as a step, Roles moving to its "Next"; `origins.md` ends with a link to
`contributing.md`.

#### Scenario: Both editions list the new pages where their readers look

- **GIVEN** both editions' `SUMMARY.md` and path pages
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each lists the six pages in their parts
- **AND** the build path takes `guides/start-from-nothing.md` as a step
  and names `guides/example-plan.md` under "Next"
- **AND** the review path names `guides/example-plan.md` and
  `guides/approve-a-plan.md` under "Next"
- **AND** the developer path names `guides/existing-project.md` under
  "Next"
- **AND** the team path takes `guides/team-rollout.md` as a step and names
  `roles.md` under "Next"
- **AND** each edition's `origins.md` links `contributing.md`

#### Scenario: A path page that does not link its new page is caught

- **GIVEN** a review path page whose "Next" lacks `guides/approve-a-plan.md`
- **WHEN** the check runs on it
- **THEN** it fails naming the page and the missing link

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
`_compagnon/changes/<name>/`, on GitHub and GitLab alike — where its points to discuss are, and how
to comment, ask for changes and approve on GitHub and on GitLab; it MUST
hold no fenced shell block. `guides/start-from-nothing.md` SHALL say what
to get first — Claude Code and a folder for the project — and its fenced
shell blocks outside "Under the hood" blocks MAY only install Compagnon or
run `compagnon init`.

#### Scenario: The two pages keep to what a non-developer can do

- **GIVEN** both editions of `guides/approve-a-plan.md` and
  `guides/start-from-nothing.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** the approving page holds no fenced shell block, names GitHub
  and GitLab, and names `_compagnon/changes/` as where the plan is
- **AND** in French, each quoted button label stands between parentheses
  after its action, never as inline code
- **AND** the starting page names Claude Code before its first shell
  block, and runs nothing but installing and `compagnon init` outside
  "Under the hood" blocks

#### Scenario: A command in a page for non-developers is caught

- **GIVEN** an approving page holding a fenced `bash` block, and a
  starting page running `compagnon validate`
- **WHEN** the check runs on them
- **THEN** each fails, naming the block or the command
