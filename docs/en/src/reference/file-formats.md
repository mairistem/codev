# File formats

codev's files are Markdown and YAML meant to be written by people and agents,
and parsed by codev. This chapter gives the exact rules, with the English
keywords. codev also reads their French equivalents (`### Exigence :`,
`## Exigences AJOUTÉES`, `DOIT`…), listed in
[Artifact language](../guides/artifact-language.md).

Everything inside fenced code blocks and HTML comments is ignored by the
parsers, so templates can carry guidance in `<!-- … -->` comments.

## Main spec

Path: `_codev/specs/<capability>/spec.md`, where `<capability>` is one or more
kebab-case segments (`ui/theme`). Only a file named exactly `spec.md` is a
main spec: other files under `_codev/specs/`, such as a `README.md`, are
ignored by `codev list --specs` and `codev validate`.

```markdown
# Theme Specification

## Purpose

Let users choose the color theme of the interface.

## Requirements

### Requirement: Theme follows the system preference by default

The application SHALL render with the operating system's color scheme until the user picks a theme.

#### Scenario: No theme chosen yet

- **GIVEN** a user who has never changed the theme setting
- **WHEN** their operating system is set to dark mode
- **THEN** the application renders with the dark theme
```

- `## Purpose` is required.
- Requirements are `### Requirement: <name>` headings inside
  `## Requirements`. The name identifies the requirement in deltas, so keep it
  unique within the spec.
- A requirement's description contains a normative keyword in upper case:
  `SHALL` or `MUST`, or `DOIT` or `DOIVENT`.
- Every requirement has at least one `#### Scenario: <name>`, with exactly four
  `#` characters. Scenario steps are bullet lines using `**GIVEN**`,
  `**WHEN**`, `**THEN**` and `**AND**`.
- Other `##` sections are allowed and preserved by merges.
- Delta headings (`## ADDED Requirements`…) are not allowed in a main spec.

When archiving creates a main spec for a new capability, it titles it from
the last segment of the capability path (`# Theme Specification`) and copies
the delta's `## Purpose`.

## Spec delta

Path: `_codev/changes/<name>/specs/<capability>/spec.md`, with the same
capability path as the main spec it modifies.

```markdown
## ADDED Requirements

### Requirement: The chosen theme is remembered

The application SHALL restore the theme the user last picked, on every device they sign in from.

#### Scenario: Returning user

- **GIVEN** a user who picked the dark theme on their laptop
- **WHEN** they sign in on their phone
- **THEN** the application renders with the dark theme

## MODIFIED Requirements

### Requirement: Theme follows the system preference by default

<the complete requirement, scenarios included, as it must read after the change>

## REMOVED Requirements

### Requirement: Legacy color picker

**Reason**: Replaced by the theme setting.
**Migration**: Users' custom colors are dropped; the default theme applies.

## RENAMED Requirements

- FROM: Theme follows the system preference by default
- TO: Theme follows the system preference until the user chooses
```

| Section | Content | Merge |
|---|---|---|
| `## ADDED Requirements` | Complete new requirements | Appended at the end of `## Requirements` |
| `## MODIFIED Requirements` | Complete requirements, with an existing name | Each replaces the requirement of the same name |
| `## REMOVED Requirements` | A `### Requirement:` heading with `**Reason**:` and `**Migration**:` lines | The requirement is deleted |
| `## RENAMED Requirements` | `FROM:` / `TO:` line pairs, with or without a leading `- ` | The heading is retitled; body and scenarios are untouched |

In `RENAMED`, write after `FROM:` and `TO:` either the bare requirement name,
as above, or the full heading in backticks, as the template does:
`` - FROM: `### Requirement: <old name>` ``.

Rules checked by validation:

- A requirement appears in at most one of `ADDED`, `MODIFIED` and `REMOVED`
  (`cross_section_conflict`), and at most once per section
  (`duplicate_requirement`).
- Inside a delta section, every `###` heading is a `### Requirement:`
  (`delta_unexpected_heading`); a section with no entry is reported as a
  warning (`delta_section_empty`).
