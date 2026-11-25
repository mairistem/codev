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

## Artifact

One of the files that make up a plan: the proposal, the specs, the design
or the tasks. Compagnon calls them artifacts; you can simply think of them
as the four parts of the plan.

## Capability

One area of what the product does, such as "login" or "invoices". The
project's specs are organized by capability, each in its own folder under
`_compagnon/specs/`.

## Change

One piece of work, from the request to its archive: a feature, a fix, a
refactor. Each change has its own folder under `_compagnon/changes/`,
holding its plan.

## CI

Continuous integration: checks a service such as GitHub runs on its own
every time someone proposes a change to the project, for example running
the tests and `compagnon validate`. A red check means something to fix.

## Claude Code

Anthropic's coding assistant. It works in your terminal, your code editor
or the Claude desktop and web apps, reads your repository and acts on it at
your request. Compagnon adds its skills to Claude Code.

## Commit

A saved step in the project's history, with a short message saying what
changed. Saving your work this way is called committing; Claude can do it
for you.

## Decision

A choice the team made once for all changes, such as "prices are always
shown with tax". Decisions live in `_compagnon/decisions/`; Claude follows them instead
of reopening them.

## Delta

The part of a change's specs that says what changes in the project's
specs: requirements added, modified or removed. On archiving, the delta is
merged into the project's specs.

## Design

The part of a plan that says how the change will be built: the technical
choices, the alternatives considered, the risks. Written in `design.md`.

## Domain

A sensitive area a change may touch: security, personal data, user
interface, performance, operations, migration. The proposal lists the
domains it touches, and each one brings its expert to review the plan and
the code.

## Frontmatter

The few lines at the very top of a file, between two `---` lines, that
describe the file rather than belong to its text: its name, its version.
Each skill starts with one.

## Git

The tool that keeps the history of a project's files: who changed what,
when, and why. Compagnon's plans live in that history beside the code.

## Kebab-case

A way of writing a name in lowercase words joined by hyphens, with no
spaces: `add-dark-mode`. Claude names each change this way, so that the
name works as a folder name.

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

## Pull request

A request to add work to the shared project, on GitHub or GitLab. Your
team reads it, comments, then approves or asks for changes; the plan and
the code usually come together in one.

## Repository

The folder that holds a project's files and their whole history, usually
shared on GitHub or GitLab. Compagnon keeps its plans in it, in the
`_compagnon/` folder.

## Requirement

One rule the product must follow, written so it can be checked: "the
application SHALL follow the system's color scheme" (SHALL means
"must"). Each requirement comes with scenarios.

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

## Schema

The list of the parts a plan holds, and the order they are written in.
Compagnon's usual schema asks for a proposal, specs, a design and tasks;
a team can define its own.

## Seal

A fingerprint of an accepted decision's text. If someone edits the
decision afterwards, the fingerprint no longer matches and `compagnon
validate` reports it.

## Skill

A set of instructions Claude Code follows for one kind of task. Compagnon
installs eight, such as planning a change or building it. Claude starts
the right one when your request matches; see [How to ask](skills.md#how-to-ask).

## Spec

A description of what the product does, as requirements and scenarios. The
project's specs, in `_compagnon/specs/`, describe the current behavior and
are the reference.

## Sub-agent

A separate copy of Claude, started for one job, that does not see your
conversation. Compagnon uses sub-agents so that a reviewer judges a plan
or code without knowing what its author intended.

## Tasks

The last part of a plan, in `tasks.md`: the list of steps to build the
change, each with the test that proves it done. Claude checks them off as it
goes.

## Terminal

The window where you type commands, such as `compagnon init`. On macOS
it is called Terminal, on Windows PowerShell. Claude Code also runs in it.

## Test

A small program that checks the product works as expected, for example
"in dark mode, the page is dark". Each scenario of a plan becomes a test,
and Claude runs them to check its own work.

## Workflow

The sequence of steps a change goes through: ask, plan, review, build,
archive. Each step has its skill. In the configuration, `workflows:` lists
the skills a project installs.
