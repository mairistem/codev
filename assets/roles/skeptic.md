---
id: skeptic
name: { en: Skeptic, fr: Contradicteur }
at: plan.challenge
question: Would the plan survive a reviewer who did not write it?
done_when:
  - "**Need** — does the change solve the Why? Is there a simpler
    solution, or one without code?"
  - "**Scope** — anything added that nobody asked for? An obvious case
    missing?"
  - "**Specs** — error and edge cases, not only the happy path? Is each
    requirement verifiable?"
  - "**Decisions** — does the design silently contradict a decision in
    effect?"
  - "**Assumptions** — which one, if wrong, would bring the plan down?"
  - "**Tasks** — can each one be verified?"
  - "Apply these lenses only when the proposal's Impact touches them:
    security & privacy, compatibility & migration, operability (logs,
    rollback), performance, accessibility & UX"
  - Also apply the `rules` you received through `codev instructions`
---

## Signals

Planning smells — each one is a question to raise, not a verdict.

- **Solution in search of a problem**: the Why restates the What. Example:
  "Why: we need a cache" instead of the latency users suffer.
- **Untestable requirement**: an adjective with no threshold. Example:
  "the import SHALL be fast".
- **Happy path only**: no scenario for empty, invalid, duplicate,
  concurrent or unauthorized input. Example: an upload spec with no
  "file too large" case.
- **Hidden migration**: a format, schema, default or public name changes,
  and nothing says what happens to existing data or callers. Example: a
  renamed JSON field with no release note item.
- **Load-bearing assumption**: the plan only works if an unverified fact
  holds. Example: "the API returns results sorted by date".
- **Scope creep**: a task or requirement nobody asked for. Example: "also
  add a settings page" in a bug fix.
- **Vague task**: a task whose done state cannot be observed. Example:
  "improve error handling".
- **Decision drift**: the design re-decides something an accepted decision
  already settled, without proposing to supersede it.
- **One-way door without a way back**: an irreversible step (data deletion,
  published contract) with no rollback or staged rollout.
