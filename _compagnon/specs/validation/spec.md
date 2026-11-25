# Validation Specification

## Purpose

Provide a reliable, localized verdict on a change or a main spec:
everything that `archive` and `sync` would refuse to write must be reported
here, upstream, with the line concerned and a stable code that a consumer
can test against.

## Requirements

### Requirement: Structural rules beyond the parser

The validator MUST reject as errors the defects that the parser lets
through because they require inspecting the content of a requirement or a
spec: a requirement without a normative keyword of the catalog in its
description — `SHALL`, `MUST`, `DOIT` or `DOIVENT`, in upper case — a
requirement with no scenario, a main spec with no extractable
requirement.

#### Scenario: Requirement without SHALL or MUST

- **GIVEN** an `## ADDED Requirements` delta with a `### Requirement: X`
  whose descriptive text is "The system does something"
- **WHEN** the validator inspects the file
- **THEN** a finding with code `requirement_no_shall` reports the line of
  the header and names the requirement `X`

#### Scenario: French requirement with DOIT

- **GIVEN** an `## Exigences AJOUTÉES` delta with a
  `### Exigence : Annulation` whose descriptive text is
  "Le portail DOIT permettre d'annuler une demande en attente." and a
  `#### Scénario : Dans les temps`
- **WHEN** the validator inspects the file
- **THEN** no `requirement_no_shall` finding is reported

#### Scenario: Requirement with no scenario at all

- **GIVEN** an `## ADDED Requirements` delta with a `### Requirement: Y`
  followed by its descriptive text but no `#### Scenario:`
- **WHEN** the validator inspects the file
- **THEN** a finding with code `requirement_no_scenario` reports the line
  of the requirement

#### Scenario: Main spec with no requirement

- **GIVEN** a main spec `_codev/specs/x/spec.md` with `## Purpose`
  but whose `## Requirements` is empty
- **WHEN** the validator inspects the spec
- **THEN** a finding with code `spec_no_requirement` reports that the spec
  has no extractable requirement

### Requirement: Consistency between sections of the same delta

The validator MUST detect inconsistencies between the four operations of
the same delta: the same requirement cannot appear in two sections at once,
a `RENAMED.TO` cannot coincide with an `ADDED` of the same name, and a
`MODIFIED` cannot reference the old name of a `RENAMED`.

#### Scenario: Requirement present in both ADDED and MODIFIED

- **GIVEN** a delta containing both `## ADDED Requirements` with
  `### Requirement: Z` and `## MODIFIED Requirements` with `### Requirement: Z`
- **WHEN** the validator inspects the delta
- **THEN** a finding with code `cross_section_conflict` names the requirement
  `Z`, the two sections involved and their respective lines

#### Scenario: RENAMED.TO collides with an ADDED of the same name

- **GIVEN** a delta containing `## ADDED Requirements` with
  `### Requirement: New Name` and `## RENAMED Requirements` with
  `FROM: Old Name` / `TO: New Name`
- **WHEN** the validator inspects the delta
- **THEN** a finding with code `rename_target_collision` reports the collision
  on `New Name`

#### Scenario: MODIFIED references the old name of a RENAMED

- **GIVEN** a delta containing `## MODIFIED Requirements` with
  `### Requirement: Old Name` and `## RENAMED Requirements` with
  `FROM: Old Name` / `TO: New Name`
- **WHEN** the validator inspects the delta
- **THEN** a finding with code `modified_uses_old_name` asks for
  `New Name` to be used in the MODIFIED section

### Requirement: Explicit zero-delta rule

A change must either produce at least one spec delta, or explicitly declare
that it will produce none; the validator MUST reject cases that contradict
this rule.

#### Scenario: Change with no delta and no skip_specs

- **GIVEN** a change whose `specs/` directory is empty and whose
  `change.yaml` does not set `skip_specs: true`
- **WHEN** the validator inspects the change
- **THEN** a finding with code `zero_delta_without_marker` asks either to
  add a delta or to declare `skip_specs: true`

#### Scenario: skip_specs declared but specs exist

- **GIVEN** a change whose `change.yaml` declares `skip_specs: true` and
  whose `specs/` directory contains at least one `.md` file
- **WHEN** the validator inspects the change
- **THEN** a finding with code `skip_specs_conflict` asks to remove
  `skip_specs: true` or to delete the files in the `specs/` directory

### Requirement: Overall verdict and exit code

On request, the validator MUST produce an overall verdict on a single item,
on all changes, on all specs, or on both; it MUST distinguish a run with no
errors from a run that reports some.

#### Scenario: Validating a named item

- **GIVEN** an initialized project with a change `add-auth`
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the report covers only that change and its exit code reflects
  the presence or absence of errors

#### Scenario: Batch validation

- **GIVEN** a project containing two changes and one main spec
- **WHEN** the user runs `codev validate --all`
- **THEN** the report covers all three items in a single document

