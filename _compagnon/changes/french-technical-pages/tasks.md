# Tasks

Every task's definition of done also includes `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace`, all passing — but from 1.1 to 2.4, where only
`the_french_book_holds_no_anglicism` may fail, on pages not yet
rewritten (the red state is shown in the pull request).

## 1. The check

- [x] 1.1 In `crates/compagnon-cli/src/docs_checks.rs`, add design,
  template and templates to the words of `anglicisms`, and « merge
  request » as « pull request » is matched; count « change(s) » after
  numerals and quantifiers too, and as the first word of a heading; skip
  identifier headings (a file name, a lone lower-case key, a `compagnon`
  command) and `<svg>` blocks; replace
  `the_french_everyday_pages_hold_no_anglicism` by
  `the_french_book_holds_no_anglicism`, which reads every chapter of
  `docs/fr/src/SUMMARY.md` plus `README.fr.md`; add the unit test
  `an_anglicism_of_a_technical_page_is_caught` (« le design et le template
  du schéma » names both words; « Ouvrez une merge request. » names
  « merge request »; « aucun change » and a heading `## Changes` name
  « change »), and `identifier_headings_are_not_prose` (`## review`,
  `## design.md`, `### compagnon review` pass); verified by that test passing,
  `an_anglicism_in_prose_is_caught`,
  `the_french_verb_changer_is_not_an_anglicism` and
  `the_english_word_in_parentheses_code_or_path_is_allowed` still passing,
  and the book test failing on the technical pages until group 2.

## 2. The pages

- [ ] 2.1 Rewrite `docs/fr/src/roles.md` to the lexicon; verified by
  `the_french_book_holds_no_anglicism` naming no line of it, and the
  Roles chapter tests of `crates/compagnon-cli/src/docs.rs`
  (`roles_chapters_list_exactly_the_catalog`,
  `roles_chapters_document_triggers_and_the_stack_filter`,
  `roles_chapters_carry_the_review_scales`) still passing.
- [ ] 2.2 Rewrite `docs/fr/src/reference/cli.md`, `configuration.md`,
  `file-formats.md` and `json-output.md`; verified by the book test naming
  none of them and `git diff --word-diff` showing no inline code span
  changed.
- [ ] 2.3 Rewrite the French "Going further" guides
  (`guides/moving-from-codev.md`, `inherited-sources.md`,
  `repository-integration.md`, `existing-project.md`, `team-rollout.md`,
  `release-notes.md`, `review-record.md` — titled « Compte rendu de
  relecture », in `SUMMARY.md` too — `mcp.md`, `artifact-language.md`,
  `custom-schemas.md`; in `mcp.md`, « un outil de design » becomes « un
  outil de conception graphique »); verified by the book test naming none of them and
  `mdbook build docs/fr`.
- [ ] 2.4 Rewrite `origins.md`, `contributing.md` and `architecture.md`,
  the words the wider list now catches in the everyday pages, and every
  « revue » naming Compagnon's review in the everyday pages (« compte
  rendu de relecture », « Relecture en ligne ou isolée »); verified by the
  book test and the listing of 3.2.
- [ ] 2.5 Rewrite the French strings of `evals/src/reliability.rs`
  (« revue » included), write the token sources' labels in inline code in
  its tables, and regenerate both pages with `cargo run -p compagnon-evals -- docs`;
  verified by `the_reliability_pages_follow_the_baseline` and the book
  test naming no line of `reliability.md`.
- [ ] 2.6 Update every link of the French book to a heading renamed in
  2.1–2.5; verified by `the_offline_manual_links_land_on_their_heading`.

## 3. Check

- [ ] 3.1 Have a French reader who does not know the English words — the
  readers' test of `missing-pages`, Julie and Tom — follow Roles,
  Origins and Contributing from the French home page: it passes when
  neither meets an English word outside parentheses or code; recorded in
  the pull request. Manual because whether a page reads as French is a
  judgement.
- [ ] 3.2 List every « issue », « revue » and « change » left in
  `docs/fr/src/` outside code (`grep -rnwiE 'issues?|revues?|changes?'
  docs/fr/src`) and check by hand that none names a GitHub or Jira issue
  (« ticket »), Compagnon's review (« relecture ») or a Compagnon change
  (« modification »); verified by the list, recorded in the pull request.
  Manual because the three words are also French.
