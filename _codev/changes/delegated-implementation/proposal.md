# Proposal: The developer role's model implements the tasks

## Why

The implementation evaluation shows that Sonnet implements codev's past
changes exactly as well as Opus: every task checked, the project's
checks passing and every acceptance assertion holding, on 20 runs each.
Sonnet does it for 56 % less. Planning stays on the strongest model
(decision 0009). Today `/codev-apply` implements in the session, so a
project cannot run its implementation on one model and its planning
and review on another. The model a project gives the `developer` role
is reported as unused (`role_model_unused`).

## What Changes

- When `codev roles --at code.implement` gives the `developer` role a
  model, `/codev-apply` hands the tasks to one sub-agent running on that
  model. The sub-agent works through them as the skill's steps say:
  study, implement, verify, check off, and stop on an ambiguity or a
  blocker. It reports back what it did and what stopped it. The session
  then keeps what it does today: the review (`code.review`) and the
  summary. Without a model, `/codev-apply` implements in the session as
  before.
- A model on the `developer` role is used. It is no longer reported as
  `role_model_unused`. The warning stays for the roles that never run as
  a sub-agent.
- `codev-evals run --fixture-config` also appends its configuration to
  the implementation cases. A run with
  `roles: { developer: { model: sonnet } }` can thus be compared with the
  Opus baseline.
- A new decision supersedes decision 0011: the model is still the
  role's, chosen by the project, with no built-in default, but the
  `developer`'s model applies at `code.implement` in either review mode.
  It records the measure of the delegated setup.

## Capabilities

### New Capabilities

### Modified Capabilities

- roles
- skills
- implementation-evaluation
- role-evaluation

### Removed Capabilities

## Impact

- `crates/codev-core/src/roles.rs` (where a role's model applies) and
  the validation of `role_model_unused`.
- `assets/workflows/apply.md`, the source of the `/codev-apply` skill;
  installed skills change at the next `codev update`.
- `evals/src/runner.rs` (`--fixture-config` on implementation cases).
- `_codev/decisions/`: a new decision.
- Documentation: the Roles chapter, the Skills chapter (which skills
  request `Agent`, and when) and the configuration reference, in
  `docs/en` and `docs/fr`; `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`
  for `--fixture-config`.

## Domains

None

## Release note

- Added: a model set on the `developer` role (`roles: { developer: { model: sonnet } }`) now runs the implementation of `/codev-apply` in a sub-agent on that model, while the session keeps the review; without a model nothing changes.
