---
name: codev-apply
description: "Implement the tasks of an already planned codev change: read tasks.md, work through each unchecked box in order, checking them off along the way. Modifies project code. Does not modify other changes, does not archive and does not sync — those steps stay explicit on the user's side."
allowed-tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash"
license: MIT
metadata:
  generator: codev
  version: "0.6.1"
---

Implement the tasks of a codev change — work through each `- [ ]` box in
`tasks.md` in order, check them off as you go, stop at the first blocker.

**Implementation boundary.** This workflow **writes project code**: it is
the only codev workflow that touches anything outside the `_codev/` folder.
In return, it is strictly confined to the named change: it modifies no other
change, it does not archive, it does not sync. Those are explicit next
steps, for the user to request.

---

## Input

A change name as argument, or nothing (in which case the change is resolved
implicitly if there is only one active change).

## Roles

Steps 3 and 5 belong to a role. At each one, run:

```bash
codev roles --at <step> --change "<name>" --json
```

Apply every criterion of every role returned (`roles[].doneWhen`), the
project's included, using the role's `knowledge` when it has one. Its
`By stack` part comes already filtered to the project's languages
(`stacks`): use it as served. If `roles` is empty, the role is disabled by
configuration: skip the step, and say so in the summary.

## Steps

### 1. Resolve the change and check that planning is complete

If the user named a change, use that one. Otherwise, run:

```bash
codev list
```

- A single active change → use it.
- Several active changes → ask the user which one, listing the names. Do
  not guess.
- No active change → say so, and suggest `/codev-propose` to create one.

Then check the planning:

```bash
codev status --change "<name>" --json
```

**Update check — once per action, before acting.** In this JSON:

- `versionRequirement.satisfied` is `false` → say in one line that this
  project requires codev `<versionRequirement.requirement>`;
- otherwise, `update.available` is `true` and `update.snoozed` is `false`
  → say in one line that codev `<update.latest>` is available, with
  `<update.url>`;
- otherwise, say nothing about updates.

In the first two cases, ask one question with two answers:

- **Update now** → run `codev self-update --yes`, report its outcome in one
  line, then continue the action.
- **Later** → for an available update, run `codev self-update --snooze`;
  for a requirement, run nothing. Then continue the action.

Never ask again during this action, and never in the middle of it.

If `isPlanningComplete` is `false`, stop: planning is not ready. Say which
artifacts are missing and suggest `/codev-propose` or editing by hand.
Implement nothing.

### 2. Read the tasks

```
Read _codev/changes/<name>/tasks.md
```

The expected format is strict:

- a task: `- [ ] X.Y Description, verified by <test or command>`
- a checked task: `- [x] X.Y …`
- groups under `## N. …` headings

If you find a different format (`-[ ]` without a space, uppercase `- [X]`,
another marker such as `- [-]`), point it out to the user and offer to fix
it before continuing.

### 3. Work through each unchecked task, in file order — `code.implement`

Ask for the roles of this step once, before the first task: the
developer's criteria apply to every task. The project's `rules` and
`context` come from `codev instructions tasks --change "<name>" --json`.

For each `- [ ]` encountered, in the order it appears:

**a. Announce.** "Task X.Y: <description summarized in a fragment>."

**b. Study.** Read the files the task concerns.

**c. Implement.** Write what the task asks for.

**d. Verify.** Run the command, the test, the observation the task cites.

**e. Check off.** Edit `tasks.md` with `Edit`: the line `- [ ] X.Y …`
becomes `- [x] X.Y …`. Be precise — a `replace_all` would also replace
tasks from other changes read earlier; avoid it. Use the full old line as
the anchor.

**f. Brief feedback to the user.** "✓ X.Y — <what it produced, in one
line>". Move on to the next one.

### 4. Stop on ambiguity or blocker

A task cannot be carried out in two cases:

