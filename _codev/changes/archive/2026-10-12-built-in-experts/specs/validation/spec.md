## ADDED Requirements

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
