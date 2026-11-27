# Reviewing a plan

This guide is for you if you do not write code but are asked to read and
approve what Claude plans to build: product owner, manager, domain expert.
You need no [terminal](../glossary.md#terminal): a
[plan](../glossary.md#plan) is four short text files, which you can read
on GitHub, in an editor, or by asking Claude to
show them to you.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 200" role="img" aria-label="Inside a plan, four files in reading order: proposal.md says why and what changes, for everyone; specs/ say what the product will do, with its scenarios, for testers; design.md says how, its choices and risks, for developers; tasks.md lists the tasks, each with its test, for Claude. Most people stop after the first.">
<defs><marker id="dg-review-a-plan-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="4" y="20" width="168" height="130" rx="12" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="3"/>
<rect x="4" y="20" width="168" height="6" rx="3" style="fill:var(--dg-you)"/>
<text x="20" y="56" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">proposal.md</text>
<text x="20" y="84" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Why</text>
<text x="20" y="106" font-size="13" style="fill:var(--dg-muted)">and what changes</text>
<text x="20" y="134" class="mono" font-size="11" style="fill:var(--dg-muted)">for everyone</text>
<line x1="174" y1="85" x2="190" y2="85" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-a-plan-arrow)"/>
<rect x="194" y="20" width="168" height="130" rx="12" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<rect x="194" y="20" width="168" height="6" rx="3" style="fill:var(--dg-line)"/>
<text x="210" y="56" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">specs/</text>
<text x="210" y="84" font-size="15" font-weight="600" style="fill:var(--dg-ink)">What it does</text>
<text x="210" y="106" font-size="13" style="fill:var(--dg-muted)">rules and scenarios</text>
<text x="210" y="134" class="mono" font-size="11" style="fill:var(--dg-muted)">for testers</text>
<line x1="364" y1="85" x2="380" y2="85" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-a-plan-arrow)"/>
<rect x="384" y="20" width="168" height="130" rx="12" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<rect x="384" y="20" width="168" height="6" rx="3" style="fill:var(--dg-line)"/>
<text x="400" y="56" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">design.md</text>
<text x="400" y="84" font-size="15" font-weight="600" style="fill:var(--dg-ink)">How</text>
<text x="400" y="106" font-size="13" style="fill:var(--dg-muted)">choices and risks</text>
<text x="400" y="134" class="mono" font-size="11" style="fill:var(--dg-muted)">for developers</text>
<line x1="554" y1="85" x2="570" y2="85" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-a-plan-arrow)"/>
<rect x="574" y="20" width="182" height="130" rx="12" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<rect x="574" y="20" width="182" height="6" rx="3" style="fill:var(--dg-claude)"/>
<text x="590" y="56" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">tasks.md</text>
<text x="590" y="84" font-size="15" font-weight="600" style="fill:var(--dg-ink)">The tasks</text>
<text x="590" y="106" font-size="13" style="fill:var(--dg-muted)">each with its test</text>
<text x="590" y="134" class="mono" font-size="11" style="fill:var(--dg-muted)">for Claude</text>
<text x="380" y="186" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">Read them left to right — most people stop after the first.</text>
</svg>
</div>
<figcaption>Inside a plan: four short files, from the why to the tasks. The first is for everyone; the further right, the more technical.</figcaption>
</figure>

