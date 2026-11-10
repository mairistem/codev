## MODIFIED Requirements

### Requirement: A delta is checked against its main spec

For each delta of a change, validation SHALL read the main spec it targets
and report as an error:

- `rename_source_missing`, every `RENAMED` whose `FROM` names no
  requirement of that spec — unless the `TO` requirement already exists
  there, which means an earlier sync already applied the rename;
- `modified_target_missing`, every `MODIFIED` requirement that spec does
  not hold — unless the same delta renames a requirement of that spec to
  this name; the message names the requirement and says it was removed or
  renamed since the change was written, by another change or by hand;
- `added_target_exists`, every `ADDED` requirement whose name that spec
  already holds with a different content — description, scenario names and
  scenario text compared whatever the language of the keywords and the
  line endings; the message names the requirement and tells to turn the
  addition into a `MODIFIED` or to rename it. The same content means an
  earlier sync already added it, which is not an error.

A `REMOVED` requirement the spec no longer holds is not reported: the
outcome it asks for is already there. The structural defects of a targeted
main spec that make the merge ambiguous — `duplicate_requirement` and
`merge_conflict_marker` — are also reported as errors of the change, so
that `codev validate <change>` and the pre-flight of sync and archive see
them.

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

#### Scenario: A requirement modified here was removed by another change

- **GIVEN** an active change whose delta modifies `Session Expiration`
- **AND** a main spec that no longer holds `Session Expiration`, since
  another branch archived its removal and was merged
- **WHEN** the user runs `codev validate <change>`
- **THEN** an error `modified_target_missing` names `Session Expiration`
- **AND** the exit code is non-zero

#### Scenario: Modifying a requirement the same delta renames

- **GIVEN** a main spec holding `Session Expiration`
- **AND** a delta renaming `Session Expiration` to `Session Timeout` and
  modifying `Session Timeout`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `modified_target_missing` is reported

#### Scenario: A requirement added here was added by another change

- **GIVEN** an active change whose delta adds `Export` with one scenario
- **AND** a main spec that already holds an `Export` requirement with a
  different description, merged from another branch
- **WHEN** the user runs `codev validate <change>`
- **THEN** an error `added_target_exists` names `Export`
- **AND** the exit code is non-zero

#### Scenario: A requirement added here was already synced

- **GIVEN** a French delta adding `### Exigence : Export` with CRLF line
  endings, already synced into an English main spec that now holds
  `### Requirement: Export` with the same description and scenarios and LF
  line endings
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `added_target_exists` is reported

#### Scenario: A targeted spec holding a duplicate fails the change

- **GIVEN** a main spec holding `Login` twice
- **AND** an active change whose delta modifies `Login`
- **WHEN** the user runs `codev validate <change>`
- **THEN** an error `duplicate_requirement` is reported for the change,
  naming the main spec and both lines

#### Scenario: A requirement removed here is already gone

- **GIVEN** a delta removing `Remember Me`
- **AND** a main spec that no longer holds `Remember Me`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no finding is reported about `Remember Me`

## ADDED Requirements

### Requirement: Git conflict markers left in planning files are reported

`codev validate` SHALL report, as the error `merge_conflict_marker` at its
line, every line that starts with seven `<`, seven `|` or seven `>`
followed by a space or the end of the line — the markers git leaves in a
file whose merge conflicted — in the markdown files it reads: the main
specs, the files of each active change, and the local decisions. A marker
inside a fenced code block is not reported — a conflict inside a fenced
example is therefore not detected — and a line of seven `=` alone
is never reported, since markdown uses it as a heading underline.

When `_codev/config.yaml`, `_codev/decisions/seal.yaml` or an active
change's `change.yaml` fails to parse and holds such a marker, `codev validate` — like every command that reads
that file — MUST fail with the code `merge_conflict_marker`, naming the
file and the line, rather than with a YAML parse error, since no check can
run on a file it cannot read.

#### Scenario: A conflict left in a main spec

- **GIVEN** a main spec whose line 40 is `<<<<<<< HEAD` and line 52 is
  `>>>>>>> feat/export`
- **WHEN** the user runs `codev validate`
- **THEN** two errors `merge_conflict_marker` are reported for that spec,
  at lines 40 and 52
- **AND** the exit code is non-zero

#### Scenario: A conflict left in an active change's design

- **GIVEN** an active change whose `design.md` holds a line
  `||||||| merged common ancestors`
- **WHEN** the user runs `codev validate <change>`
- **THEN** an error `merge_conflict_marker` is reported for that change at
  the line

#### Scenario: A conflict left in a local decision

- **GIVEN** a local decision whose line 20 is `>>>>>>> feat/sign-releases`
  outside any code block
- **WHEN** the user runs `codev validate`
- **THEN** an error `merge_conflict_marker` is reported for the decisions
  at line 20 of that file

#### Scenario: Markers shown in a code block are not reported

- **GIVEN** a decision whose body shows `<<<<<<< HEAD` inside a fenced
  code block
- **WHEN** the user runs `codev validate`
- **THEN** no `merge_conflict_marker` is reported

#### Scenario: A heading underline is not a marker

- **GIVEN** a design holding a heading underlined with `=======`
- **WHEN** the user runs `codev validate`
- **THEN** no `merge_conflict_marker` is reported

#### Scenario: A conflict left in the seal file

- **GIVEN** a project whose `_codev/decisions/seal.yaml` holds
  `<<<<<<< HEAD` at line 9
- **WHEN** the user runs `codev validate --json`
- **THEN** the command fails with the code `merge_conflict_marker`
- **AND** its message names `_codev/decisions/seal.yaml` and line 9

#### Scenario: A conflict left in the project configuration

- **GIVEN** a project whose `_codev/config.yaml` holds `<<<<<<< HEAD` at
  line 3 and no longer parses
- **WHEN** the user runs `codev validate`
- **THEN** the command fails with the code `merge_conflict_marker`
- **AND** its message names `_codev/config.yaml` and line 3

#### Scenario: A conflict left in a change's metadata

- **GIVEN** an active change whose `change.yaml` holds `>>>>>>> main` at
  line 4 and no longer parses
- **WHEN** the user runs `codev validate`
- **THEN** the command fails with the code `merge_conflict_marker`
- **AND** its message names that `change.yaml` and line 4
