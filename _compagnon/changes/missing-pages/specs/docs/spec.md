## ADDED Requirements

### Requirement: Each profile finds the pages it asked for

Each edition SHALL hold these pages, listed in its `SUMMARY.md`:
`guides/example-plan.md` (an annotated example plan),
`guides/approve-a-plan.md` (approving a plan on GitHub or GitLab) and
`guides/start-from-nothing.md` (starting from nothing) in the Everyday use
part; `guides/existing-project.md` (adopting Compagnon on an existing
project) and `guides/team-rollout.md` (deploying for a team) in the Going
further part; `contributing.md` in the Contributing part. The path pages
SHALL lead to them: the build path takes starting from nothing as a
step; the review path's "Next" names the example plan and approving; the
developer path's "Next" names adopting on an existing project; the team
path takes deploying for a team as a step; `origins.md` ends with a link
to `contributing.md`.

#### Scenario: Both editions list the new pages where their readers look

- **GIVEN** both editions' `SUMMARY.md` and path pages
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each lists the six pages in their parts
- **AND** each path page links the new pages the requirement gives it

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

### Requirement: Pages for people who do not code ask for no command

`guides/approve-a-plan.md` SHALL explain, without Claude Code or a
terminal, where a plan arrives, where its points to discuss are, and how
to comment, ask for changes and approve on GitHub and on GitLab; it MUST
hold no fenced shell block. `guides/start-from-nothing.md` SHALL say what
to get first — Claude Code and a folder for the project — and its fenced
shell blocks outside "Under the hood" blocks MAY only install Compagnon or
run `compagnon init`.

#### Scenario: The two pages keep to what a non-developer can do

- **GIVEN** both editions of `guides/approve-a-plan.md` and
  `guides/start-from-nothing.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** the approving page holds no fenced shell block and names
  GitHub and GitLab
- **AND** in French, each quoted button label stands between parentheses
  after its action, never as inline code
- **AND** the starting page runs nothing but installing and
  `compagnon init` outside "Under the hood" blocks

## MODIFIED Requirements

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
  (`ADDED`, `MODIFIED`, `REMOVED`, `RENAMED`)
- **AND** each holds no fenced shell block

#### Scenario: A shell block in the guide is caught

- **GIVEN** a guide holding a fenced `bash` block
- **WHEN** the check runs on it
- **THEN** it fails
