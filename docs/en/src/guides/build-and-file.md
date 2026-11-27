# Building and filing away

Once you approve a plan, Claude builds it task by task, checks each one,
reviews what it wrote, and files the plan away so that the project's
specs say what is now true.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 140" role="img" aria-label="Building and filing away in four steps: 1, Claude builds task by task, each one tested; 2, Claude reviews its code with fresh eyes; 3, you check the summary and the code; 4, the plan is filed away and joins the project's specs.">
<g stroke-width="2" style="fill:var(--dg-box)">
<rect x="4" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-claude)"/>
<rect x="198" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-claude)"/>
<rect x="392" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-you)"/>
<rect x="586" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-line)" stroke-dasharray="5 4"/>
</g>
<path d="M175,75h18m-6,-5l6,5l-6,5M369,75h18m-6,-5l6,5l-6,5M563,75h18m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="12" style="fill:var(--dg-muted)">
<text x="20" y="44">1 · CLAUDE</text><text x="214" y="44">2 · CLAUDE</text><text x="408" y="44">3 · YOU</text><text x="602" y="44">4 · KEPT</text>
</g>
<g font-size="16" font-weight="600" style="fill:var(--dg-ink)">
<text x="20" y="78">Builds</text><text x="214" y="78">Reviews</text><text x="408" y="78">Check</text><text x="602" y="78">Filed away</text>
</g>
<g font-size="13" style="fill:var(--dg-muted)">
<text x="20" y="102">task by task,</text><text x="20" y="120">each one tested</text><text x="214" y="102">re-reads its code</text><text x="214" y="120">with fresh eyes</text><text x="408" y="102">the summary</text><text x="408" y="120">and the code</text><text x="602" y="102">the plan joins</text><text x="602" y="120">the specs</text>
</g>
</svg>
</div>
<figcaption>Claude builds and reviews; you check; filing away keeps what is now true.</figcaption>
</figure>

## Build

When the plan suits you, say so:

> The plan is good, implement it.

Claude takes the tasks one by one, in order. After each, it runs the
[test](../glossary.md#test) the task names, and ticks the task only when
the test passes. If something is unclear, it stops and asks you rather
than guessing. When every task is done, it re-reads its own code with
fresh eyes, fixes what is clear-cut, and gives you a short summary.

<div class="example">
<p class="example-title">Example</p>

```text
> The plan is good, implement it
⏺ Update(src/theme.ts)
  ⎿ Updated src/theme.ts with 18 additions and 2 removals
⏺ Bash(npm test)
  ⎿ ✓ 42 tests passed
⏺ 7 tasks out of 7 done · 2 review notes, nothing to decide.
> The change is done, archive it
⏺ Bash(compagnon archive --change add-dark-mode)
  ⎿ ✓ Archived · moved to _compagnon/changes/archive/2026-11-27-add-dark-mode
```

Two sentences: Claude builds and reviews, then files the plan away.

</div>

<details>
<summary>Under the hood: how Claude builds, step by step</summary>

The shortcut is `/compagnon-apply <name>`.

`/compagnon-apply <name>` builds what the plan describes:

1. It checks that the plan is complete.
2. It takes each unticked task of `tasks.md` (`- [ ]`) in order, does it,
   runs the check the task names, and ticks the box only once that check
   passes.
3. It writes tests like your existing ones, then by your language's
   habits and your [`rules`](../reference/configuration.md#rules), and asks
   you how to run them when your project does not say.
4. Once every task is ticked, Claude re-reads the code it touched, the way
   a newcomer to the project would, joined by the
   [experts](../roles.md#experts-and-domains) of the domains the proposal
   ticks. It simplifies what is clear-cut, without widening the scope, and
   lists up to five review notes. A serious security or privacy finding is
   always reported.
5. Finally, it runs all the project's tests once.

Every finding goes to the review record, with the author's answer and the
reviewer's check. The summary ends with what only you can decide: for
example, the code now touches personal data that the plan did not list.

Claude stops at the first blocker or doubt and asks you, rather than
guessing. It works only on the change you named, and never syncs or
archives: you ask for those steps yourself. Apply is the only skill allowed
to run any terminal command, because it must run your tests.

<details>
<summary>Under the hood: experts' fixes and isolated mode</summary>

Claude finds how to run your tests in the project's configuration, its
manifest or its [CI](../glossary.md#ci). An expert fixes a clear defect
within the change's scope, with a test. Any
other change of behavior, it reports instead of making it. The criteria and
knowledge of both steps come from `compagnon roles`; when no role is active
for the review, the review is skipped and the summary says so. In isolated
mode, each reviewer first works alone as a
[sub-agent](../glossary.md#sub-agent) (a separate copy of Claude that does not
see your conversation), and the skill merges their findings.

</details>

</details>

## Check the result

Read the summary: the tasks done, the review notes, and what only you can
decide. Try the result, or ask Claude how to. If something is wrong, say
it in plain words; Claude corrects it before you file anything away.

## Archive

When the result suits you:

> The change is done, archive it.

Claude copies what the plan changed into the project's specs and moves
the plan to the archive: the project now remembers why the change exists.
Then save the result in the project's history
([commit](../glossary.md#commit) it), or ask Claude to.

<details>
<summary>Under the hood: what archiving does</summary>

The shortcut is `/compagnon-archive <name>`.

```bash
compagnon archive --change <name>
```

`/compagnon-archive` closes a finished change, in four steps:

1. It validates the change, and **refuses** on an error.
2. It checks the [review record](review-record.md), and **refuses**
   if a review was expected but not recorded, or if a serious finding is
   not settled yet (`review_blocked`). `compagnon review --change <name>`
   shows what is left. Claude never records a decision in your place.
3. It copies the deltas into the project's specs, as sync does.
4. It moves the change's folder to
   `_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`.

Nothing on disk changes until the whole update has been checked.
Afterwards, the project's specs describe what you shipped, and the
archived folder keeps why and how. Save both in a [commit](../glossary.md#commit). If the change
has a release note, `compagnon changelog --write` adds it to
`CHANGELOG.md` (see [Release notes and CHANGELOG](release-notes.md)).

<details>
<summary>Under the hood: when archiving refuses on the review record</summary>

Archiving reports `review_blocked` when the review record is missing for a
step where review roles were active, or when a finding of severity `high`
is still `open` or `disputed`. Sync, like archive, validates first: on an
error it writes nothing and points to `compagnon validate <name>`.

</details>

What `compagnon validate` checks, and how `--change` is chosen when
several changes are in progress, are in the
[Command-line interface](../reference/cli.md#compagnon-validate) reference
and its [conventions](../reference/cli.md#conventions).

</details>

<details>
<summary>Under the hood: syncing without archiving</summary>

```bash
compagnon sync --change <name>
```

`/compagnon-sync` copies the change's deltas into the project's specs, in
`_compagnon/specs/`, **without** moving the change. Use it when another
change in progress needs to see a new capability, or to check the result
before archiving. It tells you which specs were created, updated or left
unchanged. Usually you skip it: archiving syncs for you.

</details>

## Next step

[How it works](../workflow.md) What happens behind each step.
