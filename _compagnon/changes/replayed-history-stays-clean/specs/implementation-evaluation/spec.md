## ADDED Requirements

### Requirement: A tree replayed from before the rename stays as clean as it was

When the runner brings a tree extracted from a commit that predates the
product's rename through it — for a case's fixture and for each tree
`compagnon-evals calibrate` builds — it SHALL, after renaming, record
again the seal of every accepted decision whose seal matched its body
before the rename, and leave unchanged a seal that did not match; and,
when the tree is a Rust workspace (a `Cargo.toml` at its root), it SHALL
format it with `cargo fmt --all`, before the fixture's base commit. A
tree laid out under the current names SHALL be left as it is.

#### Scenario: A seal broken by the rename alone holds again

- **GIVEN** a commit before the rename whose decision `0001`, sealed,
  names `codev` in its body, the seal matching
- **WHEN** the runner prepares a fixture from it
- **THEN** `compagnon validate --strict` on the fixture reports no
  `decision_seal_mismatch` for `0001`

#### Scenario: A seal already broken stays broken

- **GIVEN** a commit before the rename whose decision `0002` was edited
  after its seal, the seal not matching
- **WHEN** the runner prepares a fixture from it
- **THEN** the seal of `0002` is the one the commit holds

#### Scenario: A line the rename lengthens is formatted again

- **GIVEN** a commit before the rename, formatted, a Rust workspace whose
  source holds a line that the longer name pushes past rustfmt's width
- **WHEN** the runner prepares a fixture from it
- **THEN** `cargo fmt --all --check` passes on the fixture's base commit

#### Scenario: calibrate passes the format of a formatted merged commit

- **GIVEN** the case `implement-decision-accept-command`, whose merged
  commit passes `cargo fmt --all --check` as it was committed
- **WHEN** the user runs `compagnon-evals calibrate implement-decision-accept-command`
- **THEN** it reports `format` passing on the merged commit
