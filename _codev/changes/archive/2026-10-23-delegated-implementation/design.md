# Design: The developer role's model implements the tasks

## Context

Decision 0011 makes the model a property of the role, chosen by the
project, with no built-in default. `codev roles --json` already serves
each role's `model`, the `developer` included at `code.implement`. Until
now only the reviewing roles could use it, as sub-agents in the
`isolated` review mode (decision 0007: codev renders the permission, the
skill dispatches the sub-agent). `unused_models` in
`crates/codev-core/src/roles.rs` warns `role_model_unused` for every role
that does not review, the `developer` included.

The `Agent` tool reaches a skill's `allowed-tools` through the
`{{AGENT_TOOL}}` placeholder of `propose` and `apply`.
`substitute_agent_tool` in `crates/codev-agents/src/claude.rs` replaces
it according to the render context's `review_mode` alone.

The implementation evaluation (`code.implement`) plays implementation
cases. It reads their configuration from the tree at `ref`. Today
`--fixture-config` reaches only review cases.

## Goals / Non-Goals

**Goals**

- A project can run its implementation on another model than its
  session, through the `developer` role, and measure the result with
  `codev-evals compare`.
- Without a model on the `developer`, `/codev-apply` behaves exactly as
  before.

**Non-Goals**

- A built-in default model for the `developer`. Decision 0011 holds: the
  project chooses, and the new decision records the measure that
  supports choosing Sonnet.
- Running each task in its own sub-agent, or several tasks in parallel.
- Changing the planning roles, which never run as sub-agents.

## Decisions

### Decision: One sub-agent implements every task, the session keeps the review

When the `developer` has a model, step 3 of the skill starts one
sub-agent on that model. Its brief holds:

- the change's name;
- the path of the installed skill, whose steps 2 to 4 it follows (read
  the tasks, work through them, stop on an ambiguity or a blocker);
- the developer's criteria;
- the project's `rules` and `context` for the tasks.

It returns the tasks it checked and, if it stopped, the task with the
interpretations or the blocker. The brief also carries any instruction
the session runs under, such as an evaluation's "ask no question", so
the sub-agent behaves as the session would. The session re-reads
`tasks.md` from disk rather than trusting the report. It relays a stop
as step 4 says. Once the user answers, or once the session has made its
own assumption in a run without a user, it starts a new sub-agent on the
developer's model for the remaining tasks. The session never implements
a task itself while the developer has a model, except in the fallback.
When every task is checked, it runs steps 5 (the review) and 6 (the
summary) itself, on the session's model.

A failure is the platform failing to start or stopping the sub-agent, or
an answer that reports neither checked tasks nor a stop. A reported stop
is not a failure. The retried sub-agent first inspects the working tree
for work left on the first unchecked task, so partial work is neither
lost nor duplicated. The review and its sub-agents (decision 0007) do not
change.

This follows decision 0007: codev renders the permission and serves the
model, and the skill dispatches the sub-agent. No new command is needed.

**Alternatives considered**:

- One sub-agent per task. It would have to re-read the plan and the code
  for every task, and it would lose what the previous task learned. The
  measure behind this change ran the whole implementation in one context.
- Asking the user to switch the session's model (`/model sonnet`) before
  `/codev-apply`. The review and the summary would then run on Sonnet
  too, and the choice would not be the project's.

### Decision: The sub-agent tool follows the developer's model as well as the review mode

The render context gains whether the `developer` has a model.
`substitute_agent_tool` puts `Agent` in the `apply` skill when the
review mode is `isolated` or the developer has a model. It puts it in
`propose` only in the `isolated` mode. A change of the developer's model
changes the rendered `apply` skill, and `codev update` rewrites it as it
does for any skill that still matches its checksum.

**Alternatives considered**: always listing `Agent` in `apply`. That
would grant the sub-agent tool to projects that use neither feature, and
the skills spec forbids it.

### Decision: `role_model_unused` spares the developer only

A role's model is used when the role serves a reviewing step, or when it
is the `developer`, the one role the skill dispatches at
`code.implement`. `unused_models` checks exactly that. A project role
declared at `code.implement` with a model is warned, since no sub-agent
ever runs it.

**Alternatives considered**: sparing every role that serves
`code.implement`. A project role's model there would then be accepted
silently and never applied, which is the case the warning exists for.

### Decision: `--fixture-config` also reaches implementation cases

The runner appends the file to the implementation fixture's
`_codev/config.yaml` before `codev update --force` renders the skills.
The fixture then grants the sub-agent tool as a project would. The
`review:` key stays refused (decision 0008's runs record their mode).
The `role-evaluation` requirement that limited the option to review
cases is modified to match.
`--review-mode` still applies only to review cases: an implementation
case keeps the review mode of its tree. None of the three trees at
`ref` holds a `roles:` key, so the appended mapping is never a
duplicate. A suite test checks this, so that a future case cannot
silently ignore the configuration.

**Alternatives considered**: a dedicated `--developer-model` flag. It
would be another option for one setting, where `--fixture-config` already
carries any setting a measure needs.

### Decision: A new decision supersedes 0011

Decision 0011 says a role's model applies only in the isolated review
mode. This change makes the `developer`'s model apply at
`code.implement` in either mode. The new decision restates 0011 with
that scope, so the main specs and the accepted decision agree. The role
still owns its model, the project still chooses it, and there is still
no built-in default. The decision records the delegated setup's own
measure from task 5.1, not the session-wide one. It recommends
`developer: sonnet` only if that measure meets the bar the task states.
Otherwise it records the result and recommends nothing.

## Risks / Trade-offs

- [The sub-agent checks a box without verifying.] → It follows the
  skill's own steps, which forbid checking a box without verification.
  The evaluation's checks (format, lints, tests, acceptance, tasks)
  measure the delegated runs exactly as the others.
- [The sub-agent cannot ask the user.] → It stops and reports, and the
  session asks. Step 4 already says to stop at the first ambiguity, so
  nothing is decided in the sub-agent's place.
- [Two contexts cost more than one if the session re-reads everything.]
  → The session re-reads only `tasks.md` and the diff for the review,
  which it read anyway. The measure checks the total cost.
- [The platform cannot start the sub-agent.] → The skill retries once,
  then implements in the session and says so, like the isolated review's
  fallback (decision 0007).

## Open Questions

- Whether a cheaper model also suits the review roles depends on each
  role. That is the measure of decision 0011 and stays per project.
