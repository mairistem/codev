**The review record.** Every finding of this step goes to
`_codev/changes/<name>/review.md`, the change's review record, before it
is fixed or reported.

Read codev's output as it prints it: each command gives what the step
needs, so never filter it through another program (`python3`, `jq`) — a
command your session may not approve, which costs a turn.

a. **Resume first.** Run `codev review --change "<name>"`. A finding it
   lists under a step is never raised again, and a finding it lists under
   **Waiting for an answer** with `author` or `reviewer` is answered
   before anything new. The record is the state: a session that stopped
   half-way picks up from it.

{{ISOLATED_REVIEW}}

b. **Write a section per role.** Under `## <step>` (create the file with
   `# Review record: <name>` and `<!-- codev-review: 1 -->` if it does not
   exist) — first the line `Mode: inline.` or `Mode: isolated.` and any
   `Summarized:` lines — one `### <role-id>` per role returned, holding
   its `Fallback:` line if it fell back, then its findings, or
   the line `No finding.` when it found nothing. For each role listed under
   `disabled` by the same `codev roles --at <step> --change "<name>"
   --json` call that returned the active roles, write `Disabled by
   configuration.` instead. Never write
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

{{REVIEW_SCALE}}

   - **Grade with evidence.** A `medium` or `high` finding with
     confidence `high` quotes the line it is about and states the
     scenario that fails: the input, state or operation, and what goes
     wrong. A concern that depends on something you could not see or
     verify — load, data volume, a caller, a configuration, a later
     change — and a preference with no failure are recorded with
     confidence `low`: kept in the record, not presented.
   - **Not a finding**: a concern another role owns, even when that role
     is not active; code or data the change neither adds nor modifies;
     what the plan explicitly excludes, unless excluding it breaks one of
     the change's own requirements; a missing test, unless your role's
     criteria ask for one; a matter of taste, or an alternative design
     when the current one works.
   - **One finding per defect.** The consequences of one flaw go in its
     recommendation, not in findings of their own; two defects that need
     two different fixes are two findings, even at the same place.

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

g. **Check the record.** Run `codev review --change "<name>"` and fix
   every line of its **Issues in the record** in the record itself —
   format, location, evidence — never by changing what a finding says.

h. **Report from the record.** Present only the findings `codev review`
   does not mark `(low confidence)` and that are not `rejected`. The **To
   decide** block lists every finding it shows under **Waiting for an
   answer** with `human` and every `high` finding still `open`, each with
   its id, what it is, the author's argument and the
   reviewer's answer, and the choices: fix it, accept the risk (with a
   justification), or defer it (to a change or an issue). If there is
   none, write "Nothing to decide".