- **Material ambiguity**: its wording allows several interpretations that
  would materially change the result (API choice, output format, behavior
  on an edge case). In that case:
  - **do not check it off**,
  - describe the interpretations to the user,
  - suggest splitting the task into `X.Y.a` / `X.Y.b` in `tasks.md`
    — but let the user decide or rephrase.
- **Technical blocker**: missing dependency, a test that cannot run, a
  command that requires manual intervention. In that case:
  - **do not check it off**,
  - describe the blocker,
  - suggest the resolution you think is best, without applying it.

In both cases, stop after the current task. The following tasks are not
processed until the obstacle is cleared.

### 5. Review the change — `code.review`

When every box is checked, re-read only the code this change added or
modified — its diff — with every role returned, each along its criteria
and knowledge: the newcomer, as a developer joining the project who has
seen neither the plan nor this conversation; and the experts the
proposal's `## Domains` activates, asking whether the implementation
respects their domain.

Simplify directly what is unambiguous, then re-run the affected tests.
Never widen the change's scope, never touch code the change did not add or
modify. The newcomer never changes behavior. An expert never changes
behavior silently: a behavior-changing fix becomes a review note, unless it
is a clear defect within the change's scope — then fix it, add a test that
fails without the fix, and name the fix in the summary. Keep the rest for
the review notes. Here `<step>` is `code.review`.

Now that the code exists, the roles of this step come with the domains
its diff suggests (`suggestions`). Every suggestion whose `state` is
`unticked` goes under **To decide**, with its first match and the two
choices: tick the domain — its expert then reviews this code: ask for the
roles of the step again and run that expert's review — or keep it
unticked with a reason on its line. Do not edit the proposal yourself:
the user decides.

**The review record.** Every finding of this step goes to
`_codev/changes/<name>/review.md`, the change's review record, before it
is fixed or reported.

a. **Resume first.** Run `codev review --change "<name>" --json`. A
   finding already in `findings[]` is never raised again, and a finding
   whose `awaiting` is `author` or `reviewer` is answered before anything
   new. The record is the state: a session that stopped half-way picks up
   from it.

b. **Write a section per role.** Under `## <step>` (create the file with
   `# Review record: <name>` and `<!-- codev-review: 1 -->` if it does not
   exist), one `### <role-id>` per role returned, holding its findings, or
   the line `No finding.` when it found nothing. For each role that serves
   this step but that `codev roles --change "<name>" --json` shows with
   `when: never`, write `Disabled by configuration.` instead. Never write
   a step or a role twice: add to the existing section.

c. **Write each finding** as a heading `#### R-<role-id>-<n> <title>` —
   `<n>` the next number free for that role in the whole record, never
   reused — followed by one fenced `yaml` block:

   ```yaml
   severity: high
   confidence: high
   location: "src/auth/session.rs:42-44"
   evidence: |
     let query = format!("SELECT * FROM sessions WHERE token = '{}'", token);
   recommendation: Bind the token as a parameter.
   history:
     - status: open
       by: security
   ```

   - `location`: `path:line` or `path:start-end` relative to the project
     root for code; `<file>#<heading>` relative to the change folder for
     the plan (`"design.md#Decision: Sessions live in the database"`).
     Quote it: a heading often holds `: `.
   - `evidence`: re-read the cited lines and copy the text verbatim —
     whole lines or the fragment that matters, never a paraphrase.
     `codev validate` checks it is there.
   - `severity` and `confidence`, on these scales:

     Severity:

     - `high` — The change would break a requirement, lose or expose data, or open a vulnerability, and must not be archived until it is fixed or a human accepts the risk.
     - `medium` — A real defect or risk with a limited impact or a workaround, to fix in this change or to defer deliberately.
     - `low` — An improvement, such as readability, consistency or a minor cost, that does not decide whether the change is correct.

     Confidence:

     - `high` — The evidence shows the problem directly.
     - `low` — The problem depends on something the reviewer could not see or verify.

