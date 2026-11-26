# Review record: missing-pages

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The approving page rested on a template Compagnon writes only on GitHub

```yaml
severity: high
confidence: high
location: "design.md#Context"
evidence: |
  Facts the pages rest on, from the code: on GitHub only, `compagnon init`
recommendation: 'plan_pr_template is GitHub only and its line is a placeholder the developer fills in: a GitLab reviewer would look for a section that does not exist. Point every reviewer to the changed files under `_compagnon/changes/<name>/`, the description only as a GitHub extra, and check it.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md Context and Decision: Approving through the pull request; specs/docs/spec.md (where a plan arrives; scenario names `_compagnon/changes/`)"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The French guide was checked for English delta keywords

```yaml
severity: medium
confidence: high
location: "specs/docs/spec.md#Requirement: Non-developers can read and approve a plan without the terminal"
evidence: |
  under the keywords its edition's plans show: `ADDED`, `MODIFIED`,
recommendation: 'A French plan shows `Exigences MODIFIÉES`: check each edition for its own keywords.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md scenario; tasks.md 1.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The page tasks could not be verified before the summary listed the pages

```yaml
severity: medium
confidence: high
location: "tasks.md#Tasks"
evidence: |
  each page task lists its pages in both `SUMMARY.md`, so that the tests and
recommendation: 'Tests and mdbook see only listed pages, and the EVERYDAY update turns existing tests red: list each page in its own task and widen the exemption.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md definition of done, 2.1-2.4"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 A one-time validation did not answer the example's drift

```yaml
severity: low
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  - [The example plan drifts from what Compagnon writes] → a test runs the
recommendation: 'Run the example through the parser in a test, so drift breaks the build.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md risk; specs/docs/spec.md scenario; tasks.md 2.1 the_example_plan_parses_as_compagnon_writes_it"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 The personal-data justification misstated its match

```yaml
severity: low
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] personal-data — the match is the English word "phone" in the example scenario of both home pages (`docs/site/index.html` and `fr.html`, line 702), already on the branch; not personal data
recommendation: 'Name the actual match: "phone", in both home pages.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: "proposal.md Domains"
  - status: fixed
    by: skeptic
```

### accessibility

#### R-accessibility-1 The language of quoted button labels was not decided

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Approving through the pull request"
evidence: |
  then asks Claude to take the comments into account. Labels are quoted as
recommendation: 'Quote labels as seen (GitHub in English, GitLab in both), the French action first and the label between parentheses, never as code (WCAG 3.3.2).'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: "design.md; specs/docs/spec.md scenario on labels between parentheses"
  - status: fixed
    by: accessibility
```

#### R-accessibility-2 The example plan's headings could mix with the page's

```yaml
severity: medium
confidence: low
location: "specs/docs/spec.md#Requirement: The example plan is complete and annotated"
evidence: |
  tasks — each part under a page heading of its own, its text in a quoted
recommendation: 'Quote the plan in markdown blocks under a page heading per part, the note under its own heading (WCAG 1.3.1, 2.4.6), and check the layout.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: "design.md decision on the example; specs/docs/spec.md scenario"
  - status: fixed
    by: accessibility
```
