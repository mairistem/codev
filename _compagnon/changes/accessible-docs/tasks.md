# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing, and `mdbook build docs/en` and
`mdbook build docs/fr` succeeding. Every chapter is written in both
languages, the French one in *vouvoiement*, to its own word cap.

## 1. The checks, tested on synthetic inputs

- [ ] 1.1 Add to `crates/compagnon-cli/src/docs.rs` pure check functions —
  word count outside fenced code and `<details>` blocks; Start here part
  and part order of a summary; shell commands outside `<details>`;
  glossary terms and definition caps; profile sections and links of
  Choose your path; shell blocks of a guide; `<details>` blocks (closed,
  summary prefix per language, no repeated summary) — each returning the
  offending items, with unit tests on synthetic inputs, one per scenario:
  `words_outside_code_and_under_the_hood_are_counted`,
  `a_chapter_moved_into_start_here_is_caught`,
  `a_chapter_missing_from_start_here_is_caught`,
  `a_terminal_command_outside_under_the_hood_is_caught`,
  `installing_and_initializing_are_allowed_outside_the_blocks`,
  `a_missing_or_overlong_glossary_entry_is_caught`,
  `a_link_outside_the_table_of_contents_is_caught`,
  `a_shell_block_in_review_a_plan_is_caught`,
  `a_malformed_under_the_hood_block_is_caught`,
  `two_blocks_with_the_same_name_are_caught`; verified by these tests
  passing.
- [ ] 1.2 Add the tests that apply the checks to the embedded chapters of
  both editions: `both_editions_open_with_the_start_here_path`,
  `no_chapter_is_removed` (the 22 chapters listed before the change),
  `the_start_here_path_stays_under_its_caps`,
  `the_introduction_says_who_decides`,
  `the_quickstart_walks_the_four_skills_in_order`,
  `the_quickstart_runs_no_command_outside_under_the_hood`,
  `the_glossary_defines_every_required_term`,
  `choose_your_path_links_five_profiles_to_chapters`,
  `review_a_plan_covers_the_plan_without_a_terminal`,
  `under_the_hood_blocks_are_well_formed`,
  `the_manual_starts_with_the_start_here_path`,
  `the_offline_manual_renders_a_folded_block`; and to
  `crates/compagnon-cli/tests/repository.rs`
  `both_readmes_stay_short_keep_the_one_liners_and_link_the_profiles`;
  verified by `no_chapter_is_removed` and
  `the_offline_manual_renders_a_folded_block` passing at once, and the
  others failing until groups 2 and 3 (the red state is shown in the pull
  request).

## 2. The Start here path

- [ ] 2.1 Rewrite `introduction.md` in both languages as "Compagnon in two
  minutes": what it does in plain words, a "Who decides" section, and the
  name (master and compagnons, "compagnonnage"); move "Why Compagnon",
  "Origins" and "Built with Compagnon" to a new `origins.md`; verified by
  `the_start_here_path_stays_under_its_caps`,
  `the_introduction_says_who_decides` and the existing
  `site_and_introductions_name_compagnon`.
- [ ] 2.2 Write `choose-your-path.md` in both languages: the five profiles
  of the spec, in order, each with its chapters in reading order; verified
  by `choose_your_path_links_five_profiles_to_chapters` and
  `the_start_here_path_stays_under_its_caps`.
- [ ] 2.3 Rewrite `quickstart.md` in both languages as "Your first change":
  install in one line, `compagnon init`, then one section per step —
  `/compagnon-propose`, read the plan, `/compagnon-apply`,
  `/compagnon-archive`; command output and CLI equivalents in named "Under
  the hood" blocks; move "What `compagnon init` does" to the
  `compagnon init` section of `reference/cli.md` and repoint its link;
  verified by `the_start_here_path_stays_under_its_caps`,
  `the_quickstart_walks_the_four_skills_in_order`,
  `the_quickstart_runs_no_command_outside_under_the_hood`,
  `under_the_hood_blocks_are_well_formed`, and the existing
  `both_languages_document_the_keyword_language` (its phrase stays, in a
  block) and `links_between_chapters_become_in_page_anchors`.
- [ ] 2.4 Write `glossary.md` in both languages, one `## <Term>` section
  per term; verified by `the_glossary_defines_every_required_term`.
- [ ] 2.5 Write `guides/review-a-plan.md` in both languages for
  non-developers: what the proposal, specs, design and tasks hold, what to
  check in each, how to ask Claude for changes, then approve; verified by
  `review_a_plan_covers_the_plan_without_a_terminal`.
- [ ] 2.6 Regroup both `SUMMARY.md` in the five parts of the design, and
  style `<details>` in `docs/theme/mairistem.css` and
  `crates/compagnon-cli/assets/docs.css`: keep the native open/closed
  marker, keep the global `:focus-visible` outline on `summary`, summary
  text at 4.5:1 and marker at 3:1 in the light and dark schemes; verified
  by `both_editions_open_with_the_start_here_path`,
  `the_manual_starts_with_the_start_here_path`, the existing
  `both_languages_have_the_same_chapters`, and a manual check recorded in
  the pull request — a block in the built site and in a manual written by
  `compagnon docs --write`, opened and closed with Tab, Enter and Space,
  with the focus outline and the marker visible in both schemes. Manual
  because it is a visual judgement on the browser's native control; the
  structure it relies on is tested by `under_the_hood_blocks_are_well_formed`.

## 3. Entry points

- [ ] 3.0 Replace `docs/assets/compagnon-logo.svg` and
  `compagnon-logo-dark.svg` with the "Compagnon by Mairistem" logo
  approved by the user, from this change's `logo-draft/` (regenerated by
  `logo-draft/generate-logo.py` if needed): the Mairistem logo's grid, the
  wordmark black (white on dark) and centred on the C, "BY MAIRISTEM"
  under it, right-aligned, title "Compagnon by Mairistem"; update
  `compagnon_marks_use_only_the_mairistem_palette` to accept that title on
  the two logo files; verified by that test and
  `both_readmes_show_the_light_and_dark_mark`, and the user's approval of
  the rendered logo (manual: a visual judgement, already given on the
  draft).

- [ ] 3.1 Shorten `README.md` and `README.fr.md`: the mark, the promise in
  two sentences, "Getting started" in three numbered steps with the
  `curl … | sh` and `iwr … | iex` one-liners, one link per profile to
  Choose your path, then the existing links; verified by
  `both_readmes_stay_short_keep_the_one_liners_and_link_the_profiles` and
  the existing `both_readmes_show_the_light_and_dark_mark`.
- [ ] 3.2 Align `docs/site/index.html`: a "Choose your path" block with the
  five profiles linking the documentation; verified by the existing
  `site_and_introductions_name_compagnon` and
  `inline_marks_are_named_or_decorative`, and a manual check recorded in
  the pull request that the page has no horizontal scroll at 375 px —
  manual because the layout is a visual judgement.
