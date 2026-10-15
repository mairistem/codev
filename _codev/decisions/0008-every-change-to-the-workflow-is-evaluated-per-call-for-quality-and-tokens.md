---
id: "0008"
title: "Every change to the workflow is evaluated, per call, for quality and tokens"
status: accepted
date: 2026-10-15
---

## Context

codev's skills are prompts: what they make an agent read and do decides
both the quality of its answers and the tokens it spends, and neither can
be judged by reading the change. The evaluation suite (`evals/`) measures
each step it covers — the review steps and the planning of
`/codev-propose` — for quality and for consumption, down to every call to
the model and the source of the tokens entering its context, and
`codev-evals compare` gives a verdict against the committed baseline.

## Decision

A change to what codev's agents read or do — a skill, a workflow under
`assets/workflows/`, a role, an output a skill reads such as the JSON of
`crates/codev-cli/src/contract.rs` — is evaluated with `codev-evals` on
the steps it touches before it is merged: its quality stays within the
range of the baseline's repetitions or improves, and, when it aims at
consumption, its cost falls under the baseline's floor, each call's
tokens traced to their source. A step the suite does not cover is covered
first, by its own change.

## Consequences

- Every plan that touches the workflow meets this decision among the
  decisions in force, and its tasks include the comparison; its pull
  request pastes the output of `codev-evals compare`.
- A gain that stays within the baseline's spread is not shipped as a
  saving: it is reported as such, with its numbers.
- The output tokens never decide a verdict: they carry the model's
  reasoning, which the quality measures guard.
- Evaluating costs money and time — a planning case about 1.65 USD — so
  a change selects the steps it touches (`--step`), and the baseline is
  recorded locally, not on every push.

## Alternatives considered

- Keeping the rule only in the contributing guides — it would not reach
  the agents that plan the changes.
- A CI gate that runs the suite on every pull request — it calls a model
  and costs money on each push, which the suite's workflow deliberately
  avoids.
