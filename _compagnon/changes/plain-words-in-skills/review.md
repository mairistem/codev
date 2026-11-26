# Review record: plain-words-in-skills

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The new rule contradicts the exact refusal text of an existing requirement

```yaml
severity: high
confidence: high
location: "specs/skills/spec.md#Requirement: The skills suggest the next step in plain words first"
evidence: |
  way out of a refusal, the fix for a problem it reports — it SHALL first give
recommendation: 'The main requirement "The sync and archive skills rely on the JSON contract" makes the skill say exactly "The change has errors. Run `compagnon validate <name>` to see the details.", which the new rule contradicts. Add a MODIFIED delta of that requirement with the new wording, and a task rewriting the "reply exactly" lines.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: "specs/skills/spec.md MODIFIED Requirement: The `sync` and `archive` skills rely on the JSON contract; tasks.md 2.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 All eight descriptions change with no evaluation, against decision 0008

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The evaluation runs on the six touched steps, with the user's agreement on the cost"
evidence: |
  descriptions, which only steer the choice of a skill, are
  not measured by the suite, which runs each skill by name.
recommendation: 'Decision 0008 has an unevaluated step covered first, by its own change, which is why the explore and onboard bodies wait; yet their descriptions, and the selection all eight steer, change unmeasured. Either leave the descriptions (at least explore and onboard) to a change that first covers skill selection, or propose a decision narrowing 0008 for descriptions.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: "proposal.md What Changes, specs/skills/spec.md (six descriptions), design.md, tasks.md 2.3 — the user chose to change only the six evaluated skills' descriptions"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 The rule keeps `/compagnon-continue`, which names no skill

```yaml
severity: medium
confidence: high
location: "specs/skills/spec.md#Requirement: The skills suggest the next step in plain words first"
evidence: |
  that exists — a skill of the catalog or a `compagnon` command — MUST still be
recommendation: 'update.md and sync.md name `/compagnon-continue`, which no workflow of the catalog defines; keeping every command would offer a shortcut that fails. Limit the rule to commands that exist and drop it in task 2.2.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/skills/spec.md (commands that exist; /compagnon-continue dropped); tasks.md 2.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 The init hint does not cover propose or configure not installed

```yaml
severity: low
confidence: high
location: "specs/init/spec.md#Requirement: `compagnon init` hints at `/compagnon-configure` when the generated config is thin"
evidence: |
  If the config is not thin (the user had already written rules, or a
recommendation: 'render.rs shows the thin hint only when configure is installed and otherwise names the first installed skill; the fixed sentence would name propose when it is not installed. Say what the hint does then, with a scenario and a test.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/init/spec.md (short form without propose; scenario Without propose, the hint names an installed skill); tasks.md 3.1 setup_without_propose_names_an_installed_skill"
  - status: fixed
    by: skeptic
```

### accessibility

No finding.

## code.review

Mode: isolated.

### accessibility

No finding.

### newcomer

#### R-newcomer-1 The fallback names the skill the comment says is missing

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/render.rs:170-177"
evidence: |
  // Without propose, name a skill that is installed.
recommendation: 'The branch ran only without propose, yet its fallback printed `/compagnon-propose` (`first.unwrap_or("compagnon-propose")`), unreachable behind the outer non-empty guard. Take the first skill with `if let Some(first)` and flatten `else { if … }` into `else if`.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/compagnon-cli/src/render.rs (else if let Some(first) = outcome.skills.first())"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `SIX` gives a count, not what the list means

```yaml
severity: low
confidence: high
location: "crates/compagnon-agents/src/workflows.rs:1557"
evidence: |
  const PLAIN_WORDS_SKILLS: [&str; 6] =
recommendation: 'The constant was named `SIX`, which says how many skills it holds, not what they share. Renamed `PLAIN_WORDS_SKILLS`. The test names keep `the_six_skills_…`: tasks.md and the pull request cite them by name.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/compagnon-agents/src/workflows.rs (SIX renamed PLAIN_WORDS_SKILLS)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 A sentence sits in a list of commands

```yaml
severity: low
confidence: high
location: "crates/compagnon-agents/src/workflows.rs:1836-1838"
evidence: |
  for text in commands {
recommendation: 'The loop variable was `command`, while the list also holds the fixed reply "No changes made". Renamed `text`, and the failure message with it.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/compagnon-agents/src/workflows.rs (loop variable renamed text)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 The update guardrail skips the request the rule asks for

```yaml
severity: low
confidence: low
location: "assets/workflows/update.md:153-154"
evidence: |
  suggest finishing the plan: say “finish the plan of <name>”
recommendation: 'The "No creation" guardrail gave the shortcut with no quoted request, unlike step 3 of the same body. It now says “finish the plan of <name>” before the shortcut, on the line that holds the word shortcut and the command, as the check requires.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "assets/workflows/update.md (No creation guardrail)"
  - status: fixed
    by: newcomer
```
