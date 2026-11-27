# Tunnel mockup — status: version 3 approved by the owner (2026-11-27)

Approved version: version 3 (`build_v3.py`, `index.html`), strict Mairistem
design system, approved by the owner on 2026-11-27 ("ok on y va").

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

### Version 2, revised

At the owner's request: examples look like a real Claude Code session
(prompt, tool calls, results, input box), and the pages share the home
page's identity — same header, the home hero's dark band on each page,
the same colored-rule cards, one Claude Code terminal everywhere. A
"Home page" view shows the five step cards replacing the profiles.

## Version 3 — strict Mairistem design system

At the owner's request, `build_v3.py` writes `index.html` (version 2 kept as
`index-v2.html`) following the Mairistem design system strictly: the four
brand colours plus black and white, no gradient, no blur, IBM Plex Sans
only, the DS header (dark top band, white band, primary "Installer"
button), the product hero on the soft surface with the symbol watermark,
persona cards with a coloured top rule for the five steps, upper-case
eyebrows, accordions with + and −, a dark card for "Next step" with a
yellow arrow link, and Claude Code shown as a capture in a white frame
with a 1 px border. The DS defines no dark theme: the dark mode uses its
dark surfaces (#00394d page, #004c66 cards). Checked with Playwright at
360 px and 1280 px, light and dark: no horizontal scroll, no script error.
