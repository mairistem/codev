# Tasks

## 1. Core — `has_warnings()` on `ValidateReport`

- [x] 1.1 Add the method `has_warnings(&self) -> bool` on
      `ValidateReport` in `codev-engine::validate::report`.
      Implementation: iterates `items[].findings[]`, returns `true` as
      soon as a `Finding.severity == Warning`.
- [x] 1.2 Add the same method on `ItemReport` for symmetry with the
      existing `has_errors()`.
- [x] 1.3 Tests: a report with a single warning → `true`;
      a report with only errors → `false` (errors are not
      warnings); an empty report → `false`.

## 2. CLI — `--strict` flag

- [x] 2.1 Add `#[arg(long)] strict: bool` on `Command::Validate`
      in `codev-cli::main`. Clap documentation: "Treat any finding
      (Warning included) as a reason for a non-zero exit code. Useful
      for CI and automation."
- [x] 2.2 Modify the exit-code logic in the `Command::Validate` match
      to take `strict` into account:
      ```rust
      let has_fail = if strict {
          report.has_errors() || report.has_warnings()
      } else {
          report.has_errors()
      };
      if has_fail { 1 } else { 0 }
      ```
- [x] 2.3 With `--json`, the severity displayed in each finding stays
      the finding's own — no silent promotion.

## 3. JSON contract — `hasWarnings`

- [x] 3.1 Add `has_warnings: bool` on `ValidateReportV1` (serialized
      as `hasWarnings`), always present.
- [x] 3.2 `From<&ValidateReport> for ValidateReportV1` computes
      `has_warnings` via the method added in 1.1.
- [x] 3.3 The failure `validate_shape()` in `main.rs` gains
      `"hasWarnings": false` — consistency with the shape's other
      fields.

## 4. CLI integration tests

- [x] 4.1 Test in `codev-cli::commands::tests`: clean project + `--strict`
      → no error, exit 0 (via `validate(&h.ctx(), ValidateArgs::All)`).
- [x] 4.2 Test: project with an unsealed `accepted` local ADR + `--strict`
      → `report.has_warnings()` is true, `report.has_errors()` is false.
      The CLI exit-code logic would return 1 — verifiable via
      `report.has_warnings() && strict`.
- [x] 4.3 JSON contract test: `ValidateReportV1::from(&report)` carries
      `has_warnings: true` when there is a warning, `false` otherwise.

## 5. Dogfooding and workspace integration

- [x] 5.1 After `cargo install`, run `codev validate --strict` on
      this repository — must return exit 0 (no warning today).
- [x] 5.2 Manual test: deliberately introduce a warning (for
      example, remove an entry from `seal.yaml`), run
      `codev validate --strict` → exit 1; without `--strict` → exit 0.
      Restore before committing.
- [x] 5.3 `cargo test --workspace` stays green, gains at least 5 new
      tests (report + CLI + JSON).
- [x] 5.4 `cargo clippy --workspace --all-targets` stays free of
      warnings.
- [x] 5.5 `codev validate --strict --all` stays green on this repository.
