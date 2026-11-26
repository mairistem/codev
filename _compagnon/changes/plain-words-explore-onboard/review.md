# Review record: plain-words-explore-onboard

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 Onboard lines that suggest nothing would fail the shortcut test

```yaml
severity: medium
confidence: high
location: 'design.md#Decision: The two skills join the six, with their tests'
evidence: |
  Every other line of the two bodies naming `/compagnon-` is reworded so
recommendation: 'List these lines and either exempt or reword each.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md: each line reworded; tasks.md 2.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The instruction to quote the exact command to type contradicted the rule

```yaml
severity: medium
confidence: high
location: 'tasks.md#2. The skills'
evidence: |
  the guardrails, the "make the suggestion actionable" paragraph, and a plain
recommendation: 'Reword the paragraph and check it is gone.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md 1.1 onboard_no_longer_asks_to_quote_the_command_to_type, 2.2; design.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 With several changes the closing line had no defined shape

```yaml
severity: medium
confidence: high
location: 'specs/skills/spec.md#Requirement: A reply ends with the one request to say next'
evidence: |
  with its request and shortcut; when the next step is that choice, the last
recommendation: 'Say what the last line is when the next step is a choice, with a scenario.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs: closing rule for a choice, scenario Onboard with several active changes; tasks 1.1, 3.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 Onboard's details commands were not checked to stay

```yaml
severity: low
confidence: high
location: 'tasks.md#1. The checks'
evidence: |
  `compagnon decision list` and `compagnon list --specs`, and for explore
recommendation: 'Add the details commands to the list of commands that must stay.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md 1.1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 Two Domains reasons covered part of their matches

```yaml
severity: low
confidence: high
location: 'proposal.md#Domains'
evidence: |
  the matches are the manifests (`pyproject.toml`, `package.json`, `go.mod`)
recommendation: 'Reword the reasons to cover every match.'
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

#### R-accessibility-1 Doctor's fix lines stayed bare commands with no plain words

```yaml
severity: medium
confidence: high
location: 'design.md#Decision: The two skills join the six, with their tests'
evidence: |
  Each missing check that onboard relays from `compagnon doctor` gets a few
recommendation: 'Give each relayed missing check plain words of its own before the verbatim fix.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: "design.md; specs: exemption limited to the fix after plain words; tasks.md 2.2"
  - status: fixed
    by: accessibility
```

#### R-accessibility-2 The closing line with several changes was not a request to send

```yaml
severity: medium
confidence: high
location: 'specs/skills/spec.md#Scenario: Onboard with several active changes'
evidence: |
  - **AND** its last line asks which one to continue, without a shortcut
recommendation: 'Define the last line for several changes.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: qa
    ref: "specs: closing rule for a choice and its scenario; each change listed with its request"
  - status: fixed
    by: accessibility
```

#### R-accessibility-3 Onboard's opening description presented commands as the way to drive steps

```yaml
severity: low
confidence: high
location: 'design.md#Decision: The two skills join the six, with their tests'
evidence: |
  that it is a shortcut or no longer names a command: onboard's opening
recommendation: 'Rewrite the opening description: ask in your own words, commands as shortcuts.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: architect
    ref: "design.md; tasks.md 2.2"
  - status: fixed
    by: accessibility
```
