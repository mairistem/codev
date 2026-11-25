---
id: "0014"
title: "The product is named Compagnon"
status: accepted
date: 2026-11-25
supersedes: ["0003"]
---

## Context

Until 0.6, the product was named **codev**. The name was ambiguous twice:
Codev by Cluesmith is an open-source, spec-driven methodology for Claude
Code with its own `codev` command and `codev/` folder, the same field and
nearly the same vocabulary; and, for French-speaking teams, "codev" is the
usual short name of *codéveloppement professionnel*, a peer-coaching method
for managers. Decision 0003 named the planning root after the product.

## Decision

The product is **Mairistem Compagnon**, after *compagnonnage*: the
developer is the master who leads the work, the Claude agents are the
*compagnons* who carry it out, following plans agreed beforehand. Every
name follows it — the `compagnon` binary, the `_compagnon/` planning root,
`compagnon.lock`, the `compagnon_version` key, the `/compagnon-*` skills,
the `COMPAGNON_*` variables, the `mairistem/compagnon` repository.

This supersedes 0003 on the **name** of the planning root only: its
rationale stands — one visible folder, `_`-prefixed, named by a single
constant. 0003 keeps its text: it records a choice made under the former
name.

## Consequences

- Projects set up under the former name are converted by
  `compagnon migrate`; until then, every other command refuses them with
  `legacy_root`, and a source laid out under it is refused with
  `legacy_source`.
- The former names live in one place in the code,
  `compagnon_core::legacy`, which the migration, the release bridge and the
  rename notice build on.
- The bodies of decisions 0001, 0002 and 0004 to 0013 were re-sealed with
  `compagnon decision seal --force` for the rename only: their diff changes
  names, never a choice.
- Mairistem Atelier (project Talos), which will run Compagnon autonomously
  on Jira tickets, is born on the new names.

## Alternatives considered

- **Tandem** — two riders on one bike: it says two, while a developer leads
  several agents; and the name is used by other Claude Code tools.
- **Cordée**, **Chantier**, **Établi** — evocative, but less fitting or less
  liked; Maestro, Conductor, Brigade, Baton, Atelier and Squad are already
  taken by tools of the same field.
- **Keeping codev** — the ambiguity with Cluesmith's Codev grows with both
  projects.
