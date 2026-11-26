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

## code.review

Mode: isolated.

### accessibility

#### R-accessibility-4 With no _compagnon/, onboard ends on a bare command

```yaml
severity: medium
confidence: high
location: "assets/workflows/onboard.md:100"
evidence: |
  | `_compagnon/` missing | `compagnon init`, for the user to type in a terminal |
recommendation: 'Say first in plain words what is missing and what the command does, state this case as the exception to the closing rule (a command the user types, Claude may not run it), and give an example reply.'
history:
  - status: open
    by: accessibility
```

#### R-accessibility-5 The closing request holds a placeholder the user cannot send as it is

```yaml
severity: medium
confidence: high
location: "assets/workflows/onboard.md:146"
evidence: |
  > When an idea comes up, say “plan <an idea>” (shortcut: `/compagnon-propose <an-idea>`).
recommendation: 'End on a request the user can send unchanged (Claude then asks for the idea), or explain the gap in plain words outside the quotes; one spelling of the placeholder.'
history:
  - status: open
    by: accessibility
```

### newcomer

#### R-newcomer-1 The case-loading helper took the loader's doc comment and hid what it does

```yaml
severity: medium
confidence: high
location: "evals/src/case.rs:1779"
evidence: |
  fn parse_required_choice<T: Copy>(
recommendation: 'Give the loader back its doc comment, name the helper after what it parses, pass the existing parser and the case kind.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/case.rs: parse_required_choice with ExploreOutcome::parse / OnboardState::parse; load_read_only's doc comment back"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The several-changes test depended on a missing blank line

```yaml
severity: low
confidence: medium
location: "crates/compagnon-agents/src/workflows.rs:1825"
evidence: |
  fn onboard_closes_several_changes_on_a_question() {
recommendation: 'Find the quoted reply after the marker whatever the blank lines.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/compagnon-agents/src/workflows.rs: the test reads the > lines after the marker"
  - status: fixed
    by: newcomer
```
