# Design: Newcomer readability review in /codev-apply

## Context

`assets/workflows/apply.md` is a prompt rendered into
`.claude/skills/codev-apply/SKILL.md` by `codev update`. Its steps are: 1
resolve the change, 2 read the tasks, 3 work through each task (3c already
fetches `context` and `rules` with `codev instructions tasks`), 4 stop on
ambiguity, 5 finish — run the full suite once, then summarize. The
architect's role block opens the `design` instruction in
`assets/schemas/spec-driven/schema.yaml`. Every line of either text costs
tokens on every run.

## Goals / Non-Goals

**Goals:**

- One pass, once per change, that leaves the change's code readable by
  someone who has not seen its history.
- Simplicity as an explicit criterion of the design.

**Non-Goals:**

- A machine check (complexity metrics, lint) in `codev validate`.
- Built-in size thresholds: a project states them in its `rules`.
- Refactoring code the change did not write.

## Decisions

No decision in `_codev/decisions/` constrains this change: it edits two
embedded texts and adds no code path, crate or dependency.

### Decision: A numbered step between the tasks and the finish

The review becomes step 5, "Review as a newcomer"; "Finish" becomes step 6.
It runs once, when every box is checked, so the full suite of step 6 also
covers the simplifications. A stop at a blocker (step 4) never reaches it.

**Alternatives considered**: a review per task, in step 3 (misses what only
appears across tasks, and multiplies the cost); folding it into "Finish"
(mixes reviewing and reporting, and the review would follow the full suite
instead of preceding it).

### Decision: The scope is what the change added or modified

The review reads the lines the change added or modified, not whole files.
A concern in surrounding code becomes, at most, a note.

**Alternatives considered**: whole touched files (widens the change and its
review burden, against the `apply` boundary).

### Decision: Notes inform, they do not gate

The "Readability notes" block goes into the final summary, which still
offers archiving. The user decides at review time, like the Points to
challenge of `/codev-propose`, whose cap-and-never-invent form it reuses
(three items rather than five: the review covers one diff, not a plan).

**Alternatives considered**: stopping on a note (turns taste into a
blocker nobody asked for).

### Decision: Only the architect gains a criterion

"The simplest approach that satisfies the specs, followable by a newcomer"
belongs to whoever chooses the how. The tech lead's criteria are about
sequencing and verification; a simplicity criterion there would repeat the
architect's.

**Alternatives considered**: a tech lead criterion as well (duplication).

## Risks / Trade-offs

- [A simplification breaks working code] → the affected tests run again at
  once, and the full suite runs right after the step; behavior changes are
  forbidden outright.
- [Notes become noise or are invented] → capped at three, with "No
  readability concern found" as the honest empty answer.
- [Readability is a matter of taste] → anything ambiguous is a note, not an
  edit; the project's `rules` take precedence over the generic grid.
- [The skill text grows] → one step and one summary line, no new guardrail
  repeating what the step says.
