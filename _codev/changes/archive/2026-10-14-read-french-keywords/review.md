# Review record: read-french-keywords

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 A mixed document contradicts the main spec's "Unexpected heading" scenario

```yaml
severity: high
confidence: high
location: "specs/keyword-languages/spec.md#Requirement: A keyword is recognized in every catalog language"
evidence: |
  #### Scenario: A mixed document
  
  - **GIVEN** a delta holding `## ADDED Requirements` then
    `### Exigence : Annulation`
recommendation: >
  Modify spec-parsing's "Localized structural defects" so its "Unexpected heading inside a delta section" example uses a heading that is no keyword in any language (`### Notes`), add a French "Delta header in a main spec" case, and name in task 2.2 the change to `check_unexpected_headings` in parser/delta.rs and the existing test to update.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: 'specs/spec-parsing/spec.md — MODIFIED "Localized structural defects", tasks.md 2.2 and 2.4'
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Tolerant matching changes how English is recognized, while the spec says English is unchanged

```yaml
severity: medium
confidence: high
location: "specs/keyword-languages/spec.md#Requirement: Headings are matched with tolerance for case, accents and the space before a colon"
evidence: |
  When codev compares a heading or a labelled line with a keyword, it SHALL
  ignore letter case, SHALL treat an accented letter as its unaccented form,
  and SHALL accept the text with or without spaces before its colon. The
recommendation: >
  Apply the tolerance only to the texts of languages other than English, keep today's exact English rules, pin today's English edge cases (`### requirement: x`, `#### Scenario : x` in a requirement body, `## ADDED   Requirements`) in unit tests, and run `codev validate --all` on this repository before and after with no new finding (folded in from R-migration-1).
raised_by: [skeptic, migration]
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: 'specs/keyword-languages/spec.md — tolerance limited to non-English texts; design.md matching decision; tasks.md 1.1, 1.2, 2.2, 6.2'
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The documentation task misses the pages, and a parser message, that say keywords stay in English

```yaml
severity: medium
confidence: high
location: "tasks.md#6. Documentation and finish"
evidence: |
  - [ ] 6.1 Document the accepted keywords and the catalog in `docs/en` and `docs/fr` (concepts chapter, where the requirement format is described, and the workflow chapter's validation section), including the advice to raise `codev_version` before writing French keywords, verified by `mdbook build docs/en` and `mdbook build docs/fr` and both editions having the same chapters.
recommendation: >
  List faq.md, guides/artifact-language.md, reference/configuration.md and reference/file-formats.md in both editions with what each must now say (reading accepts every catalog language; writing stays English until the follow-up change), and add a task rewriting the `delta_unexpected_heading` message in parser/delta.rs with a named test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: 'tasks.md 2.4 and 6.1'
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 `## Exigences` is a prefix of the four French delta headings

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The catalog is one embedded YAML file per language"
evidence: |
  of embedded files. A unit test loads every file and fails on a missing key,
  an unknown key, or two keys whose normalized texts collide at the same
  heading level — so a broken catalog never ships, and parsing at run time
recommendation: >
  Match section headings as a whole line and only name-carrying headings by prefix, extend the catalog test to reject a text that is a prefix of another key's text at the same level when that key is prefix-matched, and add a French "Delta header in a main spec" case.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: 'design.md — whole-line section matching and prefix check; specs/spec-parsing/spec.md "French delta header in a main spec"'
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 Non-breaking spaces before a colon are not covered

```yaml
severity: medium
confidence: low
location: "specs/keyword-languages/spec.md#Requirement: Headings are matched with tolerance for case, accents and the space before a colon"
evidence: |
  and SHALL accept the text with or without spaces before its colon. The
recommendation: >
  State that an ASCII space, U+00A0 or U+202F before the colon is accepted, add a scenario, and cover it in the 1.2 matching test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: 'specs/keyword-languages/spec.md "No-break space before the colon"; tasks.md 1.2'
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 No case for a document holding the same section in both languages

```yaml
severity: low
confidence: high
location: "specs/validation/spec.md#Requirement: The proposal sections are checked in every catalog language"
evidence: |
  and `migration` in every language. A proposal that has the section in one
  language is not reported as missing it.
