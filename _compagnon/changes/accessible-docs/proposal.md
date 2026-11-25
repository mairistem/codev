# Proposal: Documentation for every profile

## Why

The documentation is complete — about 49,000 words per language — but it
has no way in for a beginner. The "five-minute" quickstart runs 1,400 words
and mixes the Claude Code path, raw CLI output and the internals of
`compagnon init`; the introduction moves on to the architecture and a long
"Origins" section written for contributors; the vocabulary (spec, delta,
capability, change, archive, skill, decision, role, MCP) arrives
undefined; nothing tells readers which pages are for them; and nothing
reassures someone new to AI agents about who decides. People who do not
master AI — developers new to agents, and non-developers such as product
owners who only read and approve plans — are put off before they start.
The product was just renamed Compagnon: the first pages are what every new
reader sees now.

## What Changes

- A **Start here** section opens the documentation, short and free of
  jargon:
  - *Compagnon in two minutes* (the introduction, rewritten): what it does,
    in plain words — you decide, Claude proposes, nothing happens without
    your approval.
  - *Your first change* (the quickstart, rewritten): the Claude Code path
    only — `/compagnon-propose`, read the plan, `/compagnon-apply`,
    `/compagnon-archive`; the CLI equivalents and what `compagnon init` does
    underneath move into collapsible "Under the hood" blocks.
  - A **Glossary**: every term the documentation uses, in plain words.
- A **Choose your path** page names five profiles and the pages each one
  reads, in order: discovering AI agents; not coding but reading and
  approving plans; developing; configuring Compagnon for a team; contributing
  to Compagnon.
- A **Read and approve a plan** page for non-developers: what a plan
  holds, what to look for, how to ask for changes — without the terminal.
- The table of contents is regrouped in layers — Start here, Everyday use,
  Going further, Reference, Contributing — without removing content: the
  origins (moved out of the introduction), the architecture and the
  reliability report go under Contributing.
- README and README.fr shrink to the promise, three steps and links per
  profile; the landing page follows the same paths.
- A test caps the length of the Start here path, so that it stays short.

## Capabilities

### New Capabilities

### Modified Capabilities

- `docs`

### Removed Capabilities

## Impact

- `docs/en/src` and `docs/fr/src`: `SUMMARY.md` regrouped; `introduction.md`
  and `quickstart.md` rewritten; new chapters `choose-your-path.md`,
  `glossary.md`, `guides/review-a-plan.md`, `origins.md`; the same in both
  languages (French in *vouvoiement*).
- `README.md`, `README.fr.md`, `docs/site/index.html`.
- The offline manual (`compagnon docs`) follows the new order and renders
  the collapsible blocks; `crates/compagnon-cli` tests for the
  documentation are updated and extended.
- No command, file format or JSON output changes.

## Domains

- [ ] security — documentation only; no input, secret or dependency
- [ ] personal-data — no personal data
- [x] user-interface
- [ ] performance — static pages
- [ ] operations — nothing runs differently
- [ ] migration — no format, configuration or output changes; chapters keep their paths

## Release note

- Changed: The documentation now opens with a short **Start here** path — Compagnon in two minutes, your first change in Claude Code, and a glossary — and a **Choose your path** page for five profiles, including people who do not code and only approve plans. Nothing was removed: the details moved to later sections.
- Added: A **Read and approve a plan** guide for product owners and other non-developers.
