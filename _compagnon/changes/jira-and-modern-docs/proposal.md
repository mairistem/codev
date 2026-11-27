# Proposal: Refactor the documentation — accessible first, deeper on demand, Jira forward, drawn and modern

## Why

Readers told us the Jira integration is hard to find, and the owner asks
for a complete refactor of the documentation: it must be accessible first
to any profile, let whoever wants to go further do so easily, look modern
and alive, and explain how Compagnon works with diagrams wherever a
picture helps. Today the books hold about 65 000 words per language over
36 chapters; the same things are written several times (the propose step
three times, the skill table twice, the artifacts and who writes them four
times, isolated review four times), everyday pages run long (FAQ 2 900
words, skills 3 400, concepts 2 000), the only drawings are ASCII and the
reliability charts, and the site and books still show the Mairistem logo
next to the word "Compagnon" instead of the "Compagnon by Mairistem" logo.
The owner approved a mockup of the new look (`mockup/`).

## What Changes

- **A new table of contents**, same five parts, fewer and shorter everyday
  pages: Everyday use becomes Your first change, Starting from nothing,
  Installation, How it works (`workflow.md`), Asking Claude
  (`skills.md`), Start from a Jira ticket (`guides/mcp.md`, moved up from
  Going further), Reviewing a plan, An annotated example plan, Approving a
  plan on GitHub or GitLab, Concepts, FAQ. Two chapters are added:
  Decisions (`guides/decisions.md`, the decision lifecycle taken out of
  Concepts) in Going further, and Error codes and fixes
  (`reference/errors.md`, the troubleshooting entries taken out of the
  FAQ) in Reference. No chapter is removed and no file is renamed, so
  every existing link keeps working.
- **Every chapter rewritten for its reader**: each thing is explained in
  one place and linked from the others; every everyday chapter stays
  under a word cap, keeps the detail in folded "Under the hood" blocks and
  ends with a "Go further" section; the per-skill detail lives in How it
  works, the reference material in Reference.
- **Jira everywhere a reader starts**: home pages, READMEs, introduction,
  quickstart and the four path pages say that "Prepare a plan for
  PROJ-123" makes Claude read the ticket and cite it at the top of the
  plan, and link the guide; the guide is retitled "Start from a Jira
  ticket" and leads with what it changes for the reader.
- **Thirteen diagrams**, in both languages, inline SVG with a text
  alternative, legible in light and dark, working in the offline manual:
  the workflow, from a Jira ticket to a plan, the four paths, inside a
  plan, a change's folder from start to archive, how a plan is built, the
  project's records, the life of a decision, the life of a review
  finding, who reviews when, approving on GitHub or GitLab, inherited
  sources, and the crate graph.
- **The approved look** applied to both home pages and both books: dark
  hero with a static terminal example, the "Compagnon by Mairistem" logo
  in every header, path cards, workflow steps that open to a short
  explanation with a "Go further" link, gentle motion that stops under
  reduced-motion settings.
- `compagnon docs --print` leaves out the drawings and keeps their
  captions.

## Capabilities

### New Capabilities

### Modified Capabilities

- `docs`

### Removed Capabilities

## Impact

- `docs/en/src` and `docs/fr/src`: every chapter (rewritten or trimmed),
  `SUMMARY.md`, new `guides/decisions.md` and `reference/errors.md`;
  `README.md`, `README.fr.md`.
- `docs/site/index.html`, `docs/site/fr.html`; `docs/theme/`
  (`mairistem.css`, `header.hbs`, `index.hbs`, `mairistem.js`).
- `crates/compagnon-cli/src/docs.rs` (diagram styles in the offline
  manual, `--print` without SVG), `main.rs` (`--print` help),
  `docs_checks.rs` and `tests/repository.rs` (table of contents, word
  caps and Go further, Jira entry points, diagrams and contrast, headers),
  `docs.rs` chapter tests whose anchors move.
- `crates/compagnon-core/src/roles.rs` comment citing a roles.md section,
  if that section is renamed.
- One command output changes: `compagnon docs --print` no longer prints
  SVG markup. No change to the other commands, the skills or the Jira
  integration itself.

## Domains

<!-- No code diff yet (only this plan): domains ticked by judgement. -->

- [ ] security — documentation and static pages only; no input, secret or dependency added
- [ ] personal-data — no personal data handled
- [x] user-interface
- [ ] performance — static pages; inline SVG of a few kilobytes each
- [ ] operations — the existing Pages workflow publishes the site unchanged
- [ ] migration — no configuration or data change; no chapter file is renamed or removed, so links keep working; `docs --print` only leaves out SVG markup that never existed before this change

## Release note

- Changed: The documentation is rewritten for every profile: shorter everyday pages that end with "Go further", each topic explained once, thirteen diagrams of how Compagnon works, a new look and the "Compagnon by Mairistem" logo.
- Changed: The Jira integration is put forward — from the home page, the README and the first pages, you learn that "Prepare a plan for PROJ-123" makes Claude read the ticket and cite it in the plan.
- Added: Two chapters, "Decisions" and "Error codes and fixes", gather what was spread across Concepts and the FAQ.
- Changed: `compagnon docs --print` leaves out the diagrams' drawings and keeps their captions, so a terminal or an AI reads the caption instead of SVG markup.
