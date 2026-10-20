# Proposal: Give each role the model its reviews run on

## Why

Every review runs on the session's model, Opus. Measured on the
evaluation suite, the reviewers on Sonnet cost 49 % less and find every
seeded defect, with valid evidence. Six roles stay within the baseline's
range on every measure. Three — `accessibility`, `ops` and `privacy` —
raise one false positive more than the baseline allows. Planning on
Sonnet, by contrast, loses a fifth of its recall. So the model must be
chosen per role, not per session, and a team must be able to change it.

## What Changes

- Each role carries a model: `sonnet` by default for `tracer`,
  `skeptic`, `newcomer`, `security`, `performance` and `migration`; none
  for every other role, which keeps the session's model — Opus for
  planning, implementation and the three experts above.
- A project sets a role's model under `roles:` in `_codev/config.yaml`
  (`model: opus | sonnet | haiku | inherit`); an unknown value is
  reported with `invalid_role_model`.
- `codev roles --json` gives each role its `model` and `modelOrigin`.
- In the `isolated` review mode, the `propose` and `apply` skills start
  each reviewer's sub-agent on its role's model. The `inline` mode is
  unchanged: every role runs in the session.

## Capabilities

### New Capabilities

### Modified Capabilities

- roles
- skills

### Removed Capabilities

## Impact

- `codev-core::roles` and `assets/roles/*.md`: the model of each role,
  its default, its project setting and its validation.
- `codev-cli`: `model` and `modelOrigin` in the roles document of
  `contract::v1` (additions), and the human output of `codev roles`.
- `assets/workflows/fragments/review-isolated.md`: the sub-agent's model.
- Documentation in `docs/en` and `docs/fr`: the roles chapter, the
  configuration reference, the JSON reference.

## Domains

- [ ] security — no new input reaches a command; the model name is chosen from four fixed values
- [ ] personal-data — no personal data is read or written
- [ ] user-interface — no interface beyond the CLI's text output, which gains one column
- [x] performance
- [ ] operations — no log, metric, timeout or deployment changes
- [x] migration

## Release note

- Changed: Reviewers that the evaluation suite measured unchanged on Sonnet — the tracer, the skeptic, the newcomer, and the security, performance and migration experts — now run on Sonnet in the isolated review mode, for about half the cost; every other role keeps the session's model.
- Added: `roles: { <role>: { model: opus | sonnet | haiku | inherit } }` in `_codev/config.yaml` sets the model a role's review runs on.
