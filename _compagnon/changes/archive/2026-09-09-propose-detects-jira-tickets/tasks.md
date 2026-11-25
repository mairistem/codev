# Tasks

## 1. Body of the `propose` skill

- [x] 1.1 Edit `assets/workflows/propose.md` — add a new section
      **Step 0: external ticket detection**, inserted **before** step
      1 "Understand the request". Expected content:
      - Explanation: the skill scans the user's prompt to detect a
        `[A-Z]{2,}-\d+` pattern.
      - "No pattern" branch: go straight to step 1, behavior
        unchanged.
      - "Pattern detected, Atlassian MCP available" branch: call
        `mcp__claude_ai_Atlassian__getJiraIssue({issueIdOrKey:
        "<ID>"})` for the first ticket; use the result to enrich the
        writing context; cite the ticket at the top of the proposal.
      - "Pattern detected, MCP absent" branch: display the
        informational message documented in the spec; continue
        without the content; the proposal cites the ticket with the
        note "content not fetched".
      - Multi-ticket case: the first is fetched; the others are
        listed under "other mentioned ticket(s)".
- [x] 1.2 Edit the "write the proposal" step (current step 4d) to
      specify: "if a ticket was detected in step 0, insert the line
      `> Source: ticket **<ID>** — …` right after
      `# Proposal: <title>`, before `## Why`".
- [x] 1.3 Add a safeguard at the end of the "Safeguards" section: the
      skill **never** writes to Jira, **never** runs a JQL search —
      only one `getJiraIssue` per invocation, on the explicitly
      mentioned identifier.

## 2. `Workflow { id: "propose" }` entry in the CATALOG

- [x] 2.1 Extend `allowed_tools` of the `propose` entry in
      `crates/codev-agents/src/workflows.rs`: change from
      `"Bash(codev:*), Read, Write, Edit, Glob, Grep"` to
      `"Bash(codev:*), Read, Write, Edit, Glob, Grep, mcp__claude_ai_Atlassian__getJiraIssue"`.
- [x] 2.2 Update the doc comment of the `propose` entry to say that
      the skill may call the Atlassian MCP when a ticket is detected,
      without it being mandatory.

## 3. Invariant tests

- [x] 3.1 New test `propose_declare_le_mcp_atlassian_getJiraIssue`
      in `crates/codev-agents/src/workflows.rs::tests`: checks that
      `find("propose").allowed_tools.contains("mcp__claude_ai_Atlassian__getJiraIssue")`.
- [x] 3.2 Complementary test
      `propose_ne_declare_pas_dautre_mcp_atlassian`: walks the
      `allowed_tools` of `propose`, checks that no other
      `mcp__claude_ai_Atlassian__` string (prefix) is present.
      Safeguard against a silent widening of the access scope.
- [x] 3.3 Test
      `propose_cite_la_detection_de_ticket_dans_son_body`: the skill
      body contains the strings `[A-Z]{2,}-\d+` (the pattern) and
      `mcp__claude_ai_Atlassian__getJiraIssue` (the call), so that an
      accidental rewrite of the body surfaces via `grep`.

## 4. Checks and dogfooding

- [x] 4.1 `cargo test --workspace` stays green, gains at least 3 new
      tests.
- [x] 4.2 `cargo clippy --workspace --all-targets` stays free of
      warnings.
- [x] 4.3 `codev validate --strict` on this repository stays green.
- [x] 4.4 After `cargo install --path crates/codev-cli` then
      `codev update --force`, check that
      `.claude/skills/codev-propose/SKILL.md` does carry
      `mcp__claude_ai_Atlassian__getJiraIssue` in its
      `allowed-tools` frontmatter.
- [x] 4.5 Manual check (in this Claude Code session where the
      Atlassian MCP is available): simulate the invocation by
      rereading the skill body and confirming that the described flow
      works — it does cite an ID, does call the MCP, does inject the
      content.
