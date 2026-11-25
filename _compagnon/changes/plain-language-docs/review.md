# Review record: plain-language-docs

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The anglicism check would flag the French verb « change »

```yaml
severity: medium
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  - [The anglicism check flags a legitimate French word] →
recommendation: '"change" is also the present tense of "changer" ("ce qui change"), used today in introduction.md, glossary.md, workflow.md and concepts.md; a whole-word match would flag correct French. Match the noun only, and add a scenario and a test showing "ce qui change" passes.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md (noun only, after a determiner; risk restated); specs/docs/spec.md scenario The French verb « changer » is not an anglicism; tasks.md 1.1 the_french_verb_changer_is_not_an_anglicism"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Task 1.2 plans failing tests against the shared definition of done

```yaml
severity: medium
confidence: high
location: "tasks.md#1. The checks, on synthetic chapters"
evidence: |
  Every task's definition of done also includes `cargo fmt --all --check`,
recommendation: 'The tests on the real chapters fail from 1.2 until 2.6 while every task requires cargo test to pass. State the exception, or add each assertion with the chapters it covers.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md definition of done: the three chapter-wide tests are red from 1.2 to 2.6, each task verified by them reporting no problem naming its own chapters"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 Nobody owns the French requests of the skills' descriptions

```yaml
severity: medium
confidence: high
location: "proposal.md#Impact"
evidence: |
  active change `plain-words-in-skills` (« archive le change »), follow
recommendation: 'Correction C-2 covers the French example requests, but plain-words-in-skills never mentions the lexicon. Revise that change so it adopts the lexicon with a check, or take the work here, and keep the Non-Goals consistent.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "plain-words-in-skills: specs/skills/spec.md (French requests follow the lexicon, scenario A French request holding an anglicism is caught), tasks.md 2.4; design.md Non-Goals here point to it"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The French term list keeps "workflow" and leaves out « déroulé »

```yaml
severity: medium
confidence: high
location: "specs/docs/spec.md#Requirement: The everyday pages explain the technical terms they use"
evidence: |
  The listed terms are, in English, delta, artifact, capability, schema,
recommendation: 'The lexicon forbids "workflow" in French prose, so linking its first use is impossible, and « déroulé » is never linked. Put « déroulé » in the French list.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md French list (déroulé); tasks.md 1.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 The anglicism check accepts any text in parentheses

```yaml
severity: low
confidence: high
location: "design.md#Decision: Two pure checks on prose, reusing the chapter walker"
evidence: |
  - `anglicisms(md)`: the words of the lexicon's English side, matched as
recommendation: 'The requirement said "only between parentheses right after its French equivalent"; the check skips all parenthesized text. Align the two.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md: the English word may appear only between parentheses, placed after its French equivalent — what the check enforces is the parentheses"
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 An existing test requires "specs" in the French review guide

```yaml
severity: low
confidence: high
location: "tasks.md#2. The rewrite, chapter by chapter, both editions"
evidence: |
  `review_a_plan_covers_the_plan_without_a_terminal` looks for (« specs »
recommendation: 'Change that test''s French word to « spécification » in task 2.3.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md 2.3"
  - status: fixed
    by: skeptic
```

### accessibility

#### R-accessibility-1 A link to the top of the glossary would pass

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Two pure checks on prose, reusing the chapter walker"
evidence: |
  must sit in the text of a link to the term's own entry,
recommendation: 'Require the link to point to the term''s own entry (glossary.md#<anchor> an existing heading produces): otherwise a keyboard or screen-reader user lands at the top of a glossary of about 23 entries (WCAG 2.4.4).'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: "design.md (anchor of the term's entry, as mdBook produces it); specs/docs/spec.md scenario A link to the top of the glossary is caught; tasks.md 1.1"
  - status: fixed
    by: accessibility
```