recommendation: >
  Set the expected behavior when a document holds a section in two languages, with a scenario and a named test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: 'specs/validation/spec.md "The same section in two languages"; tasks.md 1.3 and 3.2'
  - status: fixed
    by: skeptic
```

#### R-skeptic-7 The `## Changements` fallback of the changelog has no scenario and no test

```yaml
severity: low
confidence: high
location: "tasks.md#3. Validation"
evidence: |
  - [ ] 3.3 Read `## Note de version`, `## Changements`, `Aucune` and the French categories in `changelog/release_note.rs` through the catalog, `Category::as_str` still returning the English name, verified by `french_release_note_is_valid`, `an_unknown_french_category_is_reported`, `aucune_release_note_is_an_internal_change` and `french_categories_parse_to_the_english_category` (scenarios "French release note and domains", "Internal change written in French", "Unknown French category").
recommendation: >
  Add a changelog scenario for an archived French change without a release note whose `## Changements` bullets are listed under Changed, with a named test in task 5.1.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: 'specs/changelog/spec.md "A French change without a release note"; tasks.md 5.1'
  - status: fixed
    by: skeptic
```

### migration

No finding.

## code.review

Mode: isolated.
Summarized: crates/codev-core/src/changelog/release_note.rs — over the material limit.
Summarized: crates/codev-core/src/keywords.rs — over the material limit.
Summarized: crates/codev-core/src/merge/mod.rs — over the material limit.
Summarized: crates/codev-core/src/parser/delta.rs — over the material limit.
Summarized: crates/codev-core/src/validate/rules.rs — over the material limit.
Summarized: crates/codev-engine/src/validate/mod.rs — over the material limit.
Summarized: docs/en/src/reference/configuration.md — over the material limit.
Summarized: docs/en/src/reference/file-formats.md — over the material limit.
Summarized: docs/fr/src/faq.md — over the material limit.
Summarized: docs/fr/src/reference/configuration.md — over the material limit.
Summarized: docs/fr/src/reference/file-formats.md — over the material limit.

### migration

No finding.

### newcomer

#### R-newcomer-1 `EnRule` claims to drive the English comparisons, but most matchers ignore it

```yaml
severity: medium
confidence: high
location: "crates/codev-core/src/keywords.rs:157-168"
evidence: |
      fn en_rule(self) -> EnRule {
          match self {
              Key::Requirement | Key::Scenario => EnRule::ExactPrefix,
recommendation: >
  The module doc and `en_rule` suggest this table decides how each English keyword is matched, but `label` never reads it, `is_normative` does its own search, and several variants only reach `_` arms: changing `RenamedFrom => IgnoreAsciiCase` would change nothing. Make every matcher go through `en_rule`, with an exhaustive match (no `_` arm), and say in the module doc which function applies which rule.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'crates/codev-core/src/keywords.rs — en_rule exhaustive, every matcher matches on it without a catch-all, module doc maps functions to rules'
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `unexpected_headings` keeps an unexplained literal check and builds its message on every line

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/parser/delta.rs:192-196"
evidence: |
          let is_requirement =
              title.starts_with("Requirement:") || requirement_header(entry.text).is_some();
recommendation: >
  Say in a comment that the literal prefixes keep the pre-catalog tolerance, and build the expected-heading message only when a finding is emitted.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'crates/codev-core/src/parser/delta.rs — comment on the literal prefixes, message built only for a finding'
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 `delta_section` maps every other key to `Renamed` with a catch-all

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/parser/shared.rs:64-69"
evidence: |
          Key::DeltaRemoved => DeltaKind::Removed,
          _ => DeltaKind::Renamed,
recommendation: >
  Write `Key::DeltaRenamed => DeltaKind::Renamed` and end with an `unreachable!` stating that `delta_title` only returns the delta keys.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'crates/codev-core/src/parser/shared.rs — explicit Renamed arm and unreachable!'
  - status: fixed
    by: newcomer
```

