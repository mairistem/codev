# Review record: forward-compatible-config

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 Four levels of nesting in `unknown_keys`

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:249-267"
evidence: |
          } else if key == "mcp"
              && let serde_norway::Value::Mapping(mcp) = value
          {
              for nested in mcp.keys().filter_map(|k| k.as_str()) {
                  if !MCP_KEYS.contains(&nested) {
recommendation: Extract the scan of the `mcp` mapping into its own function (`unknown_mcp_keys`), so `unknown_keys` reads as one loop over the top level that delegates the nested section.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs: unknown_mcp_keys extracted"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 Unexplained block boundary in `mcp_line`

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:300-302"
evidence: |
          .take_while(|(_, line)| {
              is_indented(line) || line.trim().is_empty() || line.trim_start().starts_with('#')
          })
recommendation: Say why a blank or comment line does not end the `mcp:` block (YAML lets both sit at column 0 inside it), with a named helper or a comment, so a reader does not take the predicate for a bug.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs: stays_in_block, documented"
  - status: fixed
    by: newcomer
```
