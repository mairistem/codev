# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace`, and `mdbook build docs/en` and
`mdbook build docs/fr` succeeding — except
`every_everyday_chapter_explains_its_terms`,
`the_french_everyday_pages_hold_no_anglicism` and
`the_glossary_defines_every_required_term`, which cover every chapter and
fail from task 1.2 until task 2.6, which turns them green; until then, a
task is verified by these tests reporting no problem naming its own
chapters. Each chapter is rewritten in both
editions in the same task; the French one follows the lexicon of the
design, in *vouvoiement*.

## 1. The checks, on synthetic chapters

- [x] 1.1 In `crates/compagnon-cli/src/docs_checks.rs`, add
  `unexplained_terms` and `anglicisms`, with the term lists of each
  edition (French: delta, artefact, capacité, schéma, déroulé,
  frontmatter, kebab-case), and unit tests one per scenario:
  `a_term_used_without_a_link_is_caught`,
  `a_link_to_the_top_of_the_glossary_is_caught` (a bare `glossary.md`, or
  an anchor no glossary heading produces),
  `a_term_in_code_or_under_the_hood_is_not_concerned`,
  `the_phrase_main_specs_is_caught`,
  `an_anglicism_in_prose_is_caught`,
  `the_french_verb_changer_is_not_an_anglicism`,
  `the_english_word_in_parentheses_code_or_path_is_allowed`; verified by
  these tests passing.
- [x] 1.2 Add the tests on the real chapters,
  `every_everyday_chapter_explains_its_terms` and
  `the_french_everyday_pages_hold_no_anglicism` (the ten French everyday
  chapters and `README.fr.md`), and change `glossary_terms`: English adds
  Artifact, Schema, Workflow, Frontmatter, Kebab-case; French replaces
  Change, Skill, Spec and Revue with Modification, Compétence,
  Spécification and Relecture, and adds Artefact, Schéma, Déroulé,
  Frontmatter, Kebab-case — which `the_glossary_defines_every_required_term`
  then requires, and the existing
  `a_missing_or_overlong_glossary_entry_is_caught` keeps guarding; verified by these tests failing until group 2 (the red state
  is shown in the pull request).

## 2. The rewrite, chapter by chapter, both editions

- [ ] 2.1 Glossary: the new entries (English Artifact, Schema, Workflow,
  Frontmatter, Kebab-case; French Artefact, Schéma, Déroulé, Frontmatter,
  Kebab-case), the French entries Change, Skill, Spec and Revue renamed
  Modification, Compétence, Spécification and Relecture,
  each saying the English word it stands for, every definition in
  everyday words; verified by `the_glossary_defines_every_required_term`
  and `the_french_everyday_pages_hold_no_anglicism` on the glossary.
- [ ] 2.2 Introduction, Choose your path and Your first change (the French
  title becomes « Votre première modification », in `SUMMARY.md` too);
  verified by `every_everyday_chapter_explains_its_terms`,
  `the_french_everyday_pages_hold_no_anglicism` and the existing
  `the_start_here_path_stays_under_its_caps`,
  `each_quickstart_step_leads_with_a_plain_request` and
  `both_editions_open_with_the_start_here_path`.
- [ ] 2.3 Installation and Reviewing a plan, and the French words
  `review_a_plan_covers_the_plan_without_a_terminal` looks for (« specs »
  becomes « spécification »); verified by the two new tests and that
  test.
- [ ] 2.4 The workflow and Concepts — the French title « Le workflow »
  becomes « Le déroulé », in the chapter and in `SUMMARY.md`; verified by
  the two new tests.
- [ ] 2.5 Claude Code skills — the French title « Les skills Claude Code »
  becomes « Les compétences Claude Code », in the chapter and in
  `SUMMARY.md`; its table first (« Met à jour les
  spécifications du projet avec ce que la modification apporte, sans la
  clore » for sync) and the « Comment demander » examples (« réalise la
  modification »); verified by the two new tests and the existing
  `every_skill_has_examples_of_requests`.
- [ ] 2.6 FAQ, and `README.fr.md`; verified by the two new tests and the
  existing `both_readmes_stay_short_keep_the_one_liners_and_link_the_profiles`.

## 3. Check

- [ ] 3.1 Read the ten chapters of each edition in the built site, as a
  reader who does not code: no sentence needs a term the page has not
  explained; recorded in the pull request. Manual because readability is a
  judgement; the terms it rests on are tested by
  `every_everyday_chapter_explains_its_terms`.
