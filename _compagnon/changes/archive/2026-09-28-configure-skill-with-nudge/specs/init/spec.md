## ADDED Requirements

### Requirement: `codev init` prompts for `/codev-configure` when the generated config is thin

At the end of `codev init`, the **human** output SHALL evaluate whether the
freshly written or already present `_codev/config.yaml` is **thin** —
that is, whose `context:` is shorter than 200 characters and whose
`rules:` key is empty.

If the config is thin, the last line of the human output MUST
invite the user to run `/codev-configure`:

```
→ Recommended next step: in Claude Code, type /codev-configure.
  Claude will analyze the project and enrich _codev/config.yaml
  (context, per-artifact rules) — ~30 seconds.

Or skip this step and type /codev-propose <an-idea> directly.
```

If the config is not thin (the user had already filled in the YAML,
or a `codev-configure` has already run), the output keeps its current
short form: "Restart Claude Code then type /codev-propose."

The **JSON** output MUST stay unchanged — no field added, no
promise broken. The prompt is reserved for human output, where
it does not affect scripts that consume the machine report.

#### Scenario: Thin config triggers the hint

- **GIVEN** a new project with a minimal `Cargo.toml` (hence a
  short detected `context:`, empty `rules:`)
- **WHEN** the user runs `codev init --yes`
- **THEN** the human output contains the keyword `/codev-configure`
- **AND** the JSON output (`--json`) does not contain it

#### Scenario: Non-thin config does not trigger the hint

- **GIVEN** a project whose `_codev/config.yaml` already exists with
  a `context:` of more than 200 characters
- **WHEN** the user runs `codev init --yes` (idempotence)
- **THEN** the human output does not contain `/codev-configure`
