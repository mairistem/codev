---
id: "0010"
title: "Token savings are sought in the model and the review organization, not in codev's text"
status: accepted
date: 2026-10-20
---

## Context

Between 2026-10-15 and 2026-10-20, the evaluation suite measured several
ways to make codev's sessions use fewer tokens:

| What was tried | Effect on cost | Effect on quality |
|---|---|---|
| `spec-search` second arm (#45): related requirements injected in the instructions | ±0 % | primary recall 0.69 → 0.82 |
| `spec-search` first arm: telling the agent to search | −7 % | no gain (0.73) |
| `planning-recall`: 40 related requirements, a decision per requirement | ±1 % | no gain |
| `planning-context` arm A: the full text of the first five requirements | +0.5 % | equal |
| `planning-context` arm B: arm A plus the review procedures in annexes | +0.2 % | primary recall 0.80, under the range |
| RTK, compressing shell outputs (hook in the eval fixtures) | −1 % planning, +2 % reviews | equal |
| Reviews on Sonnet, whole session | −49 % | recall 1.00, one false positive more on three roles |
| Reviews in the isolated mode, reviewers on Sonnet | −7 % | one false positive more on two roles |
| Planning on Sonnet | −59 % | primary recall 0.82 → 0.62 |

A planning run spends 47 % of its cost re-reading its context, 26 % on
new context and 26 % on output; codev's own text — the skill and its
instructions — is about a tenth of the context written. The branches of
the changes not merged (`feat/planning-recall`,
`feat/planning-context-pre-rebase`, `experiment/rtk`) keep their code and
their measures.

## Decision

A change that aims at fewer tokens works on what decides the bill: the
model a step runs on, and how many review passes and sub-agents a session
runs, measured as decision 0008 requires. Shortening or lengthening the
text codev gives the agent is not, on its own, a way to save tokens: it
is judged on quality, and its cost is only checked not to rise.

## Consequences

- The next savings are sought in the review organization — the review
  mode per role, a review proportional to the change's risk — and in the
  implementation's model, once an evaluation of implementation exists.
- Text changes that the measure shows neutral are not merged for their
  tokens; they may still be merged for their quality.
- The isolated review mode is known to cost about 73 % more than the
  inline mode on the same cases (22.34 against 12.89 USD), with the same
  recall: a project chooses it for the reviewers' independence, not for
  its cost.

## Alternatives considered

- Keep trimming codev's text: every measured attempt stayed within ±3 %,
  the run-to-run noise.
- Compress the agent's tool outputs (RTK): the bulk of what the agent
  reads comes through Claude Code's `Read` tool and `sed`, which it does
  not touch.
