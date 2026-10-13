# Tasks

## 1. Page builder

- [x] 1.1 New pure module `evals/src/reliability.rs`: `Baseline::parse` reads `report.json` and `run.json` (unknown fields ignored, a newer format refused naming the file), a report whose `run` differs from `run.json` refused naming both, verified by `a_newer_report_format_is_refused`, `a_report_of_another_run_is_refused` and `the_baseline_reads_the_committed_report`
- [x] 1.2 `page(&Baseline, Lang)`: sections, run, stat tiles, computed verdict and remarks, table with status icon and words, thresholds, limits, how to run, link to `evals/baseline`, in English and French, verified by `the_page_is_identical_twice` and `every_role_within_the_thresholds_is_said`
- [x] 1.3 Charts as inline SVG: false positives per role (sorted, threshold line, icon and label for roles over it) and the recall and severity-agreement meters (recall threshold, icon and label for roles under it, no mark for `—`), with `role="img"`, `<title>`, `<desc>` and a `<title>` per mark, no blank line inside a figure, verified by `a_role_over_the_false_positive_threshold_is_marked`, `a_role_under_the_recall_threshold_is_marked`, `a_role_without_seeded_case_shows_no_recall` and `a_figure_holds_no_blank_line`

## 2. Command and committed pages

- [x] 2.1 `codev-evals docs [--baseline <dir>]`: builds both pages before writing either, exits 2 naming the file on a missing, invalid or newer report, and runs without loading the cases, verified by `docs_refuses_a_newer_report_and_writes_nothing` in `evals/tests/cli.rs`
- [x] 2.2 Generate `docs/en/src/reliability.md` and `docs/fr/src/reliability.md` from `evals/baseline/`, list them in both `SUMMARY.md` after "Roles", verified by `the_reliability_pages_follow_the_baseline` and `both_languages_have_the_same_chapters`

## 3. Styles and rendering

- [x] 3.1 Chart and tile tokens and styles in `docs/theme/mairistem.css` (`.light`, no-JS default, `.navy`) and `crates/codev-cli/assets/docs.css` (light, `prefers-color-scheme: dark`); run the dataviz validator on the colors in both modes, verified by its output recorded in the design (all PASS)
- [x] 3.2 `codev docs` keeps the charts, verified by `the_reliability_chapter_renders_its_charts` in `crates/codev-cli/src/docs.rs`
- [x] 3.3 Build both books into the scratchpad and screenshot the chapter at 1440 px and 390 px, light and dark; fix label collisions and overflow. Manual check: no automated test can judge layout; verified by `mdbook build` with no warning and the screenshots

## 4. Documentation and links

- [x] 4.1 `docs/en/src/roles.md` and `docs/fr/src/roles.md` link to the chapter at the end of "Measuring the roles", verified by `mdbook build` of both books without warning
- [x] 4.2 `CONTRIBUTING.md` and `CONTRIBUTING.fr.md` "Changing a role": regenerate the page with `cargo run -p codev-evals -- docs` after recording the baseline, verified by `contributing_and_roles_chapters_name_the_baseline`, extended to require `codev-evals -- docs` in both guides, and by `the_documentation_check_names_the_file_that_forgets`
- [x] 4.3 `docs/site/index.html`: a "Measured, not assumed" link to the chapter in the Roles section, no figures, verified by reading the page's screenshot at 1440 px and 390 px

## 5. Done

- [x] 5.1 `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict` and `codev doctor --strict` all pass
