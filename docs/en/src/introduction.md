# Compagnon

**Plan with Claude before it codes.**

Compagnon helps you work with Claude Code, Anthropic's coding assistant,
without losing control. Before any code is written, Claude writes down what
it is about to build: a short plan you can read, discuss and correct. Once
you agree, Claude builds exactly that, and the plan stays in your
[repository](glossary.md#repository) (the folder that holds your project's
files) as a record of why things are the way they are.

You do not need to know how AI works to use it. You ask in plain words —
"add a dark mode that follows the system setting" — and Compagnon guides
Claude through the rest.

## Who decides

You do. At every step, Claude proposes and you decide:

1. **You ask** for a change, in your own words.
2. **Claude proposes a plan**: why the change is needed, what will change,
   how, and the list of tasks.
3. **You read and approve it** — or ask for corrections. Nothing is built
   before you agree.
4. **Claude builds it**, task by task, and checks its own work.
5. **You review the result** and close the change.

Nothing is applied without your approval, and every decision is written
down where your whole team can see it.

## The name

Compagnon comes from *compagnonnage*, the French tradition of craft guilds:
a master leads the work, the *compagnons* carry it out, and everyone follows
plans agreed beforehand. Here, you are the master, and Claude's [agents](glossary.md#agent)
are your compagnons.

## What you get

- **Fewer surprises.** You see what Claude will do before it does it.
- **A shared memory.** The plans and the decisions stay in the repository,
  in plain text: anyone can understand later why something was built.
- **Decisions that stick.** Once your team settles a choice, Claude follows
  it instead of reopening it on every change.

## Where to start

What will you do with Compagnon? Each answer opens a short path:

- [I review and approve plans — I do not code](paths/review.md)
- [I do not code, but I want Claude to build something for me](paths/build.md)
- [I am a developer and I have Claude write code](paths/develop.md)
- [I set Compagnon up for my team](paths/team.md)

Every term is explained, in plain words, in the [Glossary](glossary.md).

Until 0.6, Compagnon was named codev: [Moving from codev](guides/moving-from-codev.md)
explains how to convert a project set up then. Where Compagnon comes from,
and why, is told in [Origins](origins.md).
