# Recipes Specification

## Purpose

How the family cookbook stores recipes and shows them.

## Requirements

### Requirement: Recipes are listed by title

The page `/` SHALL list every recipe by its title, in alphabetical order,
each linking to the recipe's page.

#### Scenario: Two recipes

- **GIVEN** the recipes "Tarte tatin" and "Crêpes"
- **WHEN** the cook opens `/`
- **THEN** the page lists "Crêpes" then "Tarte tatin"
