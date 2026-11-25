# Proposal: `--strict` mode for `codev validate`

## Why

Today, `codev validate` exits with a **binary exit code**: 0 if there
is no error, 1 otherwise. **Warnings** — `decision_unsealed`,
`decision_dangling_deviation`, `git_source_unlocked`, `decision_id_collision`
and company — pass without influencing the exit code. That is the
right default for a human iterating (a warning does not block their
flow), but it is the wrong choice for an **automated** consumer:

- a CI that wants to reject any `push` that would leave an `unsealed`;
- a `pre-commit` hook that wants to forbid an orphaned deviation
  before it reaches `main`;
- a future MCP workflow (Jira → codev, Claude Design → codev) that
  would decide on the exit code rather than on a parser of human
  output.

Strict mode pins down the contractual guarantee "no finding, whatever
the severity ⇔ exit code 0" that automated callers expect.

## What Changes

- **New `--strict` flag** on `codev validate`, applicable to all its
  forms (`validate <item>`, `validate --all`, `validate --changes`,
  `validate --specs`).
- **Effect**: when `--strict` is present, the exit code becomes 1 as
  soon as a finding is emitted, whatever its severity (Warning
  included). Without the flag, the exit code stays binary on `Error`
  only — full compatibility.
- **No severity modified** in the report: findings come out with their
  original severity. Strict mode changes **the exit verdict**, not the
  nature of the findings. The human rendering stays identical.
- **JSON contract**: the field `hasWarnings: bool` is **added** to the
  report (`ValidateReportV1`). Additive, always serialized. The
  consumer can thus decide independently of the exit code — the exit
  code is the signal, this field is the data.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `validation` — two new ADDED requirements: behavior of the
  `--strict` flag on the exit code, and exposure of `hasWarnings` in
  the JSON contract.

## Impact

- **Code**: add the `strict: bool` flag on `Command::Validate` in
  `codev-cli::main`, add a `has_warnings()` method on
  `ValidateReport` in `codev-engine::validate::report`, and a
  two-line change in the CLI's exit-code logic.
- **JSON contract**: `ValidateReportV1` gains `hasWarnings: bool`.
  Additive — earlier consumers ignore the field.
- **File written**: none. Strict mode changes nothing on disk.
- **Migration**: none. Without `--strict`, the behavior is
  bit-identical to today.
- **Out of scope**:
  - **A `--strict-level=warning|info` mode** — for now, the binary
    strict/lax choice is enough; if the need for a configurable
    threshold arises, we will add it.
  - **A `--fix` that automatically corrects warnings** — not a
    validation change but a correction change, out of scope.
  - **`--archived`**, which would also validate archived changes —
    that is a separate change (E6 in the core roadmap).
