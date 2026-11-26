## ADDED Requirements

### Requirement: A tree replayed from before the rename stays as clean as it was

When the runner brings a tree extracted from a commit that predates the
product's rename through it — for a case's fixture and for each tree
`compagnon-evals calibrate` builds — it SHALL, after renaming, record
again the seal of every accepted decision whose seal matched its body
before the rename, and leave unchanged a seal that did not match; and,
when the tree is a Rust workspace (a `Cargo.toml` at its root), it SHALL
format with rustfmt each `.rs` file the rename changed, in the edition of
the root `Cargo.toml`, leaving every other file as committed, before the
fixture's base commit. A seal file it cannot parse SHALL be left as it
is. A missing or failing rustfmt SHALL be an error of the preparation,
naming the file. A tree laid out under the current names SHALL be left as
it is.

#### Scenario: A seal broken by the rename alone holds again

- **GIVEN** a commit before the rename whose decision `0001`, sealed,
  names `codev` in its body, the seal matching
- **WHEN** the runner prepares a fixture from it
- **THEN** the seal of `0001` in the fixture records the hash of its
  renamed body, so that `compagnon validate` finds it matching

#### Scenario: A seal already broken stays broken

- **GIVEN** a commit before the rename whose decision `0002` was edited
  after its seal, the seal not matching
- **WHEN** the runner prepares a fixture from it
- **THEN** the seal of `0002` is the one the commit holds

#### Scenario: A line the rename lengthens is formatted again

- **GIVEN** a commit before the rename, formatted, a Rust workspace whose
  source holds a line that the longer name pushes past rustfmt's width
- **WHEN** the runner prepares a fixture from it
- **THEN** `rustfmt --check` passes on that file in the fixture's base
  commit

#### Scenario: A file the rename does not touch stays as committed

- **GIVEN** a commit before the rename holding a `.rs` file not formatted
  and not naming the former name
- **WHEN** the runner prepares a fixture from it
- **THEN** that file is byte for byte as committed

#### Scenario: Without rustfmt, the preparation stops and says why

- **GIVEN** a commit before the rename whose `.rs` file the rename
  changes, and no `rustfmt` on the `PATH`
- **WHEN** the runner prepares a fixture from it
- **THEN** the preparation fails with an error naming rustfmt and the file

#### Scenario: A tree under the current names is left as it is

- **GIVEN** a commit holding `_compagnon/`, a sealed decision and a `.rs`
  file not formatted
- **WHEN** the runner prepares a fixture from it
- **THEN** the seal file and the `.rs` file are as committed

#### Scenario: calibrate passes the format of a formatted merged commit

- **GIVEN** the case `implement-decision-accept-command`, whose merged
  commit passes `cargo fmt --all --check` as it was committed
- **WHEN** the user runs `compagnon-evals calibrate implement-decision-accept-command`
- **THEN** it reports `format` passing on the merged commit
