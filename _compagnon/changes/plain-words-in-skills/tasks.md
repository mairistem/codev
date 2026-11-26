# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing.

## 1. The checks, on synthetic text

- [x] 1.1 In `crates/compagnon-agents/src/workflows.rs`, add pure checks
  returning the offending items: the lines of a body naming `/compagnon-`
  without the word `shortcut` (the `# written by /compagnon-configure`
  marker excepted), and the descriptions lacking a `"…"` request, a `« … »`
  request, or longer than 1024 characters; with unit tests on synthetic
  input `a_slash_command_not_given_as_a_shortcut_is_caught`,
  `the_configure_marker_is_not_a_suggestion` and
  `a_description_without_a_french_request_is_caught` (which asserts the
  workflow's id is named); verified by these tests passing.
- [x] 1.2 Add the tests applying them to the catalog:
  `the_six_skills_include_the_next_steps_rule`,
  `every_slash_command_in_the_six_skills_is_a_shortcut`,
  `every_description_names_english_and_french_requests`,
  `apply_suggests_archiving_in_plain_words_first` (the rendered apply body
  holds a quoted plain request before `/compagnon-archive` on its
  closing-summary line),
  `apply_refusal_suggests_finishing_the_plan_in_plain_words_first` (the
  same before `/compagnon-propose` on its planning-incomplete refusal),
  `archive_and_sync_refusals_offer_the_errors_in_plain_words` (a quoted
  plain request before `compagnon validate <name>` in both bodies), and
  `the_six_skills_still_name_their_commands` (each rendered body still
  names the commands it named before: `/compagnon-propose`,
  `/compagnon-archive`, `/compagnon-apply`, `/compagnon-configure`,
  `compagnon validate <name>`, `compagnon review --change <name>`, and the
  phrase "No changes made" in configure, as main had them); verified by
  them failing until group 2, except the last, which passes at once (the
  red state is shown in the pull request).

## 2. The skills

- [x] 2.1 Write `assets/workflows/fragments/next-steps.md` (the rule of the
  design: plain words first, in the user's language, the command in
  parentheses as a shortcut, never removed) and register it in
  `workflows.rs`; include it in `propose`, `apply`, `update`, `sync`,
  `archive` and `configure`; verified by
  `the_six_skills_include_the_next_steps_rule` and the existing
  `every_workflow_has_a_usable_body_and_description`.
- [x] 2.2 Rewrite every suggested step, refusal and fix of the six bodies
  as plain request then shortcut, keeping each command they named — the
  apply refusals and closing summary, the archive and sync refusals (still
  naming `compagnon validate <name>` and `compagnon review --change
  <name>` as the shortcut after a plain offer to show the errors, the
  "reply **exactly**" lines rewritten to the modified requirement), the
  sync suggestion to archive, dropping `/compagnon-continue`, which names
  no skill, the update refusals and next
  actions, the configure messages — and name the step rather than the
  command in boundary prose; verified by
  `every_slash_command_in_the_six_skills_is_a_shortcut`,
  `apply_suggests_archiving_in_plain_words_first`,
  `apply_refusal_suggests_finishing_the_plan_in_plain_words_first`,
  `archive_and_sync_refusals_offer_the_errors_in_plain_words`,
  `the_six_skills_still_name_their_commands` and
  `configure_declined_says_no_changes_made_word_for_word` (the declined
  configure reply begins with "No changes made." word for word, the
  request to retry on its own line: run 4.1 showed the rule rewording
  the sentence when the request sat inside it).
- [x] 2.3 Rewrite the descriptions of the six skills in the `CATALOG`
  (`explore` and `onboard` unchanged): what the skill
  does, then `Use for requests like "…", « … »` with requests specific to
  the step; verified by `every_description_names_english_and_french_requests`
  and the existing frontmatter tests of `crates/compagnon-agents/src/claude.rs`
  (`the_description_is_quoted_because_it_contains_yaml_punctuation`).

- [x] 2.4 Make the French requests follow the lexicon of the French
  documentation (change `plain-language-docs`): extend
  `description_problems` to report a « … » request holding change(s),
  skill(s), spec(s) or workflow as a noun, with the unit test
  `a_french_request_holding_an_anglicism_is_caught`; rewrite the French
  requests of the six descriptions (« réalise la modification »,
  « archive la modification »…); add the lexicon to the `next-steps`
  fragment for requests said in French; verified by that test,
  `every_description_names_english_and_french_requests` and
  `the_six_skills_include_the_next_steps_rule`.

- [x] 2.5 Add to the `next-steps` fragment the closing rule (the next step
  alone on the last line: one quoted request with its shortcut, never a
  list of options), and end the propose summary with "say “the plan is
  good, build it” (shortcut: `/compagnon-apply`)"; verified by the new tests
  `the_reply_ends_with_one_request` and
  `propose_closes_on_the_request_to_build`, and
  `every_slash_command_in_the_six_skills_is_a_shortcut`. Asked by the
  user so that Claude Code's suggested next prompt is that request.

## 3. The init hint and the documentation

- [x] 3.1 In `crates/compagnon-cli/src/render.rs`, write the closing hint of
  `compagnon init` as the modified requirement gives it, thin and not
  thin; update `setup_thin_config_with_configure_points_to_configure`
  (the plain request "configure Compagnon for this project" comes before
  `/compagnon-configure`, and a plain request before
  `/compagnon-propose <an-idea>`) and
  `setup_non_thin_config_does_not_cite_configure` (a plain request to plan
  a first change before `/compagnon-propose`), and add
  `setup_without_propose_names_an_installed_skill` (only `explore` and
  `onboard` installed: `/compagnon-explore` as the shortcut, neither
  `/compagnon-propose` nor `/compagnon-configure`); the JSON output stays
  unchanged, as the existing init JSON tests check; verified by these
  tests.
- [x] 3.2 Update the init output shown in the "Under the hood" block of
  `docs/en/src/quickstart.md` and `docs/fr/src/quickstart.md`, from a real
  run of `compagnon init --yes`, and the "Archiving" step of
  `docs/en/src/roles.md` and `docs/fr/src/roles.md` (say "archive it",
  `/compagnon-archive` as the shortcut); verified by
  `the_quickstart_runs_no_command_outside_under_the_hood`,
  `under_the_hood_blocks_are_well_formed` and
  `the_start_here_path_stays_under_its_caps` in
  `crates/compagnon-cli/src/docs_checks.rs`, and `mdbook build docs/en`
  and `docs/fr` succeeding.

## 4. Evaluation (decision 0008)

- [ ] 4.1 Ask the user's agreement on the estimated cost, then run
  `cargo run -p compagnon-evals -- run` on the steps `plan.update`,
  `code.implement`, `change.sync`, `change.archive` and
  `configure.enrich` — `plan.write` left out by the user's decision on
  the cost (about 140 USD for three repetitions), the propose body
  gaining only the shared rule — and `compare` against the committed
  baseline;
  verified by `compare` reporting the quality within the baseline's range,
  its output pasted in the pull request, and any cost rise reported with
  its numbers.
- [ ] 4.2 In a Claude Code session on a scratch project, ask in French to
  implement a change whose tasks are done, then to implement a change
  whose plan is incomplete: the closing summary suggests archiving in
  French, the refusal suggests finishing the plan in French, each with its
  shortcut; verified by the two replies quoted in the pull request.
  Manual because the language of the reply is the model's, and the suite
  has no French case.
