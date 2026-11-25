## ADDED Requirements

### Requirement: The review record of an active change is checked

For each active change whose `review.md` exists, validation SHALL report,
located at the record's lines, the findings the `review-record` capability
defines: `review_invalid`, `review_decision_incomplete`,
`review_location_missing`, `review_evidence_not_found` and
`review_role_missing` as errors; `review_unknown_section`,
`review_unknown_field`, `review_evidence_moved`,
`review_evidence_unchecked` and `review_format_unsupported` as warnings.
A change without `review.md` gets no review finding: when the record is
required is decided at archive time. Validation MUST NOT report the
status of the findings themselves — an `open` finding is not a defect of
the record. Archived changes MUST NOT be checked.

#### Scenario: A change without a record

- **GIVEN** an active change without `review.md`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `review_*` finding is reported

#### Scenario: An open high finding is not a validation error

- **GIVEN** an active change whose well-formed record holds a `high`
  finding `open`, its evidence at the cited lines
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `review_*` finding is reported
- **AND** the exit code is 0

#### Scenario: A record error fails validation

- **GIVEN** an active change whose record has a finding without
  `evidence`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `review_invalid` error is reported with the path of
  `review.md` and its line
- **AND** the exit code is 1

#### Scenario: Archived records are not checked

- **GIVEN** a project whose only invalid review record is in an archived
  change
- **WHEN** the user runs `codev validate`
- **THEN** no `review_*` finding is reported
