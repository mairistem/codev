# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, and `mdbook build docs/en` and
`mdbook build docs/fr` succeeding — except, from task 1.1 until task 2.4,
the tests 1.1 adds or changes and the everyday-pages tests its `EVERYDAY`
update affects (`the_everyday_list_follows_the_summary`,
`every_everyday_chapter_explains_its_terms`,
`the_french_everyday_pages_hold_no_anglicism`), red until the pages exist;
each page task lists its pages in both `SUMMARY.md`, so that the tests and
`mdbook build` see them. Each page is written in
both editions, the French one following the documentation's lexicon, in
*vouvoiement*.

## 1. The checks

- [x] 1.1 In `crates/compagnon-cli/src/docs_checks.rs`, add the chapter
  tests `both_editions_list_the_new_pages_where_their_readers_look` (the
  six pages in their parts; each path page linking the pages of the
  design's table; `origins.md` linking `contributing.md`),
  `every_part_of_the_example_is_annotated`,
  `the_two_pages_keep_to_what_a_non_developer_can_do`, and extend
  `review_a_plan_covers_the_plan_without_a_terminal` to the four delta
  sections under each edition's keywords (`ADDED`… in English, `AJOUTÉES`…
  in French); add the unit tests on synthetic pages
  `a_path_page_that_does_not_link_its_new_page_is_caught`,
  `a_part_of_the_example_without_its_note_is_caught`,
  `a_command_in_a_page_for_non_developers_is_caught`; update `EVERYDAY`
  for the three new everyday pages; verified
  by these tests failing until group 2 (the red state is shown in the pull
  request).

## 2. The pages

- [x] 2.1 Write `guides/example-plan.md` (listed in both `SUMMARY.md`) and add to
  `guides/review-a-plan.md` how to read a modified rule; verified by
  `every_part_of_the_example_is_annotated`,
  `review_a_plan_covers_the_plan_without_a_terminal`, the everyday pages'
  vocabulary and anglicism tests, and the new test
  `the_example_plan_parses_as_compagnon_writes_it` (each edition's spec
  block through `compagnon_core::parser::parse_delta`, no issue).
- [x] 2.2 Write `guides/approve-a-plan.md` and
  `guides/start-from-nothing.md`, listed in both `SUMMARY.md`; verified by
  `the_two_pages_keep_to_what_a_non_developer_can_do` and the everyday
  pages' tests.
- [x] 2.3 Write `guides/existing-project.md`, `guides/team-rollout.md` and
  `contributing.md`, listed in both `SUMMARY.md`, each fact checked against the code and
  `CONTRIBUTING.md`; verified by `mdbook build` and
  `every_link_of_the_paths_resolves`.
- [x] 2.4 Update the four path pages per the design's table, and
  `origins.md`; verified by
  `both_editions_list_the_new_pages_where_their_readers_look`,
  `every_path_page_has_at_most_three_steps_titled_as_their_pages`,
  `the_build_path_asks_no_command_beyond_installing`,
  `the_everyday_list_follows_the_summary` and
  `every_link_of_the_paths_resolves`.

## 3. Check

- [x] 3.1 Have the six test readers of `profile-paths` — Claire, Sophie,
  Nadia, Karim, Julie, Tom — follow their path again from the French home
  page: it passes when each reaches the page that answers the blocker they
  named before; recorded in the pull request. Manual because whether a
  page answers its reader is a judgement.
