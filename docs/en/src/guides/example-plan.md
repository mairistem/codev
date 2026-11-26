# An annotated example plan

This page shows one complete plan, as Claude writes it with Compagnon: the
dark mode of the [quickstart](../quickstart.md), the change named
`add-dark-mode`. The application already exists and always shows a light
theme; the plan makes it follow the computer's setting, and lets a user
pick a theme by hand. The plan sits in `_compagnon/changes/add-dark-mode/`,
in four files. Each part below shows one file as it is written, then what
you check there. [Reviewing a plan](review-a-plan.md) explains each part in
general; this page is the same thing on a real case.

## The proposal

The proposal, `proposal.md`, says why the work is needed and what it
changes.

```markdown
# Proposal: Add a dark mode

## Why

Users who work in the evening find the light theme tiring, and several
asked for a dark one. Their computer already has a light or dark setting:
the application ignores it.

## What Changes

- The application shows the light or the dark theme, following the
  computer's setting, instead of always the light one.
- A user can pick a theme by hand in the settings page; that choice is
  kept on their device, even when the computer's setting changes.

## Capabilities

### New Capabilities

### Modified Capabilities

- appearance

### Removed Capabilities

## Impact

- The pages' colors, the settings page.
- No change to accounts or to the server.

## Domains

- [ ] security
- [ ] personal-data
- [x] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: The application follows your computer's light or dark setting, and you can pick a theme by hand in the settings.
```

### What to check here

- **Why**: is this the problem users really have? Here, it is the
  evening users' complaint, in their words.
- **What Changes**: everything you expected is there, nothing more. Note
  what is left out: the choice is kept on the device, not in the user's
  account, so it does not follow them to another computer.
- **Domains**: the boxes ticked say which experts also read the plan;
  here, only the user interface one.
- **Release note**: the sentence users will read when the version comes
  out. Read it as they would.

## The specs

The specs, here `specs/appearance/spec.md`, say what will be true
afterwards: one rule changes, one rule is added.

```markdown
## MODIFIED Requirements

### Requirement: Color theme

The application SHALL show its pages in the theme the computer's setting asks for, light or dark, until the user picks a theme by hand.

#### Scenario: Computer set to dark mode

- **WHEN** a user who never picked a theme opens the application on a computer set to dark mode
- **THEN** the pages are shown in the dark theme

#### Scenario: Computer set to light mode

- **WHEN** a user who never picked a theme opens the application on a computer set to light mode
- **THEN** the pages are shown in the light theme

## ADDED Requirements

### Requirement: A theme picked by hand is kept

The application SHALL keep the theme a user picks in the settings page, light or dark, even when the computer's setting changes.

#### Scenario: The computer switches to dark mode

- **WHEN** a user who picked the light theme opens the application on a computer set to dark mode
- **THEN** the pages are shown in the light theme
- **AND** the settings page shows the light theme as chosen
```

### What to check here

- **The modified rule**: "Color theme" already exists. Today it says
  "The application SHALL show its pages in the light theme." The plan
  shows its new text in full, which replaces the old one: read both and
  check that the difference is exactly what you asked for.
- **The added rule**: a new rule, for the user who picks a theme by hand.
- **The scenarios**: each one is an example you can picture. Is one
  missing? Here, you might ask what happens when the user wants to go back
  to following the computer: the plan does not say.
- **SHALL** means "must"; **WHEN**, **THEN** and **AND** mean "when",
  "then" and "and". Each scenario becomes a [test](../glossary.md#test).

## The design

The design, `design.md`, explains how the work will be done.

```markdown
# Design: Add a dark mode

## Context

The pages take their colors from one style sheet, written for the light
theme. The settings page already holds the user's language.

## Goals / Non-Goals

**Goals**

- Two themes, light and dark, with the same readability.

**Non-Goals**

- Keeping the user's choice in their account, across computers.

## Decisions

### Decision: Colors as named values, one set per theme

Every color of the style sheet becomes a named value ("background",
"text", "link"), with one set for the light theme and one for the dark
theme. The browser tells which one the computer's setting asks for.

**Alternatives considered**: a second, separate style sheet for the dark
theme — two files to keep in step, and every new page written twice.

### Decision: The choice made by hand is kept in the browser

The theme picked in the settings page is kept in the browser of the
device, and read before the page is shown.

**Alternatives considered**: keeping it in the user's account — it would
follow them everywhere, but needs a change to the server; left for later.

## Risks / Trade-offs

- [The page shows light for an instant before turning dark] → the theme
  is applied before the page is drawn.
- [Some colors read badly in the dark theme] → each pair of text and
  background colors is checked for contrast.
```

### What to check here

- You do not judge the technical choices. Read the **Risks** and the
  **Alternatives considered**: they say what users might notice.
- Here, the second decision means the choice does not follow a user from
  one computer to another. If that matters to your users, say so now.
- The first risk is a flash of light color when the page opens: check
  that the answer suits you.

## The tasks

The tasks, `tasks.md`, list the steps of the work, each with the test
that proves it done.

```markdown
# Tasks

## 1. The theme follows the computer

- [ ] 1.1 Turn every color of the style sheet into a named value, with a light set and a dark set, verified by the light pages looking unchanged
- [ ] 1.2 Show the theme the computer's setting asks for, verified by the scenarios "Computer set to dark mode" and "Computer set to light mode"
- [ ] 1.3 Check the contrast of every text and background pair in both themes, verified by a contrast check on each pair

## 2. A theme picked by hand

- [ ] 2.1 Add a "Theme" choice to the settings page, verified by the choice changing the pages at once
- [ ] 2.2 Keep the choice in the browser and apply it before the page is drawn, verified by the scenario "The computer switches to dark mode"
```

### What to check here

- The order makes sense: first the themes, then the choice by hand.
- Each scenario of the specs appears in a task, so each will be tested.
- Something users see without a test? Here, every visible step names its
  check.
