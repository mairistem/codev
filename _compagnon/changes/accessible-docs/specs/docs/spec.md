## ADDED Requirements

### Requirement: The documentation opens with a short Start here path

Each language's `SUMMARY.md` SHALL open with a part titled "Start here"
("Commencer ici" in French) listing, in this order: `introduction.md`
(Compagnon in two minutes), `choose-your-path.md`, `quickstart.md` (your
first change) and `glossary.md`. The table of contents SHALL then group the
other chapters in four parts, in this order: Everyday use, Going further,
Reference, Contributing. The Contributing part MUST hold `origins.md`,
`architecture.md` and `reliability.md`. No chapter is removed; a chapter
keeps its file path, so that existing links keep working.

#### Scenario: Both editions open with the Start here path

- **GIVEN** the repository's `docs/en/src/SUMMARY.md` and
  `docs/fr/src/SUMMARY.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each lists `introduction.md`, `choose-your-path.md`,
  `quickstart.md` and `glossary.md` first, in that order, under the first
  part
- **AND** its parts are, in order, Start here, Everyday use, Going further,
  Reference, Contributing (or their French titles)

#### Scenario: A chapter moved out of the Start here path is caught

- **GIVEN** a `SUMMARY.md` that lists `concepts.md` inside the Start here
  part
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming the unexpected chapter

#### Scenario: The offline manual follows the new order

- **WHEN** the user runs `compagnon docs --print`
- **THEN** the manual starts with the introduction, then Choose your path,
  then the first change, then the glossary

### Requirement: The Start here path stays short and plain

The introduction SHALL hold at most 450 words in English and 560 in French;
the quickstart at most 900 words in English and 1,120 in French, not
counting fenced code blocks nor collapsible "Under the hood" blocks
(`<details>`). The introduction MUST say, in plain words, that the reader
decides, that Claude proposes and carries out, and that nothing is applied
without the reader's approval. The quickstart MUST walk through the
Claude Code path only — `/compagnon-propose`, reading the plan,
`/compagnon-apply`, `/compagnon-archive`; terminal commands other than the
installation and `compagnon init` appear only inside "Under the hood"
blocks.

#### Scenario: The path stays under its word caps

- **GIVEN** the four chapters of the Start here path in both languages
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming the chapter and its count when the introduction
  or the quickstart exceeds its cap

#### Scenario: Under the hood blocks are not counted

- **GIVEN** a quickstart of 850 words of prose and a 400-word
  `<details>` block
- **WHEN** the cap is checked
- **THEN** the quickstart passes

#### Scenario: A terminal command outside an Under the hood block is caught

- **GIVEN** a quickstart with a fenced `bash` block running
  `compagnon validate` outside any `<details>` block
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming the command

### Requirement: The glossary defines every term of the Start here path

`glossary.md` SHALL define, in plain words and in both languages, at least:
agent, Claude Code, skill, change, plan, proposal, spec, requirement,
scenario, delta, capability, design, tasks, archive, decision, role,
review, MCP. Each definition MUST hold at most 60 words in English and 75
in French.

#### Scenario: Every required term is defined in both editions

- **GIVEN** `docs/en/src/glossary.md` and `docs/fr/src/glossary.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming any required term without an entry, or an entry
  over its word cap

### Requirement: Choose your path guides five profiles

`choose-your-path.md` SHALL present five profiles, in this order:
discovering AI agents; not coding but reading and approving plans;
developing with Compagnon; configuring Compagnon for a team; contributing
to Compagnon. Each profile MUST list, in reading order, links to existing
chapters of the same edition, and every link MUST resolve to a chapter of
the table of contents.

#### Scenario: Every profile links to existing chapters

- **GIVEN** both editions of `choose-your-path.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each lists five profiles, each with at least two links
- **AND** it fails naming any link that is not a chapter of the table of
  contents

### Requirement: Non-developers can read and approve a plan without the terminal

`guides/review-a-plan.md` SHALL explain, in both languages, for a reader
who does not code: what a plan holds (proposal, specs, design, tasks),
what to check in each, and how to ask Claude for changes before approving.
It MUST NOT require a terminal command: it holds no fenced `bash`, `sh`,
`powershell` or `text` block of commands.

#### Scenario: The guide needs no terminal

- **GIVEN** both editions of `guides/review-a-plan.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if either holds a fenced shell block
- **AND** it fails unless each names the proposal, the specs, the design
  and the tasks

### Requirement: Under the hood blocks fold away in every rendering

A collapsible "Under the hood" block SHALL be an HTML `<details>` element
whose `<summary>` reads "Under the hood" ("Sous le capot" in French). It
MUST be closed by default, open with the keyboard as a native control, and
render its Markdown content both on the website and in the offline manual
written by `compagnon docs`.

#### Scenario: The offline manual renders a folded block

- **GIVEN** a chapter holding a `<details>` block whose content has a
  fenced code block
- **WHEN** the user runs `compagnon docs --write manual.html`
- **THEN** `manual.html` holds the `<details>` element, without an `open`
  attribute, with its `<summary>` and the code block rendered as HTML

### Requirement: The README is a short entry point

`README.md` and `README.fr.md` SHALL hold, after the mark: the promise in
two sentences, a "Getting started" section in three steps (install,
`compagnon init`, `/compagnon-propose`), and one link per profile to the
documentation's Choose your path page. Each MUST hold at most 600 words in
English and 750 in French, code blocks excluded.

#### Scenario: Both READMEs stay short and link the profiles

- **GIVEN** the repository's `README.md` and `README.fr.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails if either exceeds its cap
- **AND** it fails unless each links the Choose your path page
