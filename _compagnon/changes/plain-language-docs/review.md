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

## code.review

Mode: isolated.

### accessibility

#### R-accessibility-2 In the offline manual, a repeated heading sends its links to the wrong section

```yaml
severity: medium
confidence: high
location: "docs/en/src/workflow.md:14"
evidence: |
  (see [Archive](#archive); [Sync](#sync) can update the specs earlier).
recommendation: 'The offline manual puts every chapter on one page with one slugger: the glossary''s "Archive" takes `archive`, the workflow''s gets `archive-1`, and plain or cross-chapter fragments were left as written, so "see Archive" opened the glossary entry (WCAG 2.4.4). Map each chapter''s own anchors to those of the single page, and check that every in-page link of the manual lands on a heading.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: "crates/compagnon-cli/src/docs.rs: ChapterAnchors maps each chapter's mdBook anchors to the single page's, used for `#x` and `other.md#x`; tests a_heading_repeated_across_chapters_keeps_its_links (fails without the fix) and the_offline_manual_links_land_on_their_heading"
  - status: fixed
    by: accessibility
```

#### R-accessibility-3 "CLI" appeared on the workflow page without being spelled out

```yaml
severity: low
confidence: high
location: "docs/en/src/workflow.md:18"
evidence: |
  you can type in a [terminal](glossary.md#terminal) (the command-line tool), which does the
recommendation: 'Spell the abbreviation out or drop it (WCAG 3.1.4), in both editions.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: "docs/en/src/workflow.md and docs/fr/src/workflow.md: (the command-line tool) / (l'outil en ligne de commande)"
  - status: fixed
    by: accessibility
```

#### R-accessibility-4 "QA analyst" was never spelled out in English

```yaml
severity: low
confidence: high
location: "docs/en/src/skills.md:240"
evidence: |
  - the quality (QA) analyst writes the specs;
recommendation: 'Expand QA the first time on each page, as the French edition does.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: "docs/en/src/skills.md, concepts.md, workflow.md: quality (QA) analyst"
  - status: fixed
    by: accessibility
```

### newcomer

#### R-newcomer-1 The vocabulary check reported a phrase its name did not announce

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:569"
evidence: |
  pub fn vocabulary_problems(md: &str, lang: Lang, anchors: &[String]) -> Vec<String> {
recommendation: 'Rename the function for what it reports, extract the glossary-link test into a named helper, and say why offsets in the blanked text match the link ranges.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: unexplained_terms renamed vocabulary_problems, links_to_glossary_entry, comment on the offsets"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 A bare boolean chose whether headings count

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:446"
evidence: |
  fn prose_paragraphs(md: &str, headings: Headings) -> Vec<String> {
recommendation: 'Replace the boolean with a small enum and say at each call why headings are in or out.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: enum Headings { Keep, Skip }, comments at both calls"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 The word scanner closed its last word with an uncommented sentinel

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:500"
evidence: |
      // A trailing space closes the last word; a hyphen is kept inside a word
recommendation: 'Comment the trailing sentinel and the hyphen handling.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs words: comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 The determiners list held adjectives and elided articles

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:612"
evidence: |
  const DETERMINERS: &[&str] = &[
recommendation: 'Say in the comment that the list also holds adjectives before a noun and the elided d'' and l''.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: DETERMINERS doc comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 The test for the verb « changer » used a made-up phrase

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:871"
evidence: |
      fn the_french_verb_changer_is_not_an_anglicism() {
recommendation: 'Use real French examples and a contrasting noun case.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: « Voici ce qui change. », « Rien ne change. », « Relisez ce change. » → change"
  - status: fixed
    by: newcomer
```

#### R-newcomer-6 The everyday chapter list was copied by hand

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:1173"
evidence: |
      const EVERYDAY: [&str; 10] = [
recommendation: 'Tie the list to SUMMARY.md, so that a chapter added to those parts is not left unchecked.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: test the_everyday_list_follows_the_summary"
  - status: fixed
    by: newcomer
```

#### R-newcomer-7 Two hand-written link scanners

```yaml
severity: low
confidence: low
location: "crates/compagnon-cli/src/docs_checks.rs:523"
evidence: |
  fn links(text: &str) -> Vec<(std::ops::Range<usize>, String)> {
recommendation: 'Reuse one scanner, or say what the simple scanner assumes.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs links: doc comment states its single-level assumption"
  - status: fixed
    by: newcomer
```
