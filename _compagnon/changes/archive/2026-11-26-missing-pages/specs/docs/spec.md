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
  under the keywords its edition's plans show: `ADDED`, `MODIFIED`,
  `REMOVED`, `RENAMED` in English; `AJOUTÉES`, `MODIFIÉES`, `SUPPRIMÉES`,
  `RENOMMÉES` in French
- **AND** each holds no fenced shell block

#### Scenario: A shell block in the guide is caught

- **GIVEN** a guide holding a fenced `bash` block
- **WHEN** the check runs on it
- **THEN** it fails
