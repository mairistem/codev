# Proposal: deliver `/codev-onboard`

## Why

A user discovering codev — whether they have just installed the
binary, or they open Claude Code on a project where someone else has
already run `codev init` — does not know where to start. The six
existing skills (`propose`, `explore`, `apply`, `sync`, `archive`,
`update`) are enough to **work**, but none introduces itself. The
README explains the concept, but Claude Code does not read it.

Result today: new users type `/help`, see a list of slash commands
without hierarchy, and have no landmark to tell "where to enter" from
"when to use". `/codev-onboard` delivers that landmark — a single
invocation, a map of the terrain, a recommended action.

Also useful for the new **MCP workflows**: when the first real JVS
project plugs in a Jira MCP, the user opening it for the first time
will want an entry point explaining "here is how this codev + Jira
combination works here". The onboard skill, from the parent project or
inherited, will play that role.

## What Changes

- **New `onboard` workflow** in the catalog, invocable as
  `/codev-onboard`. Role: show the current state of the project
  (initialized or not, specs present, active changes, indexed
  decisions) and recommend the next action.
- **New file `assets/workflows/onboard.md`** — the skill's body, loaded
  via `include_str!` like the six others.
- **`onboard` is added to `DEFAULT_WORKFLOWS`** — going from
  `["propose", "explore"]` to `["propose", "explore", "onboard"]`. A
  user who runs `codev init` in a new project therefore sees
  `/codev-onboard` available immediately, with no opt-in to add in
  `config.yaml`.
- **Strict read-only boundary** — no writes, no change creation, no
  call to `codev init`. The skill **guides**, it **does not act** in
  the user's place.
- **Restricted `allowed-tools`** — `Bash(codev:*), Read, Glob`. No
  general `Bash`; no `Edit`, no `Write`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills` — two new ADDED requirements: presence of `onboard` in the
  catalog, and its inclusion in `DEFAULT_WORKFLOWS`.

### Removed Capabilities

None.

## Impact

- **Code**: new entry `Workflow { id: "onboard", … }` in the `CATALOG`
  of `codev-agents::workflows`, `DEFAULT_WORKFLOWS` extended to three
  entries, and two dedicated tests (presence, `allowed-tools`).
- **Existing test `sans_demande_installe_le_catalogue_par_defaut`** —
  minor adjustment: the expected list goes from two to three entries,
  and the "opt-in" check goes from 4 to 3 workflows
  (`apply`, `sync`, `archive`, `update` stay opt-in; `onboard`
  becomes default).
- **Test `chaque_workflow_a_un_corps_…`** — automatically covers
  `onboard`.
- **Repository config**: `_codev/config.yaml` gains `- onboard` in its
  `workflows` list — for dogfooding, since the repository uses what it
  builds.
- **JSON contract**: nothing. The skill reads existing JSON (`codev list
  --specs --json` does not exist yet — the skill will read the existing
  human outputs, consistent with `apply`/`update`).
- **Migration**: none. Projects that already have a `config.yaml` with
  an explicit `workflows:` list do **not** gain `onboard`
  automatically (the default only applies when the key is absent) —
  they add it when they want.
- **Out of scope**:
  - **A step-by-step interactive tutorial** ("now type this, then
    that…") — too prescriptive; the skill informs and proposes, the
    user acts.
  - **A `--refresh` mode that resets state** — not a presentation
    problem, not this skill's responsibility.
  - **Automatic detection of MCP patterns** (ticket numbers, Figma
    URLs) — that is the role of action skills (`propose`, `apply`),
    not of `onboard`.
