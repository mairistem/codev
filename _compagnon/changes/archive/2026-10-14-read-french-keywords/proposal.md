# Proposal: Read structural keywords from a per-language catalog, French included

## Why

A team that writes its specs in French still has to write `### Requirement:`,
`## ADDED Requirements` or `SHALL` in English, because codev only recognizes
the English keywords: a sentence like `Le portail DOIT …` is rejected by
`codev validate`. The keywords are also string literals scattered across the
parser, the validator, the merge and the proposal readers, so adding a
language today means editing code in a dozen places.

## What Changes

- A keyword catalog: every structural keyword codev reads or writes has a
  stable key (`requirement`, `scenario`, `delta.added`, `normative`,
  `proposal.domains`…), and each language is one embedded data file giving
  the text of every key. English and French ship with codev; adding a
  language means adding one file.
- The parser recognizes the keywords of every catalog language, whatever
  the project's `language`: `## Objet`, `## Exigences`, `### Exigence :`,
  `#### Scénario :`, `## Exigences AJOUTÉES` / `MODIFIÉES` / `SUPPRIMÉES` /
  `RENOMMÉES`, `**Raison**`, `**Migration**`, `DE :` / `VERS :`. English
  keeps working unchanged.
- Validation accepts `DOIT` and `DOIVENT` as normative keywords, next to
  `SHALL` and `MUST`.
- The proposal readers recognize `## Domaines` (with `Aucun`) and
  `## Note de version` (with `Aucune` and the categories `Ajouté`, `Modifié`,
  `Obsolète`, `Supprimé`, `Corrigé`, `Sécurité`); the design reader
  recognizes `## Décisions` and `### Décision :`.
- Sync and archive write the keywords in the language of the spec they
  edit, so a French spec stays French; a new main spec takes the language
  of the delta that creates it.
- Out of scope, left to a follow-up change: what codev and its skills
  *write* (schema templates, instructions, skill text) when `language` is
  `fr`, and the review record's markers.

## Capabilities

### New Capabilities

- `keyword-languages`

### Modified Capabilities

- `spec-parsing`
- `validation`
- `spec-merge`
- `changelog`
- `decisions`

### Removed Capabilities

## Impact

- `codev-core`: a new keyword catalog module and its embedded language
  files under `assets/keywords/`; the parser (`parser/shared.rs`,
  `parser/spec.rs`, `parser/delta.rs`), the validation rules
  (`validate/rules.rs`), the merge and its renderer (`merge/mod.rs`,
  `merge/render.rs`), `domains.rs` and `changelog/release_note.rs` read
  their keywords from the catalog.
- `codev-engine`: `design.rs` reads the decisions headings from the catalog.
- The JSON contract does not change: finding codes stay the same, and a
  release note category is reported by its English name whatever language
  wrote it.
- Documentation, in `docs/en` and `docs/fr`: the concepts and workflow
  chapters list the accepted keywords and the catalog.

## Domains

- [ ] security — the parser reads the same untrusted markdown as before; no new input
- [ ] personal-data
- [ ] user-interface
- [ ] performance — matching a few more keywords per heading is negligible
- [ ] operations
- [x] migration — a spec written with French keywords is rejected by an older codev; existing English specs are unaffected

## Release note

- Added: Specs and deltas can be written with French structural keywords — `### Exigence :`, `#### Scénario :`, `## Exigences AJOUTÉES`, `DOIT` and the others — next to the English ones, which keep working.
- Added: The keywords come from a per-language catalog, so a new language can be added with one file.
