## ADDED Requirements

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
