# Review record: accessible-docs

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The quickstart cap is looser than the quickstart it is meant to shorten

```yaml
severity: medium
confidence: high
location: "specs/docs/spec.md#Requirement: The Start here path stays short and plain"
evidence: |
  the introduction SHALL hold at most 450 words in English and 560 in
recommendation: 'Set the quickstart cap clearly below today''s prose length.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md: quickstart capped at 500/625; design.md pure checks decision"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Website behaviour of Under the hood blocks rests on unjustified manual checks

```yaml
severity: medium
confidence: high
location: "tasks.md#2. The Start here path"
evidence: |
    `compagnon docs --write`, opened and closed with Tab, Enter and Space,
recommendation: 'Automate or justify.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "design.md (rendering shared with the manual test, native control); tasks.md 2.6 and 3.2 state why the remaining checks are manual"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 Task 2.4 cannot be checked for its linking half

```yaml
severity: medium
confidence: high
location: "tasks.md#2. The Start here path"
evidence: |
  - [ ] 2.4 Write `glossary.md` in both languages, one `## <Term>` section
recommendation: 'Add a requirement and test or drop the linking.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md 2.4: linking dropped"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The design misses two tests the rewrite will break

```yaml
severity: low
confidence: high
location: "design.md#Context"
evidence: |
  structural keywords in" (`both_languages_document_the_keyword_language`);
recommendation: 'List both constraints and name the tests in tasks 2.1 and 2.3.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md Context; tasks.md 2.1 and 2.3"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 The release note says nothing was removed, but the design cuts Why Compagnon

```yaml
severity: low
confidence: high
location: "design.md#Decision: Regroup in place, in five parts"
evidence: |
  (compagnonnage). "Why Compagnon", "Origins" and "Built with Compagnon" move,
recommendation: 'Say where Why Compagnon goes.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md and proposal.md: Why Compagnon moves whole to origins.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 Choose your path has no length limit

```yaml
severity: low
confidence: high
location: "specs/docs/spec.md#Requirement: The Start here path stays short and plain"
evidence: |
  French, Choose your path at most 400 and 500, and the quickstart at most
recommendation: 'Cap it or say why not.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md: Choose your path capped at 400/500"
  - status: fixed
    by: skeptic
```

#### R-skeptic-7 The terminal-free requirement cannot tell commands from examples

```yaml
severity: low
confidence: low
location: "specs/docs/spec.md#Requirement: Non-developers can read and approve a plan without the terminal"
evidence: |
  It MUST hold no fenced block in a shell language (`bash`, `sh`, `shell`,
recommendation: 'Forbid shell languages only.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md: shell languages only, markdown/text quotes allowed"
  - status: fixed
    by: skeptic
```

### accessibility

#### R-accessibility-1 Every collapsible block has the same name

```yaml
severity: low
confidence: high
location: "specs/docs/spec.md#Requirement: Under the hood blocks fold away in every rendering"
evidence: |
  closed by default (no `open` attribute), whose `<summary>` starts with
recommendation: 'Summary starts with Under the hood and names the step; test the prefix.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md (summary prefix, no repeated name, scenario); design.md; tasks.md 1.1"
  - status: fixed
    by: accessibility
```

#### R-accessibility-2 Details styles must keep state marker and focus outline visible

```yaml
severity: low
confidence: low
location: "tasks.md#2. The Start here path"
evidence: |
    `crates/compagnon-cli/assets/docs.css`: keep the native open/closed
recommendation: 'Keep the marker and focus outline, check contrast, Space/Enter, both stylesheets.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: lead
    ref: "tasks.md 2.6 styling constraints and manual check in both renderings"
  - status: fixed
    by: accessibility
```

## code.review

Mode: isolated.
Summarized: crates/compagnon-cli/src/docs.rs — beyond the material limit (484 637 bytes for 163 840).
Summarized: crates/compagnon-cli/src/docs_checks.rs — beyond the material limit.
Summarized: crates/compagnon-cli/src/main.rs — beyond the material limit.
Summarized: docs/assets/compagnon-logo.svg — beyond the material limit.
Summarized: docs/en/src/reference/cli.md — beyond the material limit.
Summarized: docs/fr/src/reference/cli.md — beyond the material limit.
Summarized: docs/site/index.html — beyond the material limit.
Summarized: docs/theme/mairistem.css — beyond the material limit.

### accessibility

#### R-accessibility-3 The README logo's alt text omits the "by Mairistem" byline

```yaml
severity: low
confidence: high
location: "README.md:4"
evidence: |
  <img src="docs/assets/compagnon-logo.svg"
recommendation: 'The logo now reads "Compagnon by Mairistem" but the README img kept alt="Compagnon": a screen reader misses the byline (WCAG 1.1.1, image of text). Set alt="Compagnon by Mairistem" in README.md and README.fr.md and make the test require it.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: "README.md:4, README.fr.md:4, crates/compagnon-cli/tests/repository.rs both_readmes_show_the_light_and_dark_mark"
  - status: fixed
    by: accessibility
```

### newcomer

#### R-newcomer-1 shell_blocks has its own fence parser that disagrees with lines()

```yaml
severity: medium
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:241"
evidence: |
  pub fn shell_blocks(md: &str) -> Vec<String> {
recommendation: 'shell_blocks only knew ``` fences while lines() also knows ~~~: a ~~~bash block in guides/review-a-plan.md passed review_a_plan_covers_the_plan_without_a_terminal. Share one definition of a fence line.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs fence_marker, used by lines() and shell_blocks; ~~~sh case added to a_shell_block_in_review_a_plan_is_caught (fails without the fix)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The sections doc comment names a fixed level the function does not have

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:164"
evidence: |
  fn sections(md: &str, level: &str) -> Vec<(String, String)> {
recommendation: 'The comment said "## sections" while how_to_ask_problems calls it with "### ". Describe the level parameter.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs sections doc comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 The test named "four skills" checks three skill steps

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:664"
evidence: |
  fn the_quickstart_walks_the_four_skills_in_order() {
recommendation: 'The fourth step, reading the plan, hides in a bare p + 1 < a. Name the condition and say what the step between is.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: comment and reading_the_plan_comes_between binding; the name stays the spec scenario's, as every test of the change"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 parts[4].1 hides which part and which field are meant

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:587"
evidence: |
  for c in ["origins.md", "architecture.md", "reliability.md"] {
recommendation: 'Bind the Contributing part chapters to a named variable before the loop.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: let (_, contributing) = &parts[4] with a comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 readme_words is a third word counter with its own rules

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/tests/repository.rs:360"
evidence: |
  fn readme_words(text: &str) -> usize {
recommendation: 'Say why it differs from prose_words (the integration test cannot reach it; it skips every HTML line), accept ~~~ fences, and drop the state mutated inside filter for a plain loop.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "repository.rs readme_words: doc comment naming the differences, plain for loop, ~~~ fences"
  - status: fixed
    by: newcomer
```