- A `RENAMED` source must exist in the main spec (`rename_source_missing`,
  unless the rename was already applied by an earlier sync); a `RENAMED`
  target cannot also be `ADDED` (`rename_target_collision`); and `MODIFIED`
  uses the new name of a renamed requirement, not the old one
  (`modified_uses_old_name`).
- `MODIFIED` and `REMOVED` names must exist in the main spec; this is checked
  when the delta is merged (`modified_target_missing`).
- **New capability:** start the delta with `## Purpose`. For an existing
  capability, omit it — the main spec's Purpose is kept.
- A `REMOVED` delta that would leave a spec with no requirement is refused
  (`would_leave_spec_without_requirement`) unless the change declares
  `retire_capabilities: true`; the main spec is then deleted.

## proposal.md

The `spec-driven` template:

```markdown
# Proposal: <change title>

## Why

## What Changes

## Capabilities

### New Capabilities

### Modified Capabilities

### Removed Capabilities

## Impact

## Domains

## Release note
```

Each capability listed under *New* or *Modified Capabilities* gets its own
delta file at `specs/<capability>/spec.md`. Breaking changes are marked
**BREAKING** in *What Changes*. A capability listed under *Removed
Capabilities* requires `retire_capabilities: true`.

### Release note

What end users read in the changelog, built by
[`codev changelog`](cli.md#codev-changelog). Either one or more items:

```markdown
## Release note

- Added: `--format json` on `codev export`.
- Changed: **BREAKING** `codev export` writes to stdout by default; pass
  `--output <PATH>` to write a file.
```

or, for a change users will not notice, the single line:

```markdown
## Release note

None
```

Each item is `- <Category>: <text>`, the category being one of `Added`,
`Changed`, `Deprecated`, `Removed`, `Fixed` and `Security` — the categories of
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). An item may wrap
over several lines. A breaking item starts its text with `**BREAKING**`. The
text follows the project's artifact language.
`codev validate` reports a missing section as a warning and an unknown
category, an item without text or an empty section as errors, on active
changes only.

### Domains

