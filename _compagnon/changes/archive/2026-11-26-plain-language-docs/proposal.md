# Proposal: Plain language on the everyday pages of the documentation

## Why

The pages a newcomer reads first still speak the tool's jargon: "merges a
change's deltas into the main specs without archiving it" asks the reader
to know four terms before understanding one line. Readers who do not code,
whom the change `accessible-docs` invited in, stop there; and nothing keeps
the jargon from coming back in the next page written.

## What Changes

- The chapters of the "Start here" and "Everyday use" parts — introduction,
  Choose your path, Your first change, Glossary, Installation, The
  workflow, Claude Code skills, Reviewing a plan, Concepts, FAQ — are
  rewritten in plain language, in both editions: everyday words first,
  short sentences, an example where a definition would be abstract.
- A short list of technical terms (delta, artifact, capability, main
  specs, schema, workflow, frontmatter, kebab-case) is either replaced by
  everyday words or, where the term is needed, linked to the glossary the
  first time a chapter uses it.
- The French edition speaks French: a fixed lexicon replaces the
  anglicisms — change → modification, skill → compétence, spec →
  spécification, workflow → déroulé, review → relecture, commit →
  enregistrement, pull request → demande de fusion —, the English word
  kept only between parentheses where it helps match the screen, and in
  commands and paths. The French example requests follow it (« réalise la
  modification », not « applique le change »); `README.fr.md` too.
- The glossary gains the terms of that list it does not define yet ("main
  specs" aside: it is replaced everywhere, never defined), and
  its French entries take the lexicon's names.
- The rule is checked by the test suite, so that a future everyday page
  that uses a listed term without explaining it fails.
- The "Going further", "Reference" and "Contributing" parts keep their
  technical vocabulary: they are written for those who configure or
  change Compagnon, and need the exact names.

## Capabilities

### New Capabilities

### Modified Capabilities

- docs

### Removed Capabilities

## Impact

- `docs/en/src/` and `docs/fr/src/`: the ten chapters above, both
  editions; the offline manual (`compagnon docs`), which embeds them.
- `README.fr.md`, for the lexicon.
- `crates/compagnon-cli/src/docs_checks.rs`: the checks on the listed
  terms and on the anglicisms, and their tests; the French glossary terms
  and French page titles the existing checks name.
- The French example requests of the skills' descriptions, written by the
  active change `plain-words-in-skills` (« archive le change »), follow
  the lexicon there, through that change, not this one.

## Domains

- [ ] security
- [ ] personal-data
- [x] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Changed: The everyday pages of the documentation are written in plain language: technical terms are replaced by everyday words, or explained the first time with a link to the glossary.
- Changed: The French documentation uses French words instead of anglicisms — a « modification » rather than a « change », a « compétence » rather than a « skill ».
