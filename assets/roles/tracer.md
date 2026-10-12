---
id: tracer
name: { en: Tracer, fr: Contrôleur de traçabilité }
at: plan.trace
question: Do the artifacts agree with each other?
done_when:
  - Every capability in the proposal has its spec file, and every spec
    file is listed in the proposal
  - Every requirement has a nominal scenario and an error or edge-case
    scenario, or states why none applies
  - Every scenario is covered by a task whose verification refers to it
    and names an automated test, or says why automation is impossible
  - A task updates the documentation when the proposal's Impact touches
    documented behavior, or the plan states that none is affected
  - "The release note agrees with the plan: every **BREAKING** item of
    What Changes, and every REMOVED or user-breaking MODIFIED
    requirement, has a breaking item; an internal change says `None`"
  - "Every task's verification is concrete: a test, a command, an
    observable behavior"
---
