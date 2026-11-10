# Tasks

Definition of done for every task: `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings` and
`cargo test --workspace` pass. Engine and CLI tests use the in-memory
`FileSystem` and `FixedEnv` ports, never temporary directories, and
assume no path separator or line ending.

## 0. Prerequisite

- [ ] 0.1 Check that `evaluate-archive-sync-configure` is archived on
  `main` and this branch is rebased on it (its steps `change.archive` and
  `configure.enrich`, the shared scoring of check-scored steps, the
  `overlay/` of history cases, and their baseline in `evals/baseline/`),
  and that decision 0013 is accepted on `main`; otherwise stop: nothing
  of this change is built before. Verified by `codev-evals list` showing
  `change.archive` and `configure.enrich` cases, and
  `codev decision list` showing 0013 accepted.

## 1. The lessons ledger in the core

- [ ] 1.1 Add `codev-core::lessons`: a pure parser of `_codev/lessons.md`
  (marker `<!-- codev-lessons: 1 -->`, `### P-<n>` and `### L-<n>`
  entries, `serde_norway` blocks per decision 0006), mirroring
  `codev-core::corrections`; a file without the marker parses as "not a
  ledger". Verified by unit tests in its `#[cfg(test)] mod tests`:
  `a_well_formed_ledger_has_no_finding`,
  `windows_line_endings_are_read`, `a_file_without_the_marker_is_not_a_ledger`.
- [ ] 1.2 Add the checks: `lessons_invalid` (heading, duplicate id,
  missing block, invalid YAML, missing or empty field, wrong type,
  unknown `outcome` or `destination` with the accepted values, `where`
  required by `written`/`handed-off`/`covered`, `reason` required by
  `declined`/`set-aside`, malformed `origin` or `read` item — a folder
  that is not a single path segment included —, `pass` naming no entry),
  `lessons_unknown_field`, `lessons_format_unsupported`. Verified by
  `a_declined_lesson_needs_a_reason`,
  `an_unknown_destination_names_the_seven_values`,
  `a_malformed_origin_names_the_expected_form`,
  `an_origin_leaving_the_archive_is_invalid`,
  `an_unknown_field_is_a_warning`,
  `a_newer_format_is_unchecked_with_a_warning`,
  `a_written_lesson_needs_where`.
- [ ] 1.3 Add the pure selection of an archived change's served entries
  (last status `fixed` or `accepted-risk`), its counts (`findings`,
  `reported`, `served`, `corrections`), the `considered` marks from the
  ledger's origins, and the waiting signal (`changes` not listed in any
  pass's `read`, `corrections`, `window`, `waiting` with the threshold
  10). Verified by `only_fixed_and_accepted_risk_findings_are_served`,
  `an_entry_named_by_a_lesson_is_considered`,
  `ten_unread_changes_are_waiting`,
  `an_unconsidered_correction_is_waiting`,
  `a_same_day_change_sorting_before_a_read_one_is_unread`,
  `the_window_reaches_the_oldest_unread_change`,
  `one_unread_change_without_corrections_is_not_waiting`.

## 2. Validation of the ledger

- [ ] 2.1 In `codev-engine`, read `_codev/lessons.md` through the
  `FileSystem` port and add it to `validate_all` as an item present only
  when it has a finding; add `lessons_origin_unknown` for an origin folder
  absent from `_codev/changes/archive/` (folders looked up only there,
  as single path segments). A run naming a change or a spec,
  or `--changes` / `--specs`, does not check it. Verified by engine tests
  `a_ledger_error_fails_validation`, `a_clean_ledger_adds_no_item`,
  `an_origin_outside_the_archive_is_a_warning`,
  `validating_one_change_leaves_the_ledger_alone`, and a CLI test
  `strict_validation_fails_on_a_ledger_warning`.

## 3. `codev review --archived`

