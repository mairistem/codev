# Proposal: Give each role the model its reviews run on

## Why

Every review runs on the session's model. A team may want a cheaper
model for some reviewers, or a stronger one for others, and has no way to
say so. This change lets each project choose a role's model. It ships no
default: measured on the evaluation suite in the isolated mode, the
reviewers on Sonnet saved 7 % against Opus and raised one false positive
more than the reference for `migration` and `skeptic` — below decision
0008's bar for a default.

## What Changes

- Each role may carry a model, set by the project under `roles:` in
  `_codev/config.yaml` (`model: opus | sonnet | haiku | inherit`); no
  built-in role sets one, so nothing changes until a project does. An
  unknown value is reported with `invalid_role_model`; a model on a role
  that never reviews, with the warning `role_model_unused`.
- `codev roles --json` gives each role its `model` and `modelOrigin`.
- In the `isolated` review mode, the `propose` and `apply` skills start
  each reviewer's sub-agent on its role's model. The `inline` mode is
  unchanged: every role runs in the session.
- An inherited source's `roles:` is no longer parsed, so a key written
  for a newer codev never drops the source's context and rules.

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
  `contract::v1` (additions).
- `assets/workflows/fragments/review-isolated.md`: the sub-agent's model.
- Documentation in `docs/en` and `docs/fr`: the roles chapter, the
  configuration reference, the JSON reference.

## Domains

- [ ] security — no new input reaches a command; the model name is chosen from four fixed values
- [ ] personal-data — no personal data is read or written
- [ ] user-interface — no interface beyond the CLI's JSON output, which gains two fields
- [x] performance
- [ ] operations — no log, metric, timeout or deployment changes
- [x] migration

## Release note

- Added: `roles: { <role>: { model: opus | sonnet | haiku | inherit } }` in `_codev/config.yaml` sets the model a role's review runs on in the isolated review mode; no role sets one by default. A project that sets it should first raise `codev_version` to this release, since an older codev refuses the key — and a source shared through `inherits:` should not set it until every project inheriting from it runs this release.
