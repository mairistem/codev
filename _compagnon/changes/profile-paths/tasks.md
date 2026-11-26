# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, and `mdbook build docs/en` and
`mdbook build docs/fr` succeeding — except, from task 1.2 until task 2.4,
the tests 1.2 adds or changes, which are red until the pages exist. Each
page is written in both editions, the French one following the lexicon of
the documentation (*vouvoiement*, no anglicism).

## 1. The checks

- [x] 1.1 In `crates/compagnon-cli/src/docs_checks.rs`, add
  `choose_path_problems` and `path_page_problems`, the per-edition profile
  labels and path page paths, and a gloss check for developer chapters,
  with unit tests on synthetic pages:
  `a_profile_with_two_links_or_the_wrong_page_is_caught`,
  `a_step_labelled_differently_from_its_page_is_caught`,
  `a_path_page_with_four_steps_is_caught`,
  `a_gloss_for_developers_is_caught`, `a_home_page_that_asks_after_installing_is_caught`,
  `a_readme_missing_a_path_page_is_caught`; remove the five-profile
  check — `profile_problems`, `profile_headings`, the unit test
  `a_link_outside_the_table_of_contents_is_caught` and the chapter test
  `choose_your_path_links_five_profiles_to_chapters`; verified by these
  tests passing.
- [x] 1.2 Update the chapter tests: `START_HERE` and
  `both_editions_open_with_the_start_here_path` (the four path pages, the
  quickstart at the head of Everyday use), `the_start_here_path_stays_under_its_caps`
  (Choose your path 200/250, each path page 250/310),
  `the_manual_starts_with_the_start_here_path`, `EVERYDAY` (gains the four
  path pages) and `the_everyday_list_follows_the_summary`, and new
  `the_introduction_links_the_four_paths`,
  `the_build_path_asks_no_command_beyond_installing`,
  `every_link_of_the_paths_resolves`,
  `both_editions_ask_the_question_with_the_four_profiles`,
  `every_path_page_has_at_most_three_steps_titled_as_their_pages`,
  `developer_pages_do_not_gloss_everyday_words`; in
  `crates/compagnon-cli/tests/repository.rs`,
  `both_home_pages_lead_with_the_four_profiles` (with `lang="fr"` on
  `fr.html` and each page's link to the other carrying its `lang` and
  `hreflang`, the contributor link, the error case), the README test
  `both_readmes_stay_short_keep_the_one_liners_and_link_the_profiles`
  (linking the four path pages), and `inline_marks_are_named_or_decorative`
  and `site_and_introductions_name_compagnon` extended to
  `docs/site/fr.html`; verified by these tests failing until
  group 2 (the red state is shown in the pull request).

## 2. The pages

- [x] 2.1 Write `choose-your-path.md` (the question, the four labels, the
  contributor sentence) and the four `paths/*.md` pages of the design's
  table, in both editions, each titled with its profile's label, and
  update both `SUMMARY.md` (entries titled the same); verified by
  `both_editions_ask_the_question_with_the_four_profiles`,
  `every_path_page_has_at_most_three_steps_titled_as_their_pages`,
  `both_editions_open_with_the_start_here_path`,
  `the_start_here_path_stays_under_its_caps`,
  `the_build_path_asks_no_command_beyond_installing`,
  `the_everyday_list_follows_the_summary`,
  `every_everyday_chapter_explains_its_terms` and
  `the_french_everyday_pages_hold_no_anglicism`.
- [x] 2.2 The introduction's "Where to start" uses the four labels, linking
  the path pages; fix every link whose label differs from the chapter it
  opens ("Step by step" → the workflow chapter's title); verified by
  `the_introduction_links_the_four_paths`,
  `the_start_here_path_stays_under_its_caps`,
  `the_introduction_says_who_decides` and a search for "Step by step" /
  « Étape par étape » returning no link label.
- [x] 2.3 Remove the parenthesised explanations of "terminal" and "test"
  from the workflow, skills, concepts and FAQ chapters, both editions;
  verified by `developer_pages_do_not_gloss_everyday_words` and
  `every_everyday_chapter_explains_its_terms`.
- [x] 2.4 The home pages: `docs/site/index.html` leads with the question
  and the four profiles linking `en/paths/…`; new `docs/site/fr.html`, in
  French, the same structure linking `fr/paths/…`; each links the other;
  the READMEs' "Find your way" / « Trouver votre chemin » use the four
  labels linking the path pages; verified by
  `both_home_pages_lead_with_the_four_profiles`,
  `both_readmes_stay_short_keep_the_one_liners_and_link_the_profiles`, the
  existing `inline_marks_are_named_or_decorative` and
  `site_and_introductions_name_compagnon`, and a manual check recorded in
  the pull request that both home pages have no horizontal scroll at
  375 px and work without script — manual because layout is a visual
  judgement.

## 3. Check

- [ ] 3.1 Have one reader per profile — the four profiles and a
  contributor, in French — start from the French home page and say which
  profile they chose, how fast, and whether their path gave them what they
  came for. It passes when each reader picks the intended profile on the
  first try and reaches its three steps; what blocks them within the
  pages this change writes is fixed here, and any wish for a new page is
  recorded for the following change, not written here; recorded in the
  pull request. Manual
  because recognising oneself is a judgement; the structure it relies on
  is tested by the checks of group 1.
