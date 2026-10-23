# Review record: forward-compatible-config

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `unknown_keys` nests the `mcp` scan four levels deep

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:247-265"
evidence: |
        } else if name == "mcp"
            && let Value::Mapping(mcp) = value
        {
            let mcp_line = top_level_line(raw, "mcp");
            for key in mcp.keys() {
recommendation: Extract the `mcp` branch into its own function (`unknown_mcp_keys`), so that `unknown_keys` reads as one loop over the top level and the `mcp` scan, with its flow-mapping fallback, is named.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs — the mcp scan moved to `unknown_mcp_keys`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The `full` closure in `downgrade_target` does not say what it tests

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/update.rs:117"
evidence: |
    let full = |c: &&Comparator| c.minor.is_some() && c.patch.is_some();
recommendation: Name it after what it checks — `written_in_full` — so that `.filter(written_in_full)` reads as "an upper bound written in full", the case the doc comment describes.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/update.rs — closure renamed `written_in_full`"
  - status: fixed
    by: newcomer
```