d. **The author answers each finding still `open`** — the role that owns
   the cited artifact at `plan.challenge` (its `role.id` in
   `codev instructions`), the `developer` at `code.review`. Append to
   `history`, never rewrite it:
   - fix it when the fix is unambiguous and within the change's scope —
     `status: fixed`, `by: <author>`, `ref:` the commit or the file
     change;
   - or dispute it — `status: disputed`, `by: <author>`, `note:` the
     argument, pointing at the material that supports it;
   - or leave it `open` when only a person can decide.

e. **The reviewer re-checks each answer once**, as the finding's role: a
   fix is verified (`status: fixed`, `by: <role-id>`) or reopened
   (`status: open`, `by: <role-id>`, `note:` why it does not hold); a
   dispute is accepted (`status: rejected`, `by: <role-id>`) or maintained
   (`status: disputed`, `by: <role-id>`, `note:` why). No second round:
   what is still `open` or `disputed` goes to the human.

f. **Only a person decides a risk.** Never write `accepted-risk`, nor
   defer a `high` finding, on your own: list them under **To decide** and
   record the decision once the user gives it — `status: accepted-risk`
   with `note:` the justification, or `status: deferred` with `to:` the
   change or the issue it goes to, `by: human:<their name>` (from
   `git config user.name`, else `by: human`). A `medium` or `low` finding
   may be deferred by the author, with `to:`.

g. **Check the record.** Run `codev review --change "<name>" --json` and
   fix every entry of `issues[]` in the record itself — format, location,
   evidence — never by changing what a finding says.

h. **Report from the record.** Present only findings with `reported:
   true` that are not `rejected`. The **To decide** block lists every
   finding whose `awaiting` is `human` and every `high` finding still
   `open`, each with its id, what it is, the author's argument and the
   reviewer's answer, and the choices: fix it, accept the risk (with a
   justification), or defer it (to a change or an issue). If there is
   none, write "Nothing to decide".

### 6. Finish — suggest archiving as an explicit next step

Then run the project's full test suite once. If it fails, report the
failure instead of the summary below: the change is not done.

Otherwise, summarize:

- number of tasks done,
- main files touched (one line),
- the defects the experts fixed in step 5, one line each,
- **Review notes** — from the record's `code.review` findings reported to
  the human and not rejected, at most five items in total across the
  roles, ranked by severity, one line each, tagged with the role id:
  `[<role-id>] <finding id> (<status>) what — where — why`. A
  high-severity `security` or `privacy` finding is always listed, even
  beyond five. If nothing is worth raising, write
  "No review concern found"; never invent notes. When no role is active at
  step 5, write "The review is disabled by configuration" instead,
- **To decide** — from step 5: what only the user can decide, the
  suggested domains still `unticked` included, or "Nothing to decide".
  `codev archive` refuses while a `high` finding is `open` or `disputed`,
- "The change is ready to be archived. Run `/codev-archive` or
  `codev archive` as the next step."

**Do not archive yourself.** The user must review first.

## Output

A final summary, as described in step 6.

## Guardrails

- **Change boundary**: do not modify any file of another active or
  archived change. If a task forces you to touch another change, that is a
  sign the current change is poorly scoped — stop and ask.
- **No automatic sync or archive**: these operations are separate
  decisions, made by the user.
- **No working around errors**: a failing test is not checked off.
  `--no-verify`, `#[ignore]`, a convenient `expect_err`: each is a red flag
  that replaces silence with a lie.
- **Verify before checking off**: checking a box without having run the
  verification the task cites = a regression waiting to happen. Always
  verify.
- **The review record is the truth**: every finding is in `review.md`
  before it is fixed or reported; an accepted risk or the deferral of a
  high finding is recorded only once the user decided it.
- **Targeted editing of tasks.md**: the changed line is identified by its
  full text, never by a bare `- [ ]`. Otherwise, another box would be
  checked by accident.