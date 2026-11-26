# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing (but group 1, whose red state is
shown in the pull request).

## 1. The checks

- [x] 1.1 In `crates/compagnon-agents/src/workflows.rs`, add `explore` and
  `onboard` to `PLAIN_WORDS_SKILLS`; rename the tests that count them
  (`the_eight_skills_include_the_next_steps_rule`,
  `every_slash_command_in_the_eight_skills_is_a_shortcut`,
  `the_eight_skills_still_name_their_commands`, the latter listing for
  onboard `/compagnon-propose`, `/compagnon-configure`, `/compagnon-apply`,
  `/compagnon-explore`, `compagnon init`, `compagnon doctor`,
  `compagnon decision list` and `compagnon list --specs`, and for explore
  `/compagnon-propose`); exempt doctor's fix lines, with the unit test
  `a_relayed_doctor_fix_is_not_a_suggestion`; add
  `explore_offers_to_plan_in_plain_words_first` and
  `onboard_recommends_configuring_in_plain_words_first` (a quoted plain
  request before the command, with `plain_request_before`),
  `onboard_closes_several_changes_on_a_question` (the several-changes
  guidance asks which one on the last line, without a shortcut) and
  `onboard_no_longer_asks_to_quote_the_command_to_type`; verified by the
  unit test passing and the others failing until group 2.

## 2. The skills

- [x] 2.1 Include `{{fragment:next-steps}}` in `assets/workflows/explore.md`
  and rewrite its outcome offer and its guardrail as a plain request with
  `/compagnon-propose` as the shortcut; verified by
  `the_eight_skills_include_the_next_steps_rule`,
  `every_slash_command_in_the_eight_skills_is_a_shortcut`,
  `the_eight_skills_still_name_their_commands`,
  `the_reply_ends_with_one_request` and
  `explore_offers_to_plan_in_plain_words_first`.
- [x] 2.2 Include the fragment in `assets/workflows/onboard.md` and rewrite
  as the design says: the opening description, the recommendation table
  (arguments kept; several changes listed, closing on a question), the
  example replies, the thin-config explanation, the non-case, the
  guardrails, the "make the suggestion actionable" paragraph, and a plain
  gloss before each relayed doctor `fix`; verified by the same tests as
  2.1, `onboard_recommends_configuring_in_plain_words_first`,
  `onboard_closes_several_changes_on_a_question`,
  `onboard_no_longer_asks_to_quote_the_command_to_type`, and the existing
  `onboard_is_in_the_catalog_with_the_right_tools`,
  `onboard_cites_its_three_blocks` and
  `onboard_body_relays_compagnon_doctor`.
- [x] 2.3 Rewrite the two descriptions in the `CATALOG` with English and
  French requests; verified by
  `every_description_names_english_and_french_requests` and the
  frontmatter tests of `crates/compagnon-agents/src/claude.rs`
  (`the_frontmatter_is_complete_and_the_body_intact`,
  `every_skill_frontmatter_is_valid_yaml`,
  `the_description_is_quoted_because_it_contains_yaml_punctuation`).
- [x] 2.4 Simplify `load_read_only` in `evals/src/case.rs` with one helper
  for `outcome` and `state`; verified by the six loading tests passing
  unchanged (`an_explore_case_that_crystallizes`,
  `an_onboard_case_with_a_request`,
  `an_explore_case_with_an_unknown_outcome`,
  `an_onboard_case_without_its_state`,
  `a_read_only_case_needs_both_lists_and_known_keys`,
  `read_only_gaps_names_each_outcome_and_state_without_a_case`).

## 3. Evaluation (decision 0008)

- [ ] 3.1 Ask the user's agreement on the cost (about 7 to 8 USD), then run
  `cargo run -p compagnon-evals -- run --claude <path to Claude Code
  2.1.289> --step idea.explore --step project.onboard --repeat 5` and
  `compare` against `evals/baseline` with `--quality-only`; read the
  `onboard-several-changes` replies' last lines for the question the
  closing rule asks; verified by `compare` reporting the quality within
  the baseline's range, its output, those last lines and the cost
  recorded in the change's `evals.md` and the pull request.
