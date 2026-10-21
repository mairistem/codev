# Review record: decision-accept-command
<!-- codev-review: 1 -->

## code.review

Mode: inline.

Fallback: the installed codev (0.6.1) and the workspace build have neither
`codev roles` nor `codev review`; the step ran inline with the developer and
newcomer criteria described by the codev-apply skill. The proposal activates
no `## Domains`, so no expert ran.

### developer

#### R-developer-1 `decision new` points every unsealed status to `decision accept`

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/render.rs:461-467"
evidence: |
      } else {
          let _ = writeln!(
              out,
              "\nOpen the file to write the Context, Decision and Consequences sections,\n\
               then run `codev decision accept {}` to accept and seal it.",
recommendation: >-
  Branch on the decision's status, not on the presence of a hash: name
  `decision accept` only for `proposed`; a `rejected` or `deprecated`
  decision would be refused by `accept` with `decision_not_proposed`. Word the
  sealed branch without "accepted", since `--status superseded` is sealed too.
history:
  - status: open
    by: developer
  - status: fixed
    by: developer
    ref: "crates/codev-cli/src/render.rs — decision_created branches on the status; test decision_created_names_accept_only_for_a_proposed_decision fails without the fix"
  - status: fixed
    by: developer
```

#### R-developer-2 A leftover seal is refused with a message about `seal --force`

```yaml
severity: low
confidence: low
location: "crates/codev-engine/src/decisions_actions.rs:585-587"
evidence: |
          // A leftover entry for another body: overwriting it silently would
          // hide that the text changed after it was sealed.
          Some(_) => return Err(ActionError::SealConflict { id: local_id }),
recommendation: >-
  The reused `seal_conflict` message tells the user to run
  `codev decision seal <id> --force`, after which `accept` succeeds; that is a
  working path, but a message naming `accept` would be clearer. Only reachable
  when someone ran `decision seal` on a proposed decision and then edited it.
history:
  - status: open
    by: developer
  - status: disputed
    by: developer
    note: >-
      tasks.md 1.2 plans this refusal under the existing stable code
      (`plan_accept_refuses_a_leftover_seal_for_another_body`), and the message
      leads to a working path: after `seal --force` the entry matches the body
      and `accept` succeeds. A dedicated code would widen the contract beyond
      the spec.
  - status: rejected
    by: developer
```

### newcomer

No finding.
