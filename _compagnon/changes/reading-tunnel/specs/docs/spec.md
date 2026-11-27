## ADDED Requirements

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

## MODIFIED Requirements

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

## REMOVED Requirements

### Requirement: The documentation opens with a short Start here path

**Reason**: The Start here part and its profile pages are replaced by the
tunnel's first steps.
**Migration**: None: the old addresses disappear on the owner's request;
the introduction opens the tunnel ("The documentation reads as one tunnel
in five steps").

### Requirement: Choose your path asks one question with four answers

**Reason**: Profiles scattered readers; the tunnel guides everyone through
the same pages, with skip buttons.
**Migration**: None: `choose-your-path.html` disappears; the home page
shows the five steps of the tunnel instead.

### Requirement: Each profile has a short path page

**Reason**: The four path pages are removed with the profiles.
**Migration**: None: the path pages disappear; their readers find their
pages in the tunnel, helped by skip buttons.

### Requirement: Each profile finds the pages it asked for

**Reason**: The pages each profile asked for are now steps of the tunnel or
appendices.
**Migration**: None needed: every page stays reachable from the tunnel or
the appendices.

### Requirement: The table of contents follows the refactored order

**Reason**: Replaced by the tunnel's order.
**Migration**: Read "The documentation reads as one tunnel in five steps".

### Requirement: Everyday chapters are short and end with Go further

**Reason**: "Go further" gave every page a different exit; the tunnel's
"Next step" gives one.
**Migration**: Read "Each tunnel page leads to the next", which keeps the
word caps.

### Requirement: Moving from codev to Compagnon is documented in both languages

**Reason**: The owner wants no page kept for the past in the refactored
documentation.
**Migration**: None: on the owner's decision, the documentation keeps no
trace of codev ("The documentation holds no trace of codev").

## RENAMED Requirements

- FROM: `### Requirement: The Start here path stays short and plain`
- TO: `### Requirement: The first steps stay short and plain`
- FROM: `### Requirement: The home page asks the question first, in both languages`
- TO: `### Requirement: The home page shows the tunnel first, in both languages`
- FROM: `### Requirement: The glossary defines every term of the Start here path`
- TO: `### Requirement: The glossary defines every term of the first steps`
