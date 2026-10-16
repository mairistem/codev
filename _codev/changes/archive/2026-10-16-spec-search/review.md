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

#### R-skeptic-6 The revised skill never says how to read one listed requirement, so the agent still reads whole specs, which is the cost the Why targets

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The instructions bring the related requirements; the skill names no search"
evidence: |
  The `propose` skill's paragraph shrinks to: read the requirements the
  instructions list, modify those the change alters, and treat the list as a
  starting point. It names no search command. This follows what ICM
recommendation: "Nothing tells the agent how to read one listed requirement, so it opens the whole spec as before. Make the skill say to read each with `codev show <capability>::<requirement>` (a whole spec only when the change rewrites much of it), with a scenario and a test, or state the `codev show` form in the human `Existing requirements` line."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md and specs/instructions/spec.md: the human section names codev show <capability>::<requirement>; the skill still names no search command"
  - status: fixed
    by: skeptic
```

#### R-skeptic-7 The instructions delta adds a requirement beside the two existing requirements it changes, instead of modifying them

```yaml
severity: medium
confidence: high
location: "specs/instructions/spec.md#Requirement: `codev instructions` names the existing requirements the change likely touches"
evidence: |
  Without `--json`, the instructions SHALL print, after `Read before
  writing:` and only when not empty, `Existing requirements this change
  likely touches:`, one line per requirement as `<capability>::<requirement>
recommendation: "The main spec lists the fields the document carries and the order of the human sections exhaustively; neither has `related` or the new section. Put MODIFIED blocks for \"`codev instructions` describes one artifact of a change\" and \"The human output of `codev instructions` reads top to bottom\", `related` empty on a failure; keep the new requirement for the ranking rules."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/instructions/spec.md: MODIFIED blocks for the two requirements, related in the field list and the new section in the order; the ADDED requirement keeps the ranking rules"
  - status: fixed
    by: skeptic
```

#### R-skeptic-8 The new ranking step has no failure behavior: an empty proposal or an unreadable spec could make `codev instructions` fail

```yaml
severity: medium
confidence: high
location: "specs/instructions/spec.md#Requirement: `codev instructions` names the existing requirements the change likely touches"
evidence: |
  `requirement` and `excerpt`, the excerpt at most 160 characters of the
  requirement's description. `related` SHALL be an empty array for the
  proposal itself, when `proposal.md` does not exist yet, when the project
  has no main spec, or when nothing matches. The decisions in effect stay in
recommendation: "The ranking fails with `search_query_empty` on a query empty after filtering and with `Unreadable` on an unreadable spec, while `codev instructions` reads no spec and exits 0 today. State that an empty query gives an empty `related`, and that an unreadable spec gives a warning in `status` with the rest of `related` (or a named failure); scenarios and tests in 2.5."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/instructions/spec.md: an empty query gives an empty related; an unreadable spec is left out with a warning in status; scenarios added; tests in task 2.5"
  - status: fixed
    by: skeptic
```

#### R-skeptic-9 `related` is served to the `tasks` instructions, which `/codev-apply` and the review briefs read, steps nobody asked to change and the suite does not measure

```yaml
severity: medium
confidence: high
location: "specs/instructions/spec.md#Requirement: `codev instructions` names the existing requirements the change likely touches"
evidence: |
  Once the change folder holds its `proposal.md`, `codev instructions` SHALL
  give, for every artifact other than the proposal itself, the requirements
recommendation: "`/codev-apply` reads `codev instructions tasks` at every run and the isolated reviews call `codev instructions` for each artifact, only for `context` and `rules`: each call would parse every main spec and carry about 400 tokens nobody reads (performance: about 3 x 400 tokens per review step). Restrict `related` to the artifacts the list serves (`specs` and `design`), empty for the others; otherwise measure the apply step first, as decision 0008 asks."
raised_by: [skeptic, performance]
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/instructions/spec.md and design.md: related only for the artifacts that require the proposal directly (specs, design), empty for the others, which read no main spec; scenario and test in 2.5"
  - status: fixed
    by: skeptic
```

#### R-skeptic-10 The first arm cannot be measured as task 5.1 describes it once tasks 2.5 and 3.2, placed before it, have run