Your job is not to check the code. It is to check that the plan **solves
the right problem, in the right way for the people who will use it**: the
proposal and the specs are where you say so. Claude and the reviewing
[roles](../glossary.md#role) take care of the technical side.

## Where the plan is

Each change has its own folder, `_compagnon/changes/<name>/`, holding the
four files above. You can also simply ask:

> Show me the plan for the dark mode, in plain words.

Before you read it, Claude's reviewers have already challenged the plan:
their points, and the answer to each, are in `review.md`, in the same
folder. The [Review record](review-record.md) chapter explains how to read
it.

## The proposal: why

`proposal.md` says why the change is needed, what will change and what it
affects. Check that:

- the problem described is the real one, as users or customers experience it;
- nothing you expected is missing, and nothing unwanted was added;
- what is explicitly left out is acceptable for now.

In the proposal, a "## Domains" section lists the sensitive
[domains](../glossary.md#domain) the change touches, such as security or
personal data.

## The specs: what will be true afterwards

The files under `specs/` list **requirements** — rules the product must
follow — each with **scenarios**: a situation, an action, the expected
result. They read like this:

```text
Requirement: The theme follows the system setting by default

Scenario: No theme chosen yet
- GIVEN a user who has never changed the theme
- WHEN their computer is set to dark mode
- THEN the application shows the dark theme
```

The plan files may show a few English keywords, whatever their language:
SHALL or MUST means "must"; GIVEN / WHEN / THEN mean "given", "when",
"then".

This is the part that matters most to you. Check that:

- each scenario describes what you expect, in your own business terms;
- the unusual cases are covered: an empty list, a user without rights, an
  error;
- no scenario describes something you do not want.

Every scenario becomes a [test](../glossary.md#test): if it is written here, it will be checked.

### Reading a modified rule

A plan does not rewrite all of the project's rules: it lists only the ones
that change — its [delta](../glossary.md#delta) — sorted under four kinds
of section. These are the titles you will see:

- **`ADDED Requirements`**: new rules, which did not exist before.
- **`MODIFIED Requirements`**: rules that exist already and change.
- **`REMOVED Requirements`**: rules that disappear, each with its
  **Reason** and its **Migration** (what to do instead).
- **`RENAMED Requirements`**: rules whose title changes, with a FROM line
  (the old title) and a TO line (the new one); their text stays the same.

A modified rule shows its new text **in full**, scenarios included, under
the same title as the current rule. That new text replaces the current one
entirely: anything missing from it will be gone. To see what changes,
compare it with the current rule, under the same title in the project's
specs, in the folder `_compagnon/specs/`: on GitHub, open that
folder from the repository's page, then the `spec.md` file of the folder
named like the plan's. Or ask the developer, or Claude:

> In the dark mode plan, show me each modified rule before and after.

Check that the difference is exactly what you asked for, and that nothing
you still want was dropped on the way. The
[annotated example plan](example-plan.md) shows a modified rule and an
added one, with what to check in each.

## The design: how

`design.md` explains the technical choices. You do not need to judge them,
but read its risks and the options it ruled out: if a risk worries you, or a
trade-off affects users (speed, cost, a feature dropped), say so.

## The tasks: the steps

`tasks.md` lists the steps of the work, each with the test that proves it
done. Check only that the order makes sense to you, and that nothing a user
will see is left without a test.

## Ask for changes

You never edit the files yourself. Tell Claude what to change, in plain
words, and it updates the plan and keeps its parts consistent:

<div class="example">
<p class="example-title">Example</p>

```text
> In the plan, a user who picks a theme by hand keeps it, even if the
  system setting changes
⏺ Update(_compagnon/changes/add-dark-mode/specs/ui/theme/spec.md)
  ⎿ Updated with 1 addition: Scenario: A theme picked by hand is kept
⏺ Update(_compagnon/changes/add-dark-mode/tasks.md)
  ⎿ Updated with 1 addition: 2.3 Keep the theme picked by hand
⏺ The specs and the tasks now say it. Read the new scenario in specs/.
```

One sentence of yours; Claude changed every part of the plan it concerns.

</div>

Then read the parts that changed. Repeat until the plan says what you want.

## Approve

When the plan suits you, say so: "the plan is good, implement it". Claude
then builds exactly what the plan says, and nothing else. If your team
reviews work on GitHub, the plan comes as a
[pull request](../glossary.md#pull-request) (a request to add it to the
project). Approve it there: your approval is recorded.

## Next step

[Approving a plan on GitHub](approve-a-plan.md) Your team reviews on GitHub? Approve the plan there.
