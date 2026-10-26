# Design: Write a change's artifacts entirely in the project's language

## Context

The reading side is done: `codev-core::keywords` holds the catalog (`en`,
`fr`), every parser recognizes the keywords of both languages, and a merge
writes a main spec in its own language. The writing side is not:

- `crates/codev-engine/src/schemas.rs` embeds one template per artifact,
  in English, and reads a project schema's template from
  `templates/<file>`;
- `instructions.rs` and `contract.rs` document `language` as "prose
  only", and `render.rs` prints `Language: fr — prose only; template
  keywords stay in English`;
- `assets/workflows/propose.md` (step 4.d and its guardrails) and
  `update.md` tell the agent the keywords stay in English;
- the comment that `codev init` writes above `language:` says the same
  (`crates/codev-core/src/config/mod.rs`).

The schema's instructions (`schema.yaml`) name the keywords in English
(`## ADDED Requirements`, `SHALL`…). They are written for the agent, like
the skills, and stay in English.

## Goals / Non-Goals

**Goals**: a project with `language: fr` gets new artifacts written
entirely in French; a project with `language: en` (or none) gets exactly
what it gets today; a language the catalog does not ship keeps English
keywords with its own prose.

**Non-goals**: translating the review record, `corrections.md`,
`change.yaml` or the skills themselves; rewriting existing artifacts or
specs in another language; a third catalog language.

## Decisions

### Decision: One template file per language, not a template rendered from the catalog

The French templates are separate files under
`assets/schemas/spec-driven/templates/fr/`, and the English ones move to
`templates/en/` unchanged. A template holds more than catalog keywords:
headings codev never parses (`## Why`, `## Capabilities`, `## Impact`,
`## Context`…), guidance comments and the `**WHEN**` / `**THEN**` lines.
Rendering one template from the catalog would need keys for all of them in
the catalog, whose purpose is what codev reads (decision of
`read-french-keywords`), or leave English guidance in a French file.

A test keeps the files honest: for each language, every structural line of
each embedded template is recognized by `keywords` as a key of that
language and equals, byte for byte, the key's first text in that language
(recognition alone is lenient on case, accents and spaces); the proposal
templates' `## Domains` list parses with `domains::parse`; and the English
templates are byte-identical to today's.

**Alternatives considered**: placeholders such as `{{requirement}}`
substituted from the catalog — the untranslated headings and comments
would still need a per-language source, so two mechanisms instead of one.

### Decision: The keyword language is resolved in the core, from the primary subtag

`keywords::Lang::for_setting(language: &str) -> Lang` takes the primary
subtag of `language:`, compares it with the catalog codes without regard
to case, and falls back to `Lang::EN`. It is pure and lives next to the
catalog (decision 0001). `codev-engine::instructions` calls it once and
carries `keyword_language` with the instructions; `schemas::template`
takes that language: the built-in schema picks `templates/<lang>/<file>`
among the embedded files, a project schema tries
`templates/<lang>/<file>`, then `templates/<file>` (today's layout).

**Alternatives considered**: the full tag (`fr-CA`) — no catalog will
ship regional keywords, and `fr-CA` users write `### Exigence :`.

### Decision: The instructions carry the keywords, so the skills never translate

`InstructionsV1` gains `keywordLanguage` and `keywords` (additive fields of
`contract::v1`): `keywords` maps each catalog key to the text codev writes
in that language, `normative` and `none` to the list of their texts. A
template covers the headings it shows, but not, for example, a
`## Exigences MODIFIÉES` section the spec template only mentions in a
comment, or a `**Raison**` label: the skill reads them in `keywords`
instead of translating. The skills (`propose.md` step 4.d and guardrail,
`update.md`) are rewritten to: "write the structural keywords exactly as
`template` and `keywords` give them, in `keywordLanguage`; the schema's
instruction names them in English".

`codev-agents` renders the workflows unchanged; the installed skills of
this repository are refreshed with `codev update`, whose checksum rule
(change `fix-merge-and-skill-refresh`) rewrites them.

### Decision: Evaluation per decision 0008, on the steps the suite covers

The change touches what the planning and update agents read. Before the
merge, `codev-evals compare` runs the `plan` and `update` steps against
the committed baseline: the cases are English projects, whose templates
are byte-identical, so the comparison measures the skills' new wording;
quality must stay within the baseline's spread. The French path is
verified without a model: an integration test runs `codev instructions`
on a `language: fr` fixture, and the French templates, filled with a
sample change, pass `codev validate` and merge with `codev archive`.

**Alternatives considered**: a French planning case in the suite before
this change — decision 0008 asks for a step to be covered first; the
planning step is covered, its French variant is not a separate step. It
is recorded as a follow-up under Risks.

## Risks / Trade-offs

- [Existing French projects change behavior] → A project with
  `language: fr` today gets English-structured specs; its next changes
  will be French-structured. Two kinds of mixing follow, both valid since
  every parser reads both languages and a document may mix them
  (keyword-languages):
  - a change already planned before the upgrade: `/codev-update` keeps the
    keyword language of the artifact it revises, so its files stay
    English-structured; only a new artifact takes `keywordLanguage`;
  - a main spec: a merge writes the headings in the spec's own language,
    but copies the requirement bodies verbatim (spec-merge), so a French
    `DOIT` or `**QUAND**` lands under an English `### Requirement:`. The
    artifact-language guide says so, and how to avoid it: keep
    `language: en`, or rewrite a main spec's keywords once in French —
    codev never rewrites a spec on its own.
  The release note says it. There is no separate setting for the keyword
  language: a team that wants English keywords keeps `language: en`, a
  deliberate choice to keep one setting.
- [A French template drifts from the English one] → the template test
  checks every keyword line; a guidance comment can still drift, which a
  reviewer catches in the diff of `templates/`.
- [The agent still writes English keywords out of habit] → the skill
  text and the template agree, `codev validate` accepts both languages,
  so a slip is cosmetic, never a broken artifact.
- [No model evaluation of French planning] → a planning case may now
  append a configuration and expect the language of the keywords: a
  French case and a `pt-BR` case join the suite and its baseline, and
  `compare` fails when plans slip back to English keywords (decided by
  the author on R-skeptic-2).

## Migration Plan

Additive JSON fields, embedded templates, skills refreshed by
`codev update`. Rollback: revert the commit; artifacts already written in
French stay readable by any codev since `read-french-keywords`.
