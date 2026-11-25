# Glossary

Every term the documentation uses, in plain words, in alphabetical order.

## Agent

A program that does a task on its own, step by step, by deciding what to do
next: reading files, running commands, writing code. Claude, working in
Claude Code, is an agent. Several agents can work side by side, each on its
own part of the work.

## Archive

The last step of a change. Once the work is done and checked, archiving
merges the change's specs into the project's specs and moves the change's
folder to `_compagnon/changes/archive/`, dated. Nothing is lost: the plan
stays readable as a record.

## Capability

One area of what the product does, such as "login" or "invoices". The
project's specs are organized by capability, each in its own folder under
`_compagnon/specs/`.

## Change

One piece of work, from the request to its archive: a feature, a fix, a
refactor. Each change has its own folder under `_compagnon/changes/`,
holding its plan.

## Claude Code

Anthropic's coding assistant. It works in your terminal, your code editor
or the Claude desktop and web apps, reads your repository and acts on it at
your request. Compagnon adds its skills to Claude Code.

## Decision

A choice the team made once for all changes, such as "dates are stored in
UTC". Decisions live in `_compagnon/decisions/`; Claude follows them instead
of reopening them.

## Delta

The part of a change's specs that says what changes in the project's
specs: requirements added, modified or removed. On archiving, the delta is
merged into the project's specs.

## Design

The part of a plan that says how the change will be built: the technical
choices, the alternatives considered, the risks. Written in `design.md`.

## MCP

Model Context Protocol: a standard way to connect Claude to other tools,
such as Jira. With a Jira connection, Claude can read the ticket you
mention when it writes a plan.

## Plan

What Claude writes before building anything: the proposal, the specs, the
design and the tasks. You read it, correct it, and approve it before any
code is written.

## Proposal

The first part of a plan, in `proposal.md`: why the change is needed, what
will change, and what it affects.

## Requirement

One rule the product must follow, written so it can be checked: "the
application SHALL follow the system's color scheme". Each requirement comes
with scenarios.

## Review

A re-reading of the plan or of the code by roles that did not write it. Its
findings, and the answers to them, are kept in the change's `review.md`.

## Role

A point of view Claude takes at one step, with its own checklist: the
skeptic who looks for flaws in a plan, the newcomer who checks the code is
easy to follow, the security expert. See [Roles](roles.md).

## Scenario

A concrete example of a requirement, in three lines: given a situation,
when something happens, then the expected result. Each scenario becomes a
test.

## Skill

A set of instructions Claude Code follows for one kind of task. Compagnon
installs eight, such as planning a change or building it. Claude starts
the right one when your request matches; see [How to ask](skills.md#how-to-ask).

## Spec

A description of what the product does, as requirements and scenarios. The
project's specs, in `_compagnon/specs/`, describe the current behavior and
are the reference.

## Tasks

The last part of a plan, in `tasks.md`: the list of steps to build the
change, each with the test that proves it done. Claude checks them off as it
goes.
