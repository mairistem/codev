## ADDED Requirements

### Requirement: The `propose` and `apply` skills act on suggested domains

Before writing the proposal's `## Domains` section, the `propose` skill
SHALL run `codev roles --change <name> --json` and, for each entry of
`suggestions`, either tick the domain, or keep it unticked with a reason
on its line (`- [ ] <domain> — <reason>`) when the match does not mean
the change touches the domain; it MUST NOT leave a suggestion `unticked`
silently. At `plan.challenge`, the skeptic SHALL record a finding against
`proposal.md#Domains` for every suggestion still `unticked`, and check
that each justification answers its matches. When `diff.available` is
`false`, the skills SHALL say there is no diff and tick the domains by
judgement alone.

At `code.review`, the `apply` skill SHALL read `suggestions` from the
roles it requests and list every domain still `unticked` under "To
decide", with its first match and the two choices — tick it, which
brings its expert to review, or justify it — without editing the
proposal itself. A domain ticked at the human's request activates its
expert: the skill asks for the roles of the step again and runs that
expert's review.

The skills MUST NOT filter the `By stack` knowledge themselves: it is
served already filtered for the project's languages.

#### Scenario: The skills name the suggestions

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** the `propose` body names `suggestions`, `unticked`,
  `justified` and the justified form `- [ ] <domain> — <reason>`, and
  the skeptic's check of unticked suggestions
- **AND** the `apply` body lists an `unticked` suggestion under "To
  decide"

#### Scenario: No diff

- **GIVEN** a project outside git
- **WHEN** `/codev-propose` writes the proposal
- **THEN** it says no diff is available and ticks the domains by
  judgement

#### Scenario: The knowledge is not filtered twice

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** neither body asks the agent to keep only the project's
  languages of `By stack`
