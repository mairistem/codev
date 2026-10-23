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
  - status: open
    by: human:Ludovic
    note: Fix it within this change - a French and a pt-BR planning case, measured in 7.1.
  - status: fixed
    by: lead
    ref: "tasks.md 7.0 and 7.0.1; specs/planning-evaluation; proposal.md#What Changes"
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
