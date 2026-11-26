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

## MODIFIED Requirements

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
- **AND** it fails unless each holds the four profile labels of its
  language, each linking its path page

#### Scenario: A README missing a path page is caught

- **GIVEN** a README that links three of the four path pages
- **WHEN** the check runs on it
- **THEN** it fails naming the missing page

## REMOVED Requirements

### Requirement: Choose your path guides five profiles

**Reason**: Replaced by "Choose your path asks one question with four
answers": the five profiles overlapped, and one was a level, not an
activity.

**Migration**: The profiles now live on their own path pages; the
"Discovering AI agents" readers follow the path of their activity.