- [ ] 3.1 In `codev-engine`, read the last `n` archived changes (folder
  order), their `review.md` and `corrections.md` with the existing
  parsers, without any evidence check; a newer format or an unparsable
  block serves nothing from that file and adds its warning to the
  change's issues. Verified by engine tests
  `archived_review_serves_the_ten_newest_by_default`,
  `archived_review_serves_all_when_fewer_than_n`,
  `archived_evidence_is_not_checked`,
  `archived_corrections_are_served`,
  `a_newer_archived_record_serves_nothing_with_a_warning`.
- [ ] 3.2 In `codev-cli`, add `--archived` and `--last <n>` to
  `codev review` (clap: `--archived` conflicts with `--change`, `--last`
  requires `--archived`, `n` ≥ 1), the human output (one count line per
  change, one line per served finding and correction with its
  `considered` mark, "no archived change" when empty) and exit code 0.
  Verified by CLI tests `review_archived_lists_counts_and_settled_findings`,
  `review_archived_last_widens_the_window`,
  `review_archived_ignores_a_ledger_without_marker`,
  `review_archived_with_an_empty_archive_succeeds`,
  `review_archived_refuses_change_too`,
  `review_last_requires_archived`.
- [ ] 3.3 Add `ArchivedReviewV1` to `contract.rs` (`last`, `changes[]`
  with `name`, `counts`, `findings`, `corrections`, `issues`, served
  entries with `considered`; `lessons` with `path`, `exists`, `issues`),
  built from the outcome, never from the domain types directly, and its
  shape in the contract tests. Verified by
  `archived_review_document_exposes_the_contract_fields` and
  `a_considered_finding_carries_its_lesson_id`.

## 4. The archive report's `lessons`

- [ ] 4.1 Compute the waiting signal after a successful archive, reading
  only the unread changes' `corrections.md` and never failing the
  archive (an unreadable ledger gives `None`), and add the additive
  `lessons` object (`changes`, `corrections`, `window`, `waiting`) to
  `ArchiveReportV1`; the human output is unchanged. Time
  `codev archive --json` on a copy of this repository without a ledger,
  and note the figure in the pull request. Verified by CLI tests
  `archive_json_reports_lessons_waiting_after_ten_changes`,
  `archive_json_reports_an_unconsidered_correction`,
  `archive_json_counts_a_same_day_change_as_unread`,
  `archive_json_reports_nothing_waiting`,
  `archive_json_gives_null_lessons_for_a_newer_ledger`, and the existing
  archive human-output tests unchanged.

## 5. The skills

- [ ] 5.1 Write the lessons mode in `assets/workflows/configure.md`, as
  its own section after the default mode: the refusal without
  configuration, the update check (`{{fragment:version-check}}`), the
  stop on a usage error from `codev review --archived` naming
  `codev self-update` without reading the archive by other means, the
  reads (`codev review --archived --last <n>`,
  `codev decision list`, `_codev/config.yaml`, `codev list`, the
  ledger), the thresholds, the order of destinations with their
  criteria, `covered`, "a rule never restates a decision in effect", the
  ten-rule budget, the fixed lesson line, the diff, the explicit
  agreement and the author's right to move or decline, the origin
  comment, the proposed decision, the ledger entries, and
  `codev validate` after writing. Scope the five steps of the default
  mode to an invocation without `lessons`. Verified by catalog tests in
  `crates/codev-agents/src/workflows.rs`:
  `configure_carries_the_lessons_thresholds`,
  `configure_orders_the_lesson_destinations`,
  `configure_writes_lessons_only_on_agreement_with_their_origin`,
  `configure_reads_the_history_only_in_the_lessons_mode`,
  `configure_lessons_stops_on_an_older_binary`, and the
  existing configure catalog tests.
