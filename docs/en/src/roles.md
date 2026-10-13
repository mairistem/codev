# Roles

A codev skill is one agent, but it does not do everything in the same frame
of mind. Each step is given a role: one question it owns, and "done when"
criteria the agent checks before moving on. You and your team approve at
defined points — the agent never decides those for you.

Roles are criteria, not role-play. The agent does not pretend to be a
product owner; it checks the proposal against the product owner's list. A
role whose criteria are met is done, whatever the prose sounds like. Some
roles also carry **knowledge** — the concrete signals, pitfalls per
language and references they need to check their criteria well — see
[Experts and domains](#experts-and-domains).

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
| `security` | Security | expert (domain `security`) | `plan.challenge`, `code.review` | Can an attacker abuse what this change adds? |
| `privacy` | Privacy | expert (domain `personal-data`) | `plan.challenge`, `code.review` | Does this change collect, keep or share personal data only as it must? |
| `accessibility` | Accessibility | expert (domain `user-interface`) | `plan.challenge`, `code.review` | Can everyone use what this change adds, with a keyboard or assistive technology? |
| `performance` | Performance | expert (domain `performance`) | `plan.challenge`, `code.review` | Does this change stay fast as data and load grow? |
| `ops` | Operations | expert (domain `operations`) | `plan.challenge`, `code.review` | Can the team run, observe and roll back what this change adds? |
| `migration` | Migration | expert (domain `migration`) | `plan.challenge`, `code.review` | Can existing data, configuration and consumers move to this change safely? |
| `developer` | Developer | step | `code.implement` | Does each task work, verified? |
| `newcomer` | Newcomer | step | `code.review` | Can a developer who has not seen the plan follow this code? |

The step roles and the experts are built into codev. The artifact roles
come from the default `spec-driven` schema, which declares one role per
artifact; a [custom schema](guides/custom-schemas.md) declares its own, or
none.

To read a role's "done when" criteria, ask codev — they are the list the
agent receives:

```bash
codev roles --at plan.challenge
```

`codev roles` without `--at` lists every role with its activation; add
`--knowledge` to read a role's knowledge too. See
[`codev roles`](reference/cli.md#codev-roles).

## Steps

| Step | Skill | What happens |
|---|---|---|
| `plan.understand` | `/codev-propose` | The request is understood before the change is created; a ticket such as `PROJ-123` is read through the Jira MCP when one is configured |
| `plan.investigate` | `/codev-propose` | The code, tests, configuration and documentation are studied before each artifact is written |
| `plan.write` | `/codev-propose` | Each artifact is written, then checked against its role, which `codev instructions` returns in `role` |
| `plan.trace` | `/codev-propose` | The artifacts are checked against each other; gaps are fixed in the artifacts |
| `plan.challenge` | `/codev-propose` | The plan is re-read by the skeptic and the experts the change's domains activate; every finding goes to the [review record](#findings-severity-confidence-and-the-review-record), the author fixes or disputes it and the reviewer checks the answer; at most five **Points to challenge**, across all roles and each tagged with its role, and what you must decide end the summary |
| `code.implement` | `/codev-apply` | Each task is implemented, verified, then checked off |
| `code.review` | `/codev-apply` | Once every task is checked, the change's code is re-read by the newcomer and the active experts; every finding goes to the review record; what is unambiguous is simplified, and at most five **Review notes**, each tagged with its role, and what you must decide end the summary — a high-severity security or privacy finding is never dropped |

The procedure of each step — what to write, the caps, the guardrails —
stays in the skill. The criteria and the knowledge come from the role.

## Experts and domains

Six experts join the review steps when a change touches their domain. Each
one is active at `plan.challenge` — does the design address the domain? —
and at `code.review` — does the implementation respect it?

| Domain | Expert | Looks at |
|---|---|---|
| `security` | `security` | Input validation at trust boundaries, server-side authorization, secrets, injection, unsafe deserialization, new dependencies |
| `personal-data` | `privacy` | Purpose and minimisation, retention and deletion, data subject rights, personal data in logs, processors and transfers |
| `user-interface` | `accessibility` | Keyboard access, focus, text alternatives, form labels and errors, contrast, semantics and ARIA |
| `performance` | `performance` | Queries in loops, unbounded results, blocking I/O on hot paths, allocations in loops, cost of new dependencies |
| `operations` | `ops` | Structured logs without secrets, health and metrics, timeouts and retries, a rollback path, actionable errors |
| `migration` | `migration` | Breaking changes announced, migration paths, rollout order, reversible schema migrations, deprecation |

**Domains.** The proposal lists the domains the change touches in its
`## Domains` section — the six above as a list to tick, or the single line
`None`:

```markdown
## Domains

- [x] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [x] operations — new background worker
- [ ] migration
```

The product owner checks that every domain the change touches is ticked.
`codev validate` reports an unknown domain (`unknown_domain`) or a
malformed section (`domains_invalid`) as an error, and a missing section
on an active change (`domains_missing`) as a warning — see
[File formats](reference/file-formats.md#proposalmd).

**Activation `on_domain`.** An expert is active at its steps only when the
change's proposal ticks its domain. Without a change — `codev roles`
without `--change` — or while the proposal has no `## Domains` section, it
is inactive. `codev roles --json` says why each active role is active in
`activatedBy`: `always`, `project`, `change`, or `domain:<name>`. You can
still force an expert on or off, for the project or for one change — see
[Configuring roles](#configuring-roles).

**Knowledge.** An expert's knowledge comes in five sections, always in
this order:

- **Scope** — what it looks at, and what it leaves aside to avoid noise;
- **Signals** — concrete patterns to spot, each with a one-line example;
- **By stack** — pitfalls in Rust, TypeScript/JavaScript, Python, Go and
  Java; codev serves the part for your project's languages (see
  [Stack filter](#triggers-and-suggested-domains));
- **References** — the authoritative standards, such as the OWASP Top 10,
  WCAG 2.2, RGAA 4 or the GDPR;
- **Reporting** — how to report a finding, with its severity.

The `newcomer` carries complexity signals per language and the `skeptic`
planning smells; the other step roles carry none. Knowledge is served only
to active roles: an expert that is not active costs the agent nothing.
Read it with `codev roles --at code.review --change <name> --knowledge`.

## Triggers and suggested domains

Ticking domains is a judgement made while planning, before the code
exists. So that a domain the change ends up touching is not missed
silently, each expert carries **triggers** — evidence that a change touches
its domain — and codev matches them against the change's diff.

**Three kinds of trigger**, in groups that each name the domain they
suggest:

- `paths` — globs over the changed files, relative to the project root,
  ignoring case: `**/auth/**`, `**/*.tsx`. `*` stays within a folder, `**`
  crosses folders;
- `manifests` — dependency manifest names, at any depth: `Cargo.toml`,
  `package.json`, `go.mod`;
- `patterns` — regular expressions over the lines the diff **adds**:
  `(?i)\b(ALTER|DROP)\s+(TABLE|COLUMN)\b`.

| Expert | Its triggers, in outline |
|---|---|
| `security` | `auth`, `security`, `crypto` folders, `.env` files, keys; every dependency manifest; credential names, `eval`, `innerHTML`, `pickle`, `shell=True`, `unsafe {` |
| `privacy` | `user`, `customer`, `profile`, `account` models; e-mail, phone, birth date, street address, postcode, SSN, IBAN, IP address, geolocation |
| `accessibility` | markup, style sheets, components, server templates; `aria-*`, `role=`, `alt=`, form and media elements |
| `performance` | `*.sql`, query and repository folders, benchmarks; SQL statements, `Promise.all`, `tokio::spawn`, caches, pagination |
| `ops` | Dockerfiles, compose files, CI workflows, Kubernetes, Helm, Terraform, deployment folders, logging configuration; environment variables, timeouts, retries, health checks, metrics |
| `migration` | migrations, schemas, `*.proto`, `*.graphql`, OpenAPI, `*.d.ts`; `ALTER`/`DROP`/`RENAME`, deprecations, `serde` renames |

The built-in patterns favour precision: `access_token` rather than `token`
(a lexer has tokens), `street_address` rather than `address` (a socket
has one). What they miss for your project, add with
[`triggers:`](reference/configuration.md#triggers) in
`_codev/config.yaml`.

**The diff.** codev asks git, locally, for what the branch changed since it
left the default branch — `origin/HEAD`, else `main`, else `master` —
committed or not, plus the untracked files, limited to the project's
folder. It never contacts a remote. Outside git, without a commit or a
default branch — or in a shallow CI clone without the merge-base; use
`fetch-depth: 0` there — there is no diff: codev says so and suggests
nothing, and no command fails because of it.

Some files are left out of every trigger: the planning folder `_codev/`
and the codev skills (`planning`), vendored dependencies — `node_modules`,
`vendor`, `third_party` (`vendored`) —, lock files such as `Cargo.lock` or
`package-lock.json` (`lockfile`), and generated files such as `*.min.js`
or `*.pb.go` (`generated`). Binary files, prose (`.md`, `.txt`…) and very
large files are matched by their path only (`binary`, `prose`, `large`).
The scan stops at 2000 files, 5000 added lines per file, 1 MiB per
untracked file and 2 MiB of added lines in total, and ignores lines over
2000 characters; the diff then says it was truncated.

**Suggestions.** `codev roles --change <name>` reports the diff and each
suggested domain with its state:

- `ticked` — the proposal ticks it;
- `justified` — the proposal keeps it unticked with a reason on its line;
- `unticked` — neither: the proposal must answer it.

```markdown
## Domains

- [x] security
- [ ] personal-data — the fields matched are test fixtures
```

`codev validate` reports each `unticked` suggestion as the warning
`domain_suggested`, at the domain's line, naming the trigger and the file
that matched; `codev validate --strict` fails on it. `codev archive` and
`codev doctor` do not read the diff. The diff is the working tree's: two
active changes on one branch share it. A role set to `never` brings no
suggestion.

`/codev-propose` reads the suggestions before writing `## Domains`, and
the skeptic records a finding for each one left `unticked`. Before the
code exists the diff is usually empty, so the review is where it bites:
`/codev-apply` lists every suggestion still `unticked` under **To
decide** — tick it, and its expert reviews the code, or justify it.

**Stack filter.** codev detects your project's languages from its
dependency manifests — at the root and up to three folders below it, so a
monorepo gets every stack — and the `By stack` part of the knowledge it
serves keeps those languages only. `codev roles --json` says which in
`stacks`; with no manifest found, everything is kept. `codev roles
--all-stacks` serves every language.

## Findings: severity, confidence and the review record

What the reviewing roles find — the skeptic and the experts at
`plan.challenge`, the newcomer and the experts at `code.review` — is
written to the change's **review record**, `_codev/changes/<name>/review.md`,
before anything is fixed or reported. Each finding quotes its evidence
verbatim at a location codev can check, and carries a severity and a
confidence on one shared scale:

| Severity | Meaning |
|---|---|
| `high` | The change would break a requirement, lose or expose data, or open a vulnerability, and must not be archived until it is fixed or a human accepts the risk. |
| `medium` | A real defect or risk with a limited impact or a workaround, to fix in this change or to defer deliberately. |
| `low` | An improvement, such as readability, consistency or a minor cost, that does not decide whether the change is correct. |

| Confidence | Meaning |
|---|---|
| `high` | The evidence shows the problem directly. |
| `low` | The problem depends on something the reviewer could not see or verify. |

A low-confidence finding stays in the record and is answered like any
other, but it is not shown to you unless you are asked to decide it.

Then the team works like a team: the author — the role that owns the
artifact, or the developer for code — fixes each finding or disputes it
with an argument; the reviewer checks each answer once, verifying the fix
or accepting the dispute (`rejected`) or maintaining it; what is still
disputed comes to you, with both arguments, under **To decide**. Only you
accept a risk or defer a high-severity finding, and the record says who
decided. `codev archive` refuses a change whose record is missing for a
step where roles ran, or that still has a high-severity finding open or
disputed. The full format, the checks and the gate are in
[Review record](guides/review-record.md).

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

- **Activation.** `always`, `never` or `on_domain`. A step whose roles are
  all inactive is skipped, and the skill's summary says it is disabled by
  configuration. `on_domain` is the experts' default; it is refused on a
  role bound to no domain. Set `security: always` to have the security
  expert review every change, or `accessibility: never` in a project
  without a user interface.
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
  `architect`, `lead`), the `analyst` — the guard against a plan built on
  a misunderstanding — and the `developer`: their artifact is still
  written, the request still understood, their tasks still implemented.
  Their criteria can be extended.
- **Inherited sources** do not contribute roles: only your project's
  `roles:` is read.

`codev validate` reports an unknown role id (`unknown_role`), an activation
other than `always`, `never` or `on_domain`, or `on_domain` on a role
without domains (`invalid_role_activation`), and `never` on a role that
cannot be disabled (`role_not_disableable`); `codev status`,
`codev instructions` and `codev roles` fail with the same code.
`codev doctor` lists the active roles, the roles active on domain with
their domains, and the disabled ones.

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
after the [review](#newcomer-and-experts-codev-apply), it runs the
project's full test suite once; a failure is reported instead of the
summary. It never works around a failing test, never touches another
change, never syncs and never archives.

### Newcomer and experts (`/codev-apply`)

Once every task is checked, and before the full test suite runs, re-reads
only the code the change added or modified: the newcomer, as a developer
joining the project who has seen neither the plan nor the conversation,
and the experts the change's domains activate, each along its criteria and
knowledge. Every finding goes to the [review
record](#findings-severity-confidence-and-the-review-record) first. What
is unambiguous is simplified and the affected tests are run again; the
rest becomes at most five **Review notes** in the final summary — each
tagged with its role and its finding id: what, where, why — or "No review
concern found", followed by what you must decide. A high-severity security
or privacy finding is always listed, even beyond five.

The review never widens the change's scope and never touches code the
change did not add or modify. The newcomer never changes behavior. An
expert never changes it silently: a behavior-changing fix becomes a review
note, unless it is a clear defect within the change's scope, which is then
fixed with a test and named in the summary.

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

## Measuring the roles

codev's own repository measures the built-in reviewing roles — the
skeptic, the tracer, the newcomer and the six experts — with an evaluation
suite, `evals/`. Each case is a small project with a planned change: either
**seeded** with defects the role should find, each with its expected
severity and location, or a **clean control** holding none. The suite has
Claude Code run one skill step on each case with the evaluated role alone,
then scores the review record it wrote:

- **true positives** and **misses** — the seeded defects found or not. A
  finding counts for a defect when its role, its location (two lines of
  tolerance, the same heading of the plan, or another place the case
  allows) and its category agree: each seeded defect names a category of
  issue, such as `access-control` or `keyboard-trap`, recognised by its
  terms in the finding's title or recommendation. A finding about a
  neighbouring issue two lines away does not stand for the defect;
- **false positives** — high-confidence findings that match no seeded
  defect, in a control or next to the defect; the report marks those
  found at a defect's place but naming another issue. Low-confidence
  findings, which are never shown to you, are counted apart;
- **evidence validity** — the share of findings whose quoted evidence
  `codev review` finds at the cited lines;
- **severity agreement** — the found severity equals the expected one.

The tracer, which fixes the plan instead of recording findings, is scored
on the plan it leaves: the missing task added, a complete plan left
unchanged.

The cases are written to make a careless reviewer miss: subtle defects
next to code that does the same thing correctly (an authorization checked
on the list endpoint but not on the detail one, personal data reaching the
logs through an error message), cases with two defects of different
severity, and controls holding code that looks suspicious but is right (a
query assembled from an allow-list with bound values, logs of identifiers
that are not personal data). A case's plan cites only files and tests its
project contains, so that a gap in the case is never counted against the
role.

The reference scores are the committed baseline,
[`evals/baseline/report.md`](https://github.com/mairistem/codev/blob/main/evals/baseline/report.md):
one run per case, on Claude Code's default model at the date it names. One
run is a noisy point, not a proof; a change to a role file is judged by
its scores before and after, on the same model. The suite never runs in
regular CI and is not part of the `codev` binary.

## Where you decide

The agent stops and hands over at these points:

- **Clarifying questions.** `/codev-propose` asks before creating a change
  when an ambiguity would materially change the result, and asks again when
  an artifact needs a decision or an Open Question changes what needs to be
  built. `/codev-explore` asks the questions the code cannot answer.
  `/codev-apply` stops at an ambiguity or a blocker and lets you decide.
- **Approving the plan.** `/codev-propose` stops once the artifacts are
  written. Start your review with the Points to challenge, what is left to
  decide and the design's Open Questions; when you are satisfied, ask for `/codev-apply`. To change
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
- **Settling the review.** A dispute the reviewer maintained, or a
  high-severity finding nobody could fix, comes to you under **To decide**:
  fix it, accept the risk with a justification, or defer it to another
  change or an issue. The skill records your decision with your name; it
  never accepts a risk for you. `codev review` shows what is left.
- **Archiving.** `/codev-apply` never archives: once you have reviewed the
  implementation, starting with its Review notes, run `/codev-archive` or `codev archive`.
