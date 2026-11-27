# FAQ

## General

### Does Compagnon send my code to a model?

No. The `compagnon` program never talks to an AI model. It goes online for
two things only: to download rules shared from another
[repository](glossary.md#repository), when you ask for it
(`compagnon sources update`), and to see whether a newer version of
Compagnon exists. The writing is done by your [agent](glossary.md#agent),
Claude Code, which can read and do exactly what it usually can on your
computer.

### Does Compagnon check for updates?

Yes, and that is all it checks. At most once a day, `compagnon status` asks
GitHub for the number of the latest Compagnon version and remembers the
answer on your computer.

- Nothing about you or your project is sent. There is no tracking of any
  kind.
- If the request fails, nothing is shown, and Compagnon does not try again
  before the next day.
- The check never runs during the project's automatic checks
  ([CI](glossary.md#ci)), so their results stay the same from one run to
  the next.
- To turn it off on your computer, set `COMPAGNON_NO_UPDATE_CHECK=1`.
- Nothing is ever installed without your consent. To update, you run
  `compagnon self-update`, or you answer "Update now" when a skill asks.

<details>
<summary>Under the hood: the update request</summary>

The request is a single anonymous HTTPS call to GitHub's public API:
`https://api.github.com/repos/mairistem/compagnon/releases/latest`. It reads
the latest release number and caches it in
`~/.cache/compagnon/update-check.json`, for 24 hours. It is skipped whenever
the `CI` environment variable is set.

</details>

### Can I use Compagnon without Claude Code?

Yes, partly. The `compagnon` command works on its own: you can create a
change, write its plan documents (the [artifacts](glossary.md#artifact))
yourself following `compagnon instructions`, then check, sync and archive
it. What you lose are the skills: they do the writing for you, and they
work only in Claude Code.

### Should I save `_compagnon/` and `.claude/skills/` in the project's history (commit them)?

Yes, both. Saving a version of the project's files in its history (kept by
[git](glossary.md#git)) is called a [commit](glossary.md#commit).
`_compagnon/` is the record of your specs, decisions and changes: commit
all of it, including `_compagnon/decisions/seal.yaml` and
`_compagnon/compagnon.lock`. When `.claude/skills/` is committed, everyone
on the team gets the same skills without running `compagnon init`.

### Does a small fix need a full change?

Every change goes through the same steps, but its plan can be very short: a
one-line proposal, one requirement in its [delta](glossary.md#delta), and
two tasks. If the change does not affect how the software behaves, set
`skip_specs: true` in its `change.yaml`: it then needs no spec at all.

### Can I keep Compagnon out of `CLAUDE.md`, CI or the pull request template?

Yes. When you set up the project, `compagnon init --no-claude-md`,
`--no-ci` and `--no-pr-template` leave each of them alone, and
`--no-integration` does all three; `compagnon update` never adds them
later. You can also take them back after setup. See
[Repository integration and compagnon doctor](guides/repository-integration.md).

### Can Claude start from a Jira ticket?

Yes. Say "Prepare a plan for PROJ-123": Claude reads the ticket once,
without changing anything in Jira, and cites it at the top of the plan.
See [Start from a Jira ticket](guides/mcp.md).

### Should my reviewers work in isolation?

By default, the skill that wrote the plan or the code also reviews it,
playing each reviewing [role](glossary.md#role) in turn, in the same
conversation. In the isolated mode, each reviewer works alone, as a
separate [sub-agent](glossary.md#sub-agent) that never sees your
conversation. It costs more and its benefit has not been measured yet:
start with the changes where a fresh look matters most. See
[Inline or isolated review](roles.md#inline-or-isolated-review).

### How do I share this documentation?

Save it as a single web page, which loads nothing from the internet: you
can send it by email or drop it into a wiki.

```bash
compagnon docs --write ~/Downloads/compagnon-manual.html
```

### Something went wrong: where do I look?

When a command reports a problem, it names it with a code, such as
`decision_seal_mismatch`. [Error codes and fixes](reference/errors.md)
lists every code, with what it means and how to fix it, along with the
common problems that have no code, such as skills that do not appear in
Claude Code.

## Go further

- [Error codes and fixes](reference/errors.md)
- [Installation](installation.md)
- [Asking Claude](skills.md)
- [Start from a Jira ticket](guides/mcp.md)
- [Roles](roles.md)
- [Glossary](glossary.md)
