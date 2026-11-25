# Design: Read structural keywords from a per-language catalog, French included

## Context

Every structural keyword is a string literal at the place that reads it:
`parser/shared.rs` (`### Requirement:`, `#### Scenario:`, the delta section
kinds), `parser/spec.rs` and `parser/delta.rs` (`Purpose`, `Requirements`,
`Reason`, `Migration`, `FROM:`, `TO:`), `validate/rules.rs`
(`contains("SHALL") || contains("MUST")`), `merge/render.rs` and
`merge/mod.rs` (the headings a merge writes), `domains.rs` and
`changelog/release_note.rs` (`## Domains`, `## Release note`,
`## What Changes`, `None`, the categories), and on the engine side
`design.rs` (`## Decisions`, `### Decision: `). The matching rules already
differ: delta section kinds are compared in upper case, `Purpose` and
`Requirements` with `eq_ignore_ascii_case`, the rest exactly.

`codev-core` is pure (decision 0001): it may embed data at compile time but
reads no file. YAML is parsed with `serde_norway` (decision 0006), already a
dependency of `codev-core`.

## Goals / Non-Goals

**Goals**

- One catalog, one key per keyword, one embedded file per language, and
  no keyword literal left in the readers listed above.
- Every reader accepts every catalog language; a merge writes in the
  language of the spec it edits.
- English documents are read and written byte for byte as today.

**Non-Goals**

- What codev and its skills write in a new artifact (templates, schema
  instructions, skill text, the ADR template): the follow-up change.
- The review record's markers (`Mode:`, `No finding.`…), which the skills
  write and codev reads as a machine record.
- A project-defined language or override file under `_codev/`.

## Decisions

### Decision: The catalog is one embedded YAML file per language

`assets/keywords/en.yaml` and `assets/keywords/fr.yaml` map each key to a
text or a list of texts (the first is the written form), for example
`requirement: "### Requirement:"` and `normative: [DOIT, DOIVENT]`. A
`keywords` module of `codev-core` embeds them with `include_str!`, parses
them once behind a `std::sync::LazyLock`, and exposes a `Key` enum and a
`Lang` value. Adding a language is adding a file and one line in the list
of embedded files. A unit test loads every file and fails on a missing key,
an unknown key, two keys whose normalized texts collide at the same
heading level, or a prefix-matched key whose text is a prefix of another
key's text at the same level — so a broken catalog never ships, and parsing at run time
cannot fail in a release build.

**Alternatives considered**:

- A Rust `match` table per language: no parsing, but a translator has to
  edit Rust, which is what the user asked to avoid.
- Files read from `_codev/` at run time: lets a project add a language,
  but puts I/O in the core (decision 0001) and makes every reader fallible;
  left as a possible extension through the engine's ports.

### Decision: One matching function per kind of keyword, shared by every reader

The module exposes the few operations the readers need:
`heading(line, key) -> Option<(Lang, rest)>` for headings, `label(line, key)`
for `**Reason**:` and `FROM:` lines, `is_normative(text)` and
`text(key, lang)` for writing. A section key (`spec.requirements`,
`delta.added`, `proposal.domains`…) matches only a whole line, so
`## Exigences` is never taken for `## Exigences AJOUTÉES`; a key that
carries a name (`requirement`, `scenario`, `design.decision`) matches as
a prefix and returns the rest as the name.

Each key keeps, for its English texts, exactly the comparison its reader
applies today: exact and case-sensitive for `### Requirement:`,
`#### Scenario:`, `### Decision: `, `FROM:` and `TO:`; upper-cased with
the `REQUIREMENTS` suffix for the delta section kinds; ASCII
case-insensitive for `Purpose`, `Requirements` and the proposal sections.
The texts of the other languages are compared after folding: lower case,
the accented Latin letters French uses (`àâäçéèêëîïôöùûüÿœæ`) folded to
their base letters, and any space, U+00A0 or U+202F before a colon
dropped. No English document is therefore read differently, which tests
pin with today's edge cases (`### requirement: x` and
`#### Scenario : y` inside a requirement body, `## ADDED   Requirements`).
The normative check stays a case-sensitive substring search, as today, over
`SHALL`, `MUST`, `DOIT` and `DOIVENT`. `section(source, key)` in
`parser/section.rs` takes a key instead of a heading string and, as today,
reads the first matching section of the file: a proposal holding a section
in two languages is read from the first one.

`check_unexpected_headings` in `parser/delta.rs` accepts a requirement or
scenario heading of any language, and its `delta_unexpected_heading`
message no longer says the keyword stays in English: it names the
requirement heading of each catalog language.

The folding table is a small hand-written map: it covers French and the
other Latin-script languages likely to come, without a new dependency.

**Alternatives considered**:

- The `unicode-normalization` crate (NFD then strip combining marks):
  complete, but a new dependency for a dozen letters.
- Exact matching only: `### Exigence:` without the French space, or
  `Scenario` without its accent, would silently stop being a heading —
  the worst failure mode for a parser.

### Decision: Specs and deltas remember the language of their keywords

The parsed `Spec` records the language of its `## Requirements` heading
and the parsed `Delta` the language of its first delta section heading,
English when none matches. `render::requirement` takes a `Lang`; the merge
passes the target spec's language when editing it, and `build_new_spec`
passes the delta's language for the title line, `## Objet`/`## Purpose`
and the requirements heading. The new spec's title line follows a
per-language pattern in the catalog (`{title} Specification`,
`Spécification {title}`). Names, descriptions and scenario bodies are
copied verbatim.

**Alternatives considered**:

- Writing in the project's `language:`: a project switching language would
  get specs with mixed headings, and the core would need the configuration.
- Keeping each requirement's own heading language: a French delta merged
  into an English spec would leave mixed headings in it.

### Decision: Release note categories keep their English name in every output

`Category::parse` goes through the catalog and returns the same enum value
for `Ajouté` as for `Added`; `Category::as_str` keeps returning the English
name, which is what the JSON contract and `CHANGELOG.md` show. Finding
codes do not change either; messages that named `SHALL or MUST` name the
four normative words.

**Alternatives considered**:

- Reporting the category as written: one changelog would show `Added` and
  `Ajouté` as two categories, and the JSON contract would change.

## Risks / Trade-offs

- [An English document read differently than before] → English texts keep
  their current comparison; unit tests pin today's English edge cases, and
  `codev validate --all` on this repository gives the same findings before
  and after the change.
- [An English spec merged differently than before] → The English path
  keeps the same output: the existing golden tests in
  `crates/codev-engine/tests/sync_golden.rs` and the merge tests must pass
  unchanged, and a test pins a byte-identical English merge.
- [A French keyword taken for another key] → The catalog test rejects
  colliding normalized texts at the same level; the spec scenario "A word
  that is no keyword stays text" is tested.
- [A team writes French keywords, a teammate runs an older codev, which
  rejects them] → The documentation tells such a project to raise
  `codev_version` in `_codev/config.yaml` to the release that ships this
  change; `codev doctor` and the skills already enforce that setting.
- [The folding table misses a letter of a future language] → Adding the
  language's file comes with its letters in the table; the catalog test
  checks every text of every file folds to ASCII.

## Migration Plan

Nothing to migrate: English specs, deltas and proposals keep working. A
project that starts writing French keywords raises `codev_version` to the
first release with this change. Rollback is reverting the change; French
documents written meanwhile would then fail validation and need their
keywords put back in English.

## Open Questions

- Should a project be able to add its own language file under `_codev/`?
  Deferred: the catalog's shape allows it later without changing the
  readers.