The domains the change touches, which activate their
[experts](../roles.md#experts-and-domains). The template lists the six
domains unticked; tick the ones the change touches:

```markdown
## Domains

- [x] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [x] operations — a new background worker
- [ ] migration
```

or, when it touches none of them, write the single line `None`.

`- [x] <domain>` or `- <domain>` ticks a domain, `- [ ] <domain>` leaves it
unticked, and the text after the domain name is a free comment. The
domains are `security`, `personal-data`, `user-interface`, `performance`,
`operations` and `migration`. `codev validate` reports, on active changes
only, an unknown domain (`unknown_domain`) and a line that is neither an
item nor `None`, or `None` next to a ticked domain (`domains_invalid`), as
errors, and a missing section (`domains_missing`) as a warning.

That comment is also how you answer a
[suggested domain](../roles.md#triggers-and-suggested-domains) the change
does not touch: an unticked domain with a reason after its name is
`justified` — `- [ ] security — only renames a variable`. Leading dashes,
colons and spaces are not part of the reason; `None` justifies nothing.
`codev validate` reports each suggested domain neither ticked nor
justified as `domain_suggested`, a warning at the domain's line.

## design.md

```markdown
# Design: <change title>

## Context

## Goals / Non-Goals

## Decisions

### Decision: <the choice>

**Alternatives considered**:

## Risks / Trade-offs

## Migration Plan

## Open Questions
```

Each `### Decision: <title>` block can be promoted to an ADR with
`codev decision promote <change> "<title>"`. *Risks / Trade-offs* uses the
`[Risk] → Mitigation` form. *Migration Plan* and *Open Questions* are removed
when they do not apply.

## tasks.md

```markdown
# Tasks

## 1. Theme tokens

- [ ] 1.1 Move every color to a CSS custom property, verified by a visual diff of the settings page
- [ ] 1.2 Add the dark token set, verified by `npm test -- theme`
```

Tasks are grouped under numbered `##` headings and written as `- [ ] X.Y
<task>`; `/codev-apply` checks them off as `- [x]`. Each task states how to
verify it.

## change.yaml

Path: `_codev/changes/<name>/change.yaml`. Written by `codev new change`.
`<name>` is never `archive`, the folder of archived changes.

```yaml
schema: spec-driven
created: 2026-09-30
goal: Let users switch the UI to a dark theme
```

| Key | Type | Meaning |
|---|---|---|
| `schema` | string, required | Schema of the change: `spec-driven`, or the folder name of a project schema under `_codev/schemas/` |
| `created` | date | Creation date, `YYYY-MM-DD` |
| `goal` | string | The goal given with `--goal` |
| `skip_specs` | boolean | The change intentionally has no spec delta: the artifacts writing under `specs/` are skipped, and validation accepts zero deltas |
| `retire_capabilities` | boolean | Allows sync and archive to delete a main spec whose last requirement the change removes |
| `roles` | mapping | Role activations for this change only, over the project's [`roles:`](configuration.md#roles): `newcomer: always`. Activations only — criteria belong to the project |
| `review` | mapping | `mode: inline` or `mode: isolated` for this change only, over the project's [`review:`](configuration.md#review). Another value fails with `invalid_review_mode` |

Unknown keys are rejected — inside `review` too. A codev that predates
`review` refuses a `change.yaml` that sets it: raise the project's
[`codev_version`](configuration.md#codev_version) to the release that
introduced it before using it. A change folder without `change.yaml` uses the
project's schema. An invalid entry of `roles` is reported by
`codev validate` as an error of the change (`unknown_role`,
`invalid_role_activation`, `role_not_disableable`).

The artifacts of a `schema.yaml` may declare their [role](../roles.md) as
data under `role:` — see
[Custom schemas](../guides/custom-schemas.md#declaring-artifact-roles).

## review.md

Path: `_codev/changes/<name>/review.md`. The change's
[review record](../guides/review-record.md), written by `/codev-propose`
(`plan.challenge`) and `/codev-apply` (`code.review`), or by hand. It is
not a planning artifact: `codev status` ignores it, `codev sync` leaves
it, `codev archive` moves it with the change.

````markdown
# Review record: add-sessions

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

No finding.

### security

#### R-security-1 Session tokens are stored in clear

```yaml
severity: high
confidence: high
location: "design.md#Decision: Sessions live in the database"
evidence: |
  The session token is stored in the `sessions` table.
recommendation: Store a SHA-256 of the token and compare digests.
history:
  - status: open
    by: security
  - status: fixed
    by: architect
    ref: design.md, the decision now stores a digest
  - status: fixed
    by: security
    note: verified
```

## code.review

### newcomer

Disabled by configuration.
````

| Element | Rule |
|---|---|
| `<!-- codev-review: <n> -->` | Optional format marker; without it, format 1. A greater format is not checked and blocks archiving |
| `## <step>` | `plan.challenge` or `code.review`, once each |
| `### <role-id>` | One per role under a step, once each. Holds findings, or the line `No finding.`, or the line `Disabled by configuration.` |
| `#### <id> <title>` | A finding: `R-<role-id>-<n>`, unique in the record, never reused, followed by one fenced `yaml` block |
| `Mode: <mode>.` | Optional, under a step: `inline` or `isolated`, how the step was reviewed. Once per step |
| `Summarized: <text>.` | Optional, under a step, any number: a file of the material the reviewers received as a summary |
| `Fallback: <reason>.` | Optional, under a role, once: why the role reviewed inline in an isolated review |

| Field | Type | Meaning |
|---|---|---|
| `severity` | `high`, `medium` or `low` | See [the scale](../roles.md#findings-severity-confidence-and-the-review-record) |
| `confidence` | `high` or `low` | A `low` finding is not presented to the human |
| `location` | string | `<path>:<line>` or `<path>:<start>-<end>` from the project root, or `<file>#<heading>` from the change folder. Relative, no `..`, no drive; `\` reads as `/`. Quote it when it holds `: ` |
| `evidence` | string | The cited text, verbatim: whole lines or a fragment of each |
| `recommendation` | string | What to do |
| `history` | list | The answers, oldest first; the last one is the status |
| `raised_by` | list, optional | Every role that raised the finding, its own role first, each once: a finding merged from several reviewers in an [isolated review](../guides/review-record.md#isolated-reviews) |

| `history[]` key | Meaning |
|---|---|
| `status` | `open`, `fixed`, `disputed`, `accepted-risk`, `deferred` or `rejected` |
| `by` | A role id, `human`, or `human:<name>` |
| `ref` | The commit or the file change — required by `fixed` after `open` or `disputed` |
| `note` | The argument (`disputed`), the justification (`accepted-risk`, `by` a human), or why a fix does not hold (`open` after `fixed`) |
| `to` | Where a `deferred` finding goes: a change or an issue. A `high` one is deferred by a human |

A field this codev does not know is a warning, `review_unknown_field`, and
is ignored: a later codev may add optional fields without changing the
format. A new status, step or required field raises the format marker
instead, which an older codev refuses to check. The transitions and the checks are in
[Review record](../guides/review-record.md#what-codev-checks).

## Decision (ADR)

Path: `_codev/decisions/NNNN-<slug>.md`. Every `.md` file in this folder is
parsed as a decision.

```markdown
---
id: "0003"
title: "Use SQLite for persistence"
status: accepted
date: 2026-09-30
tags: [storage]
supersedes: ["0001"]
---

## Context

## Decision

## Consequences

## Alternatives considered
```

| Field | Required | Meaning |
|---|---|---|
| `id` | yes | Short id, conventionally four digits; quote it so YAML keeps it a string |
| `title` | yes | Title |
| `status` | yes | `accepted`, `superseded`, `proposed`, `deprecated` or `rejected` |
| `date` | yes | `YYYY-MM-DD` |
| `tags` | no | List of tags |
| `supersedes` | no | Id, or list of ids, of the decisions this one replaces |
| `deviates_from` | no | List of qualified ids of inherited decisions this one departs from |

The frontmatter is delimited by `---` lines at the very start of the file. The
body is split into `##` sections; the template's four sections are a
convention, not a requirement. Only `accepted` decisions that are not
superseded or deviated from are in effect, and `supersedes` and
`deviates_from` only take effect on an `accepted` decision: a `proposed`
supersession or deviation changes nothing until `codev decision accept`.

Files saved with Windows line endings (CRLF) are read like any other. When
codev rewrites a decision's status — `codev decision accept`, or a
predecessor marked `superseded` — it keeps the file's line endings and leaves
the body byte for byte as it was, so the seal stays valid.

## seal.yaml

Path: `_codev/decisions/seal.yaml`. Maintained by codev; do not edit it by
hand.

```yaml
version: 1
seals:
- id: '0001'
  bodySha256: sha256:fad1f272bf13eb44f97aba1bc22f36f294503d1a2471bf12ab4bfc6ee7b4d7f6
  sealedAt: 2026-09-30
```

Each entry records the SHA-256 of a decision's body — everything after the
closing `---` of the frontmatter, byte for byte — and the date it was sealed.
Commit this file with your decisions.

## codev.lock

Path: `_codev/codev.lock`. Written only by `codev sources update`, in TOML.

```toml
version = 1

[[source]]
git = "git@github.com:acme/codev-standards.git"
ref = "main"
commit = "12958274cbed0eb66985f57d3272c6d3134b672d"
resolved_at = "2026-09-30"
```

One `[[source]]` entry per `git:` source, with the `ref` it follows, the
commit it is pinned to, and, when set, its `subpath`. Commit this file.

## SKILL.md

Path: `.claude/skills/codev-<workflow>/SKILL.md`. Generated by `codev init`
and `codev update`:

```markdown
---
name: codev-explore
description: "Explore an idea, investigate a problem or clarify a need before creating a codev change. …"
allowed-tools: "Bash(codev:*), Read, Glob, Grep"
license: MIT
metadata:
  generator: codev
  version: "0.6.1"
---

<workflow instructions>
```

`metadata.version` is how `codev update` tells a skill from an older version
(regenerated) from a skill edited by hand (preserved unless `--force`). A
skill this version rendered for the other [review mode](configuration.md#review)
— with or without `Agent` in `allowed-tools` — is regenerated too.

## Validation codes

Every finding reported by `codev validate` carries a stable code. Messages
may be reworded between versions; codes are not.

| Code | Severity | Meaning |
|---|---|---|
| `spec_purpose_missing` | error | A main spec has no `## Purpose` |
| `spec_no_requirement` | error | A main spec has no requirement |
| `requirement_outside_section` | error | A `### Requirement:` outside `## Requirements` |
| `delta_header_in_main_spec` | error | A delta heading in a main spec |
| `requirement_no_shall` | error | A requirement without `SHALL`, `MUST`, `DOIT` or `DOIVENT` |
| `requirement_no_scenario` | error | A requirement without a scenario |
| `scenario_wrong_heading_level` | error | A scenario written with three `#` instead of four |
| `duplicate_requirement` | error | The same requirement twice in one delta section |
| `delta_unexpected_heading` | error | A `###` heading in a delta section that is not `### Requirement:`, such as a translated keyword |
| `cross_section_conflict` | error | The same requirement in two of `ADDED`, `MODIFIED`, `REMOVED` |
| `rename_source_missing` | error | A `RENAMED` source names no requirement of the main spec |
| `rename_target_collision` | error | A `RENAMED` target is also `ADDED` |
| `modified_uses_old_name` | error | `MODIFIED` uses the old name of a renamed requirement |
| `zero_delta_without_marker` | error | A change has no delta and does not declare `skip_specs: true` |
| `skip_specs_conflict` | error | A change declares `skip_specs: true` but has deltas |
| `decision_missing_frontmatter` | error | A file in `_codev/decisions/` has no frontmatter |
| `decision_missing_field` | error | A decision lacks a required field, or has an unknown one |
| `decision_unknown_status` | error | A decision has an unknown status |
| `decision_field_type_mismatch` | error | A frontmatter field has the wrong shape |
| `decision_seal_mismatch` | error | A sealed decision's body was edited |
| `decision_conflicting_deviations` | error | Two accepted local decisions deviate from the same inherited one |
| `release_note_invalid` | error | The proposal's `## Release note` has an unknown category, an item without text, or is empty |
| `release_note_missing` | warning | An active change's proposal has no `## Release note` section |
| `unknown_domain` | error | The proposal's `## Domains` names a domain outside the six |
| `domains_invalid` | error | The proposal's `## Domains` has a line that is neither an item nor `None`, or `None` next to a ticked domain |
| `domains_missing` | warning | An active change's proposal has no `## Domains` section |
| `domain_suggested` | warning | The change's diff suggests a domain the proposal neither ticks nor keeps unticked with a reason |
| `invalid_trigger` | error | A group of `triggers:` in `_codev/config.yaml` has no known domain, no trigger, or a glob or expression that does not compile (a warning for a key codev does not know) |
| `review_invalid` | error | The review record's layout, a finding's field, id or location, a status or a transition is wrong |
| `review_decision_incomplete` | error | A history entry lacks its `ref`, `note` or `to`, or a risk is accepted, or a high finding deferred, by someone other than a human |
| `review_location_missing` | error | A finding cites a file, a line or a heading that does not exist |
| `review_evidence_not_found` | error | A finding's evidence is nowhere in the cited file or section |
| `review_role_missing` | error | A role active at a recorded step has no section, or one that says it is disabled |
| `review_evidence_moved` | warning | A finding's evidence is in the file, more than 5 lines away from the cited lines |
| `review_evidence_unchecked` | warning | The cited file is not UTF-8 text, holds NUL bytes, or is larger than 4 MiB: the evidence is not compared |
| `review_unknown_section` | warning | A step or a role the record should not hold; it is ignored |
| `review_unknown_field` | warning | A finding field this codev does not know; it is ignored |
| `review_format_unsupported` | warning | The record comes from a newer codev; nothing in it is checked |
| `delta_section_empty` | warning | A delta section has no entry, so nothing of it is merged |
| `decision_unsealed` | warning | An accepted or superseded local decision has no seal |
| `unknown_config_key` | warning | `_codev/config.yaml` holds a top-level or `mcp` key this codev does not know — see [Unknown keys](configuration.md#unknown-keys) |
| `decision_orphan_seal` | warning | A seal has no matching decision |
| `decision_supersedes_unknown` | warning | `supersedes` names an unknown decision |
| `decision_id_collision` | warning | A local and an inherited decision share an id |
| `decision_dangling_deviation` | warning | `deviates_from` names a decision that is not indexed |
| `decision_supersession_cycle` | warning | Supersessions form a cycle; none of the decisions involved is in effect |
