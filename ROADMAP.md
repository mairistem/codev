# Roadmap

This page lists what is planned for codev, and what is deliberately out of
scope. It is a direction, not a schedule: priorities follow what users need.
To weigh in, open an issue or comment on an existing one.

## Planned

**Reading and tracking**

- `codev show` to read a spec, a change or its deltas from the terminal,
  including a diff of what a change will do to the main specs.
- Task progress in `codev status`, computed from the checkboxes of `tasks.md`.
- A check that archived changes have no unchecked task.
- Reading the specs of inherited sources, not only their context, rules and
  decisions.

**Validation**

- Warnings for specs that are hard to maintain: a placeholder or very short
  Purpose, overly long requirements, deltas that touch too many requirements.
- Faster validation of large projects, by validating items in parallel.

**Schemas and workflows**

- Commands to create, fork, validate and locate schemas, and to list templates.
- Additional workflows: resume a partially planned change, fast-forward
  through planning, verify an implementation against its specs, and archive
  several changes at once in chronological order, with detection of conflicts
  between them.

**Agents and comfort**

- A generic skills target (`.agents/skills/`), for agents other than Claude
  Code.
- Machine-wide preferences, next to the per-project configuration.
- A notice when a newer codev version is available.
- An interactive dashboard of changes, specs and decisions.

## Out of scope

- **Telemetry.** codev collects nothing.
- **Separate command files.** A Claude Code skill is already invocable as a
  slash command; a second file per workflow would only drift from it.
- **Adapters for dozens of tools.** codev focuses on doing one integration
  well. If you need broad tool coverage today,
  [OpenSpec](https://github.com/Fission-AI/OpenSpec) is a good fit.
- **A built-in feedback command.** Use
  [GitHub issues](https://github.com/mairistem/codev/issues).
