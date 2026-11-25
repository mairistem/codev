# Tasks

## 1. Shared review guidance

- [x] 1.1 `assets/workflows/_review-record.md`: the bullets "Grade with evidence", "Not a finding" and "One finding per defect" next to the scales, verified by `propose_and_apply_carry_the_evidence_rule`, `propose_and_apply_list_what_is_not_a_finding` and `propose_and_apply_report_each_defect_once` in `crates/codev-agents/src/workflows.rs` (scenarios "The evidence rule reaches the skills", "What is not a finding reaches the skills", "One finding per defect")
- [x] 1.2 Regenerate the repository's skills with `codev update --force`, verified by `.claude/skills/codev-propose/SKILL.md` and `.claude/skills/codev-apply/SKILL.md` holding "Grade with evidence" and `codev doctor --strict` passing

## 2. Roles

- [x] 2.1 Experts (`ops`, `performance`, `accessibility`, `migration`, `privacy`, `security`): a "Do not report" list in Scope and what a finding names in Reporting, as the design says, naming no case, file or fixture code, verified by `every_expert_says_what_it_does_not_report` in `crates/codev-core/src/roles.rs` and `every_builtin_role_file_parses`
- [x] 2.2 `skeptic`: the lens criterion replaced by **Domains**, the signal "Domain left to nobody", a Scope and a Reporting section, verified by `step_role_criteria_carry_the_review_grids` (updated needles) and `the_skeptic_leaves_domains_to_the_experts` (scenarios "The skeptic leaves a domain to its expert" and "A touched domain left unticked", whose agent behavior a unit test cannot run: the test checks the criteria and knowledge the agent receives)
- [x] 2.3 `newcomer`: Scope (readability only, no missing tests, no taste) and Reporting (one finding per function, medium when misread is likely), verified by `the_newcomer_groups_findings_by_function` in `crates/codev-core/src/roles.rs`

## 3. Cases

- [x] 3.1 Fix the fixtures of the design's table (`skeptic-control-small-flag`, `ops-control-graceful-shutdown`, `accessibility-control-custom-listbox`) and add the two `alternatives` (`privacy-code-python-soft-delete`, `migration-plan-breaking-rename`), verified by `every_case_of_the_suite_loads` and `categories_of_one_case_share_no_term`
- [x] 3.2 Re-score the committed baseline against the fixed cases outside the repository, verified by `cargo run -p codev-evals -- score <copy>` giving migration 4 and privacy 7 false positives, the design's "re-scored" column

## 4. Measure

- [x] 4.1 Record the baseline again with every role: remove `evals/baseline/`, `cargo build --bin codev`, `cargo run -p codev-evals -- run --out evals/baseline`, verified by `every_case_has_a_baseline_observation` and `the_baseline_agrees_with_the_scoring`
- [x] 4.2 If a role's recall dropped, find the finding that should have paired, fix the role's wording and run the suite once more (at most one extra run), verified by recall 1.00 for every role in `evals/baseline/report.md`, or the drop explained in the design
- [x] 4.3 Fill the design's "Baseline before and after" (recall, false positives, severity agreement, evidence validity per role: committed, re-scored, new), verified by reading `design.md` against both reports
- [x] 4.4 Regenerate the reliability chapter with `cargo run -p codev-evals -- docs`, verified by `the_reliability_pages_follow_the_baseline`

## 5. Documentation

- [x] 5.1 `docs/en/src/roles.md` and `docs/fr/src/roles.md`: what a confident finding shows, what is not a finding, the skeptic leaving domains to the experts, the newcomer's one finding per function; `docs/en/src/skills.md` and `docs/fr/src/skills.md`: the contrarian pass without lenses, verified by both mdBooks building without warning and reading the two editions side by side

## 6. Done

- [x] 6.1 `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `codev validate --strict` and `codev doctor --strict` all pass