#### Scenario: No errors, exit code zero

- **GIVEN** a well-formed change with no error finding
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the process exits with code `0`

#### Scenario: At least one error, non-zero exit code

- **GIVEN** a change containing at least one finding of severity `Error`
- **WHEN** the user runs `codev validate add-auth`
- **THEN** the process exits with code `1`

### Requirement: JSON report with a stable contract

When `--json` is requested, the validator MUST write exactly one JSON
document to stdout whose shape is frozen per version: the list of validated
items, and for each one its list of findings with `code`, `severity`,
`path`, `line`, `message`, plus a root-level `status` array where execution
errors land (root not found, unknown item).

#### Scenario: JSON output of a successful run

- **GIVEN** a well-formed change
- **WHEN** the user runs `codev validate add-auth --json`
- **THEN** stdout carries a single JSON document containing the list of
  items, each with an empty `findings` array, and an empty root `status`

#### Scenario: JSON output when the root cannot be found

- **GIVEN** a directory outside any `_codev/` root
- **WHEN** the user runs `codev validate --json`
- **THEN** stdout carries a single JSON document with the shape of the
  report, with its item lists empty, and a root `status` carrying an error
  entry with a stable code

### Requirement: Strict mode propagates warnings to the exit code

`codev validate [--all|--changes|--specs|<item>] [--strict]` MUST
return a non-zero exit code as soon as at least one finding is emitted,
**whatever its severity**, when `--strict` is present. Without
`--strict`, the behavior is unchanged: exit code 1 only when at least
one `Error` is present.

Strict mode **does not change** the severity of findings in the
report; it changes **only** the exit code decision rule. The human
rendering (text and JSON) remains identical.

#### Scenario: `--strict` on a clean report → exit 0

- **GIVEN** a project for which `codev validate --all` reports no
  finding
- **WHEN** the user runs `codev validate --all --strict`
- **THEN** the exit code of the command is 0

#### Scenario: `--strict` on a warning → non-zero exit

- **GIVEN** a project containing an unsealed `accepted` local ADR,
  which reports a `decision_unsealed` (warning) on validate
- **WHEN** the user runs `codev validate --strict`
- **THEN** the exit code of the command is non-zero (1)
- **AND** the human rendering still carries the `warning` line (not
  `error`) for this finding — the displayed severity is preserved

#### Scenario: Without `--strict`, a warning does not flip the exit code

- **GIVEN** the same context
- **WHEN** the user runs `codev validate` (without `--strict`)
- **THEN** the exit code of the command is 0
- **AND** the warning still appears in the output

#### Scenario: An error remains blocking, even without `--strict`

- **GIVEN** a project in which a local ADR was edited in place after
  sealing, which reports a `decision_seal_mismatch` (error)
- **WHEN** the user runs `codev validate`
- **THEN** the exit code of the command is non-zero (1) — strict mode
  is not needed for errors

### Requirement: `hasWarnings` exposed in the JSON contract

The `ValidateReportV1` JSON report MUST carry an additive field
`hasWarnings: bool` — `true` as soon as at least one finding of severity
`Warning` is present, `false` otherwise. The field is **always** present
in the output, regardless of its value or of strict mode.

This field is **informational**: a consumer that wants to decide outside
strict mode uses it. The exit code remains the official signal.

#### Scenario: `hasWarnings: true` on a report with a warning

- **GIVEN** a project that reports a `decision_unsealed` warning
- **WHEN** the user runs `codev validate --json`
- **THEN** the JSON document carries `"hasWarnings": true` at the root

#### Scenario: `hasWarnings: false` on a clean report

- **GIVEN** a project with no finding
- **WHEN** the user runs `codev validate --all --json`
- **THEN** the JSON document carries `"hasWarnings": false`

#### Scenario: `hasWarnings` independent of strict mode

- **GIVEN** a project that reports a warning and no error
- **WHEN** the user runs `codev validate --strict --json`
- **THEN** the JSON document carries `"hasWarnings": true`
- **AND** the exit code of the command is non-zero — both signals
  coexist without contradicting each other

### Requirement: A delta is checked against its main spec

For each delta of a change, validation SHALL read the main spec it targets
and report as an error `rename_source_missing` every `RENAMED` whose `FROM`
names no requirement of that spec — unless the `TO` requirement already
exists there, which means an earlier sync already applied the rename.

#### Scenario: Rename of an unknown requirement

- **GIVEN** a main spec without any requirement named `Remember Me`
- **AND** a delta renaming `Remember Me` to `Stay Signed In`
- **WHEN** the user runs `codev validate <change>`
- **THEN** an error `rename_source_missing` is reported
- **AND** the exit code is non-zero

#### Scenario: Rename already applied by a sync

