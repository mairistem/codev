# Review record: reading-tunnel

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The GitHub-only rule made the reference describe compagnon init wrongly

```yaml
severity: high
confidence: high
location: 'specs/docs/spec.md#Requirement: The documentation names GitHub only'
evidence: |
  No chapter of either edition, README or home page SHALL name GitLab:
recommendation: 'Exclude the command-line and JSON-output references, which describe compagnon init literally, or bring the command into scope.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: 'spec: exception for reference/cli.md and reference/json-output.md, new failing scenario; design and tasks 2.3 aligned; proposal says the command stays out of scope'
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The Migration notes of the removed requirements promised redirects and profile cards

```yaml
severity: medium
confidence: high
location: 'specs/docs/spec.md#REMOVED Requirements'
evidence: |
  old addresses redirect.
recommendation: 'Rewrite the Migration notes: no redirect, the address disappears, the content is in a named tunnel page.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: 'spec: three Migration notes rewritten; skip lines → skip buttons'
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 Nothing checked the wording or form of the skip button

```yaml
severity: medium
confidence: high
location: 'specs/docs/spec.md#Requirement: Each tunnel page leads to the next'
evidence: |
  who can skip and where to — "Not for you? Skip to <title>" (« Pas pour
recommendation: 'Fix one form and add failing scenarios for position, block and text.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: 'spec: "<who>? Skip to <title>" in a skip-step block before the first paragraph, new scenario; design examples end with chapter titles; tasks 3.5 tests'
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The plan deletes published addresses yet the migration box was unticked

```yaml
severity: medium
confidence: high
location: 'proposal.md#Domains'
evidence: |
  - [ ] migration — no configuration or data change; removed pages are documentation addresses only, dropped on the owner's request
recommendation: 'Tick migration so its expert reviews the link breakage.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: 'proposal: migration ticked; the migration expert reviews the plan'
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 An unchanged requirement still named the removed Start here path

```yaml
severity: low
confidence: high
location: 'specs/docs/spec.md#RENAMED Requirements'
evidence: |
  - FROM: `### Requirement: The Start here path stays short and plain`
recommendation: 'Rename the glossary requirement.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: 'spec: RENAMED entries for the glossary and home page requirements'
  - status: fixed
    by: skeptic
```

### accessibility

#### R-accessibility-1 The skip button's focus indicator could look the same as its border

```yaml
severity: low
confidence: low
location: 'tasks.md#4. The look'
evidence: |
  with a visible focus; verified by the contrast check extended to the button's text and border
recommendation: 'Give the focus its own indicator and check it.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: lead
    ref: 'design: 3 px outline plus filled background; task 4.1 checks focus color and the focused look'
  - status: fixed
    by: accessibility
```

#### R-accessibility-2 The skip button's arrow could be read aloud in the link's name

```yaml
severity: low
confidence: low
location: 'design.md#Decision: "Next step" is a markdown section; the skip button a wrapped markdown link'
evidence: |
  draw `.skip-step a` as a button (border, padding, arrow), with a visible
recommendation: 'Draw the arrow as a CSS shape with no text.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: 'design: arrow a CSS shape; task 4.1 checks the accessible name'
  - status: fixed
    by: accessibility
```

#### R-accessibility-3 The clickable mockup was never checked with the keyboard

```yaml
severity: low
confidence: low
location: 'tasks.md#1. Mockup of the tunnel, for the owner''s approval'
evidence: |
  every "Next step" and skip button moves to an existing card
recommendation: 'Plain anchors, focusable cards, and a keyboard check.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: lead
    ref: 'design and task 1.1: anchors, tabindex cards, Tab+Enter focus check'
  - status: fixed
    by: accessibility
```

### migration

#### R-migration-1 Removing Moving from codev dropped the manual steps codev users still need

```yaml
severity: medium
confidence: high
location: 'specs/docs/spec.md#Requirement: Moving from codev to Compagnon is documented in both languages'
evidence: |
  **Migration**: `compagnon migrate` stays documented in the command-line
recommendation: 'Move the steps the command does not take into the compagnon migrate section of the command-line reference, with a check.'
history:
  - status: open
    by: migration
  - status: fixed
    by: qa
    ref: 'spec: new requirement "The command-line reference says how to move from codev" with its scenarios; Migration note reworded; task 2.1 moves the content before the guide is deleted'
  - status: fixed
    by: migration
```

#### R-migration-2 The BREAKING release note did not say where each removed page went

```yaml
severity: low
confidence: high
location: 'proposal.md#Release note'
evidence: |
  - Removed: **BREAKING** The "Choose your path" page, the four profile pages, "Starting from nothing", "Repository integration" and "Moving from codev" as separate pages; their old addresses no longer exist, and their useful content is now in the tunnel.
recommendation: 'Say where each removed page went.'
history:
  - status: open
    by: migration
  - status: fixed
    by: product
    ref: 'proposal.md Release note names the new place of each removed page'
  - status: fixed
    by: migration
```

#### R-migration-3 No trace of codev leaves codev projects without the manual steps

```yaml
severity: medium
confidence: high
location: 'specs/docs/spec.md#Requirement: The documentation holds no trace of codev'
evidence: |
  (in any case): the command-line reference has no `compagnon migrate`
recommendation: 'Keep the steps compagnon migrate does not take in its reference section, or accept that codev projects rely on the command and its --help.'
history:
  - status: open
    by: migration
  - status: accepted-risk
    by: human
    note: 'The owner wants no trace of codev in the documentation; projects still on codev rely on compagnon migrate and its --help.'
