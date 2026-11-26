# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, and `mdbook build docs/en` and
`mdbook build docs/fr` succeeding — except, from task 1.2 until task 2.4,
the tests 1.2 adds or changes, which are red until the pages exist. Each
page is written in both editions, the French one following the lexicon of
the documentation (*vouvoiement*, no anglicism).

## 1. The checks

- [ ] 1.1 In `crates/compagnon-cli/src/docs_checks.rs`, add
  `choose_path_problems` and `path_page_problems`, the per-edition profile
  labels and path page paths, and a gloss check for developer chapters,
  with unit tests on synthetic pages:
  `a_profile_with_two_links_or_the_wrong_page_is_caught`,
  `a_step_labelled_differently_from_its_page_is_caught`,
  `a_path_page_with_four_steps_is_caught`,
  `a_gloss_for_developers_is_caught`; remove the five-profile check and
  its test; verified by these tests passing.
- [ ] 1.2 Update the chapter tests: `START_HERE` and
  `both_editions_open_with_the_start_here_path` (the four path pages, the
  quickstart at the head of Everyday use), `the_start_here_path_stays_under_its_caps`
  (Choose your path 200/250, each path page 250/310),
  `the_manual_starts_with_the_start_here_path`, and new
  `both_editions_ask_the_question_with_the_four_profiles`,
  `every_path_page_has_at_most_three_steps_titled_as_their_pages`,
  `developer_pages_do_not_gloss_everyday_words`; in
  `crates/compagnon-cli/tests/repository.rs`,
  `both_home_pages_lead_with_the_four_profiles` and the README test
  linking the four path pages; verified by these tests failing until
  group 2 (the red state is shown in the pull request).

## 2. The pages

- [ ] 2.1 Write `choose-your-path.md` (the question, the four labels, the
  contributor sentence) and the four `paths/*.md` pages of the design's
  table, in both editions, and update both `SUMMARY.md`; verified by
  `both_editions_ask_the_question_with_the_four_profiles`,
  `every_path_page_has_at_most_three_steps_titled_as_their_pages`,
  `both_editions_open_with_the_start_here_path`,
  `the_start_here_path_stays_under_its_caps`,
  `every_everyday_chapter_explains_its_terms` and
  `the_french_everyday_pages_hold_no_anglicism`.
- [ ] 2.2 The introduction's "Where to start" uses the four labels, linking
  the path pages; fix every link whose label differs from the chapter it
  opens ("Step by step" → the workflow chapter's title); verified by
  `the_start_here_path_stays_under_its_caps`,
  `the_introduction_says_who_decides` and a search for "Step by step" /
  « Étape par étape » returning no link label.
- [ ] 2.3 Remove the parenthesised explanations of "terminal" and "test"
  from the workflow, skills, concepts and FAQ chapters, both editions;
  verified by `developer_pages_do_not_gloss_everyday_words` and
  `every_everyday_chapter_explains_its_terms`.
- [ ] 2.4 The home pages: `docs/site/index.html` leads with the question
  and the four profiles linking `en/paths/…`; new `docs/site/fr.html`, in
  French, the same structure linking `fr/paths/…`; each links the other;
  the READMEs' "Find your way" / « Trouver votre chemin » use the four
  labels linking the path pages; verified by
  `both_home_pages_lead_with_the_four_profiles`, the README test, the
  existing `inline_marks_are_named_or_decorative` and
  `site_and_introductions_name_compagnon`, and a manual check recorded in
  the pull request that both home pages have no horizontal scroll at
  375 px and work without script — manual because layout is a visual
  judgement.

## 3. Check

- [ ] 3.1 Have one reader per profile — the four profiles and a
  contributor, in French — start from the French home page and say which
  profile they chose, how fast, and whether their path gave them what they
  came for; fix what blocks them; recorded in the pull request. Manual
  because recognising oneself is a judgement; the structure it relies on
  is tested by the checks of group 1.
