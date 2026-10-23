# Tasks

## 1. The merge

- [ ] 1.1 In `crates/codev-core/src/merge/edits.rs`, sort edits by descending end then descending start, and correct the comment; verified by the unit test `an_insertion_at_the_end_of_a_replacement_goes_after_it` in `edits.rs`, and in `merge/mod.rs` by `modifying_the_last_requirement_and_adding_one_keeps_both_whole` (a longer `Login` modified and `Export` added: `Login` whole then `Export`, and a second merge changes nothing — scenario "The last requirement modified and another added"), the existing merge tests still passing

## 2. The skills' checksum

- [ ] 2.1 In `crates/codev-agents/src/claude.rs`, render `metadata.checksum` (SHA-256 of the skill without that line) and decide a hand edit from it among skills stamped with the running version (another stamp: rewritten; matching its checksum: rewritten; not matching: preserved unless `--force`; no checksum: preserved unless it differs only by the review mode, compared without the checksum line); verified by unit tests `a_hand_edit_of_a_skill_from_another_version_is_rewritten` (scenario "A hand edit survives no upgrade"), `a_legacy_skill_of_the_other_review_mode_is_rewritten`, `a_rendered_skill_matches_its_checksum` (scenario "The checksum matches the content"), `a_skill_from_an_earlier_build_of_the_version_is_rewritten`, `a_hand_edit_of_a_checksummed_skill_is_preserved` and `a_same_version_skill_without_checksum_that_differs_is_preserved` (the three scenarios of the update requirement), the existing frontmatter test extended to `metadata.checksum` (scenario "Frontmatter parsed by a third-party YAML reader"), and the existing hand-edit and `--force` tests still passing
- [ ] 2.2 Refresh codev's own skills with the codev this change builds, `cargo run -q -p codev-cli -- update` (once with `--force`, since they carry no checksum yet), and check that a second run writes nothing; verified by its output and by `cargo run -q -p codev-cli -- doctor --strict`

## 3. Documentation

- [ ] 3.1 In `docs/en/src/reference/file-formats.md` and `docs/fr/src/reference/file-formats.md`, show `metadata.checksum` in the skill frontmatter and say how `codev update` uses it; in both `reference/cli.md`, say which skills `codev update` rewrites and preserves — another version: rewritten; the running version: rewritten when it matches its checksum, preserved when it does not, and, without a checksum, preserved unless it differs only by the review mode, one `codev update --force` bringing such skills under the checksum; rewrite the section "Editing a generated skill" of `docs/en/src/skills.md` and "Modifier une skill générée" of `docs/fr/src/skills.md` to the checksum rule; add to both `faq.md` an entry on a spec cut by an earlier codev — how to spot it (the diff of the archive of a change that modified a spec's last requirement and added requirements), that `codev validate` and a new sync do not repair it, and how to repair it; verified by `cargo test -p codev-cli` (the docs checks) and a read of the eight pages

## 4. Checks

- [ ] 4.1 Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `cargo run -q -p codev-cli -- validate --strict`; verified by all four passing
