# Review record: spec-search

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The explore skill is changed without the evaluation decision 0008 requires

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The skills search first, then read what the search names"
evidence: |
  `assets/workflows/propose.md` (step 4c, `plan.investigate`) and
  `assets/workflows/explore.md` (first step) gain a short paragraph: run
recommendation: "Decision 0008 requires a change to `assets/workflows/` to be evaluated on the steps it touches, a step the suite does not cover being covered first by its own change; the suite has no explore step (evals/src/case.rs), and tasks 5.1/5.2 measure only plan.write and review. Remove explore.md from this change (its scenario, the explore part of task 3.1 and the mentions in proposal and design), or cover explore in the suite first."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md (Goals, Non-goals, skills decision), proposal.md, specs/skills/spec.md, tasks.md 3.1: /codev-explore left out until the suite covers it"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 Search returns superseded, rejected and proposed decisions without their status

```yaml
severity: medium
confidence: high
location: "specs/search/spec.md#Requirement: `codev search` ranks the requirements and decisions that match a query"
evidence: |
  `codev search <query>` SHALL rank every requirement of the project's main
  specs — its name, its description and its scenarios — and every decision of
  the decision index, the inherited ones included, by how well they match the
recommendation: "The decision index holds every ADR whatever its status; only `in_effect` lists those in force (crates/codev-engine/src/decisions.rs). A superseded or rejected ADR ranked first would be planned from as if in force. Give each decision result its status and whether it is in effect, in the text and the JSON, with a scenario \"A superseded decision is not presented as in effect\" and its test."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/search/spec.md: decision results carry status, inEffect and supersededBy; scenario 'A superseded decision is not presented as in effect'; tasks.md 2.1 test"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The measurement tasks need a planning baseline that does not exist yet

```yaml
severity: medium
confidence: high
location: "tasks.md#5. Measure (decision 0008)"
evidence: |
  With the baseline recorded by `evaluate-planning`, run `codev-evals run --step plan.write --repeat 3 --jobs 4` with this branch's binary and compare with `codev-evals compare evals/baseline <out>`
recommendation: "The committed baseline has review cases only; evaluate-planning task 6.2 is unchecked. State that evaluate-planning 6.2 must be done before section 5, and add a first step checking the baseline holds plan.write repetitions recorded with the pinned Claude Code version."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md 5.0: prerequisite check of the evaluate-planning baseline; proposal.md Impact states it"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The new exit code 2 for search errors breaks the CLI's single failure exit code, and the design does not explain it

```yaml
severity: low
confidence: high
location: "specs/search/spec.md#Requirement: `codev search` ranks the requirements and decisions that match a query"
evidence: |
  only of common words, SHALL fail with `search_query_empty` and exit with
  code 2. A `--limit` outside 1 to 50 SHALL fail with `search_limit_invalid`
recommendation: "Every codev failure exits with 1 (`fail` in crates/codev-cli/src/main.rs), and this change's own `codev show` errors exit with 1. Use 1, and say `--limit` is validated by codev (so the JSON document carries `search_limit_invalid`), not by clap."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/search/spec.md: exit code 1 for search_query_empty and search_limit_invalid, --limit checked by codev; tasks.md 2.3"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 Decisions are stemmed in the specs' language, which may not be the language they are written in

```yaml
severity: low
confidence: low
location: "design.md#Decision: Snowball stemming with `rust-stemmers`, in the language of the specs"
evidence: |
  crate, in the language the specs' keywords are written in (`Spec.lang`):
recommendation: "ADRs have no structural keywords and inherited ones may be in another language. State that decisions are stemmed in the corpus language and accept the loss, or choose the language per source, and say which in `language`."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md: decisions stemmed in the specs language, the loss accepted and stated in a risk"
  - status: fixed
    by: skeptic
```

### security

#### R-security-1 The `codev show` capability argument is not resolved against the list of known capabilities

```yaml
severity: low
confidence: low
location: "design.md#Decision: `codev show` reads one requirement from the parsed spec"
evidence: |
  `codev show <capability>::<requirement>` slices the requirement from the
  spec's source by the span the parser gives — heading, description and
  scenarios, byte for byte — so that what an agent copies into a `MODIFIED`
recommendation: "Joining the argument to `specs/` (Layout::spec_file) lets `codev show \"../../x\"` or an absolute path read any `spec.md` outside `_codev/specs/`, without a prompt since the skills allow `Bash(codev:*)`. Accept a capability only if it is one of the names `codev_engine::specs::list` returns, `unknown_capability` otherwise; add a test `show_refuses_a_capability_outside_the_specs`."
history:
  - status: open
    by: security
  - status: fixed
    by: qa
    ref: "specs/search/spec.md: the capability must be one of the main specs, paths outside refused; scenario 'A path outside the specs'; tasks.md 2.2 test"
  - status: fixed
    by: security
```

### performance

#### R-performance-1 The "well under 100 ms" claim behind rebuilding the index at each call is a guess, and no task measures it

```yaml
severity: low
confidence: high
location: "design.md#Decision: The index is rebuilt at each call"
evidence: |
  The project's specs are a few hundred requirements: building the index in
  memory costs well under 100 ms in Rust (0.4 s in the Python prototype for
  217 documents), so `codev search` rebuilds it every time and keeps nothing
  on disk — nothing to invalidate when a spec changes, nothing to commit or
  ignore.
recommendation: "The Rust figure is an estimate; `--archive` reads every archived proposal and design (53 changes, about 2.4 MB here), and the skills may search several times per run. Time `codev search` with and without `--archive` on this repository with a release build, paste the numbers in the pull request, state the measured figure in the design, and keep one second as the point to revisit."
history:
  - status: open
    by: performance
  - status: fixed
    by: architect
    ref: "design.md: the figure marked as an estimate, a risk on --archive; tasks.md 6.0 times the release build with and without --archive"
  - status: fixed
    by: performance
```
