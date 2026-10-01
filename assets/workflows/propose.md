Create a change and write its planning artifacts, in one pass.

**Planning boundary.** This workflow produces planning artifacts only. The
request that triggered it authorizes planning only, even if it says "build",
"fix" or "implement". Do not modify any code file. When the artifacts are
complete, stop and present them. Do not move on to implementation in the same
response: wait for a new request from the user.

---

## Input

The request must contain either a change name in kebab-case, or a description
of what the user wants to build.

## Steps

### 0. Detect an external ticket (optional)

Before resolving the change name, scan the user's prompt for a ticket
identifier matching the regular pattern `[A-Z]{2,}-\d+` (for example
`PROJ-123`, `ABC-42`).

**Three branches**:

- **No match** — go straight to step 1; behavior is exactly as if
  this step did not exist.
- **Match found, Jira MCP available** — call the
  `{{JIRA_MCP_TOOL}}` tool with the identifier of the
  first ticket detected. Two strict rules:
  - **A single call** — never call the MCP twice in the same
    invocation. Any further tickets are only named.
  - **Read-only** — **never** call any other tool of the Jira MCP
    (no `search`, no `create`, no `transition`). Exactly one
    `{{JIRA_MCP_TOOL}}` call, on the exact ID mentioned.

  The result (title, description, status, type) becomes a source of
  context for writing: read it, understand what is expected, and write
  the proposal with that knowledge.

