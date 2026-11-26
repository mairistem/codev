# Design: The pages each profile still misses

## Context

The documentation now opens on one question and four path pages
(`paths/*.md`), each of at most three steps, checked in
`crates/compagnon-cli/src/docs_checks.rs` with the everyday pages'
vocabulary and anglicism checks. Six test readers (change `profile-paths`)
named what blocked them past their path; this change writes those pages.
Facts the pages rest on, from the code: `compagnon init` writes a
Compagnon section in the pull request template that names the change's
folder (`crates/compagnon-engine/src/integration.rs`,
`pr_template_body`); a change's findings and points to discuss are in its
`review.md`; `compagnon doctor` reports "no main spec" on a project
without specs and suggests planning a change to document existing
behavior (`crates/compagnon-core/src/doctor.rs`); `compagnon_version` in
`_compagnon/config.yaml` sets the version a project requires;
`CONTRIBUTING.md` holds the build, test and check commands.

## Goals / Non-Goals

**Goals**

- Each profile's real case has a page, reachable from its path in one
  click.
- The pages for people who do not code ask for no command they cannot
  type.

**Non-Goals**

- The French vocabulary of the technical pages (reference, roles,
  guides already there): another change.
- Screenshots of GitHub and GitLab: they date fast; the page names the
  buttons as both sites label them.
- Publishing what Claude builds (hosting a form online): outside
  Compagnon; the starting page says to ask a developer or Claude.

## Decisions

### Decision: Guides in the parts their readers already use

The three pages for people who do not code — the example plan, approving,
starting from nothing — go to Everyday use, after "Reviewing a plan", so
that the vocabulary and anglicism checks of the everyday pages cover them.
Adopting on an existing project and deploying for a team go to Going
further, beside repository integration. `contributing.md` goes to the
Contributing part, after `origins.md`, and summarises `CONTRIBUTING.md`
rather than copying it. Each path takes the page that answers its reader:

| Path | Steps | Next, if you need it |
|---|---|---|
| Review and approve | introduction, Reviewing a plan, Glossary | The example plan, Approving a plan on GitHub or GitLab, FAQ |
| Build without coding | introduction, Starting from nothing, Reviewing a plan | The example plan, Glossary |
| Develop | Your first change, Claude Code skills, The workflow | Adopting on an existing project, Concepts, FAQ |
| Team | Installation, Deploying for a team, Repository integration | Roles, Configuration, MCP servers, Inherited sources |

**Alternatives considered**: one long "FAQ for non-developers" — the
readers asked for tasks, not questions; growing the quickstart to cover
starting from nothing — it already holds a developer's first change.

### Decision: The example plan is a real plan, annotated part by part

The example is the dark mode plan the quickstart already uses, written in
full as Compagnon would write it — proposal, one spec with a modified
requirement and a scenario, design, tasks — each part followed by "What to
check here", addressed to a reviewer. The French edition writes the plan
in French with French keywords, as Compagnon does for a French project, so
that it doubles as the "before / after" example "Reviewing a plan" points
to for a modified rule. Each part gets a page heading of its own
("The proposal", "The specs"…); the plan's text is shown in a quoted
`markdown` block under it, so that its own headings (`## Why`,
`### Exigence :`) never join the page's outline; the note follows under a
heading "What to check here" / « Ce qu'on vérifie ici ». A screen reader
moving by headings hears the example's parts and their notes, never the
plan's inner headings.

**Alternatives considered**: linking a plan of this repository — written
in English, about Compagnon itself, far from a reader's product.

### Decision: Approving through the pull request

A reviewer without Claude Code reads and answers the plan where the team
already works: the pull request (GitHub) or merge request (GitLab). The
page says where the plan's folder is named (the Compagnon section of the
description), where the points to discuss are (`review.md` in that
folder), how to comment on a line, how to ask for changes and how to
approve, with each button's label on both sites; and that the developer
then asks Claude to take the comments into account. Labels are quoted as
the reader sees them: GitHub's in English, GitLab's in English and in
French (GitLab follows the user's language). The French edition names the
action in French first and the label between parentheses — « approuver
(bouton « Approve ») » — so that the anglicism check, which skips
parentheses, lets "Review changes" through, and never as code, which a
reader who does not code would take for a command.

**Alternatives considered**: asking reviewers to install Claude Code —
what the readers said they could not do.

### Decision: The checks cover what a machine can tell

`docs_checks.rs` checks that the six pages are listed in their parts, that
the path pages link them as the table says, that the example plan holds
its four parts each followed by its note and, in French, French keywords,
that the approving page holds no shell block and names GitHub and GitLab,
that the starting page runs only installing and `compagnon init`, and that
"Reviewing a plan" names the four sections of a delta. Whether the pages
answer their readers is checked by the same test readers, by hand.

## Risks / Trade-offs

- [GitHub or GitLab renames a button] → the page describes the action
  first ("approve"), the label second.
- [The example plan drifts from what Compagnon writes] → it follows the
  `spec-driven` template's headings, which `compagnon validate` parses;
  a task validates the French and English example plans by writing them as
  a change in a scratch project.
- [Six more pages to keep in two languages] → each is short and checked
  by the everyday pages' checks.