- **GIVEN** a delta renaming `Remember Me` to `Stay Signed In`
- **AND** a main spec that already contains `Stay Signed In` and no longer
  contains `Remember Me`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `rename_source_missing` is reported

### Requirement: The release note of an active change is checked

For each active change whose `proposal.md` exists, validation SHALL report
the warning `release_note_missing` when the proposal has no
`## Release note` section, and the error `release_note_invalid` — with the
line concerned — for a bullet whose category is not one of `Added`,
`Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`, for a bullet with
no text, and for a section that holds neither bullets nor `None`. Archived
changes MUST NOT be checked: their history is not rewritten.

#### Scenario: Valid release note

- **GIVEN** an active change whose release note is
  `- Added: a flag.` and `- Changed: **BREAKING** a default.`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `release_note_*` finding is reported

#### Scenario: Internal change

- **GIVEN** an active change whose release note is `None`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `release_note_*` finding is reported

#### Scenario: Missing section

- **GIVEN** an active change whose proposal has no `## Release note`
  section
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `release_note_missing` warning is reported
- **AND** the exit code is 0 without `--strict`

#### Scenario: Unknown category

- **GIVEN** an active change whose release note has `- Improved: speed.`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `release_note_invalid` error names `Improved` and its line
- **AND** the exit code is 1

#### Scenario: Empty bullet

- **GIVEN** an active change whose release note has `- Fixed:` with no text
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `release_note_invalid` error is reported

#### Scenario: Archived changes are not checked

- **GIVEN** a project whose only change without a release note is archived
- **WHEN** the user runs `codev validate`
- **THEN** no `release_note_*` finding is reported

### Requirement: The specs validated are the capabilities listed

`codev validate` — without argument, with `--all` or with `--specs` — SHALL
validate exactly the main specs of the capabilities that `codev list
--specs` lists, and `codev validate <name>` SHALL find a spec under the same
names. A file under `_codev/specs/` that is not a capability's `spec.md`
SHALL be ignored without finding: it is neither validated nor reported, and
it never makes the command fail.

#### Scenario: A stray Markdown file is ignored

- **GIVEN** a project with a valid main spec `_codev/specs/user-auth/spec.md`
  and a file `_codev/specs/notes/myspec.md`
- **WHEN** the user runs `codev validate --specs --json`
- **THEN** the exit code is 0
- **AND** `items` holds one spec item, `user-auth`, and no item mentions
  `notes`

#### Scenario: The whole project with a stray file

- **GIVEN** the same project
- **WHEN** the user runs `codev validate`
- **THEN** the exit code is 0

#### Scenario: A stray file is not an item

- **GIVEN** the same project
- **WHEN** the user runs `codev validate notes/my`
- **THEN** the command fails with `unknown_item`

### Requirement: The domains of an active change are checked

The `## Domains` section of a proposal SHALL list, as bullets, the domains
among `security`, `personal-data`, `user-interface`, `performance`,
`operations` and `migration`: `- [x] <domain>` or `- <domain>` lists it,
`- [ ] <domain>` leaves it unticked, and any text after the domain name is
a free comment. The single line `None` says the change touches none of
them; a section whose items are all unticked says the same. HTML comments
and fenced blocks are ignored.

For each active change whose `proposal.md` exists, validation SHALL report:

- the error `unknown_domain`, with its line, for a bullet whose domain is
  not one of the six, naming the valid domains;
- the error `domains_invalid`, with its line, for a line of the section
  that is neither a bullet nor `None`, and for `None` combined with a
  ticked domain;
- the warning `domains_missing` when the proposal has no `## Domains`
  section.

Archived changes MUST NOT be checked: their history is not rewritten.

#### Scenario: Ticked domains

- **GIVEN** an active change whose `## Domains` section holds
  `- [x] security`, `- [ ] performance` and `- personal-data — new e-mail
  field`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no domain finding is reported

#### Scenario: No domain

- **GIVEN** an active change whose `## Domains` section is `None`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no domain finding is reported

#### Scenario: Unknown domain

- **GIVEN** an active change whose `## Domains` section holds
  `- [x] securty`
- **WHEN** the user runs `codev validate <change>`
- **THEN** an `unknown_domain` error names `securty` and its line
- **AND** the exit code is 1

#### Scenario: `None` with a ticked domain

- **GIVEN** an active change whose `## Domains` section holds `None` and
  `- [x] security`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `domains_invalid` error is reported

#### Scenario: Missing section

- **GIVEN** an active change whose proposal has no `## Domains` section
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `domains_missing` warning is reported
- **AND** the exit code is 0 without `--strict`

#### Scenario: Archived changes are not checked

- **GIVEN** a project whose only change without a `## Domains` section is
  archived
- **WHEN** the user runs `codev validate`
- **THEN** no domain finding is reported

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

### Requirement: The suggested domains of an active change are checked

