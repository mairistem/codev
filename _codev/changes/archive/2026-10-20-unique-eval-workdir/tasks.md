# Tasks

## 1. The working folder

- [x] 1.1 In `evals/src/runner.rs`, a pure `default_work_dir(temp, secs, pid)` naming the folder `codev-evals-<secs>-<pid>`, used by `main.rs` when `--work` is not given; verified by the unit test `two_runs_started_in_the_same_second_get_their_own_folder`

## 2. Checks

- [x] 2.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked` and `codev validate --strict`; verified by all four passing
