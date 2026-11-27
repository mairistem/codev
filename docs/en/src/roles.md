# Roles

A Compagnon skill is one agent, but it does not do everything in the same frame
of mind. Each step is given a role: one question it owns, and "done when"
criteria the agent checks before moving on. You and your team approve at
defined points — the agent never decides those for you.

Roles are criteria, not role-play. The agent does not pretend to be a
product owner; it checks the proposal against the product owner's list. A
role whose criteria are met is done, whatever the prose sounds like. Some
roles also carry **knowledge** — the concrete signals, pitfalls per
language and references they need to check their criteria well — see
[Experts and domains](#experts-and-domains).

Roles are data, not prose buried in the skills. Compagnon serves them: the
skills ask `compagnon roles --at <step>` at each step and apply what comes
back, and `compagnon instructions` hands each artifact's role to the agent.
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

The step roles and the experts are built into Compagnon. The artifact roles
come from the default `spec-driven` schema, which declares one role per
artifact; a [custom schema](guides/custom-schemas.md) declares its own, or
none.

To read a role's "done when" criteria, ask Compagnon — they are the list the
agent receives:

```bash
compagnon roles --at plan.challenge
```

`compagnon roles` without `--at` lists every role with its activation; add
`--knowledge` to read a role's knowledge too. See
[`compagnon roles`](reference/cli.md#compagnon-roles).

## Steps

A change goes through seven steps, in two skills. At each one, some roles
write or build, others review, and you decide after the reviews:

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 406" role="img" aria-label="Who reviews when. The steps of a change across: plan.understand, plan.investigate, plan.write, plan.trace and plan.challenge in /compagnon-propose, then code.implement and code.review in /compagnon-apply. The roles down: the analyst works at plan.understand; the investigator at plan.investigate; the four artifact roles write at plan.write; the tracer reviews at plan.trace; the skeptic reviews at plan.challenge; the six experts review at plan.challenge and code.review when the change ticks their domain; the developer works at code.implement; the newcomer reviews at code.review; you approve the plan after plan.challenge and settle what is left after code.review.">
<text x="365" y="20" text-anchor="middle" class="mono" font-size="13" style="fill:var(--dg-muted)">/compagnon-propose</text>
<line x1="154" y1="30" x2="576" y2="30" style="stroke:var(--dg-line)" stroke-width="2"/>
<text x="666" y="20" text-anchor="middle" class="mono" font-size="13" style="fill:var(--dg-muted)">/compagnon-apply</text>
<line x1="584" y1="30" x2="748" y2="30" style="stroke:var(--dg-line)" stroke-width="2"/>
<text x="193" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">plan.</text>
<text x="193" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">understand</text>
<text x="279" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">plan.</text>
<text x="279" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">investigate</text>
<text x="365" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">plan.</text>
<text x="365" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">write</text>
<text x="451" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">plan.</text>
<text x="451" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">trace</text>
<text x="537" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">plan.</text>
<text x="537" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">challenge</text>
<text x="623" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">code.</text>
<text x="623" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">implement</text>
<text x="709" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">code.</text>
<text x="709" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">review</text>
<rect x="4" y="78" width="752" height="30" rx="6" style="fill:var(--dg-box)"/>
<text x="14" y="98" font-size="14" style="fill:var(--dg-ink)">Analyst</text>
<text x="14" y="128" font-size="14" style="fill:var(--dg-ink)">Investigator</text>
<rect x="4" y="138" width="752" height="30" rx="6" style="fill:var(--dg-box)"/>
<text x="14" y="158" font-size="14" style="fill:var(--dg-ink)">Artifact roles (4)</text>
<text x="14" y="188" font-size="14" style="fill:var(--dg-ink)">Tracer</text>
<rect x="4" y="198" width="752" height="30" rx="6" style="fill:var(--dg-box)"/>
<text x="14" y="218" font-size="14" style="fill:var(--dg-ink)">Skeptic</text>
<text x="14" y="248" font-size="14" style="fill:var(--dg-ink)">Experts (6)</text>
<rect x="4" y="258" width="752" height="30" rx="6" style="fill:var(--dg-box)"/>
<text x="14" y="278" font-size="14" style="fill:var(--dg-ink)">Developer</text>
<text x="14" y="308" font-size="14" style="fill:var(--dg-ink)">Newcomer</text>
<rect x="4" y="318" width="752" height="44" rx="6" style="fill:var(--dg-box)"/>
<text x="14" y="338" font-size="14" font-weight="600" style="fill:var(--dg-ink)">You</text>
<line x1="580" y1="40" x2="580" y2="362" style="stroke:var(--dg-line)" stroke-width="1.5" stroke-dasharray="4 4"/>
<circle cx="193" cy="93" r="8" style="fill:var(--dg-claude)"/>
<circle cx="279" cy="123" r="8" style="fill:var(--dg-claude)"/>
<circle cx="365" cy="153" r="8" style="fill:var(--dg-claude)"/>
<circle cx="451" cy="183" r="7" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3"/>
<circle cx="537" cy="213" r="7" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3"/>
<circle cx="537" cy="243" r="7" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3" stroke-dasharray="3 2.5"/>
<circle cx="709" cy="243" r="7" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3" stroke-dasharray="3 2.5"/>
<circle cx="623" cy="273" r="8" style="fill:var(--dg-claude)"/>
<circle cx="709" cy="303" r="7" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3"/>
<circle cx="537" cy="333" r="8" style="fill:var(--dg-you)"/>
<text x="537" y="357" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">approve</text>
<circle cx="709" cy="333" r="8" style="fill:var(--dg-you)"/>
<text x="709" y="357" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">settle</text>
<circle cx="20" cy="387" r="7" style="fill:var(--dg-claude)"/>
<text x="34" y="392" font-size="13" style="fill:var(--dg-muted)">works</text>
<circle cx="120" cy="387" r="6" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3"/>
<text x="134" y="392" font-size="13" style="fill:var(--dg-muted)">reviews</text>
<circle cx="240" cy="387" r="6" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3" stroke-dasharray="3 2.5"/>
<text x="254" y="392" font-size="13" style="fill:var(--dg-muted)">reviews if its domain is ticked</text>
<circle cx="520" cy="387" r="7" style="fill:var(--dg-you)"/>
<text x="534" y="392" font-size="13" style="fill:var(--dg-muted)">you decide</text>
</svg>
</div>
<figcaption>Who reviews when: Claude's roles write and review at each step; the experts join only when the change touches their domain, and you decide after each review.</figcaption>
</figure>

| Step | Skill | What happens |
|---|---|---|
| `plan.understand` | `/compagnon-propose` | The request is understood before the change is created; a ticket such as `PROJ-123` is read through the Jira MCP when one is configured |
| `plan.investigate` | `/compagnon-propose` | The code, tests, configuration and documentation are studied before each artifact is written |
| `plan.write` | `/compagnon-propose` | Each artifact is written, then checked against its role, which `compagnon instructions` returns in `role` |
| `plan.trace` | `/compagnon-propose` | The artifacts are checked against each other; gaps are fixed in the artifacts |
| `plan.challenge` | `/compagnon-propose` | The skeptic and the active experts re-read the plan; their findings go to the [review record](#findings-severity-confidence-and-the-review-record); at most five **Points to challenge**, each tagged with its role, and what you must decide end the summary |
| `code.implement` | `/compagnon-apply` | Each task is implemented, verified, then checked off |
| `code.review` | `/compagnon-apply` | The newcomer and the active experts re-read the change's code; what is unambiguous is simplified; at most five **Review notes**, each tagged with its role, and what you must decide end the summary — a high-severity security or privacy finding is never dropped |

The procedure of each step — what to write, the caps, the guardrails —
stays in the skill and is told in [How it works](workflow.md). The
criteria and the knowledge come from the role.

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
`compagnon validate` reports an unknown domain (`unknown_domain`) or a
malformed section (`domains_invalid`) as an error, and a missing section
on an active change (`domains_missing`) as a warning — see
[File formats](reference/file-formats.md#proposalmd).

**Activation `on_domain`.** An expert is active at its steps only when the
change's proposal ticks its domain. Without a change — `compagnon roles`
without `--change` — or while the proposal has no `## Domains` section, it
is inactive. `compagnon roles --json` says why each active role is active in
`activatedBy`: `always`, `project`, `change`, or `domain:<name>`. You can
still force an expert on or off, for the project or for one change — see
[Configuring roles](#configuring-roles).

**Knowledge.** An expert's knowledge comes in five sections, always in
this order:

- **Scope** — what it looks at, what it leaves aside, and what it does
  not report — for instance, the ops expert asks for no feature flag
  when deploying the previous version undoes the change, and the
  performance expert reports no constant-factor saving;
- **Signals** — concrete patterns to spot, each with a one-line example;
- **By stack** — pitfalls in Rust, TypeScript/JavaScript, Python, Go and
  Java; Compagnon serves the part for your project's languages (see
  [Stack filter](#triggers-and-suggested-domains));
- **References** — the authoritative standards, such as the OWASP Top 10,
  WCAG 2.2, RGAA 4 or the GDPR;
- **Reporting** — how to report a finding, with its severity, and what
  it must name: the failure an operator meets, the attack, the data and
  who receives it, the existing file or consumer that breaks.

The `newcomer` carries complexity signals per language, and groups what
makes one function hard to follow into one finding; it judges
readability, not correctness, and the tests the change has, not those it
lacks. The `skeptic` carries planning smells: it challenges the plan's
logic and evidence — need, scope, specs, decisions, assumptions, tasks,
and steps with no way back — and leaves each domain to its expert,
checking only that the proposal's `## Domains` ticks every domain the
plan touches, so that its expert reviews it. The other step roles carry
none. Knowledge is served only
to active roles: an expert that is not active costs the agent nothing.
Read it with `compagnon roles --at code.review --change <name> --knowledge`.

## Triggers and suggested domains

Ticking domains is a judgement made while planning, before the code
exists. So that a domain the change ends up touching is not missed
silently, each expert carries **triggers** — evidence that a change touches
its domain — and Compagnon matches them against the change's diff.

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
`_compagnon/config.yaml`.

**The diff.** Compagnon asks git, locally, for what the branch changed since it
left the default branch — `origin/HEAD`, else `main`, else `master` —
committed or not, plus the untracked files, limited to the project's
folder. It never contacts a remote. Without a diff, Compagnon says so and
suggests nothing, and no command fails because of it.

<details>
<summary>Under the hood: when there is no diff, and what it leaves out</summary>

Outside git, without a commit or a default branch — or in a shallow CI
clone without the merge-base; use `fetch-depth: 0` there — there is no
diff.

Some files are left out of every trigger: the planning folder `_compagnon/`
and the Compagnon skills (`planning`), vendored dependencies — `node_modules`,
`vendor`, `third_party` (`vendored`) —, lock files such as `Cargo.lock` or
`package-lock.json` (`lockfile`), and generated files such as `*.min.js`
or `*.pb.go` (`generated`). Binary files, prose (`.md`, `.txt`…) and very
large files are matched by their path only (`binary`, `prose`, `large`).
The scan stops at 2000 files, 5000 added lines per file, 1 MiB per
untracked file and 2 MiB of added lines in total, and ignores lines over
2000 characters; the diff then says it was truncated.

</details>

**Suggestions.** `compagnon roles --change <name>` reports the diff and each
suggested domain with its state:

- `ticked` — the proposal ticks it;
- `justified` — the proposal keeps it unticked with a reason on its line;
- `unticked` — neither: the proposal must answer it.

```markdown
## Domains

- [x] security
- [ ] personal-data — the fields matched are test fixtures
```

`compagnon validate` reports each `unticked` suggestion as the warning
`domain_suggested`, at the domain's line, naming the trigger and the file
that matched; `compagnon validate --strict` fails on it. `compagnon archive` and
`compagnon doctor` do not read the diff. The diff is the working tree's: two
active changes on one branch share it. A role set to `never` brings no
suggestion.

`/compagnon-propose` reads the suggestions before writing `## Domains`, and
the skeptic records a finding for each one left `unticked`. Before the
code exists the diff is usually empty, so the review is where it bites:
`/compagnon-apply` lists every suggestion still `unticked` under **To
decide** — tick it, and its expert reviews the code, or justify it.

**Stack filter.** Compagnon detects your project's languages from its
dependency manifests — at the root and up to three folders below it, so a
monorepo gets every stack — and the `By stack` part of the knowledge it
serves keeps those languages only. `compagnon roles --json` says which in
`stacks`; with no manifest found, everything is kept. `compagnon roles
--all-stacks` serves every language.


## Findings: severity, confidence and the review record

What the reviewing roles find — the skeptic and the experts at
`plan.challenge`, the newcomer and the experts at `code.review` — is
written to the change's **review record**, `_compagnon/changes/<name>/review.md`,
before anything is fixed or reported. Each finding quotes its evidence
verbatim at a location Compagnon can check, and carries a severity and a
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

Every reviewing role grades the same way. A `medium` or `high` finding
shown to you with confidence `high` quotes the line it is about and
states the scenario that fails; a concern that depends on what the
reviewer could not see — load, a caller, a configuration — and a
preference with no failure are recorded with confidence `low`. A role
does not report a concern another role owns, code or data the change
does not touch, what the plan explicitly excludes, a missing test its
criteria do not ask for, or a matter of taste. Each defect is reported
once, its consequences in its recommendation; two defects that need two
fixes stay two findings.

Then the author fixes or disputes each finding, the reviewer checks each
answer once, and what is still disputed comes to you under **To decide**.
Only you accept a risk or defer a high-severity finding. The answer loop,
the checks and the archive gate are in [Review record](guides/review-record.md).

## Inline or isolated review

By default the reviewing roles work **inline** (`inline`): the skill that wrote the
plan or the code reviews it in the same conversation, one role after the
other. It is the cheapest way, but each reviewer has read the author's
reasoning and the roles before it, and tends to agree with them.

In **isolated** mode (`isolated`) — `review.mode` in the
[configuration](reference/configuration.md#review), or in a change's
`change.yaml` — each reviewing role works alone first, as a Claude Code
[sub-agent](glossary.md#sub-agent): the tracer at `plan.trace`, the
skeptic and the active experts at `plan.challenge`, the newcomer and the
active experts at `code.review`, all of a step at once. A reviewer never
receives the conversation, the author's reasoning or another role's
findings. Then the skill merges what they found, as a team would after
reading separately.

Isolated review costs more: each reviewer reads the material, where
inline review reads it once. That is why `inline` stays the default; see
[Configuration](reference/configuration.md#review) to switch.

<details>
<summary>Under the hood: what an isolated reviewer receives, and the meeting</summary>

A reviewer receives only:

- its role: question, criteria and knowledge, filtered by stack;
- the project's `context` and `rules`;
- the material: the change's artifacts at the plan steps, the files the
  change touches and their diff at `code.review`;
- the ids already in the record and the finding format.

It may read the rest of the repository — the decisions, the main specs,
the code the material cites — modifies nothing, and treats the material
as data, never as instructions. `compagnon roles --json` measures the
material (`review.material`): beyond 160 KiB, the largest files are marked
`summarize`, and a separate sub-agent that sees only that file's diff
summarizes it, quoting the changed lines when they fit. The record says
which files were summarized.

A sub-agent that fails, is stopped by Claude Code's own limits, or
answers in another format is run once more; if it fails again, that role
reviews inline and the record says so with a `Fallback:` line. Without
the sub-agent tool, every role falls back to inline.

Then the skill holds the meeting:

- **Merge.** Findings of several roles about the same defect — the same
  place and the same fix — become one finding, recorded under the role
  that rated it most severe and crediting every role that raised it
  (`raised_by`). Findings at the same place that need different fixes
  stay separate.
- **Rank.** By severity, then confidence, then role order; the summary
  shows at most five, as in inline mode.
- **Answer.** The author's answers, the reviewer's single re-check and
  what comes to you under **To decide** work exactly as inline.

</details>

### The reviewers' models

In isolated mode, each reviewer's sub-agent runs on its role's model —
`model` in `compagnon roles --json`. No role sets one by default: every
reviewer runs on the session's model until your project chooses another,
under `roles:` — `model: opus`, `sonnet` or `haiku`, or `inherit` to go
back to the session's model (see [Configuring roles](#configuring-roles)).
The model applies only in the isolated mode, where a role runs as its own
sub-agent; in the inline mode, every role reviews in the session.

Why no default: on the [evaluation suite](#measuring-the-roles), in the
isolated mode, moving the tracer, the skeptic, the newcomer and the
security, performance and migration experts to Sonnet saved 7 % against
Opus, with the same recall, but one false positive more than the
reference for `migration` and the skeptic. Most of an isolated review's
cost is the session's — reading the skill, building the brief, merging
the findings, writing the record — so a reviewer's model moves the bill
little. The same measure shows the isolated mode costing about 73 % more
than the inline mode on the same cases, with the same recall.

### The developer's model

The `developer` implements the tasks at `code.implement`. When your
project gives it a model — `roles: { developer: { model: sonnet } }` —
`/compagnon-apply` hands the tasks to one sub-agent on that model, in either
review mode. The session keeps the review and the summary on its own
model. If the sub-agent stops on an ambiguity or a blocker, the session
asks you, then starts a new sub-agent on the same model with your answer.
If the sub-agent cannot run, the skill retries once, then implements in
the session and says so. Without a model, `/compagnon-apply` implements in
the session.

We recommend `developer: sonnet` ([decision 0012](reference/configuration.md#roles)).
On the [implementation evaluation](#measuring-the-roles), with an Opus
session and the developer on Sonnet, every quality measure stayed at
1.00: tasks checked, format, lints, tests, and the hidden acceptance
scripts. The cost fell by about 30 %, while the run took longer in wall
time, since the sub-agent's work adds to the session's. With the whole
session on Sonnet, the saving reaches 56 %, but the review and the
summary then run on Sonnet too. Planning stays on the strongest model.

## Configuring roles

Every role is active by default. In `_compagnon/config.yaml`, `roles:` maps a
role id to its setting:

```yaml
roles:
  newcomer: never            # short form: always | never
  skeptic:
    when: always             # optional, always when absent
    done_when:               # added after the role's own criteria
      - The rollback plan is stated
    model: inherit           # opus | sonnet | haiku | inherit
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
- **Model.** `opus`, `sonnet` or `haiku` replaces the role's default
  model, and `inherit` keeps the session's — see
  [The reviewers' models](#the-reviewers-models). It applies to the roles
  of `plan.trace`, `plan.challenge` and `code.review` in the isolated
  mode, and to the `developer` in either mode — see
  [The developer's model](#the-developers-model). Raise [`compagnon_version`](reference/configuration.md#compagnon_version)
  first: an older Compagnon refuses the key.
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

`compagnon validate` reports an unknown role id (`unknown_role`), an activation
other than `always`, `never` or `on_domain`, or `on_domain` on a role
without domains (`invalid_role_activation`), and `never` on a role that
cannot be disabled (`role_not_disableable`), and a model other than
`opus`, `sonnet`, `haiku` or `inherit` (`invalid_role_model`); `compagnon status`,
`compagnon instructions` and `compagnon roles` fail with the same code. A model on
a role that serves no reviewing step and is not the `developer` never
applies: `compagnon validate` warns with `role_model_unused`.
`compagnon doctor` lists the active roles, the roles active on domain with
their domains, and the disabled ones.


## Implementing a change (`/compagnon-apply`)

The developer implements the tasks in order, and checks a task off only
once its verification passes. Then the newcomer and the active experts
review the code the change added or modified, before the full test suite
runs once. [How it works](workflow.md) tells the whole step; the roles'
criteria are below.

<details>
<summary>Under the hood: the developer's and the reviewers' criteria</summary>

### Developer (`/compagnon-apply`)

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
after the [review](#newcomer-and-experts-compagnon-apply), it runs the
project's full test suite once; a failure is reported instead of the
summary. It never works around a failing test, never touches another
change, never syncs and never archives.

### Newcomer and experts (`/compagnon-apply`)

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

</details>

## The other skills

The other skills keep a fixed stance, written in the skill itself. These
stances are not roles: they are not in the catalog and cannot be
configured. What each skill does is in [Asking Claude](skills.md).

| Skill | Stance | Question it owns | Writes |
|---|---|---|---|
| `/compagnon-explore` | [Thinking partner](#thinking-partner-compagnon-explore) | Is this worth building, and how? | Nothing |
| `/compagnon-sync`, `/compagnon-archive` | [Maintainer of the living specs](#maintainer-of-the-living-specs-compagnon-sync-compagnon-archive) | Do the main specs describe what was planned and shipped? | Through the CLI only |
| `/compagnon-update` | [Editor](#editor-compagnon-update) | Is the revised artifact still consistent with the others? | `_compagnon/changes/<name>/` only |
| `/compagnon-onboard` | [Guide](#guide-compagnon-onboard) | Where does this project stand, and what comes next? | Nothing |
| `/compagnon-configure` | [Project setup analyst](#project-setup-analyst-compagnon-configure) | What must an agent know about this project before writing? | `_compagnon/config.yaml`, after confirmation |

<details>
<summary>Under the hood: each stance's criteria</summary>

### Thinking partner (`/compagnon-explore`)

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
  approach, scope, and an offer to run `/compagnon-propose`), information is
  missing (what, and who can provide it), or it was not worth pursuing.

It writes no file, creates no change, and does not implement even when
asked.

### Maintainer of the living specs (`/compagnon-sync`, `/compagnon-archive`)

Keeps `_compagnon/specs/` describing the system. `/compagnon-sync` merges a change's
deltas without moving it; `/compagnon-archive` merges them and moves the change
to `_compagnon/changes/archive/<date>-<name>/`. Done when:

- planning is complete — otherwise nothing is run;
- the merge goes through `compagnon sync` or `compagnon archive`, never through a
  file the skill writes itself;
- the specs created, updated and unchanged are reported (and, for archive,
  where the change moved).

On a validation error, both refuse: they answer that the change has errors
and point to `compagnon validate <name>`, without retrying or fixing the change
themselves. Any other error is relayed as the CLI wrote it.

### Editor (`/compagnon-update`)

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
- `compagnon validate` has run and its report is relayed.

It refuses to create a missing artifact, to touch an archived change, or to
modify code.

### Guide (`/compagnon-onboard`)

Introduces Compagnon to someone discovering it. Done when it has shown three
blocks: what Compagnon is, the state of the project (counts of specs,
decisions and changes, and up to five missing setup items from
`compagnon doctor` with their fix), and exactly one recommended next action,
quoted as the command to type. It is strictly read-only: it never runs a
fix, not even `compagnon init`.

### Project setup analyst (`/compagnon-configure`)

Proposes the `context:` and per-artifact `rules:` of `_compagnon/config.yaml`
from the project itself. Done when:

- the reading stays within its budget — README and CONTRIBUTING in full, at
  most 5 documentation files, 8 source files and 3 test files;
- `context:` holds 2 to 5 factual lines that complement the detected stack;
- `rules:` holds 1 to 2 rules for `specs`, `design` and `tasks`, each
  positive, checkable on review and grounded in what the project does, plus
  testing rules taken from the project's existing tests;
- the diff is shown and you are asked before anything is written.

It writes only on your yes, changes only `context:` and `rules:`, keeps
every other field and comment, and refuses if `_compagnon/config.yaml` does not
exist.

</details>

## Measuring the roles

Compagnon's own repository measures the built-in reviewing roles — the
skeptic, the tracer, the newcomer and the six experts — with an evaluation
suite, `evals/`: small projects seeded with defects each role should find,
and clean controls where it should find nothing. The suite never runs in
regular CI and is not part of the `compagnon` binary.
[Reliability of the roles](reliability.md) shows the scores role by role,
how they are measured, the thresholds each role meets or fails and the
limits of the measure.

<details>
<summary>Under the hood: the baseline and judging a change to a role</summary>

The reference scores are the committed baseline,
[`evals/baseline/report.md`](https://github.com/mairistem/compagnon/blob/main/evals/baseline/report.md):
five runs of every case, on Claude Code's default model at the date it
names, each score the median of the five runs. A model does not answer the
same way twice, so a change to a role file is judged with
`compagnon-evals compare --quality-only` against the baseline, on the same
model: each role's median must stay within the range the baseline's runs
spanned.

</details>

## Where you decide

The agent stops and hands over at these points:

- **Clarifying questions.** `/compagnon-propose` asks before creating a change
  when an ambiguity would materially change the result, and asks again when
  an artifact needs a decision or an Open Question changes what needs to be
  built. `/compagnon-explore` asks the questions the code cannot answer.
  `/compagnon-apply` stops at an ambiguity or a blocker and lets you decide.
- **Approving the plan.** `/compagnon-propose` stops once the artifacts are
  written. Start your review with the Points to challenge, what is left to
  decide and the design's Open Questions; when you are satisfied, ask for `/compagnon-apply`. To change
  an artifact first, use `/compagnon-update`.
- **Accepting decisions.** A design follows the decisions in effect or
  explicitly proposes to supersede one. A decision takes effect only when
  you run [`compagnon decision accept`](concepts.md#decision).
- **Confirming the configuration.** `/compagnon-configure` writes only after you
  answer yes to its diff.
- **Applying a ripple effect.** `/compagnon-update` reports what else the
  revision affects; you accept, decline or defer each follow-up.
- **Updating Compagnon.** When a newer Compagnon is available or the project
  requires another version, the skill asks **Update now** or **Later**
  before acting (see [Update notice](skills.md#update-notice)).
- **Configuring the roles.** Which review roles run, and which criteria
  they add, is your project's choice in `roles:` — see
  [Configuring roles](#configuring-roles).
- **Settling the review.** A dispute the reviewer maintained, or a
  high-severity finding nobody could fix, comes to you under **To decide**:
  fix it, accept the risk with a justification, or defer it to another
  change or an issue. The skill records your decision with your name; it
  never accepts a risk for you. `compagnon review` shows what is left.
- **Archiving.** `/compagnon-apply` never archives: once you have reviewed the
  implementation, starting with its Review notes, say "archive the change"
  (shortcut: `/compagnon-archive`, or `compagnon archive` in a terminal).

## Go further

- [Review record](guides/review-record.md)
- [How it works](workflow.md)
- [Configuration](reference/configuration.md)
- [Custom schemas](guides/custom-schemas.md)
- [Reliability of the roles](reliability.md)
