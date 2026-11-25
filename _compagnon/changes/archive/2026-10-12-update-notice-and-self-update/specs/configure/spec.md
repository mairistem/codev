## MODIFIED Requirements

### Requirement: The `configure` skill enriches `_compagnon/config.yaml` by analyzing the project

The compagnon catalog SHALL expose a `configure` workflow — installed
under `.claude/skills/compagnon-configure/SKILL.md`, invocable as
`/compagnon-configure` — whose role is to enrich `_compagnon/config.yaml`
by analyzing the project, without modifying the workflows, the
detected MCPs, or the schema.

The procedure MUST be:

1. **Read the existing file** — the project's `_compagnon/config.yaml`.
2. **Read-only exploration** — root `README.md`,
   `CONTRIBUTING.md` if it exists, the contents of `docs/`, a sample
   of the most frequently edited source files (via `git log --pretty=format: --name-only | sort | uniq -c | sort -rn | head`),
   and the structure of the main directories.
3. **Patch proposal** — a new value for `context:` (2 to 5
   lines focused on what matters for steering the skills — stack
   beyond the language, API conventions, error style, comment tone,
   structural library choices) and for `rules:` per artifact
   (`specs:`, `design:`, `tasks:` — 1 to 2 rules each, grounded in
   what the project actually does).
4. **Display the diff** as output, without writing.
5. **Write only on confirmation** from the user.

The skill MUST **preserve** the fields it does not touch — `schema`,
`workflows`, `mcp`, `inherits`, `compagnon_version`, as well as existing
provenance comments. It does not rewrite the `mcp:` key detected by
`compagnon init`.

The skill MUST **refuse to act** if `_compagnon/config.yaml` is missing —
it points the user to `compagnon init`.

#### Scenario: Role documented in the catalog

- **GIVEN** the compagnon workflow catalog
- **WHEN** the `configure` workflow is resolved
- **THEN** its entry exists (`find("configure").is_some()`)
- **AND** its `body` cites the five steps (read, exploration,
  proposal, diff, confirmation)
- **AND** its `body` explicitly lists the preserved fields
  (`schema`, `workflows`, `mcp`, `inherits`, `compagnon_version`)

#### Scenario: Confirmation required before writing

- **GIVEN** a project whose `_compagnon/config.yaml` has a thin context
- **WHEN** the user runs `/compagnon-configure`
- **THEN** a diff of the change is displayed
- **AND** the file is written only after explicit confirmation from
  the user

#### Scenario: Refusal when the config is missing

- **GIVEN** a project without `_compagnon/config.yaml`
- **WHEN** the user runs `/compagnon-configure`
- **THEN** the skill refuses to act
- **AND** points to `compagnon init`
