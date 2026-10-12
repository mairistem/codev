# Tasks

Every task follows the same order: the regression test is written first and
seen failing (`cargo test -p <crate> <test>` reports the failure), then the
fix makes it pass. Every task's definition of done also includes
`cargo fmt --all --check`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` and
`cargo test --workspace --locked`, all passing. Tests follow the project's
conventions: unit tests in the `#[cfg(test)] mod tests` next to the code,
behavior-describing names, the in-memory `MemoryFileSystem` and `FixedEnv`
ports (through the CLI `Harness`) in engine and CLI tests, never a temporary
directory, no assumption about path separators or line endings — paths with
`\` are built explicitly where the Windows behavior is tested.

The scenarios that the MODIFIED requirements carry over unchanged keep the
tests that cover them today, listed in the tasks of the archived change
`specify-core-commands`; only the new or changed scenarios are listed
below.

## 1. Capabilities are folders holding `spec.md`

- [x] 1.1 Make `specs::list` keep a file only when its last segment is
  exactly `spec.md` (changes: "A Markdown file named like a spec is not a
  capability", "Capabilities, nested ones included"), verified by the new
  test `a_file_whose_name_ends_with_spec_md_is_not_a_capability` and the
  existing `lists_capabilities_including_nested_ones` and
  `ignores_what_is_not_a_spec` (codev-engine specs).
- [x] 1.2 Cover `validate` with a stray file (validation: "A stray Markdown
  file is ignored", "The whole project with a stray file", "A stray file is
  not an item"), verified by the new tests
  `validate_specs_ignores_a_stray_markdown_file`,
  `validate_all_ignores_a_stray_markdown_file` and
  `validate_does_not_find_a_stray_file_by_name` (codev-cli commands).

## 2. The folder is the schema's name

- [x] 2.1 Carry the resolution name in `ResolvedSchema`, record it in
  `change.yaml`, and use it as the schema name of `status` and
  `instructions` (schemas: "A schema whose `name:` differs from its folder is
  recorded by its folder", "The folder names the schema, not its `name:`",
  "The configured default", "`--schema` wins over the configuration"),
  verified by the new tests
  `new_change_records_the_folder_of_a_schema_named_differently` and
  `status_and_instructions_name_the_schema_by_its_folder` (codev-cli
  commands), `a_resolved_schema_is_named_by_its_folder` (codev-engine
  schemas), and the existing
  `new_change_takes_the_schema_from_the_flag_then_the_configuration`.
- [x] 2.2 Report `schema_name_mismatch` in `codev schemas` (schemas: "A
  schema whose `name:` differs from its folder", "Matching names raise no
  warning"), verified by the new tests
  `schemas_warn_when_the_name_differs_from_the_folder` and
  `schemas_with_matching_names_raise_no_warning` (codev-cli commands).

## 3. `archive` is a reserved change name

- [x] 3.1 Refuse `archive` in `ChangeId::parse` with a message saying it is
  reserved (changes: "The reserved name `archive` is refused", "Names that
  only contain `archive` are accepted"), verified by the new tests
  `archive_is_a_reserved_name` and `names_containing_archive_are_accepted`
  (codev-core id) and `new_change_refuses_the_reserved_name_archive`
  (codev-cli commands), which checks nothing is written.
- [x] 3.2 Cover `--change archive` and the inference with an archive
  (changes: "The archive folder named as a change", "The archive folder is
  not an active change"), verified by the new tests
  `status_refuses_the_archive_folder_as_a_change` and
  `status_infers_the_single_change_despite_the_archive` (codev-cli
  commands), and `status_failure_keeps_the_shape` (codev-cli main) for the
  `changeName: null` of the failure document.

## 4. The `instructions` failure keeps `language`

- [x] 4.1 Add `language` to the `instructions` failure shape, resolved by a
  best-effort `configured_language` (instructions: "The failure document
  keeps the configured language", "The failure document outside a
  project", "Unknown artifact"), verified by the extended
  `instructions_failure_keeps_the_shape` (codev-cli main) and the new tests
  `configured_language_reads_the_project_configuration` and
  `configured_language_defaults_to_en_outside_a_project` (codev-cli
  commands).

## 5. Portable paths in every JSON document

- [x] 5.1 Add `codev_core::layout::slashed` and replace `contract::to_slash`,
  `contract::paths_slash`, `main::paths`, `commands::portable` and the
  inline copy of `codev-engine::changelog` with it, verified by the moved
  tests `slashed_does_not_double_the_separator_on_an_absolute_path` and
  `slashed_translates_the_windows_separator` (codev-core layout), and the
  existing init tests writing `inherits:` paths (codev-cli commands).
- [x] 5.2 Route every path of `contract.rs` and `main.rs` through it, with
  `StatusV1::new` taking paths and a `NewChangeV1::from` conversion
  (json-output: "Status of a change on Windows", "Instructions on Windows",
  "Files written on Windows", "Unix paths are unchanged"), verified by the
  new tests `status_paths_use_slashes_on_windows`,
  `instructions_paths_use_slashes_on_windows`,
  `new_change_paths_use_slashes_on_windows`,
  `unix_paths_are_written_unchanged` (codev-cli contract),
  `setup_paths_use_slashes_on_windows` (codev-cli main), and the guard test
  `json_documents_never_render_a_native_path` (codev-cli contract).

## 6. Documentation and verification

- [x] 6.1 Update docs/en and docs/fr (same chapters, French in vouvoiement):
  reference/cli.md (the reserved name `archive`, what `list --specs` lists,
  the `schema_name_mismatch` warning of `codev schemas`),
  reference/json-output.md (`language` in the `instructions` failure, the
  `schema_name_mismatch` warning, portable paths for every path),
  guides/custom-schemas.md (the folder is the schema's name, the warning),
  reference/file-formats.md (the reserved change name, what is a capability
  under `_codev/specs/`); verified by building both books with mdbook
  without warnings.
- [x] 6.2 Run `codev validate --strict` and `codev doctor --strict` on the
  repository; verified by both exiting with code 0.
