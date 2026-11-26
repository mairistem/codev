# Proposal: The evaluation suite covers explore and onboard

## Why

`plain-words-in-skills` converted six skills to suggest their next step in
plain words, and left `explore` and `onboard` as they were: the evaluation
suite does not cover them, and decision 0008 asks that "a step the suite
does not cover is covered first, by its own change". Both are the skills
a newcomer meets first — "show me around Compagnon", "I have a vague
idea" — and both are read-only by contract: `explore` writes no file and
ends in one of three stated outcomes; `onboard` modifies nothing and
recommends one next action from the project's state. Neither has a
measure today, so a change to their text could not be judged.

## What Changes

- Two new steps in `compagnon-evals`, each with its own cases under
  `evals/cases/`, a small project in `base/` and what is expected:
  - `idea.explore` — `/compagnon-explore` on a request; the cases cover
    the three outcomes the skill names (it crystallizes into a change to
    propose; information is missing; it is not worth pursuing, the code
    already does it);
  - `project.onboard` — `/compagnon-onboard` on a project; the cases
    cover the rows of its recommendation table (no `_compagnon/`; thin
    configuration; configured with no active change; an active change
    whose plan is incomplete; an active change ready to implement).
- Each run is scored without a model: no file modified, created or
  removed (both skills are read-only), no change created; the agent's
  messages hold the case's `contains` texts and none of its `absent`
  texts — for `onboard`, the expected next action; for `explore`, the
  outcome and, when it crystallizes, the way to propose it, never an
  implementation.
- The report, `compare` and the reliability chapter give the two steps'
  measures, as for the configure step.
- A baseline of the new cases is recorded with the user's agreement on
  its cost (estimated 15 to 25 USD for five repetitions), and committed.
- Out of scope: converting the two skills' text and descriptions to the
  plain-words rule — a following change, measured against this baseline.

## Capabilities

### New Capabilities

- read-only-skills-evaluation

### Modified Capabilities

### Removed Capabilities

## Impact

- `evals/src/` (case loading, the runner's prompt and observation, the
  scoring, the report and `compare`), new cases under `evals/cases/`,
  `evals/baseline/` (new cases recorded), the reliability chapter of both
  books (generated).
- `CONTRIBUTING.md` and `CONTRIBUTING.fr.md`: the steps the suite covers.
- No change to the skills, the CLI or the core: the skills are measured
  as they are.

## Domains

- [ ] security
- [ ] personal-data — the match is the word "phone" in the home page's example (`docs/site/index.html`), from another change on the branch
- [ ] user-interface — the matches are the documentation and its stylesheet, changed by other changes on the branch; this change adds evaluation cases only
- [ ] performance — the match is « cache » in French text of `evals/src/reliability.rs`, from another change on the branch
- [ ] operations
- [ ] migration

## Release note

None
