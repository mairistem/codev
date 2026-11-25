## MODIFIED Requirements

### Requirement: `compagnon init` hints at `/compagnon-configure` when the generated config is thin

At the end of `compagnon init`, the **human** output SHALL evaluate whether
the freshly written or already present `_compagnon/config.yaml` is
**thin** — that is, whether its `rules:` key is absent or empty.

The `context:` field is no longer part of the definition of "thin" —
the `compagnon init` probe systematically fills it from the detected
manifests, which makes its length useless for guessing whether the
user has actually filled in their config. `rules:`, on the other hand,
are always an explicit user choice; their presence is the only
reliable indicator.

If the config is thin, the last lines of the human output MUST
invite the user to ask Claude Code, in plain words, to configure
Compagnon, with `/compagnon-configure` given as the shortcut:

```
→ Recommended next step: in Claude Code, ask
  "configure Compagnon for this project" (shortcut: /compagnon-configure).
  Claude will analyze the project and enrich _compagnon/config.yaml
  (context, per-artifact rules) — ~30 seconds.

Or skip this step and describe your first change, for example
"plan <an-idea>" (shortcut: /compagnon-propose <an-idea>).
```

If the config is not thin (the user had already written rules, or a
`compagnon-configure` has already run), the output keeps its short form,
in plain words with the shortcut: "Restart Claude Code so it discovers the
skills, then ask it to plan your first change (shortcut:
/compagnon-propose)."

The **JSON** output MUST remain unchanged — no field added, no promise
broken. The hint is reserved for the human output, where it does not
affect scripts that consume the machine report.

#### Scenario: Config without rules triggers the hint

- **GIVEN** a new project with a minimal `Cargo.toml` (hence a
  detected `context:`, but no `rules:` written)
- **WHEN** the user runs `compagnon init --yes`
- **THEN** the human output contains the keyword `/compagnon-configure`
- **AND** it contains the plain request "configure Compagnon for this
  project" before it
- **AND** the JSON output (`--json`) does not contain it

#### Scenario: Config with rules does not trigger the hint

- **GIVEN** a project whose `_compagnon/config.yaml` already exists and
  carries at least one entry in `rules:` (for example
  `rules: { specs: [...] }`)
- **WHEN** the user runs `compagnon init --yes` (idempotence)
- **THEN** the human output does not contain `/compagnon-configure`
- **AND** it suggests planning a first change in plain words, with
  `/compagnon-propose` given after it as the shortcut

#### Scenario: A long auto-detected context does not suppress the hint

- **GIVEN** a TypeScript project with many dependencies
  (auto-detected context longer than 200 characters), without `rules:`
- **WHEN** the user runs `compagnon init --yes`
- **THEN** the human output does contain `/compagnon-configure` — the
  length of the auto-detected context no longer suppresses the hint
