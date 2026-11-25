# Design: refine the output of `/codev-onboard`

## Context

See `proposal.md`. A trivial wording change to the skill's markdown
body; no code structure is touched.

## Decisions

### Decision: the "archived changes" line is **conditional**

On a freshly initialized project, `_codev/changes/archive/` may be
empty (or contain only `.gitkeep`). Displaying "0 archived changes"
would be noise — the line is displayed **only if the count is
strictly positive**. Consistent with the skill's principle of compact
output.

### Decision: the reading is done via a filtered `ls`, not a codev CLI

`codev list --archived` does not exist (E6 in the roadmap, not
delivered). The skill therefore counts the folders directly via a
shell command equivalent to `ls _codev/changes/archive/ | grep -v '^\.'`.
It is robust: it filters out hidden files (`.gitkeep`), and the other
folders have the form `<date>-<name>/`. Deferrable until E6 ships —
the substitution will then be trivial.

### Decision: the recommendation cites `README.md` unconditionally

The cost of a "read `README.md`" suggestion on a project without a
README is negligible — the user tries, finds nothing, moves on.
Detecting the file's presence before suggesting it would add an extra
`Glob` for zero value. **Rejected alternative**: conditional
detection. Rejected for simplicity.

## Migration Plan

None. The new behavior applies from the next `codev update` + restart
of the Claude Code session.
