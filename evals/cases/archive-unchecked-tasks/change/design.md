# Design: Fewer turns in the review steps

## Context

A traced review run (the `security-code-sql-injection` case, `stream-json`)
took 8 turns: (1) read the skill and `codev roles --at code.review`;
(2) the diff, the files and `codev review --json`; (3) `codev roles
--change … --json | python3 …` to find the roles set to `never`, denied;
(4) the same with `grep`; (5) the context files; (6) write `review.md`;
(7) `codev review` to check; (8) the summary. Turns 3 and 4 exist only
because the review-record procedure sends the agent to a second call for
the disabled roles. Over the 200 baseline runs, 186 of 266 permission
denials are `codev roles --change … | python3` (77) and `codev review
--json | python3` (109): the agent filters a JSON it was told to read.
The human output of `codev review` already holds what the agent looks for
(each finding's status and who acts next, the record's defects, whether
the change can be archived).

## Goals / Non-Goals

**Goals**: remove the turns spent on the disabled roles and on filtering
codev's JSON, without changing what a review finds.

**Non-goals**: shortening the skills themselves (a separate change),
changing the reviewing roles or the record's format, the model of the
reviewers.

## Decisions

### Decision: `codev roles --at` lists the roles disabled at the step

`RolesV1` gains `disabled` (additive to v1, decision on the JSON contract
kept): with `--at`, each role that serves the step and whose activation is
`never`, with `id`, `name` and `whenOrigin`. The core computes it next to
`active_at` (`disabled_at`), pure, from the resolved roles; the engine and
the CLI pass it through. An `on_domain` role whose domain is not ticked is
inactive, not disabled: the record expects no section for it.

**Alternatives considered**: telling the skill to call `codev roles
--change` without `--at` keeps a second call and a search the agent does
by filtering. Listing every inactive role mixes "turned off by someone"
with "not concerned by this change", which the record treats differently.

### Decision: The skills read `codev review` as printed, and never filter codev's output

The review-record procedure resumes and checks with `codev review
--change <name>` (human output), and states once that codev's output is
read as printed, not piped into another program. The JSON stays available
and documented; the procedure no longer asks for it at these two points.
The isolated review's brief lists the ids already in the record from the
same human output.

**Alternatives considered**: adding a compact JSON view (`--brief`) adds a
contract to maintain for what the human output already says. Allowing
`python3` in the skills' `allowed-tools` would let any program run under
the skill's grant.

### Decision: The evaluation prompt defers to the skill

The runner's prompt for `plan.challenge` and `code.review` says to check
the record "as the skill says" instead of naming `codev review --json`.
Otherwise the suite would measure the prompt's instruction, not the skill.
This is part of the candidate run: the baseline was recorded with the old
prompt and old skills, and the comparison judges both together, which is
what a user's run changes too.

## Risks / Trade-offs

- [The agent reads less of the record and misses a defect `--json` would
  show] → The human output lists the record's defects as `codev validate`
  does, and the quality side of `codev-evals compare` checks recall,
  false positives and evidence validity per role against the baseline's
  range.
- [The gain stays within the noise] → The candidate runs every case three
  times; `compare` passes only under the baseline's floor. If quality
  holds but cost does not beat the floor, the change is reported as such,
  with its numbers, not presented as a gain.
- [An installed skill meets an older binary without `disabled`] → Skills
  and binary are refreshed together by `codev update`; a missing field
  reads as no disabled role, and the record's check only requires
  sections for active roles.

- [A rule that removes denials adds turns] → Measured, then dropped. A
  second candidate run added a rule asking for each `codev` command as a
  call of its own, not chained after `cd … &&`: permission denials fell
  from 26 to 12, but turns rose from 305 to 371 (above the baseline's
  323) with the same cost (12.74 against 12.69 USD) and duration. The
  change aims at turns, so the rule was removed (review finding
  R-performance-2) and the first run, which measured the shipped text,
  is the reference.

## Migration Plan

After the comparison passes, the candidate run is completed to five
repetitions (`run --repeat 5` resumes it) and becomes the new baseline, so
the next change is compared with codev as it now is.
