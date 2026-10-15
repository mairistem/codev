# Design: A merge that keeps a modified last requirement whole, and an update that refreshes its own skills

## Context

The merge is pure: `merge_into_existing` builds a list of byte-range
edits over the main spec, and `apply_edits` applies them sorted by
descending end, so that earlier offsets stay valid. Its comment assumes
that two edits never end at the same point. They do when a delta modifies
the section's last requirement — whose span runs to the end of the
section — and adds requirements, inserted at that same end: the stable
sort keeps the delta's order, the replacement goes first and lengthens the
text, and the insertion then lands at the old offset, inside the new text.
That is what cut `role-evaluation`'s last scenario at the archive of
`measure-consumption`.

`codev update` decides a skill was edited by hand when its content differs
from the rendering while carrying the running version's stamp. Between two
builds of one version — every build from source, codev's own repository —
the skill text changes and the stamp does not, so codev takes its own
earlier rendering for a hand edit.

## Goals / Non-Goals

**Goals**: compose a modification and an addition at the end of a section;
tell codev's renderings from hand edits whatever the version.

**Non-goals**: detecting a corrupted spec after the fact in `codev
validate` (the merge no longer produces one); repairing the
`role-evaluation` spec, which the change `evaluate-planning` restores
through its own delta.

## Decisions

### Decision: Ties between edits ending at one point put the insertion first

`apply_edits` sorts by descending end, then, for the same end, by
descending start: a zero-width insertion at `end` (start = end) is applied
before a replacement `start..end`, so the replacement no longer moves it.
Applied first, the insertion sits after `end`; the replacement of
`start..end` then rewrites only what lies before it. The comment that
called the case impossible is corrected.

**Alternatives considered**: inserting the added requirements after the
merge, on the merged text — a second pass that recomputes offsets the
plan already knows; forbidding a delta that modifies the last requirement
and adds one — a valid delta, refused.

### Decision: A skill carries the checksum of its own rendering

The rendering writes `checksum: "<sha256>"` under `metadata`, computed
over the whole skill without that line, with `codev_core::update::sha256_hex`
(the hashing codev already uses). The checksum decides only among skills stamped
with the running version: a skill stamped with another version is
rewritten, as before, so a hand edit never survives an upgrade and
`codev doctor` never meets a preserved skill it calls outdated. Among the
running version's skills, one whose content without the checksum line
hashes to the checksum is codev's and is rewritten; one that does not has
been edited and is preserved. A skill without the line was rendered
before checksums: it keeps today's rule — preserved unless it differs from
the rendering only by the review mode, compared with the checksum line
left out — so that no hand edit made before this change is overwritten
within its version.

**Alternatives considered**: a manifest of rendered hashes under
`_codev/` — a second file to keep in step and to commit; bumping the
version on every change of a skill — every build from source would still
meet the case.

## Risks / Trade-offs

- [A user edits a skill and also rewrites its checksum line] → They meant
  to make it codev's; `codev update` then rewrites it, as for any
  rendering.
- [The checksum line changes every skill once] → The first `codev update`
  after this release rewrites skills stamped with an earlier version, as
  it already does for any release. A project on a build from source with
  unchecksummed skills of the same version runs `codev update --force`
  once, as codev's own repository does in task 2.2.
