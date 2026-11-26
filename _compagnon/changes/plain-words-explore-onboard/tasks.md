# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing.

## 1. The checks

- [ ] 1.1 In `crates/compagnon-agents/src/workflows.rs`, add `explore` and
  `onboard` to `PLAIN_WORDS_SKILLS`, rename the tests that count them
  (`the_eight_skills_include_the_next_steps_rule`,
  `every_slash_command_in_the_eight_skills_is_a_shortcut`,
  `the_eight_skills_still_name_their_commands`, the latter listing
  `/compagnon-propose`, `/compagnon-configure`, `/compagnon-apply`,
  `/compagnon-explore` and `compagnon init` for onboard and
  `/compagnon-propose` for explore), and exempt doctor's fix lines with
  the unit test `a_relayed_doctor_fix_is_not_a_suggestion`; verified by
  that test passing and the renamed ones failing until group 2 (the red
  state shown in the pull request).

## 2. The skills

- [ ] 2.1 Include `{{fragment:next-steps}}` in `assets/workflows/explore.md`
  and rewrite its outcome offer and its guardrail as a plain request with
  `/compagnon-propose` as the shortcut; verified by
  `the_eight_skills_include_the_next_steps_rule`,
  `every_slash_command_in_the_eight_skills_is_a_shortcut` and
  `the_eight_skills_still_name_their_commands`.
- [ ] 2.2 Include the fragment in `assets/workflows/onboard.md` and
  rewrite its recommendation table, example replies and guardrails as
  plain requests with their shortcuts (arguments kept), closing on one
  request; verified by the same three tests and the existing onboard tests
  (`onboard_is_in_the_catalog_with_the_right_tools`).
- [ ] 2.3 Rewrite the two descriptions in the `CATALOG` with English and
  French requests; verified by `every_description_names_english_and_french_requests`
  and the frontmatter tests of `crates/compagnon-agents/src/claude.rs`.
- [ ] 2.4 Simplify `load_read_only` in `evals/src/case.rs` with one helper
  for `outcome` and `state`; verified by the four loading tests of
  `explore-onboard-evaluation` passing unchanged.

## 3. Evaluation (decision 0008)

- [ ] 3.1 Ask the user's agreement on the cost (about 8 USD), then run
  `cargo run -p compagnon-evals -- run --claude <2.1.289> --step
  idea.explore --step project.onboard --repeat 5` and `compare` against
  `evals/baseline` with `--quality-only`; verified by `compare` reporting
  the quality within the baseline's range, its output and the cost
  recorded in the change's `evals.md` and the pull request.
