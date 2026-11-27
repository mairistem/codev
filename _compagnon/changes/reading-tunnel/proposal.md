# Proposal: One reading tunnel instead of reader profiles

## Why

The owner read the published documentation after the refactor
(`2026-11-27-jira-and-modern-docs`): the home page is right, but the books
have no reading logic. A reader who clicks "next" from the introduction
goes through Choose your path, four profile pages that mostly do not
concern them, then the glossary — a dictionary — before reaching anything
useful; each profile page then sends to chapters spread over three parts.
The profile system scatters readers instead of guiding them, and it
weighs on the specification (six requirements exist only for it). The
owner asks for a complete refactor: a guided "tunnel" read from start to
end, very simple for beginners while experts can open the detail without
cluttering it; diagrams and concrete examples on every page; no profiles,
no trace of codev, and GitHub only.

## What Changes

- **One tunnel in five steps**, both editions, in this order:
  1. **Discover** — Compagnon in two minutes;
  2. **Get started** — Installation (which now holds "starting from
     nothing"), Your first change;
  3. **Every day** — Asking Claude, Start from a Jira ticket, Reviewing a
     plan, Approving a plan on GitHub, Building and filing away (new:
     apply, sync and archive, taken out of How it works);
  4. **Understand** — How it works, Concepts, Decisions, Roles, Review
     record;
  5. **For your team** — Adopting Compagnon on an existing project,
     Deploying Compagnon for a team (which now holds repository
     integration and `compagnon doctor`), Inherited sources, Artifact
     language, Release notes and CHANGELOG, Custom schemas.
- **Simple for beginners, deeper for experts**: in the first three steps,
  what is shown by default is written for a beginner — no configuration,
  option or internals; those fold into "Under the hood" blocks or live in
  the Understand and For your team steps and the appendices.
- **A diagram and a concrete example on every page**: each page of the
  first four steps draws what it explains, and each page of the first
  three shows a real example — a request and what comes of it; three new
  diagrams (installing, asking Claude, building and filing away).
- **Each tunnel page ends with "Next step"** linking the next page by its
  title, so mdBook's "next" and the page's own link tell the same story.
- **No profiles; skip buttons instead**: a page some readers do not need
  opens with a button in plain words — "Not for you? Skip to …" — that
  jumps ahead in the tunnel. The home page's four profile cards become the
  five steps of the tunnel; the READMEs link the start of the tunnel.
- **Appendices, outside the tunnel**: the reference chapters (command
  line, configuration, file formats, JSON output, error codes), the
  glossary, the FAQ, the annotated example plan. Contributing is
  unchanged.
- **Removed, with nothing kept for the past**: Choose your path, the four
  path pages, Starting from nothing and Repository integration as pages
  (their content moves into the tunnel), and Moving from codev as a page
  ; the documentation keeps no trace of codev at all: no page, no
  `compagnon migrate` section in the reference, no mention in the
  introduction or the origins. No redirect: the old addresses simply disappear.
- **GitHub only**: the tunnel, the glossary, the FAQ, the example plan,
  the READMEs and the home pages name GitHub alone; the command-line
  reference keeps describing what `compagnon init` prints while the
  command prints it.
- **A clickable mockup of the tunnel first** — the table of contents, each
  page's purpose, its skip button and its "Next step" — approved by the
  owner before any chapter is rewritten.

## Capabilities

### New Capabilities

### Modified Capabilities

- `docs`

### Removed Capabilities

## Impact

- `docs/en/src`, `docs/fr/src`: `SUMMARY.md`; removed `choose-your-path.md`,
  `paths/*.md`, `guides/start-from-nothing.md`,
  `guides/repository-integration.md`, `guides/moving-from-codev.md`; new
  `guides/build-and-file.md`; `introduction.md`, `installation.md`,
  `quickstart.md`, `workflow.md`, `guides/approve-a-plan.md`,
  `guides/team-rollout.md` reshaped; every tunnel page gains its "Next
  step"; links to removed pages rewritten; GitLab removed everywhere.
- `docs/theme/mairistem.css` and `crates/compagnon-cli/assets/docs.css`
  (the skip button), `docs/site/index.html`, `docs/site/fr.html` (five
  step cards), `README.md`, `README.fr.md`, `CHANGELOG.md` (its codev note
  links the command-line reference instead of the removed guide).
- In the change folder: `mockup/` and an adapted copy of `build_home.py`.
- `crates/compagnon-cli/src/docs_checks.rs`, `docs.rs`, `tests/repository.rs`:
  checks of the Start here path, profiles, parts and the codev guide
  replaced by checks of the tunnel; the offline manual follows the new
  order.
- Out of scope: `compagnon init` still prints a GitLab CI job when it skips
  the GitHub one (`crates/compagnon-cli/src/render.rs`) — a behavior of the
  command, not of the documentation, left for its own change if wanted.
- No change to the commands, the skills or the configuration.

## Domains

<!-- No code diff yet (only this plan): domains ticked by judgement. -->

- [ ] security — documentation only; no input, secret or dependency
- [ ] personal-data — no personal data handled
- [x] user-interface
- [ ] performance — static pages
- [ ] operations — the existing Pages workflow publishes unchanged
- [x] migration

## Release note

- Changed: The documentation reads as one guided path in five steps — discover, get started, every day, understand, for your team — each page ending with the next step and, where some readers can skip it, a button saying where to go; the reference, the glossary and the FAQ move to appendices.
- Removed: **BREAKING** Seven pages and their addresses, with no redirect: "Choose your path" and the four profile pages (the home page now shows the five steps), "Starting from nothing" (now a section of Installation), "Repository integration" (now in Deploying Compagnon for a team) and "Moving from codev" (the documentation no longer covers moving from codev); How it works' Apply, Sync and Archive sections move to the new "Building and filing away".
- Changed: The documentation names GitHub only.
