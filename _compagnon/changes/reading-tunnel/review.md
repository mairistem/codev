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
