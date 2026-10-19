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

## Roles

Some steps belong to a role. At each one, run:

```bash
codev roles --at <step> --change "<name>" --json
```

Without `--change` for `plan.understand`: the change does not exist yet.
Apply every criterion of every role returned (`roles[].doneWhen`), the
project's included, using the role's `knowledge` when it has one. Its
`By stack` part comes already filtered to the project's languages
(`stacks`): use it as served. If `roles` is empty, the role is disabled by
configuration: skip the step, and say so in the summary.

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

### 1. Understand the request — `plan.understand`

If nothing clear is provided, ask an open question, without offering a list
of choices:

> What change do you want to make? Describe what you want to build or fix.

Apply the analyst's criteria. Derive a kebab-case name from the description
("add user authentication" → `add-user-auth`).

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

{{fragment:version-check}}

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
| `role` | The role that owns this artifact (`plan.write`): check every `doneWhen` criterion before moving on. `null` when the schema declares none |

`context` and `rules` are arrays of blocks, each carrying its `origin`,
ordered from most general to most specific. When two blocks contradict each
other, the last one wins — and report the contradiction to the user rather
than settling it silently.

If `skipped` is present, this artifact must **not** be created: move on to
the next one.

**b. Read the dependencies from disk**, even if you already saw them in the
conversation — the user may have edited the files in the meantime.

**c. Study the project before writing — `plan.investigate`.** Ask for the
roles of this step once, before the first artifact, and apply the
investigator's criteria before writing each one.

Once the proposal is written, the instructions list the existing
requirements the change likely touches (`related`). Read each one before
writing the delta specs and the design, and modify, in a `MODIFIED` block
copied whole from its spec, every one the change alters, rather than add a
requirement beside it. The list is a starting point ranked by relevance,
not a complete one: a requirement it does not name may still be
concerned. With no listed requirement, work from the request and the code.

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

**Special case: the proposal's `## Domains`.** Before writing it, run
`codev roles --change "<name>" --json` and read `diff` and `suggestions`:
each suggestion is a domain a trigger of the change's diff points to —
a path, a dependency manifest or an added line, listed in its `matches`.
For each one, either tick the domain, or, when the match does not mean
the change touches it, keep it unticked with the reason on its line:
`- [ ] <domain> — <reason>`. Never leave a suggestion `unticked`
silently. When `diff.available` is `false`, or the diff holds only
planning files, say there is no diff to go by and tick by judgement
alone; the review, once the code exists, checks again.

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

### 6. Check traceability — `plan.trace`

Re-read every artifact from disk and check it against the tracer's
criteria. Fix the gaps directly in the artifacts.

When `review.mode` is `isolated`, the tracer checks alone: run it as a
sub-agent as step 7's **Review mode** describes — the brief, the
summaries, the retry and the fallback (steps i to iv) — with the
artifacts as material. Then fix each gap it returns in the artifacts
yourself. This step writes no review record: name a fallback, if any, in
the summary.

### 7. Challenge the plan — `plan.challenge`

Re-read the artifacts as reviewers who did not write them: every role
returned — the skeptic, and the experts the proposal's `## Domains`
activates — each along its criteria and knowledge. An expert asks whether
the design addresses its domain. Here `<step>` is `plan.challenge`.

The skeptic also reads `suggestions` from the roles of this step: every
suggestion whose `state` is `unticked` is a finding located at
`proposal.md#Domains`, and every `justified` one is checked — does its
reason answer the files and lines in its `matches`?

{{fragment:review-record}}

Ask the user only if a finding changes the scope materially.

### 8. Show the final status

```bash
codev status --change "<name>"
```

{{fragment:memory-capture}}

## Output

Summarize:

- the change name and its location;
- the artifacts created, one line each, plus any conditional artifact skipped
  and why;
- each step skipped because its role is disabled by configuration;
- "The artifacts needed for implementation are ready.";
- **Points to challenge** — from the record's `plan.challenge` findings
  reported to the human and not rejected, at most five items in total
  across the roles, ranked by severity, one line each, tagged with the role
  id: `[<role-id>] <finding id> (<status>) what — why it matters — which
  artifact`. If nothing is worth raising, write
  "No point to challenge found"; never invent points. When no role is
  active at step 7, write "The skeptic is disabled by configuration"
  instead;
- **To decide** — from step 7: what only the user can decide, or
  "Nothing to decide";
- "Review them. When you are ready, ask me to apply this change."

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
- Points to challenge are capped at five, across all roles.
- Every finding is in `review.md` before it is fixed or reported; an
  accepted risk or the deferral of a high finding is recorded only once
  the user decided it.
- **Jira MCP — strictly read-only.** Never call any Jira MCP
  tool other than
  `{{JIRA_MCP_TOOL}}`, never twice in the
  same invocation, never to write (`create`, `transition`,
  `addComment`…). One detected ticket = one `getJiraIssue`, full stop.
