# Proposal: Search recipes by ingredient

## Why

The cookbook holds more than a hundred recipes, and the cook usually
starts from what is in the fridge. Today the only way in is the list of
titles.

## What Changes

- A search box on `/` finds the recipes whose ingredients hold every word
  typed, ignoring case and accents.

## Capabilities

### New Capabilities

### Modified Capabilities

- recipes

### Removed Capabilities

## Impact

- `recipes/app.py`, `recipes/store.py`, and their tests.

## Domains

None

## Release note

- Added: The home page has a search box that finds recipes by ingredient.
