# Tasks

## 1. English product

- [x] 1.1 Translate core and engine messages, comments, identifiers and tests; verified by `cargo test --workspace`
- [x] 1.2 Translate CLI output, clap help, skills and templates; verified by the remaining-French grep over crates/ and assets/
- [x] 1.3 English-only parsers, `project/NNNN` qualifier, `--preset full|minimal|custom`; verified by the parser and CLI tests

## 2. Artifact language

- [x] 2.1 `language:` in config, validated, exposed by `codev instructions`; verified by `language_is_read_from_the_project` and `an_invalid_language_is_rejected_with_a_hint`
- [x] 2.2 `codev init --language` and locale detection; verified by `init_writes_the_language_detected_from_the_locale` and `init_language_flag_wins_over_the_locale`
- [x] 2.3 propose, update and configure write prose in `language`; verified by reading the regenerated `.claude/skills/`

## 3. Documentation

- [x] 3.1 English mdBook under docs/en with outputs from real runs; verified by an mdBook build with no warnings
- [x] 3.2 French mdBook under docs/fr, README.fr.md, CONTRIBUTING.fr.md; verified by `both_languages_have_the_same_chapters`
- [x] 3.3 `codev docs --lang`, chapters from SUMMARY.md, in-page anchors; verified by `links_between_chapters_become_in_page_anchors` and a dangling-anchor check on the written HTML

## 4. Fixes found while documenting

- [x] 4.1 Sync validates like archive, both refuse with `validation_failed`; verified by `sync_refuses_a_change_that_fails_validation`
- [x] 4.2 RENAMED heading form and `rename_source_missing`; verified by `renamed_with_the_template_form_retitles` and `rename_from_an_unknown_requirement_is_an_error`
- [x] 4.3 `delta_unexpected_heading` and `delta_section_empty`; verified by `translated_requirement_heading_is_reported`

## 5. Project

- [x] 5.1 CI, docs site and Dependabot workflows; verified by `codev validate --strict` and a local mdBook build
