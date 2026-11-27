# Review record: profile-paths

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The build path's ban on commands contradicted its own steps and was not checked

```yaml
severity: medium
confidence: high
location: "specs/docs/spec.md#Requirement: Each profile has a short path page"
evidence: |
  take, as a step, a chapter whose fenced shell blocks outside "Under the
recommendation: 'The rule banned any visible command, yet the quickstart, a step of the build path, shows `/compagnon-…` shortcuts and the skills chapter in its Next asks to run `compagnon update`. Word the rule as what it means — fenced shell blocks on the step chapters — drop developer chapters from its Next, and test it.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md (rule on step chapters' shell blocks, scenario The build path asks no command beyond installing); design.md table (Next: Glossary, FAQ); tasks.md 1.2, 2.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The same labels in the introduction and the READMEs were not checked

```yaml
severity: medium
confidence: high
location: "specs/docs/spec.md#Requirement: Choose your path asks one question with four answers"
evidence: |
  SHALL be used by the introduction, the READMEs and the home pages of the
recommendation: 'Check the labels, not only the links, in the READMEs, and add a scenario and a test for the introduction.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md scenarios The introduction uses the four labels and the README labels; tasks.md 1.2 the_introduction_links_the_four_paths, 2.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The French home page was outside the mark and naming checks

```yaml
severity: medium
confidence: high
location: "tasks.md#2. The pages"
evidence: |
  - [ ] 2.4 The home pages: `docs/site/index.html` leads with the question
recommendation: 'Both cited tests read only index.html; extend the mark requirement and both tests to fr.html.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "specs/docs/spec.md MODIFIED Compagnon's mark identifies the product in every documentation surface (fr.html); tasks.md 1.2 extends inline_marks_are_named_or_decorative and site_and_introductions_name_compagnon"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The home pages' cross-links and contributor link had no scenario

```yaml
severity: medium
confidence: high
location: "specs/docs/spec.md#Requirement: The home page asks the question first, in both languages"
evidence: |
  its edition, and the link for contributors; these links are plain
recommendation: 'If index.html forgot fr.html, the French page would be unreachable with every test green: check the cross-links, the contributor link and an error case.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md home-page scenarios (contributor link, lang/hreflang cross-links, A home page that asks after installing is caught); tasks.md 1.1, 1.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 Removing the five-profile check stopped checking that links resolve

```yaml
severity: low
confidence: high
location: "specs/docs/spec.md#Requirement: Each profile has a short path page"
evidence: |
  #### Scenario: Every link of the paths resolves
recommendation: 'Keep a check that every link of Choose your path and the path pages resolves, anchors included.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md scenario Every link of the paths resolves; tasks.md 1.2 every_link_of_the_paths_resolves"
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 The reader test had no clear end

```yaml
severity: low
confidence: low
location: "tasks.md#3. Check"
evidence: |
  - [ ] 3.1 Have one reader per profile — the four profiles and a
recommendation: 'Say what passing means and send wishes for new pages to the following change.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md 3.1: passes when each reader picks the intended profile first try; new pages recorded for the following change"
  - status: fixed
    by: skeptic
```

### accessibility

#### R-accessibility-1 The path pages' titles were not set, so a link could differ from the page it opens

```yaml
severity: medium
confidence: high
location: "specs/docs/spec.md#Requirement: Each profile has a short path page"
evidence: |
  Each path page SHALL be titled with its profile's label, word for word —
recommendation: 'Title each path page and its SUMMARY entry with its profile label, and check it (WCAG 2.4.4, 2.4.6, 3.2.4).'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md path page title rule and scenario; design.md checks; tasks.md 2.1"
  - status: fixed
    by: accessibility
```

#### R-accessibility-2 The French home page's language and link back were not specified

```yaml
severity: low
confidence: high
location: "specs/docs/spec.md#Requirement: The home page asks the question first, in both languages"
evidence: |
  French, `docs/site/fr.html` (`lang="fr"`), each linking the other with the
recommendation: 'Require lang="fr", lang/hreflang on the cross-links, and point the English page''s French links to fr.html (WCAG 3.1.1, 3.1.2).'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md home-page requirement and scenario; tasks.md 1.2"
  - status: fixed
    by: accessibility
```

## code.review

Mode: isolated.

### accessibility

#### R-accessibility-3 The French home page declared two English alternates and none for itself

```yaml
severity: low
confidence: high
location: "docs/site/fr.html:17"
evidence: |
  <link rel="alternate" hreflang="fr" href="fr.html">
recommendation: 'fr.html kept index.html''s alternate to the English docs beside the English home page, both hreflang="en", and none for French: keep one alternate per language, the two home pages, on both pages, and check it.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: "docs/site/index.html and fr.html: alternates en → ./, fr → fr.html; repository.rs both_home_pages_lead_with_the_four_profiles checks them"
  - status: fixed
    by: accessibility
```

#### R-accessibility-4 A path's first step named the introduction differently from its heading

```yaml
severity: low
confidence: high
location: "docs/en/src/introduction.md:1"
evidence: |
  # Compagnon in two minutes
recommendation: 'The step read "Compagnon in two minutes" while the page''s heading was "Compagnon", and the check compared against the sidebar, not the heading: retitle the introduction and check step text against the heading, as the design says.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: "introduction.md titled Compagnon in two minutes / Compagnon en deux minutes; docs_checks chapter_titles reads each chapter's # heading; repository.rs accepts the longer title"
  - status: fixed
    by: accessibility
```

### newcomer

#### R-newcomer-1 The link-blanking function's name and comment said less than it does

```yaml
severity: medium
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:738"
evidence: |
  fn prose_without_links(text: &str) -> String {
recommendation: 'It blanks the text of every link but a glossary one, not only targets: rename it and say so.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: without_link_targets renamed prose_without_links, comment states the condition"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The profile-link check exists twice and the comment pointed at the wrong copy

```yaml
severity: medium
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:183"
evidence: |
  pub fn profile_link_problems(text: &str, labels: &[&str; 4], targets: &[String; 4]) -> Vec<String> {
recommendation: 'Say which pages each copy checks, and that the integration test mirrors it because it cannot reach it.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs profile_link_problems comment; repository.rs profile_problems comment names its mirror"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 An unexplained skip in the Origins-link check

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs_checks.rs:222"
evidence: |
      let from_last_profile = labels
recommendation: 'Name the slice and say why the first link is skipped.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs: from_last_profile and its comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 The same link clean-up chain repeated without a name

```yaml
severity: low
confidence: low
location: "crates/compagnon-cli/src/docs_checks.rs:160"
evidence: |
  fn chapter_of(target: &str) -> &str {
recommendation: 'Move it into one named helper.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "docs_checks.rs chapter_of, used by path_page_problems and the build-path test"
  - status: fixed
    by: newcomer
```
