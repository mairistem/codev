# Proposal: Put Jira forward, draw the workflow, and give the documentation a modern developer look

## Why

Readers told us the Jira integration is hard to find: it lives in one
"Going further" guide titled "Jira and other MCP servers", and the home
pages, the READMEs, the introduction, the quickstart and three of the four
path pages never mention it. Yet "start a plan from a ticket" is often the
first thing a team wants. The documentation also explains the workflow in
words only — no picture of ask → plan → approve → build — and its look
(home pages and books) reads as a corporate page rather than a tool for
developers working with AI. Finally, the "Compagnon by Mairistem" logo
exists under `docs/assets/` but the site and book headers still show the
Mairistem logo with the word "Compagnon" beside it.

## What Changes

- **Jira, everywhere a reader starts**: a one-line "Start from a Jira
  ticket" mention — say "Prepare a plan for PROJ-123", Claude reads the
  ticket and cites it at the top of the plan — on both home pages, both
  READMEs and the introduction; a short "You have a Jira ticket?" box in
  "Your first change"; a link in the "Next" list of the four path pages (the team page
  already has one, reworded).
- **The Jira guide, benefit first**: the guide keeps its file
  (`guides/mcp.md`, no broken link) but is retitled "Start from a Jira
  ticket" ("Partir d'un ticket Jira"), moves to the "Everyday use" part,
  and opens with what changes for the reader and a picture of the flow,
  before the configuration; other MCP servers stay as its last section.
- **Diagrams**: three inline SVG drawings, in both languages — the
  workflow (ask → plan → approve → build → file away) in the introduction
  and the workflow chapter; "From a Jira ticket to a plan" in the Jira
  guide and on the home pages; the four paths in "Choose your path".
  Each carries a text alternative, reads in light and dark themes, works
  in the offline manual (`compagnon docs`) with no external file, and
  leaves `compagnon docs --print` readable: `--print` leaves out the
  drawings and keeps their captions.
- **New look**: a modern, developer- and AI-oriented design for the home
  pages and both books — the Mairistem palette kept, a dark hero with a
  static terminal-style example of a conversation with Claude, monospace
  accents, cards, sharper code blocks — first as a mockup the user
  approves, then applied to the theme.
- **The "Compagnon by Mairistem" logo** replaces the Mairistem logo plus
  word pairing in the headers of the home pages and the books; the
  Mairistem logo stays in the footer.

## Capabilities

### New Capabilities

### Modified Capabilities

- `docs`

### Removed Capabilities

## Impact

- `docs/site/index.html`, `docs/site/fr.html` (header, hero, Jira
  section, diagram).
- `docs/theme/` (`mairistem.css`, `header.hbs`, `index.hbs`, possibly
  `mairistem.js`): new look and logo, for both books.
- `docs/en/src` and `docs/fr/src`: `SUMMARY.md`, `introduction.md`,
  `quickstart.md`, `workflow.md`, `choose-your-path.md`, `paths/develop.md`,
  `paths/review.md`, `paths/build.md`, `paths/team.md`, `guides/mcp.md`,
  `reference/cli.md` (the `--print` help); `README.md`, `README.fr.md`.
- `crates/compagnon-cli/src/docs.rs` (offline manual: diagram styles in
  light and dark, `--print` without SVG markup), `main.rs` (the `--print`
  help text), `docs_checks.rs` / `tests/repository.rs` (checks for Jira
  on every entry point, diagram alternatives and contrast, the logo in
  headers).
- One command output changes: `compagnon docs --print` no longer prints
  the diagrams' SVG markup. No change to the other commands, the skills
  or the Jira integration itself.

## Domains

<!-- No code diff yet (only this plan): domains ticked by judgement. -->

- [ ] security — documentation and static pages only; no input, secret or dependency added
- [ ] personal-data — no personal data handled
- [x] user-interface
- [ ] performance — static pages; inline SVG of a few kilobytes
- [ ] operations — the existing Pages workflow publishes the site unchanged
- [ ] migration — no configuration or data change; the Jira guide keeps its URL; the only output change, `docs --print` leaving out SVG markup that never existed before this change, removes nothing a consumer had

## Release note

- Changed: The documentation puts the Jira integration forward — from the home page, the README and the first pages, you learn that "Prepare a plan for PROJ-123" makes Claude read the ticket and cite it in the plan.
- Changed: The documentation has a new look, the "Compagnon by Mairistem" logo, and diagrams of the workflow, of the path from a Jira ticket to a plan, and of the four reading paths.
- Changed: `compagnon docs --print` leaves out the diagrams' drawings and keeps their captions, so a terminal or an AI reads the caption instead of SVG markup.
