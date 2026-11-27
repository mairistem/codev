# Mockup — status: waiting for approval

Published for review: https://claude.ai/artifact/7n3EqydtQfniLv3BRXtoZ2 (version 1).

Built by `build.py` from `src.html` and the three `*-diagram.html` parts
(run from the repository root). `index.html` is the assembled page; it has
two views (home page, book chapter "Start from a Jira ticket") and a
light/dark preview switch that exists only in the mockup.

Checked with Playwright at 320 px and 1280 px, light and dark, both views:
no horizontal scroll, no console error, focus visible (the four path cards
show focus on the whole card).

Added after the user's notes during task 1.1 (accessible first, easy to go
further, dynamic, many diagrams): the workflow steps open to a short
explanation with a "Go further" link; a third diagram, "Inside a plan";
gentle motion (terminal lines, hover lift) that stays off under
`prefers-reduced-motion`, with every text readable at rest.
