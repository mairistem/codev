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
