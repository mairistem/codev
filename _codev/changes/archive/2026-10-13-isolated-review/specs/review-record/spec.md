## ADDED Requirements

### Requirement: A finding raised by several roles credits each of them

A finding MAY carry `raised_by`, the list of the roles that raised it,
the role of its section first. Validation SHALL report as
`review_invalid`, at the finding's block, a `raised_by` that is not a
list of role ids, that names a role twice, or whose first entry is not
the finding's own role. A record without `raised_by` stays valid: the
finding was raised by its role alone. `raised_by` does not change who
re-checks the answers: the finding's own role does.

`codev review --json` SHALL give each finding `raisedBy`, the list as
written or the finding's role alone, and each role of a step
`credited`, the ids of the findings of other roles of that step whose
`raised_by` names it. The human output SHALL name, for a finding raised
by several roles, the other roles.

#### Scenario: A shared finding

- **GIVEN** a record whose `### security` section holds `R-security-1`
  with `raised_by: [security, skeptic]` and whose `### skeptic` section
  says `No finding.`
- **WHEN** the user runs `codev review --json`
- **THEN** `R-security-1` has `raisedBy: ["security", "skeptic"]`
- **AND** the `skeptic` role of that step has `credited:
  ["R-security-1"]`, and no review finding is reported

#### Scenario: A finding without `raised_by`

- **GIVEN** a record whose findings carry no `raised_by`
- **WHEN** the user runs `codev review --json`
- **THEN** each finding's `raisedBy` holds its role alone

#### Scenario: A malformed `raised_by`

- **GIVEN** a finding of `### security` with `raised_by: [skeptic,
  security]`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error at the finding's block says the
  first entry must be `security`

### Requirement: The record says how each step was reviewed

Under a step heading, before its first role, the record MAY hold the
line `Mode: inline.` or `Mode: isolated.`, and any number of lines
`Summarized: <what was summarized>`. Under a role heading, it MAY hold
one line `Fallback: <why the role ran inline>`. A final period is
optional. Validation SHALL report as `review_invalid` a `Mode:` line
whose value is neither `inline` nor `isolated`, naming both, and a step
or role holding two `Mode:` or two `Fallback:` lines. These lines do not
change the role's state: a role still holds findings, `No finding.` or
`Disabled by configuration.`.

`codev review --json` SHALL give each step `mode` (`null` when not
written) and `summarized` (the texts, in file order), and each role
`fallback` (`null` when not written). The human output SHALL print them.

#### Scenario: An isolated step with a summary and a fallback

- **GIVEN** a record whose `## code.review` starts with `Mode:
  isolated.` and `Summarized: src/big.rs, diff over the limit.`, and
  whose `### newcomer` says `Fallback: the sub-agent timed out twice.`
  then `No finding.`
- **WHEN** the user runs `codev review --json`
- **THEN** the step has `mode: "isolated"` and `summarized: ["src/big.rs,
  diff over the limit"]`
- **AND** the `newcomer` role has `fallback: "the sub-agent timed out
  twice"` and `state: "no-finding"`

#### Scenario: A record written before isolated review

- **GIVEN** a record without `Mode:`, `Summarized:` or `Fallback:` lines
- **WHEN** the user runs `codev review --json`
- **THEN** each step has `mode: null` and `summarized: []`, and each role
  `fallback: null`

#### Scenario: An unknown mode in the record

- **GIVEN** a record whose `## plan.challenge` holds `Mode: parallel.`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error at that line names `inline` and
  `isolated`
