# Approving a plan on GitHub or GitLab

This page is for you if your team works on GitHub or GitLab and you are
asked to approve a plan, without Claude Code and without a
[terminal](../glossary.md#terminal). Everything happens in your web
browser, on the site your team already uses.

Read [Reviewing a plan](review-a-plan.md) first: it says what to check in
each part of a plan. [An annotated example plan](example-plan.md) shows a
complete one.

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
   "Add single comment" to send this one now.
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
   the speech bubble that appears. Write your comment, then click
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
