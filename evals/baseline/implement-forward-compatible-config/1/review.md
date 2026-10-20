# Review record: forward-compatible-config

<!-- codev-review: 1 -->

## code.review

Mode: isolated.

### developer

Fallback: `codev roles` and `codev review` are unavailable in the installed codev (0.6.1) and in this repository's build (0.5.0); the developer role was briefed from the project's context and rules (`codev instructions tasks`) and ran as a read-only sub-agent, and the record was checked by hand.

#### R-developer-1 An unknown `changelog` or `inherits` key in an inherited source still drops that source

```yaml
severity: medium
confidence: high
location: "crates/codev-engine/src/config.rs:324-351"
evidence: |
                  match load(fs, &source_config_path) {
                      Err(err) => warnings.push(Warning::new(
                          "inherit_unreadable",
                          format!("inherited source `{raw}` is unreadable: {err}"),
recommendation: Inherited sources go through the project's `load`, so `ChangelogConfig` and `InheritSource` stay strict for them. A shared source written for a newer codev, holding `changelog: {since: v1.0.0}` or `inherits: [{path: x, pin: y}]`, then fails to parse. A path source loses its context and rules behind an `inherit_unreadable` warning; a git source loses them with no message at all. This contradicts the design ("only their `context` and `rules` are read, every other key — known or not — is already ignored") and the docs. Read inherited sources tolerantly, through a struct holding only `context`, `rules` and `inherits` (presence only, for the non-transitive warning). Add a test with an unknown `changelog` key in an inherited source.
history:
  - status: open
    by: developer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs — inherited sources read through `load_inherited` (context, rules, presence of inherits only); test `an_inherited_source_strict_section_key_is_ignored_too` fails without the fix"
  - status: fixed
    by: developer
```

#### R-developer-2 The `config.keys` fix says `codev self-update` even when the running codev is above the requirement

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/doctor.rs:221-235"
evidence: |
      let requirement_unsatisfied = matches!(facts.version.requirement, Some((_, false)));
          let fix = if requirement_unsatisfied {
              "codev self-update"
recommendation: Take `codev_version: "<0.7"`, codev 0.7.1 running and one unknown key. `config.keys` gives the fix `codev self-update`, which refuses the implicit downgrade. Give the self-update fix only when `requirement_side` is `Below`, otherwise the "correct or remove" fix. The doctor spec's table ("`codev self-update` when the running codev does not satisfy `codev_version`") must be revised along with it.
history:
  - status: open
    by: developer
  - status: deferred
    by: developer
    to: "a follow-up codev change revising the doctor spec's `config.keys` fix (the current table mandates `codev self-update` whenever `codev_version` is unsatisfied)"
```

#### R-developer-3 The "Validation codes" table does not list `unknown_config_key`

```yaml
severity: low
confidence: high
location: "docs/en/src/reference/file-formats.md:374"
evidence: |
  | `delta_section_empty` | warning | A delta section has no entry, so nothing of it is merged |
recommendation: The table lists every code `codev validate` reports, and json-output.md links to it for item findings. Add `unknown_config_key` (warning) in docs/en and docs/fr.
history:
  - status: open
    by: developer
  - status: fixed
    by: developer
    ref: "docs/en/src/reference/file-formats.md and docs/fr/src/reference/file-formats.md — `unknown_config_key` row added"
  - status: fixed
    by: developer
```

#### R-developer-4 Two config tests check exactly the same thing

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:609-622"
evidence: |
      fn a_strict_section_error_without_codev_version_names_the_key() {
          let err = load_str("inherits:\n  - path: ~/shared\n    pin: abc\n").unwrap_err();
      fn an_unknown_inherits_key_still_fails() {
          let err = load_str("inherits:\n  - path: ~/shared\n    pin: abc\n").unwrap_err();
recommendation: Both tests load the same input and make the same assertions. Make one of them add coverage, for example through `resolve` on a `git` entry.
history:
  - status: open
    by: developer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs — `an_unknown_inherits_key_still_fails` now goes through `resolve` on a git entry"
  - status: fixed
    by: developer
```