```yaml
severity: medium
confidence: high
location: "tasks.md#5. Measure (decision 0008)"
evidence: |
  - [ ] 5.1 First arm — the skill of 3.1, telling the agent to run `codev search`: with the baseline recorded by `evaluate-planning`, run `codev-evals run --step plan.write --repeat 3 --jobs 4` with this branch's binary and compare with `codev-evals compare evals/baseline <out>`: planning quality within the baseline's range or better, cost under its floor; verified by the comparison pasted in the pull request, its verdict, and the sources table showing where `search`, `file` and `codev search`/`codev show` moved
recommendation: "Tasks run in file order: after 2.5 and 3.2 \"this branch's binary\" is the second arm. Pin 5.1 to the binary and skills before 2.5 and 3.2, or move it ahead of 2.5, and say where its output lives."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md: the first arm is task 2.4.b, before 2.5 and 3.2, with the binary of commit 3c5a36c and its output directory"
  - status: fixed
    by: skeptic
```

#### R-skeptic-11 Shipping the first arm has no way back: the specs, design and code of the plan describe only the second arm

```yaml
severity: medium
confidence: high
location: "tasks.md#5. Measure (decision 0008)"
evidence: |
  - [ ] 5.3 Ship the arm whose verdict passes, the second if both do and it costs less; if neither passes, revise the instructions' list or the skill paragraph (not the cases) and measure again, or stop and report the numbers; verified by a passing verdict or a written decision in the pull request
recommendation: "If the first arm ships, the specs, design and tasks would still require `related` and forbid naming a search command. Add a task for the arm-1 outcome — revert 3.2 and 4.2 (and 2.5 or justify keeping it), restore the first arm's skills delta and update the design before archive — and say which arm ships when both pass and the second costs more."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md: 5.3 says which arm ships when both pass; 5.5 reverts to the first arm's specs, skill and tests if it ships"
  - status: fixed
    by: skeptic
```

#### R-skeptic-12 The skills chapter keeps saying the skill runs `codev search`, and the existing docs test requires it to

```yaml
severity: medium
confidence: high
location: "tasks.md#4. Documentation"
evidence: |
  - [ ] 4.2 Document `related` in `docs/{en,fr}/src/reference/json-output.md` (the instructions document) and in the description of `codev instructions` in `docs/{en,fr}/src/reference/cli.md`, and say in `docs/{en,fr}/src/skills.md` that the planning skill reads the existing requirements its instructions list; verified by the unit test `both_languages_document_the_related_requirements` in `crates/codev-cli/src/docs.rs`
recommendation: "`docs/{en,fr}/src/skills.md` say the skill runs `codev search`, and `both_languages_document_search_and_show` requires skills.md to name it. Make 4.2 replace that paragraph in both editions and narrow the 4.1 test to cli.md and json-output.md."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md 4.2: replaces the skills.md paragraph in both editions and narrows the 4.1 test to cli.md and json-output.md"
  - status: fixed
    by: skeptic
```

#### R-skeptic-13 The migration reason no longer matches the revised change, which now changes an existing public JSON document

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] migration — suggested by the case format of `evaluate-planning`, which this branch builds on; this change only adds commands and JSON documents, and the skills reach users through the usual `codev update`
recommendation: "The change now adds `related` to the existing instructions document of `contract::v1`, a public API read by installed skills. Tick migration, or rewrite the reason to address that change and the mix of old skills with a new binary; correct the design's Migration Plan either way."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: "proposal.md#Domains: the migration reason covers the added related field and old skills with a new binary; design.md Migration Plan corrected"
  - status: fixed
    by: skeptic
```

#### R-skeptic-14 'The specs' language' picks the stemmer from the structural keywords, which the skills always write in English, so a French-prose project is stemmed in English

```yaml
severity: medium
confidence: low
location: "design.md#Decision: The instructions bring the related requirements; the skill names no search"
evidence: |
  proposal is written in the artifact language, the specs' language in
  practice, so the query is in the right language without asking the agent
  to translate anything; it is several sentences long, which ranks better
recommendation: "The stemming language is the keywords' (`Spec.lang`), which the skills write in English whatever `language:` is; a `language: fr` project has French prose stemmed with English Snowball. Stem in the configured prose `language:` when a stemmer exists for it, or add a scenario for French prose with English keywords."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "specs/search/spec.md and design.md: stem in the project's language: when Snowball knows it; scenario A French project revised; task 2.6"
  - status: fixed
    by: skeptic
```

#### R-skeptic-15 The skills scenario for an empty list checks the agent's behavior, which no test can observe, and its requirement says nothing about an empty list

```yaml
severity: low
confidence: high
location: "specs/skills/spec.md#Requirement: The `propose` skill reads the existing requirements its instructions name"
evidence: |
  - **GIVEN** a project whose `_codev/specs/` is empty
  - **WHEN** the `codev-propose` skill writes the delta specs
  - **THEN** it goes on with the request and the code, the list of existing
    requirements being empty
