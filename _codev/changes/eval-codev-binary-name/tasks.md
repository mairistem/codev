# Tasks

## 1. The check

- [x] 1.1 In `evals/src/runner.rs`, a pure `codev_name_problem(path)` and its call in `preflight`, which then fails with exit code 2 before any case runs; verified by the unit test `a_codev_binary_under_another_name_is_refused` (scenario "A codev binary under another name")

## 2. Documentation

- [x] 2.1 `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`: the binary frozen for a comparison keeps the name `codev`, in a folder of its own; verified by the suite's test `contributing_chapters_say_the_frozen_binary_keeps_its_name`

## 3. The implementation baseline again

- [ ] 3.1 Record the `code.implement` baseline again on Opus with the frozen binary named `codev`; verified by `every_case_has_a_baseline_observation` and by no run's transcript holding `unrecognized subcommand`

## 4. Checks

- [ ] 4.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`; verified by all four passing
