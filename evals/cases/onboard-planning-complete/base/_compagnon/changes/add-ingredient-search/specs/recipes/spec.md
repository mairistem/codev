## ADDED Requirements

### Requirement: Recipes are found by ingredient

The page `/` SHALL offer a search box; `/?q=<words>` SHALL list, by title,
the recipes whose ingredients hold every word of `q`, ignoring case and
accents.

#### Scenario: Two words

- **GIVEN** a recipe of crêpes made with eggs and milk, and an omelette made with eggs
- **WHEN** the cook searches "oeufs lait"
- **THEN** only the crêpes are listed
