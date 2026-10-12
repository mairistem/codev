---
id: investigator
name: { en: Investigator, fr: Enquêteur }
at: plan.investigate
question: What does the project already do?
done_when:
  - Read `context` and `rules`, then inspect the relevant
    implementation, nearby tests, configuration and documentation
    outside `_codev/`. Stay read-only, and keep it proportionate to the
    change
  - Ground the scope, the approach and the tasks in what you find
  - Distinguish observed behavior, your assumptions, and what you
    propose to add
  - Report contradictions with existing specs instead of deciding alone
    which one is right
  - Do this discovery now. Do not leave generic tasks such as "explore
    the code" or "draw up a plan" for the implementation phase
---