- [ ] 5.2 Add to `assets/workflows/archive.md`, in the success report,
  the single conditional invitation line on `lessons.waiting`, giving
  `/codev-configure lessons <lessons.window>` and naming
  `lessons.changes` and `lessons.corrections`; nothing when the field is
  absent, `null` or false; `allowed-tools` unchanged. Verified by
  catalog tests `archive_invites_a_lessons_pass_only_when_waiting`
  (the line is conditioned on `lessons.waiting` being `true`, and the
  text says an absent, `null` or `false` `lessons` gives no line: the
  scenarios "Nothing waiting" and "An older binary") and
  `archive_allowed_tools_are_unchanged`. The agent's obedience to that
  condition is measured on `change.archive` (6.6).
- [ ] 5.3 Check by hand, once, the scenarios a headless run cannot play
  because they need a second turn from the author: "A rule carries its
  origin", "A declined lesson is
  remembered", "A decision stays proposed", "The author moves a lesson",
  "Too many rules on an artifact" (a merge is a judgement the scored
  lesson line does not carry); and the archive skill's "An older binary"
  (the suite runs the codev under test, so no case can play it), in a
  scratch copy of this repository. Verified by the outcome of each,
  written under this task when it is checked.
- [ ] 5.4 Run `codev update` on this repository so the installed skills
  match. Verified by `codev doctor` reporting no stale skill.

## 6. The `configure.lessons` evaluation step

- [ ] 6.1 Add the case kind to `evals/src/case.rs`: `step:
  configure.lessons`, `ref`, `last`, `summary`, `expected.lessons`
  (`origins`, `destinations`), `expected.absent`, the optional
  `overlay/` loaded and laid by the code `evaluate-archive-sync-configure`
  added for sync and archive cases (no symbolic link, nothing laid
  outside the case's directory), and the loading errors. Verified by
  `a_lessons_case_loads`, `a_lessons_case_with_an_unknown_origin_fails`,
  `an_overlay_with_a_symbolic_link_fails`,
  `a_lessons_case_with_an_unknown_destination_fails`.
- [ ] 6.2 Add the runner's fixture (tree at `ref`, overlay, skills,
  commit) and prompt (`/codev-configure lessons <last>`, no question,
  stop after the diff), `--step configure.lessons`, and the observation
  of messages, modified files and consumption. Verified by
  `the_lessons_prompt_names_the_mode_and_the_window` and
  `a_lessons_write_is_recorded_as_modified`.
- [ ] 6.3 Add the scoring (lesson lines parsed, `[covered by …]` as
  `covered`; expected, absent and no-write checks as named checks of the
  shared scoring of check-scored steps, which gives pass share, runs
  modifying a file and the per-case verdict; recall and unexpected
  lessons added by this step), the report section, the comparison and
  the reliability section in both languages.
  "Nothing written without agreement" is played by every run's no-write
  check. Verified by `a_lesson_found_for_another_destination_fails`,
  `a_declined_lesson_proposed_again_fails_absent`,
  `a_covered_line_satisfies_covered`,
  `compare_fails_on_a_lessons_recall_regression`,
  `report_has_a_lessons_section_with_its_measures`,
  `report_has_no_lessons_section_without_cases`.
