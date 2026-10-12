# Proposal: Built-in experts with their own knowledge, activated by the domains a change touches

## Why

A role today is a question and a "done when" list: it says what to check,
not the knowledge needed to check it well — "no vulnerability" is worthless
without knowing where vulnerabilities appear. And no role covers security,
personal data, accessibility, performance, operations or migration, so a
plan touching them is reviewed by a generalist skeptic at best.

## What Changes

- Each built-in step role moves to its own file, `assets/roles/<id>.md`: a
  YAML frontmatter holding the catalog data (id, names, step or steps,
  question, required, done when, and the expert's domains), then a
  markdown body — the role's **knowledge** — in fixed sections: Scope,
  Signals, By stack, References, Reporting. Existing criteria stay
  verbatim; `newcomer` (complexity signals per stack) and `skeptic`
  (planning smells) gain a knowledge body, the others none.
- A role may serve several steps: `at` accepts a list.
- Six expert roles: `security`, `privacy`, `accessibility`, `performance`,
  `ops` and `migration`, each at `plan.challenge` and `code.review`, each
  bound to a domain (`security`, `personal-data`, `user-interface`,
  `performance`, `operations`, `migration`).
- The proposal gains a `## Domains` section: the six domains as a list to
  tick, or `None`. `codev validate` reports an unknown domain
  (`unknown_domain`, error), a malformed section (`domains_invalid`,
  error) and a missing section on an active change (`domains_missing`,
  warning); archived changes are never checked.
- A new activation, `on_domain`: the role is active at its steps when the
  change's proposal ticks one of its domains. Experts default to it; the
  other roles keep `always`. `on_domain` on a role without domains is
  `invalid_role_activation`. Per-project and per-change overrides keep
  working.
- `codev roles --json` gains, per role, `steps`, `domains`,
  `activatedBy` (`always`, `domain:<name>`, `project`, `change`, or `null`
  when inactive) and `knowledge` (the role's body, served for active roles
  only). Human output prints the knowledge with `--knowledge`. Without
  `--change`, `on_domain` roles are inactive.
- **BREAKING** `analyst` becomes required: `analyst: never` is now
  `role_not_disableable`.
- `codev doctor`'s `config.roles` line names the roles active by domain.
- `/codev-propose`'s challenge applies every active role (the skeptic and
  the active experts) with their knowledge; the Points to challenge stay
  capped at five in total, each tagged with its role id.
  `/codev-apply`'s review applies the newcomer and the active experts; its
  **Review notes** (formerly Readability notes) are capped at five in
  total, each tagged with its role id, except that a high-severity security
  or privacy finding is never dropped. An expert never changes behavior
  silently: a behavior-changing fix is reported, unless it is a clear
  defect within the change's scope, then fixed with a test.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `roles` — file-per-role catalog with knowledge, multi-step roles,
  experts and their domains, `on_domain` activation, `analyst` required,
  `knowledge` and `activatedBy` in `codev roles`.
- `validation` — the `## Domains` section of an active change and its
  codes.
- `skills` — the proposal's `## Domains` in the default schema; the
  challenge and the review with the experts, their caps and the severity
  exception.
- `doctor` — `config.roles` names the roles active by domain.
- `docs` — the Roles chapter lists the experts and explains domains,
  knowledge and `on_domain`.

### Removed Capabilities

None.

## Impact

- `assets/roles.yaml` replaced by `assets/roles/*.md` (twelve files);
  `assets/schemas/spec-driven/schema.yaml` and `templates/proposal.md`
  (`## Domains`); `assets/workflows/propose.md` and `apply.md`; the
  regenerated skills under `.claude/skills/`.
- `codev-core`: `roles` (role file parsing, steps list, domains,
  `on_domain`, `activatedBy`, analyst required), new `domains` module
  (proposal parser and codes), `doctor` (`config.roles` message),
  `validate` (code uniqueness).
- `codev-engine`: `roles` (the change's domains), `validate` (domains
  findings).
- `codev-cli`: `codev roles --knowledge`, `RoleV1` fields, human rendering,
  doctor facts, docs parity test.
- JSON contract v1: additive — `steps`, `domains`, `activatedBy`,
  `knowledge` on a role; the value `on_domain` for `when`; three
  validation codes.
- Documentation, `docs/en` and `docs/fr`: `roles.md`,
  `reference/configuration.md`, `reference/file-formats.md`,
  `reference/cli.md`, `reference/json-output.md`, `skills.md`,
  `workflow.md` (review notes); the landing page's challenged-plan card.
- A project whose configuration holds `analyst: never` fails until the
  entry is removed; active changes without `## Domains` get a warning,
  which fails `codev validate --strict`.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [x] migration

## Release note

- Added: Six built-in experts — security, privacy, accessibility,
  performance, operations and migration — challenge the plan and review the
  code when the change's proposal ticks their domain, each with concrete
  knowledge (signals, pitfalls per language, references) served by
  `codev roles`.
- Added: The proposal has a `## Domains` section listing the domains the
  change touches, or `None`; `codev validate` reports an unknown domain or
  a malformed section as an error and a missing section as a warning,
  which fails `codev validate --strict`: add the section, or `None`, to
  your active changes.
- Added: A role's activation can be `on_domain`; `codev roles --json`
  returns each role's `steps`, `domains`, `activatedBy` and, for active
  roles, its `knowledge`, which `codev roles --knowledge` prints.
- Changed: `/codev-propose` caps its Points to challenge at five across the
  skeptic and the experts, and `/codev-apply` ends with up to five Review
  notes (formerly Readability notes), each tagged with its role; a
  high-severity security or privacy finding is never dropped.
- Changed: **BREAKING** The `analyst` role can no longer be disabled:
  remove `analyst: never` from `roles:` in `_codev/config.yaml` or a
  change's `change.yaml`.
- Changed: `codev doctor` names the roles activated by domain.
