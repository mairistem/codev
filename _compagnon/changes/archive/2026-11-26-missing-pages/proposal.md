# Proposal: The pages each profile still misses

## Why

Six test readers, one per profile, found their path at once but stopped
short of their real case: a reviewer does not know how to approve on
GitHub or GitLab, nor how to read a modified rule; someone who does not
code has neither Claude Code nor a project to start from; a developer's
project is not empty; a team lead finds the agent's roles but not who in
the team approves what; a contributor finds no "how to contribute" page.
The paths now exist; these pages are what they lead to.

## What Changes

- New pages, in both editions:
  - **An annotated example plan**: a complete plan — proposal, specs,
    design, tasks — with, beside each part, what a reviewer checks there;
    the French edition shows the plan as Compagnon writes it in French.
  - **Approving a plan on GitHub or GitLab**: where the plan arrives,
    where the "points to discuss" are, how to comment, ask for changes and
    approve, without Claude Code or a terminal.
  - **Starting from nothing**: what to get (Claude Code, a folder for the
    project), the two commands to type once, and the first request.
  - **Adopting Compagnon on an existing project**: what `compagnon init`
    adds to a project that already exists, why nothing needs documenting
    up front, and how the specs grow change by change.
  - **Deploying for a team**: installing on every workstation, one
    Compagnon version for all, which files to commit, the check that
    blocks a pull request, and who approves a plan and who approves the
    code.
  - **Contributing**: building, testing and checking Compagnon, and the
    change-first rule, pointing to `CONTRIBUTING.md` for the details.
- "Reviewing a plan" explains how to read a modified rule: the added,
  modified, removed and renamed sections, before and after.
- The path pages lead to the new pages: the build path starts from
  nothing and names the example under "Next"; the review path's "Next"
  names the example and approving; the developer path's names adopting
  on an existing project; the team path takes "Deploying for a team" as a
  step, Roles moving to its "Next"; Origins ends on Contributing.

## Capabilities

### New Capabilities

### Modified Capabilities

- docs

### Removed Capabilities

## Impact

- `docs/en/src/` and `docs/fr/src/`: six new pages, `SUMMARY.md`,
  `guides/review-a-plan.md`, the four path pages, `origins.md`.
- `crates/compagnon-cli/src/docs_checks.rs`: checks on the new pages.
- The offline manual (`compagnon docs`) embeds the new pages.

## Domains

- [ ] security
- [ ] personal-data — the match is the English word "phone" in the example scenario of both home pages (`docs/site/index.html` and `fr.html`, line 702), already on the branch; not personal data
- [x] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Six pages each profile asked for — an annotated example plan, approving a plan on GitHub or GitLab, starting from nothing, adopting Compagnon on an existing project, deploying it for a team, and contributing — and "Reviewing a plan" explains how to read a modified rule.
