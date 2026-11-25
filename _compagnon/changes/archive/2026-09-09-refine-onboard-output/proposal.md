# Proposal: refine the output of `/codev-onboard`

## Why

The smoke test of `/codev-onboard` on this repository (2026-09-09)
surfaced two points of friction in use:

1. The "here you have" block counts **active** changes (0 for this
   repository) but ignores **archived** ones — 11 today. A newcomer to
   a mature project sees "0 active changes" and thinks the project has
   produced nothing; the history stays invisible.
2. The default recommendation with no active change is
   `/codev-propose <an-idea>` — abstract. A newcomer without a precise
   idea needs to get a feel for the project first; the natural first
   reflex is to open `README.md` before inventing a change.

## What Changes

- **The "here you have" block also counts archived changes** — an
  extra line "`X archived change(s)`" when that number is non-zero.
  Silent if zero (no noise on a new project).
- **The default recommendation explicitly cites `README.md`** —
  "get a feel for the project by reading `README.md`, then
  `/codev-propose <an-idea>`". The `/codev-explore <topic>` suggestion
  is still mentioned as an alternative if the user has a question but
  no idea for an action yet.
- **No boundary change** — still strictly read-only; the
  `allowed-tools` stay unchanged (`Bash(codev:*), Read, Glob`).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills` — the requirement `Skill onboard presents codev and
  recommends the next action` is modified to cover these two
  additions. The rest (read-only boundary, `allowed-tools`, presence
  in the default catalog) is unchanged.

### Removed Capabilities

None.

## Impact

- **Code**: edit of the `assets/workflows/onboard.md` body (block 2
  and block 3). The `CATALOG` entry stays identical — same id, same
  description, same `allowed-tools`.
- **Tests**: the invariant `onboard_cite_ses_trois_blocs` keeps
  passing (the keywords `codev is` / `here you have` / `what's next`
  remain present). No test to add — the new details live in the body;
  the spec frames the broad outline.
- **JSON contract**: nothing. The skill parses no JSON and produces no
  structured output.
- **Migration**: none. The behavior changes at the next Claude Code
  session after `codev update`.
- **Out of scope**:
  - **Distinguishing archived changes by date/age** — a simple count
    is enough; a temporal hierarchy could come later if a need
    arises.
  - **Detecting the presence of a `README.md`** — the skill suggests
    it unconditionally; if the repository has none, the user finds
    out by trying. Zero cost, sufficient robustness.
