# Tunnel mockup — status: waiting for approval

Published for review: https://claude.ai/artifact/3izBEyZG1q5R8ForP9FxZS (version 1).

`build.py` writes `index.html`: the tunnel's table of contents and one card
per chapter (purpose; what everyone sees — diagram and example; what is
folded for experts; skip button; next step). Checked with Playwright at
320 px and 1280 px, light and dark: no horizontal scroll, no console error,
every link reaches a card, and focus follows a "Next step" taken with the
keyboard.
