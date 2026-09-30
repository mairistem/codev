Explore an idea, investigate a problem, clarify a need — without committing
to anything.

**No writing.** This workflow creates no change, no artifact and no code. It
modifies no file. It is a thinking partner, and its only output is a shared
understanding.

---

## Input

A topic, a question, or nothing at all. If the user did not specify anything,
ask:

> What do you want to explore?

## How to explore

**Start with what the repository already knows.** Do not ask the user what
the code can answer. In this order:

```bash
codev list --specs          # behaviors already specified
codev list                  # changes in progress, to avoid duplicating work
```

Then read the relevant specs, the decisions in effect in
`_codev/decisions/`, and the relevant implementation.

**Ask the questions that actually move things forward.** A good question is
about a dependency, a constraint or a trade-off the code cannot settle — not
about a fact you could have looked up. When you ask a question, recommend a
default answer and say why.

**Compare the options candidly.** Two or three approaches, each with what it
costs and what it rules out. A recommendation, not a catalog. If one option
is better, say so.

**Check against existing decisions.** If a direction departs from an
accepted decision in `_codev/decisions/`, flag it early: either the direction
changes, or the decision is the thing to propose superseding. Do not silently
reopen a choice that has already been settled.

**Draw when it is clearer than a paragraph.** An ASCII diagram for a data
flow, a state machine or a topology.

## Output

The exploration ends in one of these three ways, and you say which one:

1. **It crystallizes.** Summarize in three points: the problem, the chosen
   approach, the scope. Then offer: "I can create the change — ask me for
   `/codev-propose`."
2. **Information is missing.** Name precisely what is missing and who or
   what can provide it.
3. **It was not worth pursuing.** Say so. An exploration that concludes "this
   is not a problem" or "the code already does it" is a successful
   exploration.

## Guardrails

- Write no file. Create no change. Modify no code.
- Do not present an assumption as an observed fact. Say "I assume" when you
  are assuming.
- If the user asks you to implement during the exploration, do not: tell
  them you go through `/codev-propose` first, and why.
