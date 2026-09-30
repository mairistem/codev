## Purpose

Describes the `/codev-configure` skill — the only one that lets Claude
enrich `_codev/config.yaml` from a read-only exploration of the
project. It comes after `codev init`: the CLI probe has
filled in what it could (stack, MCPs), the skill covers the rest
(conventions, style, structuring choices) that only an LLM can read.

## ADDED Requirements

### Requirement: `configure` skill enriches `_codev/config.yaml` by analyzing the project

The codev catalog SHALL expose a `configure` workflow — installed
under `.claude/skills/codev-configure/SKILL.md`, invocable as
`/codev-configure` — whose role is to enrich `_codev/config.yaml`
by analyzing the project, without modifying the workflows, the
detected MCPs, or the schema.

The operation MUST be:

1. **Read the existing file** — the project's `_codev/config.yaml`.
2. **Read-only exploration** — root `README.md`,
   `CONTRIBUTING.md` if it exists, contents of `docs/`, a sample
   of the most edited source files (via `git log --pretty=format: --name-only | sort | uniq -c | sort -rn | head`),
   structure of the main folders.
3. **Patch proposal** — new value for `context:` (2 to 5
   lines focused on what matters for steering the skills — stack
   beyond the language, API conventions, error style, comment
   tone, structuring library choices) and for per-artifact
   `rules:` (`specs:`, `design:`, `tasks:` — 1 to 2
   rules each, grounded in what the project does).
4. **Display of the diff** in the output, without writing.
5. **Write on user confirmation** only.

The skill MUST **preserve** the fields it does not touch — `schema`,
`workflows`, `mcp`, `inherits`, as well as existing provenance
comments. It does not rewrite the `mcp:` key detected by
`codev init`.

The skill MUST **refuse to act** if `_codev/config.yaml` is absent —
it points the user to `codev init`.

#### Scenario: Role documented in the catalog

- **GIVEN** the codev workflow catalog
- **WHEN** the `configure` workflow is resolved
- **THEN** its entry exists (`find("configure").is_some()`)
- **AND** its `body` cites the five steps (read, exploration,
  proposal, diff, confirmation)
- **AND** its `body` explicitly lists the preserved fields
  (`schema`, `workflows`, `mcp`, `inherits`)

#### Scenario: Mandatory confirmation before writing

- **GIVEN** a project with a `_codev/config.yaml` whose context is thin
- **WHEN** the user runs `/codev-configure`
- **THEN** a diff of the modification is displayed
- **AND** the file is written only after explicit confirmation by
  the user

#### Scenario: Refusal if config is absent

- **GIVEN** a project without `_codev/config.yaml`
- **WHEN** the user runs `/codev-configure`
- **THEN** the skill refuses to act
- **AND** points to `codev init`