- [ ] 6.4 Write the six cases under `evals/cases/` (symbolic links;
  long functions and too many parameters; compatibility with older
  codevs with a proposed decision in the overlay; workflow input reaching
  the shell declined in the overlay's ledger; a finding seen once; a
  single correction in an overlay's archived change), with their `ref`
  chosen after the changes they cite. Verified by `codev-evals list`
  showing six `configure.lessons` cases and by their loading tests.
- [ ] 6.5 Record the baseline: `codev-evals run --step configure.lessons
  --repeat 5 --jobs 4` with this branch's binary, committed under
  `evals/baseline/`; it meets the bar set in the design (median recall
  ≥ 0.8, median pass share ≥ 0.6, no run modifying a file), else the
  skill text is revised and the runs repeated. Verified by the report
  and by `every_case_has_a_baseline_observation` and
  `every_baseline_case_has_five_repetitions` in `evals/tests/suite.rs`,
  extended if needed so that they cover `configure.lessons` cases.
- [ ] 6.6 Measure the steps this change touches (decision 0008) against
  the baseline `evaluate-archive-sync-configure` recorded. In
  `evals/cases/`, add `contains: ["/codev-configure lessons"]` to the
  `done` `change.archive` cases (their trees hold more than ten archived
  changes and no ledger), and add the case `archive-lessons-not-waiting`
  (the change of `archive-merges-and-moves`, an overlay
  `_codev/lessons.md` whose pass `P-1` reads every archived folder of the
  tree, `outcome: done`, `absent: ["/codev-configure lessons"]`), its five
  baseline runs recorded with `main`'s binary; score the baseline again
  (`codev-evals score evals/baseline`) and commit its report. Then run
  with this branch's binary `codev-evals run --step change.archive
  --step configure.enrich --repeat 5 --jobs 4` and
  `codev-evals compare evals/baseline <candidate>`: no regression on
  either step, case by case, the edited archive cases and
  `archive-lessons-not-waiting` passing in every candidate run, and the
  configure cost per run compared with the baseline's; else the skill
  text is revised and the runs repeated. Also give the rendered
  `codev-configure` skill's token size before and after, under a ceiling
  of 1,500 tokens added, and the rendered `codev-archive` skill's; no
  other step reads a changed text. Verified by the `compare` verdict and
  the numbers pasted in the pull request, with the output of
  `codev-evals compare` for `configure.lessons`, and by the existing
  `every_case_has_a_baseline_observation`,
  `every_baseline_case_has_five_repetitions` and
  `the_baseline_agrees_with_the_scoring`, with
  `the_suite_covers_the_sync_and_archive_paths` updated to seven
  `change.archive` cases.

## 7. Documentation

- [ ] 7.1 Document, in both `docs/en` and `docs/fr` (same chapters,
  French in vouvoiement): the lessons mode in `skills.md`, and the
  invitation line in its `/codev-archive` section; `--archived`
  and `--last` in `reference/cli.md`; `ArchivedReviewV1` and the
  archive report's `lessons` in `reference/json-output.md`; the ledger
  format and what an older codev does with it in
  `reference/file-formats.md`, with the `lessons_*` codes in its codes
  table;
  the history in `guides/review-record.md`; and the evaluation step in
  `CONTRIBUTING.md` / `CONTRIBUTING.fr.md` and the generated reliability
  chapter. Verified by `mdbook build` of both books and the docs tests
  that compare chapter lists.

## 8. One-time catch-up on this repository

- [ ] 8.1 With the mode merged into this branch, run
  `/codev-configure lessons <number of archived changes>` on this
  repository, after reading the decisions (proposed included), `rules:`,
  `context:` and the active changes so that nothing existing is
  recreated. Record every lesson in `_codev/lessons.md` under one pass
  `P-1`: written (destination), handed off, covered (by which decision,
  rule or change), declined, or set aside with the reason, each write
  confirmed by the author. Verified by `codev validate --strict` clean
  and by the ledger holding an outcome for each known recurrence:
  compatibility with older codevs (covered by decision 0013),
  symbolic links leaving the project, workflow input reaching the shell,
  long functions and too many parameters. A lesson handed off as a
  `check` names in `where` the change to propose; proposing it is left
  to the author.
- [ ] 8.2 Ask the author's explicit agreement to read the feedback
  entries of this project's Claude Code memory
  (the `memory/` folder of this project under `~/.claude/projects/`).
  With it, propose for each project instruction (never a personal
  preference) a destination in codev, through the same mode's line and
  diff; write only what is confirmed, record each in the ledger, delete
  nothing from the memory. Without it, record in this task that it was
  not done. Verified by the memory folder unchanged (`ls -l` before and
  after) and the ledger entries naming their source.
## 9. Final verification

- [ ] 9.1 Run `codev validate --strict` and the full definition of done
  on the branch. Verified by both passing.
