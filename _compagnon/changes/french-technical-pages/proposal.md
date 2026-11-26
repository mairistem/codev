# Proposal: The French technical pages speak French too

## Why

The French "Start here" and "Everyday use" pages follow the lexicon of the
French edition (change `plain-language-docs`), but the pages after them —
Roles, the "Going further" guides, the Reference, Origins, Architecture,
Reliability — still say « le change », « la skill », « les specs », « le
workflow », « le design », « le template », « la pull request ». A reader
who follows a link from an everyday page meets two vocabularies for the
same thing; the readers' test of `missing-pages` stopped on Origins and
Roles for that reason. A count over the French book finds about 800 such
words in 22 pages, most in `roles.md`, `reference/cli.md` and
`reference/file-formats.md`.

## What Changes

- Every French chapter of the book, in all its parts, follows the lexicon
  of the French edition in prose and headings: change → modification,
  skill → compétence, spec → spécification, workflow → déroulé, review →
  relecture, commit → enregistrement, pull request and merge request →
  demande de fusion; the lexicon gains design → conception and template →
  modèle, and issue → ticket, written by hand.
- The English word stays where the reader sees it as such: between
  parentheses after the French word, in inline and fenced code, command
  names, flags, configuration keys, JSON fields, error codes, paths and
  command output.
- « artefact » and « delta » stay: both are defined in the French
  glossary, and « artefact » is a French word.
- The existing check of the French everyday pages runs on every French
  chapter, with the two new words of its list (design, template).
- Headings renamed by the rewrite keep their links: every link of the
  French book to a renamed heading is updated.

## Capabilities

### New Capabilities

### Modified Capabilities

- docs

### Removed Capabilities

## Impact

- `docs/fr/src/`: every chapter outside "Start here" and "Everyday use"
  (`roles.md`, `guides/*.md` of "Going further", `reference/*.md`,
  `origins.md`, `architecture.md`, `reliability.md`, `contributing.md`),
  plus the few words left in the everyday pages that the wider list now
  catches.
- `evals/src/reliability.rs`: the French text of the generated chapter
  `reliability.md`, regenerated with `cargo run -p compagnon-evals -- docs`
  from the committed baseline (no model is called, nothing is paid).
- `crates/compagnon-cli/src/docs_checks.rs`: the anglicism list and the
  test that applies it, renamed for the whole French book.
- Not touched: the English edition, `README.fr.md` (already checked),
  `CONTRIBUTING.fr.md`, the home pages, the skills, the CLI's messages,
  which are English.

## Domains

- [ ] security
- [ ] personal-data — the match is the English word "phone" in the example scenario of the English home page (`docs/site/index.html`), from another change on the branch; not personal data
- [x] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Changed: The whole French documentation now speaks French — « modification », « compétence », « spécification », « déroulé », « demande de fusion » — with the English word only between parentheses where it appears on screen.
