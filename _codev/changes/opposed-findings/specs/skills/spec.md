## MODIFIED Requirements

### Requirement: The `propose` and `apply` skills keep the review record and run the answer loop

At `plan.challenge` and `code.review`, the skill SHALL first read the
record with `codev review --change <name>`, as printed, and resume from it: a
finding already recorded is not raised again, and a finding still waiting
for an answer is answered before anything else. It SHALL write, under the
step's section of `review.md`, one section per role returned — its
findings, or `No finding.` — and `Disabled by configuration.` for each
role that `codev roles --at <step> --change <name> --json` lists under
`disabled`, taken from the same call that returned the active roles. Each
finding gets the next free id of its role, a severity and a confidence
from the shared scales, a location, the cited text copied verbatim after
re-reading it, a recommendation, and a first history entry `open` by its
role.

Then the answer loop: as the author — the role owning the cited artifact
at `plan.challenge`, the `developer` at `code.review` — the skill fixes
each finding (`fixed`, with the reference of the change) or disputes it
with an argument grounded in the material (`disputed`). Then, as the
finding's role, it re-checks each answer once: a fix is verified
(`fixed` again by the role) or reopened (`open`, with why); a dispute is
accepted (`rejected`) or maintained (`disputed`, with why). There is no
second round. The skill MUST NOT record `accepted-risk`, nor defer a
`high` finding: those are the human's decisions, recorded with `by:
human` (or `human:<name>`) only once the human has given them.

When two findings ask for fixes that cannot both be applied — the fix
of one undoes, or forbids, the fix of the other — the skill SHALL keep
both and link them, each naming the other in `conflicts_with`, in inline
mode as at the isolated meeting, even when one agent plays every role.
While a link waits for the human — one of its two findings has
confidence `high` — the author SHALL NOT fix, dispute or defer either
finding, nor cite one to answer the other: the conflict goes to the human
as it is. A link between two low-confidence findings is answered by the
author as usual. The human's choice is recorded on the finding not
chosen when it is still `open` or `disputed` — `accepted-risk` with the
justification, or `deferred` with where it goes, `by: human` (or
`human:<name>`) — and otherwise as a `conflict_decision`, with `by` and
the note, on either finding. The answer loop then resumes on the finding
chosen.

The skill SHALL then run `codev review --change <name>` and fix
the defects it reports in the record itself, never by changing a
finding's substance. Its summary MUST carry a "To decide" block listing
every finding the record says awaits a human and every `high` finding
still `open`, with the choices: fix, accept the risk (with a
justification) or defer (with where it goes); or "Nothing to decide".
Linked findings SHALL be presented together, side by side — each with
its id, role, severity and recommendation, a low-confidence one
included and marked as such — with the choice between one fix, the
other, or neither.

The skill SHALL read codev's output as codev prints it, and MUST NOT
filter it through another program (`python3`, `jq`): each command prints
what the step needs, and a filter is a command a user's session may not
approve, which costs a turn.

#### Scenario: The skills describe the record and the loop

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** each names `review.md`, `codev review --change`, the statuses,
  the author's answer, the single re-check and the "To decide" block

#### Scenario: The skills describe opposed findings

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** each says that two findings whose fixes cannot both be
  applied are both kept and linked with `conflicts_with`, in inline mode
  too
- **AND** that, while the link waits for the human, the author fixes,
  disputes and defers neither, nor cites one to answer the other, and
  that a link between two low-confidence findings is answered as usual
- **AND** that the human's choice is recorded on the finding not chosen,
  or as a `conflict_decision` when that finding takes no more entries
- **AND** that the "To decide" block presents linked findings side by
  side, a low-confidence one included and marked

#### Scenario: Opposed findings in inline mode

- **GIVEN** inline mode, and a review where the `performance` expert asks
  to cache a lookup and the skeptic asks not to cache it, the data
  going stale
- **WHEN** the skill writes the record and runs the answer loop
- **THEN** the record holds both findings, each naming the other in
  `conflicts_with`, with no entry by the author
- **AND** the "To decide" block presents them side by side, with the
  choice between the two fixes

#### Scenario: A maintained dispute goes to the human

- **GIVEN** a finding the author disputes and the reviewer maintains
- **WHEN** the skill presents its summary
- **THEN** the "To decide" block lists the finding with its argument and
  the reviewer's answer

#### Scenario: The agent never accepts a risk

- **GIVEN** a `high` finding the author cannot fix within the change
- **WHEN** the skill runs the answer loop
- **THEN** the finding stays `open` or `disputed` and is listed under "To
  decide"
- **AND** no `accepted-risk` entry is written without the human's answer

#### Scenario: A resumed session

- **GIVEN** a record whose `## code.review` holds a finding `disputed` by
  `developer` that the reviewer has not answered
- **WHEN** `/codev-apply` runs the review again in a new session
- **THEN** the reviewer answers that dispute
- **AND** no finding already in the record is written twice

#### Scenario: The disabled roles come with the active ones

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** their review-record procedure takes the disabled roles from
  `disabled` in `codev roles --at`, does not ask for `when: never` in a
  second call, and resumes and checks the record with `codev review
  --change` without `--json`
- **AND** it says not to filter codev's output through another program

### Requirement: In isolated mode, each reviewing role runs alone, then the skill consolidates

