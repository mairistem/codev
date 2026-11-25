# Reviewing a plan

This guide is for you if you do not write code but are asked to read and
approve what Claude plans to build: product owner, manager, domain expert.
You need no [terminal](../glossary.md#terminal): a plan is four short text files, which you can read
in your code host (GitHub, GitLab), in an editor, or by asking Claude to
show them to you.

Your job is not to check the code. It is to check that the plan **solves
the right problem, in the right way for the people who will use it**.
Claude and the reviewing [roles](../glossary.md#role) take care of the technical side.

## Where the plan is

Each change has its own folder, `_compagnon/changes/<name>/`. You can also
simply ask:

> Show me the plan for the dark mode, in plain words.

## The proposal: why

`proposal.md` says why the change is needed, what will change and what it
affects. Check that:

- the problem described is the real one, as users or customers experience it;
- nothing you expected is missing, and nothing unwanted was added;
- what is explicitly left out is acceptable for now.

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
"then". In the proposal, a "## Domains" section lists the sensitive
[domains](../glossary.md#domain) the change touches, such as security or
personal data.

This is the part that matters most to you. Check that:

- each scenario describes what you expect, in your own business terms;
- the unusual cases are covered: an empty list, a user without rights, an
  error;
- no scenario describes something you do not want.

Every scenario becomes a [test](../glossary.md#test): if it is written here, it will be checked.

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

> In the plan, a user who picks a theme by hand keeps it, even if the
> system setting changes.

> Remove PDF export from this change: we will do it later.

Then read the parts that changed. Repeat until the plan says what you want.

## Approve

When the plan suits you, say so: "the plan is good, implement it". Claude
then builds exactly what the plan says, and nothing else. If your team
reviews work on GitHub or GitLab, the plan comes as a
[pull request](../glossary.md#pull-request) (a request to add it to the
project). Approve it there: your approval is recorded.
