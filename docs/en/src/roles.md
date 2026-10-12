# Roles

A codev skill is one agent, but it does not do everything in the same frame
of mind. Each step is given a role: one question it owns, and "done when"
criteria the agent checks before moving on. You and your team approve at
defined points — the agent never decides those for you.

Roles are criteria, not role-play. The agent does not pretend to be a
product owner; it checks the proposal against the product owner's list. A
role whose criteria are met is done, whatever the prose sounds like.

The four artifact roles — product owner, QA analyst, architect, tech lead —
are written in the schema: each artifact's instruction starts with a
`Role:` line and a `Done when:` list, which `codev instructions` hands to
the agent. A [custom schema](guides/custom-schemas.md) defines its own. The
other roles below name the steps of each skill's workflow, as the
[skills](skills.md) carry them out.

## Planning a change (`/codev-propose`)

`/codev-propose` plays every role below, in this order, in one pass. It
writes planning artifacts only, and stops for your review.

| Role | Question it owns | Done when | Where |
|---|---|---|---|
| Business analyst | What must be built? | The request is understood — if it is not clear, an open question is asked; an ambiguity that would materially change the scope, the observable behavior, compatibility or the acceptance criteria is asked before the change is created; a minor detail becomes a reasonable assumption recorded in the artifacts; a ticket identifier such as `PROJ-123` is read with a single read-only call to the Jira MCP when one is configured, and cited at the top of the proposal | Steps 0–1: ticket, understanding the request |
| Code investigator | What does the project already do? | `context` and `rules` are read; the relevant implementation, nearby tests, configuration and documentation are inspected, read-only and in proportion to the change; scope, approach and tasks are grounded in what was found; observed behavior, assumptions and proposals are kept apart; contradictions with existing specs are reported, not settled alone; no "explore the code" task is left for implementation | Step 4c, before writing each artifact |
| Product owner | Why this change, and what changes? | Why states the problem and why now in one or two sentences; What Changes are concrete bullets; every breaking change is marked **BREAKING**; every capability listed has a spec file (or `skip_specs: true`); the release note is written for end users, categorized, with breaking items marked, or `None` for an internal change; no implementation detail | [`proposal.md`](reference/file-formats.md#proposalmd) |
| QA analyst | What observable behavior must hold? | Each requirement uses SHALL or MUST on observable behavior; each requirement has at least one nominal scenario and one error or edge-case scenario, or says why none applies; no internal names, libraries or implementation steps | [Spec deltas](reference/file-formats.md#spec-delta) |
| Architect | How will it be built? | Each decision names the alternative it rejects and why; each risk has a mitigation; the accepted decisions that constrain the approach are cited and followed, or a supersession is explicitly proposed; the approach is the simplest that satisfies the specs, and a newcomer can follow the design without knowing its history; Open Questions only holds what can genuinely wait | [`design.md`](reference/file-formats.md#designmd) — written only when the change calls for it |
| Tech lead | In what order, and how is each step verified? | Each task fits one work session and is ordered by dependency; each task names its verification (test, command, observable behavior); every spec scenario is covered by a task's verification, with an automated test — a manual check only when automation is impossible, with the reason; the affected documentation is updated in a task, or the plan states that none is affected; tests follow the project's existing suite, then its language's conventions and the project's `rules` | [`tasks.md`](reference/file-formats.md#tasksmd) |
| Traceability checker | Do the artifacts agree with each other? | Every capability in the proposal has its spec file, and every spec file is listed in the proposal; every requirement has a nominal and an error or edge-case scenario; every scenario is covered by a task whose verification names an automated test, or says why automation is impossible; a task updates the documentation when the Impact touches documented behavior; the release note agrees with the plan; every verification is concrete. Gaps are fixed in the artifacts | Step 6 |
| Skeptical reviewer (contrarian) | Would the plan survive a reviewer who did not write it? | The plan is re-read for need, scope, specs, decisions, assumptions and tasks — plus security & privacy, compatibility & migration, operability, performance and accessibility & UX when the Impact touches them, and the project's `rules`; what is unambiguous is fixed; what needs a human decision is recorded in the design's Open Questions (or the proposal's, without a design); no scope is added; at most five **Points to challenge**, ranked by impact, or "No point to challenge found" | Step 7 |

The last role is yours: the skill presents the plan with its Points to
challenge and waits. See [Where you decide](#where-you-decide).

## The other skills

| Skill | Role | Question it owns | Writes |
|---|---|---|---|
| `/codev-explore` | [Thinking partner](#thinking-partner-codev-explore) | Is this worth building, and how? | Nothing |
| `/codev-apply` | [Developer](#developer-codev-apply) | Does each task work, verified? | Project code, `tasks.md` checkboxes |
| `/codev-apply` | [Newcomer reviewer](#newcomer-reviewer-codev-apply) | Can a developer who has not seen the plan follow this code? | Simplifications within the change |
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

### Developer (`/codev-apply`)

Implements the tasks of a planned change, in file order. Done when, for
each task:

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
after the [newcomer review](#newcomer-reviewer-codev-apply), it runs the
project's full test suite once; a failure is reported instead of the
summary. It never works around a failing test, never touches another
change, never syncs and never archives.

### Newcomer reviewer (`/codev-apply`)

Once every task is checked, and before the full test suite runs, re-reads
only the code the change added or modified, as a developer joining the
project who has seen neither the plan nor the conversation. Done when:

- each changed function makes sense without the plan or the conversation;
- names say what things do, with no obscure abbreviation;
- a function too long or too nested is split, when it splits simply;
- dense one-liners, premature genericity and single-use abstractions give
  way to the simple version, unless the clever one is justified;
- comments explain the why where it is not obvious, and never paraphrase
  the code;
- tests read as a description of the behavior;
- the project's `rules`, such as size thresholds, are met;
- what is unambiguous is simplified and the affected tests are run again;
  the rest becomes at most three **Readability notes** in the final summary
  — what, where, why — or "No readability concern found".

It never changes behavior, never widens the change's scope, and never
touches code the change did not add or modify.

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
- **Archiving.** `/codev-apply` never archives: once you have reviewed the
  implementation, starting with its Readability notes, run `/codev-archive` or `codev archive`.
