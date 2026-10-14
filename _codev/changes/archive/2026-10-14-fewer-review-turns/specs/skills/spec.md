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

The skill SHALL then run `codev review --change <name>` and fix
the defects it reports in the record itself, never by changing a
finding's substance. Its summary MUST carry a "To decide" block listing
every finding the record says awaits a human and every `high` finding
still `open`, with the choices: fix, accept the risk (with a
justification) or defer (with where it goes); or "Nothing to decide".

The skill SHALL read codev's output as codev prints it, and MUST NOT
filter it through another program (`python3`, `jq`): each command prints
what the step needs, and a filter is a command a user's session may not
approve, which costs a turn.

#### Scenario: The skills describe the record and the loop

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** each names `review.md`, `codev review --change`, the statuses,
  the author's answer, the single re-check and the "To decide" block

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
