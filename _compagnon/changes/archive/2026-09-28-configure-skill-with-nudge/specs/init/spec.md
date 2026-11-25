## ADDED Requirements

### Requirement: `compagnon init` prompts for `/compagnon-configure` when the generated config is thin

At the end of `compagnon init`, the **human** output SHALL evaluate whether the
freshly written or already present `_compagnon/config.yaml` is **thin** —
that is, whose `context:` is shorter than 200 characters and whose
`rules:` key is empty.

If the config is thin, the last line of the human output MUST
invite the user to run `/compagnon-configure`:

```
→ Recommended next step: in Claude Code, type /compagnon-configure.
  Claude will analyze the project and enrich _compagnon/config.yaml
  (context, per-artifact rules) — ~30 seconds.

Or skip this step and type /compagnon-propose <an-idea> directly.
```

If the config is not thin (the user had already filled in the YAML,
or a `compagnon-configure` has already run), the output keeps its current
short form: "Restart Claude Code then type /compagnon-propose."

The **JSON** output MUST stay unchanged — no field added, no
promise broken. The prompt is reserved for human output, where
it does not affect scripts that consume the machine report.

#### Scenario: Thin config triggers the hint

- **GIVEN** a new project with a minimal `Cargo.toml` (hence a
  short detected `context:`, empty `rules:`)
- **WHEN** the user runs `compagnon init --yes`
- **THEN** the human output contains the keyword `/compagnon-configure`
- **AND** the JSON output (`--json`) does not contain it

#### Scenario: Non-thin config does not trigger the hint

- **GIVEN** a project whose `_compagnon/config.yaml` already exists with
  a `context:` of more than 200 characters
- **WHEN** the user runs `compagnon init --yes` (idempotence)
- **THEN** the human output does not contain `/compagnon-configure`