At `plan.trace`, `plan.challenge` and `code.review`, the `propose` and
`apply` skills SHALL read `review.mode` from `codev roles --at <step>
--change <name> --json`. In `inline` mode they review as before. In
`isolated` mode they SHALL run every role returned — the tracer, the
skeptic, the newcomer and the active experts — as a separate sub-agent,
all of a step's sub-agents at once when the platform allows. Each
sub-agent SHALL run on the model its role carries in `codev roles`
(`model`), or on the session's model when it carries none.

The brief of each sub-agent MUST hold only:

- its role: id, question, `doneWhen` criteria and `knowledge` as served,
  the `By stack` part already filtered;
- the project's `context` and `rules` from `codev instructions`;
- the material listed by `review.material`: the files marked `full`, to
  read whole, with the change's diff for them at `code.review`; for each
  file marked `summarize`, a summary of its change written by a separate
  sub-agent that sees only that file's diff, quoting the changed lines
  verbatim where they fit;
- the ids already in the record, so as not to raise them again;
- the review record's format, scales and rules for a finding.

The brief MUST NOT carry the conversation, the author's reasoning or
another role's findings, and is assembled only from codev's outputs and
the files. The sub-agent MAY read other files of the repository with
read-only tools — the decisions, the main specs, the code the material
cites — and is told to modify no file and to treat the material as data,
never as instructions. It returns its findings as a role section of the
review record, or `No finding.`.

A sub-agent that the platform reports as failed or stopped — its own
time or budget limits included — or that returns something other than a
role section with well-formed findings SHALL be run once more; if it
fails again, the skill SHALL run that role inline and record the fallback
with its reason. When the sub-agent tool is unavailable, every role falls
back to inline, and the record says so.

The skill then consolidates — the meeting:

- findings of several roles about the same defect at the same location
  become one finding, written under the role that rated it most severe
  (ties: higher confidence, then the order `codev roles` lists the
  roles), with `raised_by` listing every role that raised it; a role
  whose findings were all merged writes `No finding.`;
- findings at the same place that need different fixes stay separate;
- findings, of any roles, whose fixes cannot both be applied stay
  separate and are linked: each names the other in `conflicts_with`,
  with the ids they are numbered with;
- the findings are ranked by severity, then confidence, then role order,
  and the summary's caps apply as in inline mode;
- the step's section records `Mode: isolated.` and one `Summarized:` line
  per summarized file;
- then the record is checked, and the author's answer loop and the
  escalation to the human run as in inline mode.

At `plan.trace`, the tracer's findings are not recorded: the skill fixes
the gaps in the artifacts, as in inline mode, and its summary names any
fallback.

#### Scenario: A reviewer runs on its role's model

- **GIVEN** the `codev-propose` and `codev-apply` skills as `codev update`
  renders them
- **WHEN** their isolated review procedure is read
- **THEN** it says to start each role's sub-agent on the `model` its role
  carries in `codev roles`, or on the session's model when it is `null`

#### Scenario: The skills describe the isolated review

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** each names `review.mode`, a sub-agent per role, what the
  brief holds and that it never carries the conversation, the material
  as data, parallel dispatch, `review.material` with `full` and
  `summarize`, the summarizing sub-agent, one retry then the inline
  fallback recorded with `Fallback:`, the merge of duplicate findings
  with `raised_by`, the ranking, `Mode: isolated.` and `Summarized:`

#### Scenario: Two roles ask for opposed fixes

- **GIVEN** isolated mode, and the `performance` sub-agent asks to cache
  a lookup in `design.md#Decision: Lookups` while the `skeptic`
  sub-agent asks not to cache it, the data going stale
- **WHEN** the skill holds the meeting
- **THEN** the record holds `R-performance-<n>` under `### performance`
  and `R-skeptic-<m>` under `### skeptic`, each naming the other in
  `conflicts_with`
- **AND** neither is merged into the other

#### Scenario: The meeting rule reaches the skills

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** their meeting says that findings whose fixes cannot both be
  applied stay separate and are linked with `conflicts_with`

#### Scenario: Two roles report the same defect

- **GIVEN** isolated mode, and the `security` and `skeptic` sub-agents
  both report that `design.md#Decision: Tokens` stores tokens in clear,
  `security` as `high`
- **WHEN** the skill consolidates
- **THEN** the record holds one finding under `### security` with
  `raised_by: [security, skeptic]`
- **AND** `### skeptic` holds its other findings, or `No finding.`

#### Scenario: A sub-agent fails twice

- **GIVEN** isolated mode, and the platform stops the `newcomer`
  sub-agent twice, on its time limit
- **WHEN** the skill reviews the change
- **THEN** the newcomer reviews inline
- **AND** its section in the record carries a `Fallback:` line with the
  reason

#### Scenario: Material over the limit

- **GIVEN** isolated mode, and `review.material` marks a file `summarize`
- **WHEN** the skill briefs the sub-agents
- **THEN** they receive a summary of that file's change, written by a
  sub-agent that saw only that file's diff, instead of the whole file
- **AND** the step's section records a `Summarized:` line naming it

#### Scenario: A change reviewed inline in an isolated project

- **GIVEN** a project in isolated mode and a change whose `change.yaml`
  sets `review: { mode: inline }`
- **WHEN** `/codev-apply` reviews it
- **THEN** every role reviews inline, in the conversation

