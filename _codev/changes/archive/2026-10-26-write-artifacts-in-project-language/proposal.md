# Proposal: Write a change's artifacts entirely in the project's language

## Why

codev reads French structural keywords since `read-french-keywords`, but it
still never writes them: the embedded templates exist only in English, and
`/codev-propose` and `/codev-update` tell the agent that `## Why`,
`### Requirement:` or `SHALL` "stay in English". A team with
`language: fr` gets French sentences under English headings, and cannot
have a plan written entirely in its language. The parser and the merge are
ready, so the writing side is the only missing half.

## What Changes

- The embedded `spec-driven` schema ships its four templates in each
  language of the keyword catalog, English and French. The French
  templates write every catalog keyword in French (`## Changements`,
  `### Exigence :`, `## Exigences AJOUTÉES`, `DOIT`…) and translate the
  headings and guidance codev does not parse (`## Pourquoi`, `## Impact`,
  `**QUAND**` / `**ALORS**`).
- `codev instructions` serves the template of the project's keyword
  language: the language of `language:` when the catalog ships it,
  matched on its primary subtag (`fr-CA` → `fr`), and English otherwise
  (`pt-BR` → English keywords, Portuguese prose). A project schema may
  hold a template per language under `templates/<language>/`, and falls
  back to `templates/<file>` as today.
- `codev instructions --json` gains `keywordLanguage` and `keywords`: the
  language the structural keywords are written in, and the text to write
  for each catalog key in that language. The human output's `Language:`
  line says which language the keywords are written in, instead of
  "template keywords stay in English".
- **BREAKING** for a project whose `language:` is a catalog language
  other than English (today `fr`): `/codev-propose` and `/codev-update`
  write the structural keywords exactly as `template` and `keywords` give them, in `keywordLanguage`,
  instead of always in English; the citation line of a ticket follows the
  language too. `/codev-apply` is unchanged: it only checks boxes off.
- The comment above `language:` in a generated `_codev/config.yaml` says
  that the language applies to the structural keywords as well.
- The planning evaluation can measure it: a planning case may append a
  configuration to its fixture (`language: fr`) and expect the language
  of the keywords; a French case and a `pt-BR` case join the suite, and
  `compare` fails when plans slip back to English keywords.
- The review record (`review.md`) and the corrections file
  (`corrections.md`) keep their English format: they are records codev
  parses with a fixed vocabulary, out of this change's scope.

## Capabilities

### New Capabilities

### Modified Capabilities

- instructions
- skills
- keyword-languages
- init
- planning-evaluation

### Removed Capabilities

## Impact

- `assets/schemas/spec-driven/templates/`: the English templates move to
  `en/`, and a `fr/` set is added; `crates/codev-engine/src/schemas.rs`
  embeds both and resolves a template per language, project schemas
  included.
- `crates/codev-core/src/keywords.rs`: the keyword language of a
  `language:` value, and the texts of a language for the instructions.
- `crates/codev-engine/src/instructions.rs`, `crates/codev-cli/src/contract.rs`
  (`keywordLanguage`, `keywords`, additive fields of `InstructionsV1`) and
  `render.rs` (the `Language:` line).
- `crates/codev-core/src/config/mod.rs`: the comment above `language:`.
- `assets/workflows/propose.md` and `update.md`, rendered into the skills;
  codev's own installed skills refreshed with `codev update`.
- The evaluation of the planning and update steps, as decision 0008
  requires for a change to what the agents read.
- Documentation, in English and French: the artifact-language guide, the
  `codev instructions` and JSON references, the configuration reference,
  the file formats and the skills chapter.

## Domains

- [ ] security — no new input: the language comes from `language:`, already validated
- [ ] personal-data — no personal data is read or written
- [ ] user-interface — no interface beyond the CLI output and the artifacts' text
- [ ] performance — templates are embedded and chosen once per call
- [ ] operations — no log, metric, timeout or deployment change
- [x] migration

## Release note

- Changed: **BREAKING** A project whose `language:` is French now gets its
  planning artifacts written entirely in French, structural keywords
  included (`## Changements`, `### Exigence :`, `DOIT`). Every teammate and
  the CI need a codev that reads French keywords: raise `codev_version` to
  this release. Existing artifacts and specs keep their keywords. To keep
  English keywords, set `language: en`, which also writes the prose in
  English.
- Added: `codev instructions --json` gives `keywordLanguage` and
  `keywords`, the language and the texts of the structural keywords to
  write; a project schema may hold one template set per language under
  `templates/<language>/`.
