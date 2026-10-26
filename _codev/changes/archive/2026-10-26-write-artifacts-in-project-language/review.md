# Review record: write-artifacts-in-project-language

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 "Mixed files cannot occur" is false, and the plan builds on it

```yaml
severity: medium
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  - [Existing French projects change behavior] → A project with
    `language: fr` today gets English-structured specs; its next changes
    will be French-structured. Mixed files cannot occur: a merge writes a
    main spec in that spec's own language (spec-merge), and every parser
    reads both.
recommendation: >
  A merge copies requirement bodies verbatim (spec-merge, "A merge writes
  keywords in the language of the spec it edits"), so the first French
  change archived into an existing English main spec puts `DOIT` and
  `**QUAND**` under `### Requirement:`; and `/codev-update` on a change
  planned before the upgrade would write French keywords into its English
  artifacts (raised by migration as well: the release note's "Existing
  artifacts and specs keep their keywords" is wrong for changes in
  progress). Correct the risk; specify that `/codev-update` keeps the
  keyword language of the artifact it revises, with a scenario and a test;
  state the mixed main spec in the design and the artifact-language guide,
  with how to avoid it.
raised_by: [skeptic, migration]
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: 'design.md — Risks rewritten; specs/skills/spec.md — update keeps the artifact''s keyword language, scenario "An artifact already written keeps its keywords"; tasks.md 4.1 and 6.1'
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Nothing checks the behavior the Why asks for: the French plan is deferred, not automated

```yaml
severity: medium
confidence: high
location: "tasks.md#4. The skills and the generated configuration"
evidence: |
  `evals/cases/`, deferred to its own change (design, Risks), and are covered meanwhile by this test and by 5.1
recommendation: >
  The project rule allows a manual check only when automation is
  impossible, and deferring is not impossibility. The suite already hosts
  25 planning cases, among them `plan-requirement-spanish`, and a case
  carries its own configuration. Add a French planning case and a `pt-BR`
  one to `evals/cases/` — first, through their own change, as decision
  0008 asks for a step not covered, or within this change — and include
  them in the comparison of task 7.1; failing that, a person accepts the
  gap with a reason.
history:
  - status: open
    by: skeptic
  - status: disputed
    by: lead
    note: The planning step is covered by the suite, and 7.1 compares it; the French variant is a new case, which costs about 1.65 USD per run and per repetition, and decision 0008 sends a step not covered to its own change first. Adding it here, adding it first, or accepting the gap is a choice of cost and order for the person, not an evident fix.
  - status: disputed
    by: skeptic
    note: The finding holds — the Why is French output, and no automated check shows an agent writing it; only a person can choose between the three ways out.
  - status: fixed
    by: lead
    ref: "tasks.md 7.0 and 7.0.1; specs/planning-evaluation; proposal.md#What Changes"
    note: The author (human:Ludovic) chose to fix it within this change - a French and a pt-BR planning case, measured in 7.1.
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The template test checks that keywords are recognized, not that they match the first text the spec requires

```yaml
severity: low
confidence: high
location: "design.md#Decision: One template file per language, not a template rendered from the catalog"
evidence: |
  A test keeps the files honest: for each language, every structural line of
  each embedded template is recognized by `keywords` as the text of its key
  in that language, and the English templates are byte-identical to today's.
recommendation: >
  Recognition folds case, accents and the space before a colon, so a
  template with `### Exigence:` would pass while the spec requires the
  first text. Assert byte equality with the key's first text for each
  structural line.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: 'design.md — the template test asserts byte equality with the first text; tasks.md 2.1'
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The French proposal template does not say that domain ids stay codev's vocabulary

```yaml
severity: low
confidence: low
location: "specs/instructions/spec.md#Requirement: The template is served in the project's keyword language"
evidence: |
  catalog keyword it holds with that language's first text, and its other
  headings, guidance comments and the `**WHEN**` / `**THEN**` scenario lines
  in that language.
recommendation: >
  Domain names are identifiers, not catalog keys: a translated
  `- [ ] sécurité` is an `unknown_domain` error. Say that domain names stay
  as codev defines them, and run `domains::parse` on each proposal
  template in the template test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: 'specs/instructions/spec.md — identifiers stay as codev defines them; tasks.md 2.1 — proposal_templates_list_the_domains_codev_knows'
  - status: fixed
    by: skeptic
```

### migration

