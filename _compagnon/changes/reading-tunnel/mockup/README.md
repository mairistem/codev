# Tunnel mockup — status: principle approved, look rejected (version 1)

Version 1 (2026-11-27): the owner approves the principle — the five steps,
the pages, the two layers, the skip buttons, the next steps — but rejects
the look ("very old school"). Version 2 shows real pages in the approved
modern style, with a five-step progress bar and a next-step card.

Published for review: https://claude.ai/artifact/3izBEyZG1q5R8ForP9FxZS (version 1).

`build.py` writes `index.html`: the tunnel's table of contents and one card
per chapter (purpose; what everyone sees — diagram and example; what is
folded for experts; skip button; next step). Checked with Playwright at
320 px and 1280 px, light and dark: no horizontal scroll, no console error,
every link reaches a card, and focus follows a "Next step" taken with the
keyboard.

## Version 2

`build_v2.py` writes `index.html` (version 1 kept as `index-v1.html`): two
real pages in the approved look — "Start from a Jira ticket" and "Your
first change" — and an overview of the tunnel. New on each page: a
five-segment progress bar with the page's place in its step, a stepper
sidebar (done steps ticked), the skip button as a pill, the example as a
conversation with Claude, "Under the hood" as closed accordions, and a
"Next step" card. Checked with Playwright at 360 px and 1280 px, light
and dark: no horizontal scroll, no script error.
