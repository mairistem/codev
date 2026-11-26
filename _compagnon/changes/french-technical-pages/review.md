# Review record: french-technical-pages

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The check missed « change » after a quantifier or in a heading

```yaml
severity: medium
confidence: high
location: "specs/docs/spec.md#Scenario: The French book holds no anglicism of the lexicon"
evidence: |
  - **THEN** none holds, in prose or headings and outside parentheses, the words
recommendation: 'Count « change(s) » after numerals and quantifiers and as the first word of a heading, and list the bare « change » left by hand.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md scenario on quantifiers and headings; design.md decision on the checked words; tasks.md 1.1 and 3.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Identifier headings would be flagged though they must stay

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The English word stays where the reader meets it as English"
evidence: |
  Inline and fenced code, command names, flags (`--change`), configuration
recommendation: 'Exempt headings naming a file, a lone lower-case configuration key or a compagnon command, with a unit test, keeping the headings other tests pin.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md decision on the checked words; specs/docs/spec.md scenario on identifier headings; tasks.md 1.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The reliability page's source labels come from data

```yaml
severity: medium
confidence: high
location: "design.md#Decision: `reliability.md` is rewritten at its source"
evidence: |
  labels of the token sources come from the baseline's data (`skill`,
recommendation: 'Write the source labels in inline code in the tables and skip the svg chart in the check.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md decision on reliability.md; tasks.md 1.1 and 2.5"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 « revue » and « relecture » would name the same thing

```yaml
severity: medium
confidence: high
location: "design.md#Decision: « artefact » and « delta » stay; « revue » becomes « relecture » everywhere"
evidence: |
  « revue » is French, but the glossary names
recommendation: 'Rewrite every « revue » naming Compagnon review across the whole French book and the reliability strings, links included, and list the rest by hand.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md decision on revue; tasks.md 2.4, 2.5, 3.2; specs/docs/spec.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 The book has one legitimate French « design »

```yaml
severity: low
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  the book today, « un outil de design » (`guides/mcp.md`, a tool such as
recommendation: 'Correct the claim and reword that line in task 2.3.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md risks; tasks.md 2.3"
  - status: fixed
    by: skeptic
```

### accessibility

#### R-accessibility-1 English words in parentheses get no language marking

```yaml
severity: low
confidence: low
location: "design.md#Decision: The English word stays where the reader meets it as English"
evidence: |
  The English word between parentheses is not marked with `lang="en"`: it
recommendation: 'Record a decision: mark the words lang="en", or state that the technical-term exception applies.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: "design.md decision: technical-term exception of WCAG 3.1.2 and RGAA 8.7, as the everyday pages"
  - status: fixed
    by: accessibility
```

## code.review

Mode: isolated.

### accessibility

No finding.

### newcomer

#### R-newcomer-1 shell_blocks and lines recognised fences differently

```yaml
severity: medium
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:509"
evidence: |
  pub fn shell_blocks(md: &str) -> Vec<String> {
recommendation: 'Share one fence walk between lines and shell_blocks, so a quoted fence is handled the same in both.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: step_fence used by lines and shell_blocks; a_fence_quoted_in_a_longer_fence_stays_code checks shell_blocks too"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 fence_marker returned a language still holding fence characters

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:56"
evidence: |
  fn step_fence<'a>(open: &mut Option<Fence<'a>>, trimmed: &'a str) -> bool {
recommendation: 'Return a Fence with its character, length and stripped language.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: struct Fence, fence_marker returns it"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 The change rule was split across two lists and undocumented

```yaml
severity: medium
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:955"
evidence: |
  fn is_noun_change(words: &[String], i: usize, heading: bool) -> bool {
recommendation: 'One NOUN_MARKERS list beside DETERMINERS, an is_noun_change helper, and a doc comment listing every exclusion.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: NOUN_MARKERS, is_noun_change, anglicisms doc comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 « avant lui » referred to a feminine noun

```yaml
severity: low
confidence: high
location: "evals/src/reliability.rs:1137"
evidence: |
  était avant elle, et demandent à `/compagnon-apply` de l'implémenter, relecture comprise. \
recommendation: 'Write « avant elle » in the three strings, then regenerate.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/reliability.rs: three strings; docs/fr/src/reliability.md regenerated"
  - status: fixed
    by: newcomer
```