No finding.

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/workflows.rs, crates/codev-cli/src/{commands,contract,docs,main,render}.rs, crates/codev-core/src/config/mod.rs, crates/codev-core/src/keywords.rs, crates/codev-engine/src/{config,instructions}.rs, docs/en/src/reference/{cli,file-formats,json-output}.md, docs/fr/src/reference/{cli,configuration,file-formats,json-output}.md, docs/fr/src/skills.md, evals/src/{case,compare,plan,report,runner,score}.rs, evals/tests/suite.rs — too large to read whole; each diff is under two hundred lines, so the reviewers got it verbatim from `git diff`, not a summary.

### migration

#### R-migration-1 A project schema with a single English template gets French keywords

```yaml
severity: medium
confidence: high
location: "crates/codev-engine/src/schemas.rs:175-180"
evidence: |
  let per_language = templates.join(lang.code()).join(file);
  let path = if fs.exists(&per_language) {
      per_language
  } else {
      templates.join(file)
  };
recommendation: >
  A project with `language: fr` and a schema written before this change,
  with English templates at `templates/<file>`, got `keywordLanguage: fr`
  and French `keywords` beside an English template: the skill, told to
  write keywords as `template` and `keywords` give them, mixes the two.
  Set `keywordLanguage` from the template actually served, and document it.
history:
  - status: open
    by: migration
  - status: fixed
    by: developer
    ref: 'fdab6da — keywordLanguage is the language of the served template''s first keyword heading (codev_core::keywords::text_language), else the setting''s; test a_single_english_template_keeps_english_keywords; spec, json-output, artifact-language guide and propose skill updated'
  - status: fixed
    by: migration
```

### newcomer

#### R-newcomer-1 `with_appended_config` ends a replaced block on an unindented list item

```yaml
severity: medium
confidence: high
location: "evals/src/runner.rs:333-348"
evidence: |
  None => skipping = false,
recommendation: >
  The last arm ends the skip on any comment or unindented `- item`, so a
  fixture's `workflows:\n- propose` replaced by the case keeps `- propose`
  without its key, and the YAML breaks. Let a key's block run to the next
  top-level key, and say so in the doc comment.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'fdab6da — only a top-level key changes the state; test appended_config_replaces_a_key_the_fixture_already_sets covers an unindented list'
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `keywords_v1` repeats the rule `written` already applied

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/contract.rs:902-917"
evidence: |
  let value = if matches!(key, Key::Normative | Key::None) {
recommendation: Give `Key` an `is_choice()` that both `written` and `keywords_v1` call.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'fdab6da — Key::is_choice'
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 The doc comment of `keyword_of` promises a third value the function does not return

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/schemas.rs:314-316"
evidence: |
  /// written in, and the text the catalog has for it there.
recommendation: Drop the last clause; bind `line.trim_start_matches("- ")` once; say why `tasks.md` is skipped.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'fdab6da'
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 The keyword check is an inline nested loop inside `score_plan`

```yaml
severity: low
confidence: high
location: "evals/src/plan.rs:165-187"
evidence: |
  let keywords_off = case.keyword_language.as_deref().map(|code| {
recommendation: Move it into a function, name the checked artifacts in a constant saying tasks hold no keyword, rename `rest` to `path`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'fdab6da — keyword_lines_off, KEYWORD_ARTIFACTS'
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 The test named for the `review:` refusal also checks two unrelated refusals

```yaml
severity: low
confidence: high
location: "evals/src/case.rs:2356-2372"
evidence: |
  fn a_case_config_setting_review_is_refused() {
recommendation: Split it into three tests named for each behavior.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'fdab6da — a_case_config_that_is_not_a_mapping_is_refused, a_keyword_language_outside_the_catalog_is_refused'
  - status: fixed
    by: newcomer
```

#### R-newcomer-6 `Lang::for_setting` takes an `Option` that every caller fills with `Some`

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/keywords.rs:55-58"
evidence: |
  pub fn for_setting(language: Option<&str>) -> Lang {
recommendation: Take `&str` and drop the `let … else`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'fdab6da'
  - status: fixed
    by: newcomer
```

#### R-newcomer-7 In `render::instructions`, `keywords` names a language code

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/render.rs:404-415"
evidence: |
  let keywords = instructions.keyword_language.code();
recommendation: Rename it `keyword_code` and bind `prose` so both messages use named placeholders.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'fdab6da'
  - status: fixed
    by: newcomer
```
