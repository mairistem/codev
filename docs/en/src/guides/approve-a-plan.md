# Approving a plan on GitHub

This page is for you if your team works on GitHub or GitLab and you are
asked to approve a plan, without Claude Code and without a
[terminal](../glossary.md#terminal). Everything happens in your web
browser, on the site your team already uses.

Read [Reviewing a plan](review-a-plan.md) first: it says what to check in
each part of a plan. [An annotated example plan](example-plan.md) shows a
complete one.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 230" role="img" aria-label="Approving on GitHub or GitLab, the request's timeline in five steps: 1, the plan is pushed in a pull request or merge request; 2, you comment on its lines, and Claude updates the plan until it suits you; 3, you approve the request; 4, Claude builds exactly what the plan says; 5, the team reviews the code.">
<defs><marker id="dg-approve-a-plan-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="4" y="6" width="752" height="218" rx="14" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="1.5"/>
<text x="740" y="30" text-anchor="end" font-size="13" style="fill:var(--dg-muted)">Pull request (GitHub) or merge request (GitLab)</text>
<path d="M226,102 C222,58 80,58 77,98" style="fill:none;stroke:var(--dg-you)" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#dg-approve-a-plan-arrow)"/>
<text x="152" y="58" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">Claude updates the plan</text>
<line x1="96" y1="120" x2="206" y2="120" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-approve-a-plan-arrow)"/>
<line x1="248" y1="120" x2="358" y2="120" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-approve-a-plan-arrow)"/>
<line x1="400" y1="120" x2="510" y2="120" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-approve-a-plan-arrow)"/>
<line x1="552" y1="120" x2="662" y2="120" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-approve-a-plan-arrow)"/>
<circle cx="76" cy="120" r="16" style="fill:var(--dg-claude)"/>
<text x="76" y="125" text-anchor="middle" font-size="14" font-weight="600" style="fill:var(--dg-box)">1</text>
<text x="76" y="162" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Plan pushed</text>
<text x="76" y="184" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">the developer sends</text>
<text x="76" y="201" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">Claude's plan</text>
<circle cx="228" cy="120" r="16" style="fill:var(--dg-you)"/>
<text x="228" y="125" text-anchor="middle" font-size="14" font-weight="600" style="fill:var(--dg-on-you)">2</text>
<text x="228" y="162" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Comments</text>
<text x="228" y="184" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">you comment on</text>
<text x="228" y="201" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">the plan's lines</text>
<circle cx="380" cy="120" r="16" style="fill:var(--dg-you)"/>
<text x="380" y="125" text-anchor="middle" font-size="14" font-weight="600" style="fill:var(--dg-on-you)">3</text>
<text x="380" y="162" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Approval</text>
<text x="380" y="184" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">you approve</text>
<text x="380" y="201" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">the request</text>
<circle cx="532" cy="120" r="16" style="fill:var(--dg-claude)"/>
<text x="532" y="125" text-anchor="middle" font-size="14" font-weight="600" style="fill:var(--dg-box)">4</text>
<text x="532" y="162" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Claude builds</text>
<text x="532" y="184" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">exactly what</text>
<text x="532" y="201" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">the plan says</text>
<circle cx="684" cy="120" r="16" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="684" y="125" text-anchor="middle" font-size="14" font-weight="600" style="fill:var(--dg-ink)">5</text>
<text x="684" y="162" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Code reviewed</text>
<text x="684" y="184" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">the team reviews</text>
<text x="684" y="201" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">the code, then merges</text>
</svg>
</div>
<figcaption>Approving on GitHub or GitLab: the plan arrives as a request, you comment until it suits you, you approve, and only then does Claude build the code that the team reviews.</figcaption>
</figure>

## Where the plan arrives

When a developer has had Claude write a plan, they send it to the team as
a request to add it to the project: a
[pull request](../glossary.md#pull-request) on GitHub, a merge request on
GitLab. You usually receive a notification or an email with its link.

On both sites, the plan is among the request's changed files, in one
folder: `_compagnon/changes/<name>/`, where `<name>` is the change's short
name, such as `add-dark-mode`. The folder holds the four parts of the plan:

- `proposal.md`: why;
- `specs/`: what will be true afterwards;
- `design.md`: how;
- `tasks.md`: the steps.

On GitHub only, the request's description may also name that folder, in
its "compagnon" section, on the line `- compagnon change:`, if the
developer filled it in.

## Where the points to discuss are

Before you see the plan, Claude's reviewers have already challenged it.
What they found, and how each point was answered, is written in
`review.md`, in the same folder. Start there: it shows you what is still
open, and which parts of the plan to read most closely.

## On GitHub

1. **Open the changed files.** In the pull request, click the
   "Files changed" tab. Find the files under `_compagnon/changes/<name>/`.
2. **Comment on a line.** Move the mouse over the line, then click the
   blue "+" that appears beside it. Write your comment, then click
   "Start a review" to gather several comments and send them together, or
   "Add single comment" to send this one now. Without a mouse, move to the
   line with the Tab key: the same comment button appears, and a screen
   reader announces it as a button to add a comment. You can also write
   your comments in the "Conversation" tab, naming the file and the line.
3. **Send your answer.** Click "Review changes" (at the top right of the
   changed files; GitHub's newer page may call it "Submit review"). Write
   a short summary, then choose one:
   - to ask for changes: "Request changes";
   - to approve: "Approve";
   - to comment without deciding: "Comment".

   Then click "Submit review".

## On GitLab

GitLab shows its buttons in your language. The labels are given here in
English, then in French.

1. **Open the changed files.** In the merge request, click the "Changes"
   tab ("Modifications"). Find the files under `_compagnon/changes/<name>/`.
2. **Comment on a line.** Move the mouse over the line number, then click
   the speech bubble that appears (without a mouse, move to the line
   with the Tab key: a screen reader announces the same comment button;
   or write in the "Overview" tab, naming the file and the line). Write your comment, then click
   "Start a review" ("Commencer une revue") to gather several comments, or
   "Add comment now" ("Ajouter un commentaire maintenant") to send this
   one at once.
3. **Ask for changes.** If you started a review, click "Finish review"
   or "Submit review" ("Terminer la revue" / "Soumettre la revue"), then
   choose "Request changes" ("Demander des modifications") where your
   GitLab offers it. Otherwise, your comments are enough: say clearly
   that the plan must change before you approve.
4. **Approve.** On the request's "Overview" tab ("Vue d'ensemble"), click
   "Approve" ("Approuver"). Your approval is recorded on the request.

If you cannot find a button, look for the action rather than the word:
both sites rename their buttons from time to time.

## After your comments

You do not edit the plan yourself. The developer opens Claude Code and
asks Claude to take your comments into account, for example:

> Take the comments on the pull request into account in the plan.

Claude updates the plan, and the developer sends the new version to the
same request. Read the parts that changed, answer your comments, and
approve when the plan says what you want. Only then does Claude build.

## Go further

- [Reviewing a plan](review-a-plan.md)
- [An annotated example plan](example-plan.md)
- [Review record](review-record.md)
