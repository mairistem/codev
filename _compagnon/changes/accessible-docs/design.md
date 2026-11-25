# Design: Documentation for every profile

## Context

The documentation is two mdBooks, `docs/en` and `docs/fr`, with the same
chapters in the same order. `crates/compagnon-cli/build.rs` embeds every
chapter listed in each `SUMMARY.md`, in order, so the offline manual
(`compagnon docs`) follows the table of contents. `SUMMARY.md` already
groups chapters under `# Part` headings: Getting started, Using Compagnon,
Guides, Reference, Appendix.

The entry pages are long: the introduction holds 857 words of prose in
English (978 in French), the quickstart 864 (968), and README 870 (1,006).
The introduction also carries "Why Compagnon" and a long "Origins" section,
and the quickstart puts the internals of `compagnon init` and the CLI
equivalent of each step in the reader's way.

Tests already pin parts of chapters: `docs.rs` checks the roles,
configuration, CLI, JSON and skills chapters, and that the manual starts
with the `# Compagnon` title; `tests/repository.rs` checks the READMEs, the
migration guide and the site. The distribution spec requires the README's
"Getting started" section to show the `curl … | sh` and `iwr … | iex`
one-liners. No decision in `_compagnon/decisions/` governs the
documentation's structure.

## Goals / Non-Goals

**Goals**

- A reader new to AI agents, or who does not code, reaches a first
  understanding in the four Start here chapters, under the word caps.
- Every reader finds the pages for their profile from one page.

**Non-Goals**

- Rewriting the reference chapters, the roles chapter or the guides beyond
  moving them; their density serves the readers who reach them.
- Changing a chapter's file path: links from outside keep working.
- A third language, or a separate beginners' book.

## Decisions

### Decision: Regroup in place, in five parts

`SUMMARY.md` keeps its chapters and gets five parts:

| Part | Chapters |
|---|---|
| Start here / Commencer ici | introduction, choose-your-path, quickstart, glossary |
| Everyday use / Au quotidien | installation, workflow, skills, guides/review-a-plan, concepts, faq |
| Going further / Aller plus loin | roles, the guides (moving-from-codev, inherited-sources, repository-integration, release-notes, review-record, mcp, artifact-language, custom-schemas) |
| Reference / Référence | reference/cli, configuration, file-formats, json-output |
| Contributing / Contribuer | origins, architecture, reliability |

The introduction stays first and keeps its `# Compagnon` title, so the
manual still opens on it. "Why Compagnon" shrinks into the introduction's
plain-words pitch; "Origins" and "Built with Compagnon" move to a new
`origins.md` under Contributing.

**Alternatives considered**: a separate beginners' book — two texts to keep
in step, twice, which drift; trimming the documentation — loses what the
advanced profiles use; a new "Start here" folder with copies — duplicates.

### Decision: The quickstart is the Claude Code path; the rest folds away

`quickstart.md` becomes "Your first change": install (one line, linking
the Installation chapter), `compagnon init`, then four Claude Code steps —
`/compagnon-propose`, read the plan (linking Read and approve a plan),
`/compagnon-apply`, `/compagnon-archive`. What `compagnon init` does
moves to the `compagnon init` section of the CLI reference, which already
links to it (`quickstart.md#what-compagnon-init-does`, the link then points
within the reference). The command output and the CLI equivalent of each
step move into
`<details><summary>Under the hood</summary> … </details>` blocks, with a
blank line after `<summary>` so that both mdBook and pulldown-cmark render
the Markdown inside.

`<details>` is the native, keyboard-operable disclosure element: no script,
and it works in the offline manual. The manual's stylesheet
(`crates/compagnon-cli/assets/docs.css`) and the site theme
(`docs/theme/mairistem.css`) give it a visible summary and focus outline.

**Alternatives considered**: a separate "CLI path" chapter — the reader
loses the link between a skill and what it runs; tabs — need JavaScript,
absent from the offline manual.

### Decision: The glossary is one heading per term

`glossary.md` holds one `## <Term>` heading per term, in alphabetical
order, each followed by one short paragraph — headings give each term an
anchor the other chapters link to on first use. The French edition uses
the French terms where the documentation does (proposition, exigence,
scénario, capacité, conception, tâches, décision, rôle, revue) and keeps
the others (agent, Claude Code, skill, change, plan, spec, delta, archive,
MCP).

**Alternatives considered**: a table — long definitions wrap badly and a
row has no anchor.

### Decision: Word caps checked by the documentation tests

A helper in `crates/compagnon-cli/src/docs.rs` tests counts the words of a
chapter outside fenced code blocks and `<details>` blocks, and the tests
apply the caps of the spec to the embedded chapters of both languages; the
README checks join the existing ones in `tests/repository.rs`. The caps
leave French 25 % more room, the usual growth of a translation.

**Alternatives considered**: a CI script — the project's checks live in the
test suite.

## Risks / Trade-offs

- [A cap blocks a useful sentence later] → the cap is the point: the
  sentence goes to a later chapter or an Under the hood block; the cap can
  be raised deliberately in a change.
- [The French path exceeds its cap after translation] → the French text is
  written to its own cap, not translated word for word.
- [A link into a section that moves] → only one exists, from the CLI
  reference to "What `compagnon init` does", in both languages; it is
  updated with the move. "Origins" moves to `origins.md`; no link points to
  its anchor.
- [A non-developer meets a terminal command in Read and approve a plan]
  → the test forbids shell blocks in that guide.

## Migration Plan

Documentation only: no data, configuration or output changes. Rollback is
a revert of the change's commit.
