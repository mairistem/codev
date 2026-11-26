# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing.

## 1. The fixture

- [ ] 1.1 In `bring_through_rename` (`evals/src/runner.rs`), carry the seals
  through the rename as the design says; extend
  `a_case_pinned_before_the_rename_is_migrated_before_it_runs`
  (`evals/tests/fixture.rs`) with a sealed decision naming the former name
  whose seal matches, and one edited after its seal; verified by that test
  asserting the first seal holds the renamed body's hash and the second is
  unchanged (scenarios "A seal broken by the rename alone holds again"
  and "A seal already broken stays broken"); add
  `an_unparsable_seal_file_is_left_as_it_is` and
  `a_tree_under_the_current_names_is_left_as_it_is` (seal file and an
  unformatted `.rs` file unchanged) in `evals/tests/fixture.rs`.
- [ ] 1.2 Format the Rust files the rename changed, as the design says;
  extend the same test with a `Cargo.toml` (edition 2024) and a `.rs`
  file formatted under the former name whose line the new name pushes
  past the width, and an unformatted `.rs` file not naming it, declared
  from the first by `mod untouched;`; verified by
  `rustfmt --check --edition 2024` passing on the first and the second
  byte for byte as committed (scenarios "A line the rename lengthens is
  formatted again" and "A file the rename does not touch stays as
  committed"), and by `without_rustfmt_the_preparation_fails` (a `PATH`
  without rustfmt; the error names rustfmt and the file).

## 2. Check

- [ ] 2.1 Run `cargo run -p compagnon-evals -- calibrate
  implement-decision-accept-command` (no model, no cost); verified by its
  output reporting `format on the merged commit: passes` (scenario
  "calibrate passes the format of a formatted merged commit"), pasted in
  the pull request, with every other implementation case that has a
  merged commit calibrated the same way. Manual because it replays the
  real history and builds each tree with cargo, several minutes per case;
  the behaviour it relies on is tested by 1.1 and 1.2.
