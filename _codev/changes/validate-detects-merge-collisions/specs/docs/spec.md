## ADDED Requirements

### Requirement: Parallel changes are documented in both languages

The documentation of each language SHALL hold a "Parallel changes" guide —
the file `guides/parallel-changes.md`, listed in its table of contents —
that states the rule of the project's workflow: a change is archived inside
its own pull request, as its last commit, after its branch was updated
from the default branch, and CI runs `codev validate --strict` on the
result of the merge. The guide MUST describe what `codev validate` reports
after two branches meet and how to resolve each case: a duplicate
requirement, a duplicate decision id and its renumbering, a duplicate
seal, a conflict marker, a `MODIFIED` whose target is gone, an `ADDED`
already present, and the conflicts of the generated files — the seal file,
the `CHANGELOG.md` managed section and the skills — resolved by
regenerating them. It MUST say that a pipeline running on the branch
alone, rather than on its merge result, does not see these collisions,
and that a merge result is only current when the forge requires branches
to be up to date before merging or uses a merge queue; without either, a
collision between two pull requests merged one after the other is caught
only on the default branch, after the merge. The JSON output reference
MUST list `merge_conflict_marker` among the command error codes.

#### Scenario: The guide exists in both languages

- **GIVEN** the repository's `docs/en` and `docs/fr`
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails if either table of contents does not list
  `guides/parallel-changes.md`

#### Scenario: Every new finding code is documented

- **GIVEN** the codes `merge_conflict_marker`, `modified_target_missing`,
  `added_target_exists`, `decision_id_duplicate` and
  `decision_seal_duplicate`
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails if one of them is missing from the guide or from the
  finding-code table of `reference/file-formats.md`, in either language
- **AND** it fails if `merge_conflict_marker` is missing from the error
  table of `reference/json-output.md`, in either language
