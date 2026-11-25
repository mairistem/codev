## ADDED Requirements

### Requirement: The documentation opens with a short Start here path

Each language's `SUMMARY.md` SHALL open with a part titled "Start here"
("Commencer ici" in French) listing exactly, in this order:
`introduction.md` (Compagnon in two minutes), `choose-your-path.md`,
`quickstart.md` (your first change) and `glossary.md`. The table of
contents SHALL then group the other chapters in four parts, in this order:
Everyday use, Going further, Reference, Contributing ("Au quotidien",
"Aller plus loin", "Référence", "Contribuer"). The Contributing part MUST
hold `origins.md`, `architecture.md` and `reliability.md`. No chapter is
removed: every chapter listed before this change stays listed, under its
file path, so that existing links keep working.

#### Scenario: Both editions open with the Start here path

- **GIVEN** the repository's `docs/en/src/SUMMARY.md` and
  `docs/fr/src/SUMMARY.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each lists exactly `introduction.md`, `choose-your-path.md`,
  `quickstart.md` and `glossary.md`, in that order, under its first part
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
  then the first change, then the glossary

### Requirement: The Start here path stays short and plain

Not counting fenced code blocks nor "Under the hood" blocks (`<details>`),
the introduction SHALL hold at most 450 words in English and 560 in
French, Choose your path at most 400 and 500, and the quickstart at most
500 and 625 — about a third of today's quickstart. The introduction MUST
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
  introduction, Choose your path or the quickstart exceeds its cap

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
decision, role, review, MCP — under the French names the French edition
uses where it has them. Each definition MUST hold at most 60 words in
English and 75 in French.

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

### Requirement: Choose your path guides five profiles

`choose-your-path.md` SHALL present five profiles, each a `## ` section, in
this order: discovering AI agents; not coding but reading and approving
plans; developing with Compagnon; configuring Compagnon for a team;
contributing to Compagnon. Each profile MUST list, in reading order, at
least two links to chapters of the same edition, and every link MUST
resolve to a chapter of the table of contents.

#### Scenario: Every profile links to existing chapters, in order

- **GIVEN** both editions of `choose-your-path.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds the five profile sections in the order of the
  requirement, each with at least two links to chapters

#### Scenario: A link outside the table of contents is caught

- **GIVEN** a Choose your path page linking `guides/missing.md`
- **WHEN** the check runs on it
- **THEN** it fails naming `guides/missing.md`

### Requirement: Non-developers can read and approve a plan without the terminal

`guides/review-a-plan.md` SHALL explain, in both languages, for a reader
who does not code: what a plan holds (proposal, specs, design, tasks),
what to check in each, and how to ask Claude for changes before approving.
It MUST hold no fenced block in a shell language (`bash`, `sh`, `shell`,
`console`, `powershell`, `pwsh`); quoting part of a plan in a `markdown`
or `text` block is allowed.

#### Scenario: The guide covers the plan without a terminal

- **GIVEN** both editions of `guides/review-a-plan.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each names the proposal, the specs, the design and the tasks
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
showing the `curl … | sh` and `iwr … | iex` one-liners, and one link per
profile to the documentation's Choose your path page. Each MUST hold at
most 600 words in English and 750 in French, code blocks excluded.

#### Scenario: Both READMEs stay short, keep the one-liners and link the profiles

- **GIVEN** the repository's `README.md` and `README.fr.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if either exceeds its cap
- **AND** it fails unless each "Getting started" section shows both
  one-liners, `compagnon init` and `/compagnon-propose`
- **AND** it fails unless each links Choose your path