`codev validate` SHALL read the diff once, as the `domain-suggestions`
capability defines it, and, for each active change whose `proposal.md`
exists, report the warning `domain_suggested` for each suggested domain
whose state is `unticked`. The warning SHALL be located in `proposal.md`,
at the line of the domain's bullet when the section lists it, otherwise at
the `## Domains` heading, otherwise at line 1; its message SHALL name the
domain, the first matching trigger with its source, the file it matched,
the number of other matches, and the two ways out: tick the domain, or
keep it unticked with a reason on its line. A domain that is `ticked` or
`justified` gets no finding.

The diff is the working tree's: every active change is compared to the
same diff. Without a diff — outside git, without a default branch — no
`domain_suggested` is reported and validation does not fail because of
it. The pre-flight of `codev sync` and `codev archive`, and the
validation `codev doctor` counts, MUST NOT read the diff. Archived
changes MUST NOT be checked.

#### Scenario: An unticked suggestion

- **GIVEN** an active change whose proposal lists `- [ ] security` and
  whose diff adds `src/auth/session.rs`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `domain_suggested` warning on `proposal.md`, at the line of
  `- [ ] security`, names `security`, `**/auth/**` and
  `src/auth/session.rs`
- **AND** the exit code is 0, and 1 with `--strict`

#### Scenario: A justified or ticked suggestion

- **GIVEN** an active change whose proposal lists
  `- [ ] security — only renames a variable` and whose diff adds
  `src/auth/session.rs`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `domain_suggested` warning is reported
- **AND** none either once the line becomes `- [x] security`

#### Scenario: No diff

- **GIVEN** a project outside any git repository with an active change
  whose proposal says `None`
- **WHEN** the user runs `codev validate --strict`
- **THEN** no `domain_suggested` warning is reported and the exit code
  is 0

#### Scenario: Archive does not read the diff

- **GIVEN** an active change with an unticked suggested domain and no
  other finding
- **WHEN** the user runs `codev archive <change>`
- **THEN** the change is archived and git was not asked for a diff

### Requirement: The proposal sections are checked in every catalog language

The checks of a proposal's release note and domains SHALL apply to the
section written with any catalog language's heading — `## Note de version`
and `## Domaines` in French — with the same findings and codes. A release
note category and the single line `None` are accepted in any catalog
language (`Ajouté`, `Aucune`…); domain names stay the identifiers
`security`, `personal-data`, `user-interface`, `performance`, `operations`
and `migration` in every language. A proposal that has the section in one
language is not reported as missing it. When a proposal holds the same
section in two languages, the first one in the file is read and the other
is ignored, as a second section of the same heading is today.

#### Scenario: French release note and domains

- **GIVEN** an active change whose proposal holds `## Note de version`
  with `- Ajouté : un export.` and `## Domaines` with `- [x] migration`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `release_note_*` and no domain finding is reported

#### Scenario: Internal change written in French

- **GIVEN** an active change whose `## Note de version` is `Aucune` and
  whose `## Domaines` is `Aucun`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `release_note_*` and no domain finding is reported

#### Scenario: Unknown French category

- **GIVEN** an active change whose `## Note de version` holds
  `- Amélioré : la vitesse.`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `release_note_invalid` error names `Amélioré` and its line

#### Scenario: Translated domain name

- **GIVEN** an active change whose `## Domaines` holds `- [x] sécurité`
- **WHEN** the user runs `codev validate <change>`
- **THEN** an `unknown_domain` error names `sécurité` and its line

#### Scenario: The same section in two languages

- **GIVEN** an active change whose proposal holds `## Domains` with
  `- [x] migration`, then `## Domaines` with `- [x] sécurité`
- **WHEN** the user runs `codev validate <change>`
- **THEN** the domains read are those of `## Domains`, and no
  `unknown_domain` finding names `sécurité`

### Requirement: The corrections of an active change are checked

For each active change whose `corrections.md` exists, validation SHALL
report, located at the file's lines, the findings the `corrections`
capability defines: `corrections_invalid` as an error;
`corrections_unknown_field`, `corrections_too_long` and
`corrections_format_unsupported` as warnings. With `--strict`, those
warnings fail validation as every other warning does. Archived changes
MUST NOT be checked.

#### Scenario: A change without corrections

- **GIVEN** an active change without `corrections.md`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `corrections_*` finding is reported

#### Scenario: A corrections error fails validation

- **GIVEN** an active change whose `corrections.md` holds an entry
  without `did`
- **WHEN** the user runs `codev validate <change> --json`
- **THEN** a `corrections_invalid` error is reported with the path of
  `corrections.md` and its line
- **AND** the exit code is 1

#### Scenario: A warning under strict mode

- **GIVEN** an active change whose only finding is a
  `corrections_unknown_field` warning
- **WHEN** the user runs `codev validate <change> --strict`
- **THEN** the exit code is 1
