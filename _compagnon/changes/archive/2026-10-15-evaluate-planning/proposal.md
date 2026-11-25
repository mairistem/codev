# Proposal: Evaluate the planning step, so that every change to the skills is measured

## Why

The next ways to make codev's skills consume fewer tokens — loading a
skill's steps on demand, a `codev search` over the specs, a cheaper model
for the reviewers — change how `/codev-propose` plans, and the evaluation
suite only measures the review steps. A study (44 headless runs replaying
this repository's history) found that `/codev-propose` misses about one
requirement in three that it should modify, and spends about 40 % of its
input tokens finding its way before writing anything: there is something
to measure, on both quality and consumption. Without a planning step in
the suite, none of those changes can be shipped with the proof the project
now asks for — fewer tokens, the same quality of answer.

## What Changes

- A new kind of case for the step `plan.write`: a user request, in any
  language, replayed against this repository as it was before a change
  that is now archived. The fixture is that past state, rebuilt from the
  pinned commit — nothing is copied into the repository. What the case
  expects is what the real change did: the requirements it modified
  (primary ones, and collateral ones counted apart) and the accepted
  decisions it had to follow or supersede.
- Each planning run is scored without a model: the share of the expected
  requirements modified by the plan's deltas, read with codev's own
  parser; the requirements added where one should have been modified;
  the expected decisions the plan ignores; whether the plan validates;
  and what the run consumed, now with the tool calls it made and how many
  came before it wrote its first artifact.
- The suite starts with the 17 historical requests of the study that
  expect a requirement, its 5 cases on decisions and 3 more on
  requirements; the committed baseline gains their runs, five per
  case, so that `codev-evals compare` judges a change to `/codev-propose`
  the way it judges a change to a role.
- The runner reads Claude Code's event stream instead of its final
  document, for every case, to count tool calls.
- The contributing guides extend the rule that a role change shows its
  scores: a pull request meant to reduce consumption pastes the output of
  `codev-evals compare` — quality within the baseline's range, consumption
  under its floor —, and any other pull request that changes what a skill
  does (the workflows under `assets/workflows/`, or the JSON a skill
  reads) pastes `codev-evals compare --quality-only`, which still shows
  its consumption.
- Every call to the model is accounted for: per call, the tokens written
  to and read from the cache and produced, and where the tokens entering
  the context come from — the skill read, each `codev` subcommand, the
  files, the searches, the model's own output. The report ranks these
  sources by cost and `compare` shows how each one changes, with the
  input and output per call; `compare` also names the measures where the
  quality improved.
- A new decision, accepted: every change to the workflow is evaluated,
  per call, for quality and tokens, before it is merged.
- `codev-evals run --step` selects the cases of a step; resuming an
  output directory refuses another Claude Code version.

## Capabilities

### New Capabilities

- planning-evaluation

### Modified Capabilities

- role-evaluation

### Removed Capabilities

## Impact

- `evals/`: a planning case format and its loading (`case.rs`), the
  runner (fixture from `git archive`, the prompt, the event stream, the
  plan's files and its validation), the scoring of a plan (new module),
  the report and `compare` by step as well as by role; `evals` gains a
  dependency on `codev-core`, for its parser.
- `evals/cases/`: 25 planning cases (a `case.yaml` each, no fixture
  directory); `evals/baseline/`: recorded again in full, review and
  planning cases, every call accounted for.
- `.github/workflows/evals.yml` and `ci.yml`: a full-history checkout,
  so that the pinned commits exist when cases load; the evaluation
  workflow gains a `step` input, review cases by default, and bounds a
  planning run to one repetition.
- `CONTRIBUTING.md`, `CONTRIBUTING.fr.md`, and the generated
  `docs/{en,fr}/src/reliability.md`, regenerated from the new baseline:
  it gains the planning figures and where codev's tokens go, by source
  and per call, in both languages.
- `_codev/specs/role-evaluation/spec.md`: the scenario "A report of
  another run", cut by an earlier merge, is restored.
- `_codev/decisions/0008-*`: the decision above, promoted from the
  design and accepted.
- No change to codev's own behavior: no crate of `crates/` changes, no
  skill, no JSON contract.

## Domains

- [ ] security — the runner already confines the agent in Claude Code's sandbox; the fixture is this repository's own history, with no secret
- [x] personal-data — the raw event streams (`claude.jsonl`) hold transcripts and the operator's paths: stripped of their `system/init` event, kept local and git-ignored, 14 days as a CI artifact; the committed observations hold no path of the machine
- [x] user-interface
- [ ] performance — the change measures what the skills consume; it does not change it
- [x] operations
- [ ] migration — the observation keeps its format, gaining optional fields; earlier output directories stay readable

## Release note

None
