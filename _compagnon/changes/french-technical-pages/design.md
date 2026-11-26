# Design: The French technical pages speak French too

## Context

The lexicon of the French edition is checked by `anglicisms(md)` in
`crates/compagnon-cli/src/docs_checks.rs`: it reads prose and headings
(`prose_paragraphs(md, Headings::Keep)`), drops what sits between
parentheses (`without_parentheses`), and reports skill(s), spec(s),
workflow(s), review(s), commit(s), « change(s) » after a determiner and
« pull request ». The test `the_french_everyday_pages_hold_no_anglicism`
applies it to the 17 chapters of `EVERYDAY` and to `README.fr.md`.

The other French chapters were never rewritten. A count of the same words,
plus design, template, issue and merge request, finds about 800 in 22
pages: `roles.md` (≈110), `reference/cli.md` (≈110),
`reference/file-formats.md` (≈85), `guides/custom-schemas.md`,
`reference/json-output.md`, `reference/configuration.md`,
`guides/artifact-language.md`, `reliability.md`, `origins.md` (30 to 45
each), then the other guides, `architecture.md` and `contributing.md`.

`reliability.md` is generated: `evals/src/reliability.rs` holds its French
text and `the_reliability_pages_follow_the_baseline` (in
`evals/tests/suite.rs`) fails when the page differs from what the
generator writes. The page cannot be edited by hand.

Links between chapters name heading anchors; the offline manual test
`the_offline_manual_links_land_on_their_heading`
(`crates/compagnon-cli/src/docs.rs`) checks every link of both books lands
on a heading, so a renamed heading whose links were not updated fails it.

## Goals / Non-Goals

**Goals:**

- The same lexicon on every French page, checked by the test suite.
- No fact, command, flag, key or output changed: only words of prose and
  headings.

**Non-Goals:**

- The English edition, `CONTRIBUTING.fr.md`, the home pages, the skills
  and the CLI's own messages.
- Simplifying the technical pages: they stay written for developers; only
  their vocabulary changes.
- Glossing technical words on these pages: the rule of the everyday pages
  (first use links to the glossary) is not extended.

## Decisions

### Decision: The check runs on every chapter of the French `SUMMARY.md`

The test, renamed `the_french_book_holds_no_anglicism`, takes its chapters
from `docs/fr/src/SUMMARY.md` rather than from `EVERYDAY`, plus
`README.fr.md`. A chapter added later is checked without touching the test.

**Alternatives considered**: a second list of the technical chapters —
rejected, it would miss the next page added to the book.

### Decision: design, template and merge request join the checked words; issue is fixed by hand

`anglicisms` gains design, template, templates and « merge request » (as
« pull request » is matched, the word « merge » followed by a word starting
with « request »), and workflows, reviews, commits already in. « issue »
stays out of the check: it is also a French word (« l'issue d'une
commande »); the rewrite replaces it by « ticket » where it means a GitHub
or Jira issue.

**Alternatives considered**: checking « issue » after a determiner —
rejected, « une issue » is French as well, and a check that refuses
correct French would push writers to work around it.

### Decision: « artefact », « delta » and « revue » stay

« artefact » is a French word and the glossary defines it (`## Artefact`);
« delta » is defined there too (`## Delta`); the edition already uses both
in the checked everyday pages. « revue » is French; the rewrite still
prefers « relecture » where it names Compagnon's review (the lexicon's
review → relecture), and the chapter « Compte rendu de revue » becomes
« Compte rendu de relecture », in `SUMMARY.md` and in its title.

**Alternatives considered**: replacing « artefact » by « partie du plan »
— rejected, a schema's artifacts are not all parts of a plan, and the word
is in every command's output (`Artifact:`).

### Decision: The English word stays where the reader meets it as English

Inline and fenced code, command names, flags (`--change`), configuration
keys (`skills:`), JSON fields (`changeName`), error codes, paths, and
the output of commands stay as they are. Where the prose names something
the screen shows in English — a button, a section of a file, a CLI word —
the French word comes first and the English one follows between
parentheses: « une modification (change) ».

### Decision: `reliability.md` is rewritten at its source

The French strings of `evals/src/reliability.rs` follow the lexicon, then
`cargo run -p compagnon-evals -- docs` regenerates both pages from the
committed baseline. The command reads `evals/baseline/report.json` and
`run.json` only: it calls no model and costs nothing. The English page is
regenerated unchanged. Decision 0008 does not apply: no skill text or role
changes.

### Decision: Pages are rewritten in parallel, by groups, against a word-for-word brief

The rewrite goes to sub-agents, one per group of pages (Roles; Reference;
the "Going further" guides; Origins, Contributing and Architecture; the
reliability generator), each with the lexicon, the rules of the decisions
above, and the instruction to change words only. Each group's diff is then
read for a changed fact: a code span, a number or a link target that moved.

## Risks / Trade-offs

- [A fact changes while rewording a reference page] → the rewrite touches
  only words outside code; the review reads the diff with the inline code
  spans compared before and after (`git diff --word-diff`).
- [A renamed heading breaks a link from another chapter or from the
  English edition's language switch] → the offline manual test fails on
  the French book; links from the English book never point into the French
  one.
- [A legitimate French « design » or « template » is refused] → none is
  in the book today; if one appears, it is reworded (« la conception
  graphique »), which the lexicon prefers anyway.
- [Size: about 800 replacements] → by groups in parallel, each group
  checked by the test before the next step.
