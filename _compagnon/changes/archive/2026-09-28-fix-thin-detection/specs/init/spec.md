## MODIFIED Requirements

### Requirement: `codev init` prompts for `/codev-configure` when the generated config is thin

At the end of `codev init`, the **human** output SHALL evaluate whether
the freshly written or already present `_codev/config.yaml` is
**thin** — that is, one whose `rules:` key is missing or empty.

The `context:` field no longer enters the definition of "thin" — the
`codev init` probe systematically fills it from the detected
manifests, which makes its length useless for guessing whether the
user has really filled in their config. `rules:`, on the contrary, are
always an explicit user choice; their presence is the only reliable
indicator.

If the config is thin, the last line of the human output MUST invite
the user to run `/codev-configure`:

```
→ Recommended next step: in Claude Code, type /codev-configure.
  Claude will analyze the project and enrich _codev/config.yaml
  (context, per-artifact rules) — ~30 seconds.

Or skip this step and type /codev-propose <an-idea> directly.
```

If the config is not thin (the user had already written rules, or a
`codev-configure` has already run), the output keeps its current short
form: "Restart Claude Code, then type /codev-propose."

The **JSON** output MUST remain unchanged — no field added, no promise
broken. The suggestion is reserved for the human output, where it does
not affect scripts that consume the machine report.

#### Scenario: Config without rules triggers the hint

- **GIVEN** a new project with a minimal `Cargo.toml` (hence a
  detected `context:`, but no `rules:` written)
- **WHEN** the user runs `codev init --yes`
- **THEN** the human output contains the keyword `/codev-configure`
- **AND** the JSON output (`--json`) does not contain it

#### Scenario: Config with rules does not trigger the hint

- **GIVEN** a project whose `_codev/config.yaml` already exists and
  carries at least one entry in `rules:` (for example
  `rules: { specs: [...] }`)
- **WHEN** the user runs `codev init --yes` (idempotence)
- **THEN** the human output does not contain `/codev-configure`

#### Scenario: Long auto-detected context does not suppress the hint

- **GIVEN** a TypeScript project with many dependencies (auto-detected
  context of more than 200 characters), without `rules:`
- **WHEN** the user runs `codev init --yes`
- **THEN** the human output does contain `/codev-configure` — the
  length of the auto-detected context no longer suppresses the hint
