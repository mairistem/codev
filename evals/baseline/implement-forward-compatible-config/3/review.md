# Review record: forward-compatible-config

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `key_line` hides two lookups behind an optional parent

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:282-305"
evidence: |
  fn key_line(lines: &[&str], key: &str, parent: Option<u32>) -> Option<u32> {
              .take_while(|(_, line)| !top_level(line) || line.starts_with('#'))
recommendation: >-
  Split it into `top_level_key_line(lines, key)` and
  `nested_key_line(lines, key, parent_line)`, with named helpers
  (`is_indented`, `starts_with_key`) instead of closures, so that the
  block boundary of the nested lookup — "until the next top-level key,
  comments excepted" — reads as such. Behavior unchanged.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs (top_level_key_line, nested_key_line)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The `config.keys` fix converts both branches to `String`

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/doctor.rs:232-238"
evidence: |
          let fix = if requirement_unsatisfied {
              "codev self-update".to_string()
recommendation: >-
  Keep both branches as `&str`: `not_ok` already takes
  `impl Into<String>`, so the two `.to_string()` calls are noise.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/doctor.rs (config.keys fix as &str)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 A test compares the finding's message with itself

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/commands.rs:3949"
evidence: |
                  "message": item["findings"][0]["message"],
recommendation: >-
  Assert the finding's fields one by one (path, line, severity, code) and
  that the message names the key, rather than building an expected value
  out of the actual one, which reads as a tautology.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-cli/src/commands.rs (validate_reports_an_unknown_config_key_as_a_warning)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 The relative path of `config.yaml` is computed twice

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/validate/mod.rs:156-161"
evidence: |
      let file = layout.config_file();
      let relative = file
          .strip_prefix(layout.project_root())
recommendation: >-
  `config::resolve` builds the same project-relative, slash-separated path
  for its warnings: share one helper in `config` so that the warning of
  `codev status` and the finding of `codev validate` cannot name the file
  differently.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs (relative_config_file, shown_path), crates/codev-engine/src/validate/mod.rs"
  - status: fixed
    by: newcomer
```
