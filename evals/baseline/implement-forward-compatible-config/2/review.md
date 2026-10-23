# Review record: forward-compatible-config

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 Garbled doc comment on the upper-bound test

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/update.rs:77"
evidence: |
  /// Whether a comparator `current` fails is an upper bound: `<` and `<=`
recommendation: Reword as "Whether `comparator`, which `current` fails, is an upper bound", so a reader does not have to parse the sentence twice to learn the precondition.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/update.rs — doc comment of is_upper_bound_failure reworded"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 Early return on `is_some()` hides a plain fallback chain

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/update.rs:143-145"
evidence: |
      if exact_upper.is_some() {
          return exact_upper;
      }
recommendation: Chain the two fallbacks with `exact_upper.or_else(|| …)`, which reads as the order the doc comment states (exact upper bound, else lowest admitted version).
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/update.rs — downgrade_target chains its fallbacks with or_else"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 Test comment describes an assertion the test does not make

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/commands.rs:4004"
evidence: |
          // A warning: `codev doctor --strict` counts only what is missing.
recommendation: The comment sits above the unsatisfied-requirement case, which checks the `codev self-update` fix, not the strict exit; a reader looks for a missing-count assertion that is not there. Assert that `config.keys` is not counted as missing in the first case, and give the second case a comment of its own.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-cli/src/commands.rs — doctor_reports_an_unknown_config_key comments match their assertions"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 Section-end comment contradicts the predicate below it

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:274"
evidence: |
                      // The section ends at the next line written at column 0.
recommendation: The predicate also lets blank lines and column-0 comments through; say "the section ends at the next key written at column 0" so the comment matches what the closure does.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs — key_line comment says the section ends at the next column-0 key"
  - status: fixed
    by: newcomer
```
