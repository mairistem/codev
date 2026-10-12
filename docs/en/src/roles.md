# Roles

A codev skill is one agent, but it does not do everything in the same frame
of mind. Each step is given a role: one question it owns, and "done when"
criteria the agent checks before moving on. You and your team approve at
defined points — the agent never decides those for you.

Roles are criteria, not role-play. The agent does not pretend to be a
product owner; it checks the proposal against the product owner's list. A
role whose criteria are met is done, whatever the prose sounds like.

Roles are data, not prose buried in the skills. codev serves them: the
skills ask `codev roles --at <step>` at each step and apply what comes
back, and `codev instructions` hands each artifact's role to the agent.
Your project can turn a role off, add its own criteria, and override the
activation for one change — see [Configuring roles](#configuring-roles).

## Built-in roles

| Id | Name | Kind | Step | Question it owns |
|---|---|---|---|---|
| `analyst` | Analyst | step | `plan.understand` | What must be built? |
| `investigator` | Investigator | step | `plan.investigate` | What does the project already do? |
| `product` | Product owner | artifact (`proposal`) | `plan.write` | Why this change, and what changes? |
| `qa` | QA analyst | artifact (`specs`) | `plan.write` | What observable behavior must hold? |
| `architect` | Architect | artifact (`design`) | `plan.write` | How will it be built? |
| `lead` | Tech lead | artifact (`tasks`) | `plan.write` | In what order, and how is each step verified? |
| `tracer` | Tracer | step | `plan.trace` | Do the artifacts agree with each other? |
| `skeptic` | Skeptic | step | `plan.challenge` | Would the plan survive a reviewer who did not write it? |
| `developer` | Developer | step | `code.implement` | Does each task work, verified? |
| `newcomer` | Newcomer | step | `code.review` | Can a developer who has not seen the plan follow this code? |

The step roles are built into codev. The artifact roles come from the
default `spec-driven` schema, which declares one role per artifact; a
[custom schema](guides/custom-schemas.md) declares its own, or none.

To read a role's "done when" criteria, ask codev — they are the list the
agent receives:

```bash
codev roles --at plan.challenge
```

`codev roles` without `--at` lists every role with its activation. See
[`codev roles`](reference/cli.md#codev-roles).

## Steps

| Step | Skill | What happens |
|---|---|---|
| `plan.understand` | `/codev-propose` | The request is understood before the change is created; a ticket such as `PROJ-123` is read through the Jira MCP when one is configured |
| `plan.investigate` | `/codev-propose` | The code, tests, configuration and documentation are studied before each artifact is written |
| `plan.write` | `/codev-propose` | Each artifact is written, then checked against its role, which `codev instructions` returns in `role` |
| `plan.trace` | `/codev-propose` | The artifacts are checked against each other; gaps are fixed in the artifacts |
| `plan.challenge` | `/codev-propose` | The plan is re-read by a reviewer who did not write it; what is unambiguous is fixed, what needs you goes to the design's Open Questions, and at most five **Points to challenge** end the summary |
| `code.implement` | `/codev-apply` | Each task is implemented, verified, then checked off |
| `code.review` | `/codev-apply` | Once every task is checked, the change's code is re-read as a newcomer; what is unambiguous is simplified, and at most three **Readability notes** end the summary |

The procedure of each step — what to write, the caps, the guardrails —
stays in the skill. The criteria come from the role.

## Configuring roles

Every role is active by default. In `_codev/config.yaml`, `roles:` maps a
role id to its setting:

```yaml
roles:
  newcomer: never            # short form: always | never
  skeptic:
    when: always             # optional, always when absent
    done_when:               # added after the role's own criteria
      - The rollback plan is stated
```

- **Activation.** `always` or `never`. A step whose role is `never` is
  skipped, and the skill's summary says it is disabled by configuration.
- **Extra criteria.** `done_when` entries are added after the role's own
  criteria, never replacing them. They carry the origin `project`, like your
  [`context` and `rules`](reference/configuration.md#context).
- **Per change.** A change's `change.yaml` can override the activation for
  that change only:

  ```yaml
  roles:
    newcomer: always
  ```

- **What cannot be disabled.** The artifact roles (`product`, `qa`,
  `architect`, `lead`) and the `developer`: their artifact is still
  written, their tasks still implemented. Their criteria can be extended.
- **Inherited sources** do not contribute roles: only your project's
  `roles:` is read.

`codev validate` reports an unknown role id (`unknown_role`), an activation
other than `always` or `never` (`invalid_role_activation`), and `never` on a
role that cannot be disabled (`role_not_disableable`); `codev status`,
`codev instructions` and `codev roles` fail with the same code.
`codev doctor` lists the active and disabled roles.

## Implementing a change (`/codev-apply`)

### Developer (`/codev-apply`)

Implements the tasks of a planned change, in file order. For each task,
it applies the developer's criteria:

- the files concerned are read before writing;
- the code, tests, configuration and documentation the task asks for are
  written — nothing more;
- tests are written like the project's existing suite, then by the
  conventions of its language and the project's `rules`;
- the verification the task cites is run and passes — the test command
  comes from `context`, `rules`, the manifest or the CI configuration, and
  the skill asks you when it finds none;
- only then is the box checked off.

It stops at a material ambiguity or a technical blocker without checking
the task off, and describes the options. When every box is checked, and
after the [newcomer review](#newcomer-codev-apply), it runs the
project's full test suite once; a failure is reported instead of the
summary. It never works around a failing test, never touches another
change, never syncs and never archives.

### Newcomer (`/codev-apply`)

Once every task is checked, and before the full test suite runs, re-reads
only the code the change added or modified, as a developer joining the
project who has seen neither the plan nor the conversation, along the
newcomer's criteria. What is unambiguous is simplified and the affected
tests are run again; the rest becomes at most three **Readability notes**
in the final summary — what, where, why — or "No readability concern
found".

It never changes behavior, never widens the change's scope, and never
touches code the change did not add or modify.

## The other skills

The other skills keep a fixed stance, written in the skill itself. These
stances are not roles: they are not in the catalog and cannot be
configured.

| Skill | Stance | Question it owns | Writes |
|---|---|---|---|
| `/codev-explore` | [Thinking partner](#thinking-partner-codev-explore) | Is this worth building, and how? | Nothing |
| `/codev-sync`, `/codev-archive` | [Maintainer of the living specs](#maintainer-of-the-living-specs-codev-sync-codev-archive) | Do the main specs describe what was planned and shipped? | Through the CLI only |
| `/codev-update` | [Editor](#editor-codev-update) | Is the revised artifact still consistent with the others? | `_codev/changes/<name>/` only |
| `/codev-onboard` | [Guide](#guide-codev-onboard) | Where does this project stand, and what comes next? | Nothing |
| `/codev-configure` | [Project setup analyst](#project-setup-analyst-codev-configure) | What must an agent know about this project before writing? | `_codev/config.yaml`, after confirmation |

### Thinking partner (`/codev-explore`)

Explores an idea, a problem or a need, without committing to anything. Done
when:

- what the repository already knows comes first — specs, changes in
  progress, decisions in effect, the relevant code — so you are never asked
  what the code can answer;
- the questions asked are about a dependency, a constraint or a trade-off,
  each with a recommended default answer and why;
- two or three options are compared, each with what it costs and rules
  out, and a recommendation is made;
- a direction that departs from an accepted decision is flagged early;
- assumptions are stated as assumptions;
- the exploration ends one of three ways, named: it crystallizes (problem,
  approach, scope, and an offer to run `/codev-propose`), information is
  missing (what, and who can provide it), or it was not worth pursuing.

It writes no file, creates no change, and does not implement even when
asked.

### Maintainer of the living specs (`/codev-sync`, `/codev-archive`)

Keeps `_codev/specs/` describing the system. `/codev-sync` merges a change's
deltas without moving it; `/codev-archive` merges them and moves the change
to `_codev/changes/archive/<date>-<name>/`. Done when:

- planning is complete — otherwise nothing is run;
- the merge goes through `codev sync` or `codev archive`, never through a
  file the skill writes itself;
- the specs created, updated and unchanged are reported (and, for archive,
  where the change moved).

On a validation error, both refuse: they answer that the change has errors
and point to `codev validate <name>`, without retrying or fixing the change
themselves. Any other error is relayed as the CLI wrote it.

### Editor (`/codev-update`)

Revises one planning artifact that already exists in an active change.
Done when:

- the artifact and the others it might affect are read from disk before
  writing;
- the revision stays within the artifact's contract — expected sections,
  scenario format, checkboxes;
- the ripple effect is announced: an orphaned or missing spec file after a
  proposal edit, a task citing a removed requirement or renamed scenario, a
  design decision that no longer matches, a task contradicting a
  requirement — with a suggested next action, applied only on your
  confirmation;
- `codev validate` has run and its report is relayed.

It refuses to create a missing artifact, to touch an archived change, or to
modify code.

### Guide (`/codev-onboard`)

Introduces codev to someone discovering it. Done when it has shown three
blocks: what codev is, the state of the project (counts of specs,
decisions and changes, and up to five missing setup items from
`codev doctor` with their fix), and exactly one recommended next action,
quoted as the command to type. It is strictly read-only: it never runs a
fix, not even `codev init`.

### Project setup analyst (`/codev-configure`)

Proposes the `context:` and per-artifact `rules:` of `_codev/config.yaml`
from the project itself. Done when:

- the reading stays within its budget — README and CONTRIBUTING in full, at
  most 5 documentation files, 8 source files and 3 test files;
- `context:` holds 2 to 5 factual lines that complement the detected stack;
- `rules:` holds 1 to 2 rules for `specs`, `design` and `tasks`, each
  positive, checkable on review and grounded in what the project does, plus
  testing rules taken from the project's existing tests;
- the diff is shown and you are asked before anything is written.

It writes only on your yes, changes only `context:` and `rules:`, keeps
every other field and comment, and refuses if `_codev/config.yaml` does not
exist.

## Where you decide

The agent stops and hands over at these points:

- **Clarifying questions.** `/codev-propose` asks before creating a change
  when an ambiguity would materially change the result, and asks again when
  an artifact needs a decision or an Open Question changes what needs to be
  built. `/codev-explore` asks the questions the code cannot answer.
  `/codev-apply` stops at an ambiguity or a blocker and lets you decide.
- **Approving the plan.** `/codev-propose` stops once the artifacts are
  written. Start your review with the Points to challenge and the design's
  Open Questions; when you are satisfied, ask for `/codev-apply`. To change
  an artifact first, use `/codev-update`.
- **Accepting decisions.** A design follows the decisions in effect or
  explicitly proposes to supersede one. A decision takes effect only when
  you run [`codev decision accept`](concepts.md#decision).
- **Confirming the configuration.** `/codev-configure` writes only after you
  answer yes to its diff.
- **Applying a ripple effect.** `/codev-update` reports what else the
  revision affects; you accept, decline or defer each follow-up.
- **Updating codev.** When a newer codev is available or the project
  requires another version, the skill asks **Update now** or **Later**
  before acting (see [Update notice](skills.md#update-notice)).
- **Configuring the roles.** Which review roles run, and which criteria
  they add, is your project's choice in `roles:` — see
  [Configuring roles](#configuring-roles).
- **Archiving.** `/codev-apply` never archives: once you have reviewed the
  implementation, starting with its Readability notes, run `/codev-archive` or `codev archive`.