```

## code.review

Mode: isolated.
Summarized: none — each reviewer read the change's diff (`git diff main...HEAD`) itself, file by file, instead of summaries.

### accessibility

#### R-accessibility-4 The Markdown code inside the Jira page's example box is unreadable in the light theme

```yaml
severity: medium
confidence: high
location: 'docs/theme/mairistem.css:2118-2122'
evidence: |
  .content .example pre {
    margin: 0;
    border-radius: var(--mx-radius-sm);
    background: #1e1e1e;
  }
recommendation: 'Give highlighted tokens inside .example a light colour of their own (at least 4.5:1 on #1e1e1e) in every theme.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: '4b680f0 — .content .example pre code [class^="hljs-"] pinned to #e6e6e6'
```

#### R-accessibility-5 Inline code inside an example box is white on white in the dark theme

```yaml
severity: medium
confidence: high
location: 'docs/theme/mairistem.css:2109-2112'
evidence: |
  .content .example > p:not(.example-title) {
    margin: 12px 8px 6px;
    color: #40565e;
  }
recommendation: 'Pin inline code inside the box to light colours in mairistem.css and docs.css.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: '4b680f0 — .example p code is #00394d on #eef3f4 in both stylesheets'
```

#### R-accessibility-6 Screen readers read the "+" and "−" signs as part of the "Under the hood" names

```yaml
severity: low
confidence: high
location: 'docs/theme/mairistem.css:1819-1830'
evidence: |
  .content details > summary::after {
recommendation: 'Give the generated sign an empty text alternative in the theme, docs.css and the home pages.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: '4b680f0 — content: "+" / "" and "\2212" / "" everywhere'
```

#### R-accessibility-7 Home-page eyebrow labels are below 4.5:1 in the light scheme

```yaml
severity: medium
confidence: high
location: 'docs/site/index.html:42'
evidence: |
  --eyebrow: #566c74;
recommendation: 'Darken --eyebrow (it was #6e858d: 3.89:1 on white, 3.64:1 on #f4f8f9) in both home pages.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: '4b680f0 — --eyebrow #566c74: 5.54:1 on white, 5.18:1 on #f4f8f9'
```

#### R-accessibility-8 Grey text on the pale background is just below 4.5:1 on the home pages

```yaml
severity: low
confidence: high
location: 'docs/site/index.html:41'
evidence: |
  --muted: #5a7078;
recommendation: 'Darken --muted (it was #5f767e, 4.49:1 on #f4f8f9) in both home pages.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: '4b680f0 — --muted #5a7078: 4.88:1 on #f4f8f9'
```

#### R-accessibility-9 The dark-mode focus ring on the home pages is below 3:1 on the alternate sections

```yaml
severity: low
confidence: high
location: 'docs/site/index.html:48'
evidence: |
  --focus: #ff5331;
recommendation: 'Override --focus with #ffab03 in the dark-scheme block of both home pages.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: '4b680f0 — dark scheme --focus #ffab03: 4.98:1 on #004c66'
```

### migration

No finding.

### newcomer

#### R-newcomer-1 The comment in removed_pages_problems gives the wrong reason for its folder-prefix guard

```yaml
severity: medium
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:348-362'
evidence: |
  fn names_page(text: &str, page: &str) -> bool {
recommendation: 'Name the case the guard handles (a chapter of the same name in another folder) and replace the slice arithmetic with named parts in a small helper.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'd9427ab — names_page(text, page), documented with guides/review.md vs paths/review.md'
```

#### R-newcomer-2 a_long_every_day_chapter_is_caught calls no check

```yaml
severity: medium
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:1716-1718'
evidence: |
  fn a_long_every_day_chapter_is_caught() {
recommendation: 'Move the cap into a named function and test it.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'd9427ab — chapter_cap and word_cap_problems, used by the test and by the_first_steps_stay_under_their_caps'
```

#### R-newcomer-3 Bare numbers and tuple fields stand for tunnel steps

```yaml
severity: low
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:2530-2536'
evidence: |
  fn every_tunnel_page_draws_and_shows() {
recommendation: 'Name the step boundaries; consider a struct for TUNNEL.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'd9427ab — BEGINNER_STEPS, DIAGRAM_STEPS and beginner_chapters(); TUNNEL keeps its tuple'
```

#### R-newcomer-4 skip_problems takes six parameters and uses an iterator comparison to find the title

```yaml
severity: low
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:456-479'
evidence: |
  pub fn skip_problems(
recommendation: 'Write the title check directly; consider passing one tunnel view instead of two closures.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'd9427ab — the title check is a direct match; the two closures stay so the unit tests can pass a tunnel of their own'
```

#### R-newcomer-5 Two copied mention scanners and copied file lists

```yaml
severity: low
confidence: high
location: 'crates/compagnon-cli/src/docs_checks.rs:598-612'
evidence: |
  pub fn lines_naming(text: &str, word: &str) -> Vec<String> {
recommendation: 'One lines_naming(text, word) and one shared ENTRY_POINTS list.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'd9427ab — lines_naming and ENTRY_POINTS'
```

#### R-newcomer-6 Unexplained 6 and hard-coded 5 in the tunnel script

```yaml
severity: low
confidence: high
location: 'docs/theme/mairistem.js:121-133'
evidence: |
  if (path.endsWith('/' + chapter + '.html')) { here = i; }
recommendation: 'Use endsWith and STEPS.length.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: 'd9427ab — endsWith, STEPS.length and a label per edition'
```