- **Match found, no Jira MCP** — tell the user:

  > Ticket **<ID>** is mentioned but no Jira MCP is available in this
  > session — the proposal will be written without its content.

  Then continue with what you know (the user's prompt alone). The
  proposal still cites the ticket at the top, marked "content not
  retrieved".

**Multiple tickets**: if two or more tickets are mentioned
(`PROJ-123 and PROJ-456`), only the first is retrieved through the MCP.
The others are listed at the top of the proposal under the line
"other ticket(s) mentioned: `<list>`", for traceability — a reader will
look them up by hand.

### 1. Understand the request

If nothing clear is provided, ask an open question, without offering a list
of choices:

> What change do you want to make? Describe what you want to build or fix.

Derive a kebab-case name from the description ("add user authentication"
→ `add-user-auth`).

Do not proceed until you understand what must be built. If the request
contains an ambiguity that would materially change the scope, the observable
behavior, compatibility or the acceptance criteria, ask before creating the
change. For a minor detail, make a reasonable assumption and record it in the
artifacts.

### 2. Create the change

```bash
codev new change "<name>"
```

Add `--schema "<name>"` only if the user explicitly asked for a particular
workflow. Otherwise, omit the flag to keep the configured schema.

If the user asks which workflows exist: `codev schemas --json`.

### 3. Get the build order

```bash
codev status --change "<name>" --json
```

Fields to use:

- `applyRequires` — the artifacts required before implementation
- `artifacts[]` — each with its `status` and its `requires` edges
- `planningHome`, `changeRoot` — the resolved paths. Use them; never assume
  a path relative to the repository

### 4. Create each artifact of the required set

Use your task list to track progress.

For each artifact whose `status` is `ready`:

**a. Get its instructions.**

```bash
codev instructions <artifact-id> --change "<name>" --json
```

The response contains:

| Field | Use |
|---|---|
| `instruction` | The schema's guidance for this artifact type. Final authority |
| `template` | The structure of the file to produce |
| `language` | The language to write the prose in (ISO 639 code, e.g. `fr`) |
| `resolvedOutputPath` | Where to write. If it is a glob pattern, `instruction` says how to choose the concrete path |
| `context` | Project context — a **constraint on you**, never content to copy |
| `rules` | Rules specific to this artifact — also a constraint, never content |
| `dependencies` | The artifacts already done, to read for orientation |
| `unlocks` | What creating this one will make possible |

`context` and `rules` are arrays of blocks, each carrying its `origin`,
ordered from most general to most specific. When two blocks contradict each
other, the last one wins — and report the contradiction to the user rather
than settling it silently.

If `skipped` is present, this artifact must **not** be created: move on to
the next one.

**b. Read the dependencies from disk**, even if you already saw them in the
conversation — the user may have edited the files in the meantime.

**c. Study the project before writing.** Read `context` and `rules`, then
inspect the relevant implementation, nearby tests, configuration and
documentation outside `_codev/`. Stay read-only, and keep it proportionate to
the change.

- Ground the scope, the approach and the tasks in what you find.
- Distinguish observed behavior, your assumptions, and what you propose to
  add.
- Report contradictions with existing specs instead of deciding alone which
  one is right.
- Do this discovery now. Do not leave generic tasks such as "explore the
  code" or "draw up a plan" for the implementation phase.

**d. Write the file**, using `template` as its structure. Then check that it
exists at the expected location.

Write every sentence in `language`, whatever language the conversation is
in: the file is read and approved by the team, and `language` is the team's
choice. Structural keywords stay exactly as `template` gives them, in
English — headings such as `## Why` or `### Requirement:`, delta sections
such as `## ADDED Requirements`, and `**WHEN**` / `**THEN**` / `SHALL` /
`MUST`. codev parses them; a translated keyword is a broken artifact.

**Special case: the change's first artifact when a ticket was detected
in step 0.** Right after `# Proposal: <title>`, before `## Why`, insert a
citation line:

```
# Proposal: <title>

> Source: ticket **<ID>** — "<title>" (<status>)

## Why
[…]
```

Without a connected MCP, the line becomes:

```
> Source: ticket **<ID>** — content not retrieved
```

And for multiple tickets, add a second line right after:

```
> other ticket(s) mentioned: <ID2>, <ID3>
```

**e. Announce briefly**: "Created: `<artifact-id>`".

### 5. Loop until the required set is complete

After each creation, run `codev status --change "<name>" --json` again.

The required set is `applyRequires` **plus every artifact reachable from
those identifiers by following the `requires` edges**, transitively. With the
`spec-driven` schema, this closes over `proposal`, `specs`, `design`, `tasks`.

Two pitfalls to know:

- `status` only looks at whether files exist. An `applyRequires` artifact
  marked `done` does **not** guarantee that its dependencies exist: writing
  `tasks.md` first marks `tasks` as done even though `specs` was never
  written. Build the required set from the `requires` edges, not from the
  statuses.
- Dependencies are enablers, not gates. If a required artifact stays
  `blocked` only because you skipped a conditional dependency, write it
  anyway.

Skip an artifact in only two cases: its `status` is already `skipped`, or its
own `instruction` declares it conditional (the `spec-driven` `design.md` is
one of these). Tell the user, and do not come back to it.

If an artifact needs a decision from the user, ask for it, then resume.

### 6. Check traceability

Re-read every artifact from disk and check that:

- every capability in the proposal has its spec file, and every spec file
  is listed in the proposal;
- every requirement has a nominal scenario and an error or edge-case
  scenario, or states why none applies;
- every scenario is covered by a task whose verification refers to it;
- every task's verification is concrete: a test, a command, an observable
  behavior.

Fix the gaps directly in the artifacts.

### 7. Challenge the plan

Re-read the artifacts as a skeptical reviewer who did not write them:

1. **Need** — does the change solve the Why? Is there a simpler solution,
   or one without code?
2. **Scope** — anything added that nobody asked for? An obvious case
   missing?
3. **Specs** — error and edge cases, not only the happy path? Is each
   requirement verifiable?
4. **Decisions** — does the design silently contradict a decision in
   effect?
5. **Assumptions** — which one, if wrong, would bring the plan down?
6. **Tasks** — can each one be verified?

Apply these lenses only when the proposal's Impact touches them: security
& privacy, compatibility & migration, operability (logs, rollback),
performance, accessibility & UX. Also apply the `rules` you received
through `codev instructions`.

Fix what is unambiguous directly. Record what needs a human decision in the
Open Questions of `design.md`, or in the proposal if there is no design.
Ask the user only if a finding changes the scope materially.

### 8. Show the final status

```bash
codev status --change "<name>"
```

## Output

Summarize:

- the change name and its location;
- the artifacts created, one line each, plus any conditional artifact skipped
  and why;
- "The artifacts needed for implementation are ready.";
- "Review them. When you are ready, ask me to apply this change."
- **Points to challenge** — from step 7, at most five items ranked by
  impact, one line each: what, why it matters, which artifact. If nothing
  is worth raising, write "No point to challenge found"; never invent
  points.

## Guardrails

- The request that triggered this workflow authorizes planning only. Any
  implementation instruction it contained does not carry over here.
- Create every artifact the implementation phase transitively depends on,
  not only those listed in `applyRequires`.
- Always re-read the dependencies from disk before creating an artifact.
- `context` and `rules` are never copied into the files produced.
- Prose follows `language`; structural keywords always stay in English.
- If a change with this name already exists, ask the user whether they want
  to continue it or create another one.
- Check that each written file exists before moving on to the next.
- The contrarian pass corrects or records; it never adds scope.
- Points to challenge are capped at five.
- **Jira MCP — strictly read-only.** Never call any Jira MCP
  tool other than
  `{{JIRA_MCP_TOOL}}`, never twice in the
  same invocation, never to write (`create`, `transition`,
  `addComment`…). One detected ticket = one `getJiraIssue`, full stop.
