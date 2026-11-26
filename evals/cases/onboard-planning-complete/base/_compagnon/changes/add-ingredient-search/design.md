# Design: Search recipes by ingredient

## Context

Recipes live in a SQLite table, their ingredients as one text column
(`recipes/store.py`). A hundred recipes is small.

## Goals / Non-Goals

**Goals:**

- Find recipes by ingredient words, ignoring case and accents.

**Non-Goals:**

- Ranking the results, or searching the steps.

## Decisions

### Decision: Filter in Python over a normalized copy of the ingredients

Each word and the ingredients are lowercased and stripped of accents
(`unicodedata`), and a recipe matches when every word is in its
ingredients. A hundred rows read at once cost nothing.

**Alternatives considered**: SQLite FTS5 — rejected, an index to maintain
for a hundred rows.

## Risks / Trade-offs

- [The cookbook grows to thousands of recipes] → move to FTS5 then.