recommendation: "Add a sentence to the requirement (with no listed requirement, the skill works from the request and the code) and rewrite the scenario as GIVEN the rendered skill / THEN it says so, so that the named unit test can verify it."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/skills/spec.md: a sentence on an empty list; the scenario checks the rendered skill"
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

Second pass, on the revised plan: no finding.
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

Second pass, on the revised plan: its finding was merged into R-skeptic-9.

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/workflows.rs, crates/codev-cli/src/commands.rs, contract.rs, main.rs, render.rs — over the material limit; summarized by one sub-agent for these five files rather than one per file, to save tokens.
Summarized: crates/codev-engine/src/instructions.rs, docs/en/src/reference/cli.md, docs/fr/src/reference/cli.md, docs/fr/src/reference/json-output.md — over the material limit; summarized by one sub-agent for these four files.

### security

No finding.

### performance

No finding.

### newcomer

#### R-newcomer-1 `search` mixes five jobs and re-implements `as_str` in two matches

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/search.rs:260-275"
evidence: |
          let status = match &entry.decision.status {
              DecisionStatus::Unknown(raw) => raw.clone(),
              other => other.as_str().to_string(),
          };
recommendation: "`DecisionStatus::as_str` already returns `raw` for `Unknown`, and `Origin::as_str` already returns \"project\": use them. Move the decision documents and hits into their own function, so that `search` reads as validate, load, add decisions, rank, archive."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/search.rs: add_decisions; status and origin through as_str"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `load_requirements(…, true)` boolean flag and an else branch that cannot run

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/search.rs:167-175"
evidence: |
      }) = load_requirements(fs, layout, true)
      else {
          return (Vec::new(), Vec::new());
      };
recommendation: "Replace the bare boolean with a small enum or two entry points; in the lenient path, return the corpus directly so the dead `else` goes away; say why the `filter_map` cannot hit its `None` arm, or keep requirements apart."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/search.rs: the Unreadable enum (Skip, Fail); the skipped path says why it cannot fail; a comment on why hits holds requirements only"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 `search_archive` special-cases one file inside a two-file loop and re-parses `## Why` its own way

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/search.rs:323-333"
evidence: |
          for file in ["proposal.md", "design.md"] {
              if let Ok(content) = fs.read_to_string(&dir.join(&name).join(file)) {
                  if file == "proposal.md" {
recommendation: "Read the two files with two plain statements, and add a pure `proposal_section(proposal, \"Why\")` in codev-core next to `proposal_query`, used by both, so there is one definition of a proposal's section."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/search.rs: two plain reads; crates/codev-core/src/search.rs: proposal_section, used by proposal_query and the archive"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 The `// French` marker sits at the end of an English line in `COMMON_WORDS`

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/search.rs:51"
evidence: |
          "with", "would", "you", "your", // French
recommendation: "Split the list into English and French constants, or keep the comment on its own line."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/search.rs: ENGLISH_COMMON_WORDS and FRENCH_COMMON_WORDS, is_common"
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 `rank` indexes documents as an anonymous tuple, and `f` means two different things

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/search.rs:121-164"
evidence: |
      let indexed: Vec<(BTreeMap<&str, f64>, f64)> = documents
recommendation: "A private `IndexedDocument { frequencies, length }` and names `doc` and `frequency`, so that the IDF table and the BM25 formula read without decoding a tuple."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/search.rs: IndexedDocument { frequencies, length }; doc and frequency"
  - status: fixed
    by: newcomer
```

#### R-newcomer-6 The related-requirements block nests three matches inside the already long `for_artifact`

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/instructions.rs:207-224"
evidence: |
      let related = match proposal {
          Some(id) if artifact.requires.contains(&id) => {
              match fs.read_to_string(&change_dir.join("proposal.md")) {
recommendation: "Extract a function with early returns; rename `unreadable` to `warnings`; take the file name from the `generates` found just above rather than repeating \"proposal.md\"."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/instructions.rs: related_for with early returns; the file name from generates"
  - status: fixed
    by: newcomer
```

#### R-newcomer-7 Some test names hide part of what the test checks

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/search.rs:346-355"
evidence: |
      fn the_same_input_ranks_the_same_way() {
recommendation: "Move the \"a query matching nothing ranks nothing\" assertion into its own test, and split `requirement_document`'s checks out of `a_requirement_is_sliced_verbatim`."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/search.rs: tests a_query_matching_nothing_ranks_nothing and a_requirement_document_is_named_capability_colon_colon_name"
  - status: fixed
    by: newcomer
```

