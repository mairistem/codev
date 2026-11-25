# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing, and `mdbook build docs/en` and
`mdbook build docs/fr` succeeding. Every chapter is written in both
languages, the French one in *vouvoiement*, to its own word cap.

## 1. Tests first

- [ ] 1.1 Add to the tests of `crates/compagnon-cli/src/docs.rs` a helper
  counting a chapter's words outside fenced code blocks and `<details>`
  blocks, and the tests
  `both_editions_open_with_the_start_here_path` (the four chapters first,
  then the five parts in order),
  `a_chapter_moved_into_start_here_is_caught` (on a synthetic summary),
  `the_start_here_path_stays_under_its_caps`,
  `under_the_hood_blocks_are_not_counted` (on a synthetic chapter),
  `the_quickstart_runs_no_command_outside_under_the_hood`,
  `the_glossary_defines_every_required_term`,
  `choose_your_path_links_five_profiles_to_chapters`,
  `review_a_plan_needs_no_terminal`, and
  `the_manual_starts_with_the_start_here_path`; verified by the helper's
  and the synthetic tests passing, and the chapter tests failing before
  group 2 (the red state is shown in the pull request).
- [ ] 1.2 Add to `crates/compagnon-cli/tests/repository.rs`
  `both_readmes_stay_short_and_link_the_profiles`, and to the `docs.rs`
  tests `the_offline_manual_renders_a_folded_block` (a `<details>` block
  with a code block, rendered by `render_html`, closed by default);
  verified by the second passing as soon as written and the first failing
  before group 3.

## 2. The Start here path

- [ ] 2.1 Rewrite `introduction.md` in both languages as "Compagnon in two
  minutes": what it does in plain words, who decides (you), what Claude
  does, that nothing is applied without your approval, and the master and
  his compagnons; move "Origins" and "Built with Compagnon" to a new
  `origins.md`; verified by `the_start_here_path_stays_under_its_caps`
  for the introduction.
- [ ] 2.2 Write `choose-your-path.md` in both languages: the five profiles
  of the spec, each with its chapters in reading order; verified by
  `choose_your_path_links_five_profiles_to_chapters`.
- [ ] 2.3 Rewrite `quickstart.md` in both languages as "Your first change":
  install in one line, `compagnon init`, then `/compagnon-propose`, read the
  plan, `/compagnon-apply`, `/compagnon-archive`; command output and CLI
  equivalents in `<details><summary>Under the hood</summary>` blocks; move
  "What `compagnon init` does" to the `compagnon init` section of
  `reference/cli.md` and repoint its link there; verified by
  `the_start_here_path_stays_under_its_caps`,
  `the_quickstart_runs_no_command_outside_under_the_hood`, and
  `links_between_chapters_become_in_page_anchors` still passing.
- [ ] 2.4 Write `glossary.md` in both languages, one `## <Term>` heading
  per term, and link each Start here chapter's first use of a term to it;
  verified by `the_glossary_defines_every_required_term`.
- [ ] 2.5 Write `guides/review-a-plan.md` in both languages for
  non-developers: what the proposal, specs, design and tasks hold, what to
  check in each, how to ask Claude for changes, then approve; verified by
  `review_a_plan_needs_no_terminal`.
- [ ] 2.6 Regroup both `SUMMARY.md` in the five parts of the design and
  style `<details>` in `docs/theme/mairistem.css` and
  `crates/compagnon-cli/assets/docs.css` (visible summary, focus outline);
  verified by `both_editions_open_with_the_start_here_path`,
  `the_manual_starts_with_the_start_here_path`,
  `the_offline_manual_renders_a_folded_block`, the existing
  `both_languages_have_the_same_chapters`, and a keyboard check of a block
  in the built site with Playwright (Tab to the summary, Enter opens it).

## 3. Entry points

- [ ] 3.1 Shorten `README.md` and `README.fr.md`: the mark, the promise in
  two sentences, "Getting started" in three steps keeping the `curl … | sh`
  and `iwr … | iex` one-liners, one link per profile to Choose your path,
  then the existing links; verified by
  `both_readmes_stay_short_and_link_the_profiles` and the existing
  `both_readmes_show_the_light_and_dark_mark`.
- [ ] 3.2 Align `docs/site/index.html`: a "Choose your path" block with the
  five profiles linking the documentation; verified by
  `site_and_introductions_name_compagnon` and a rendering of the page at
  phone width with Playwright (no horizontal scroll).
