## ADDED Requirements

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
itself to the Contributing part. The same four labels, in the same words,
SHALL be used by the introduction, the READMEs and the home pages of the
site.

#### Scenario: Both editions ask the question with the four profiles

- **GIVEN** both editions of `choose-your-path.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds the four profile labels of its language, in order,
  each linking its path page and nothing else
- **AND** a link to the Contributing part follows them

#### Scenario: A profile with two links or the wrong page is caught

- **GIVEN** a Choose your path page whose first profile links both
  `paths/review.md` and `glossary.md`
- **WHEN** the check runs on it
- **THEN** it fails naming that profile

### Requirement: Each profile has a short path page

Each path page SHALL open, under its title, with one sentence saying who it
is for and how long the path takes, then list at most three numbered
steps, each holding exactly one link to a chapter of the same edition whose
text is that chapter's title, then a "Next, if you need it" ("Ensuite, si
besoin") section, and end with a link back to Choose your path. The path
for people who do not code but want Claude to build something MUST NOT
send them to a page whose visible text asks them to type a command other
than installing Compagnon or `compagnon init`.

#### Scenario: Every path page has at most three steps, titled as their pages

- **GIVEN** the four path pages of both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each has one to three numbered steps, each with one link whose
  text equals the title of the chapter it opens
- **AND** each links back to Choose your path

#### Scenario: A step labelled differently from its page is caught

- **GIVEN** a path page whose step reads "[Step by step](workflow.md)"
  while `workflow.md` is titled "The workflow"
- **WHEN** the check runs on it
- **THEN** it fails naming "Step by step"

### Requirement: The home page asks the question first, in both languages

The site SHALL have a home page in English, `docs/site/index.html`, and in
French, `docs/site/fr.html`, each linking the other. Each MUST show, before
any installation command or technical section, the question of Choose your
path and its four profiles in its language, each linking the path page of
its edition, and the link for contributors. The pages need no script.

#### Scenario: Both home pages lead with the four profiles

- **GIVEN** `docs/site/index.html` and `docs/site/fr.html`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds the four labels of its language, each linking
  `en/paths/…` or `fr/paths/…`
- **AND** the first label comes before the first installation command

### Requirement: Developer pages do not gloss everyday developer words

The workflow, skills, concepts and FAQ chapters, read mostly by
developers, SHALL NOT explain in parentheses what a terminal or a test is
("the window where you type commands", "a small program that checks…");
the glossary link on first use stays.

#### Scenario: A gloss for developers is caught

- **GIVEN** the workflow chapter saying "a terminal (the window where you
  type commands)"
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** it fails naming the chapter

## MODIFIED Requirements

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
- **AND** it fails unless each links the four path pages of its language

## REMOVED Requirements

### Requirement: Choose your path guides five profiles

**Reason**: Replaced by "Choose your path asks one question with four
answers": the five profiles overlapped, and one was a level, not an
activity.

**Migration**: The profiles now live on their own path pages; the
"Discovering AI agents" readers follow the path of their activity.
