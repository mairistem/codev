# Review record: forward-compatible-config

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 Unknown-key detection nests four levels deep

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:242-259"
evidence: |
        } else if key == "mcp"
            && let serde_norway::Value::Mapping(mcp) = value
        {
            for nested in mcp.keys().filter_map(|k| k.as_str()) {
                if !MCP_KEYS.contains(&nested) {
recommendation: Extract the `mcp` branch into a function returning the section's unknown keys, so `unknown_keys` reads as one loop over the top level plus one call.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs — the `mcp` branch moved to `unknown_mcp_keys`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The end of the `mcp` block is an unexplained condition

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:293-296"
evidence: |
        .take_while(|(_, line)| {
            let content = line.trim_start();
            content.is_empty() || content.starts_with('#') || indentation(line) > 0
        })
recommendation: Say in a comment that the section's block runs until the next top-level line, blank lines and comments included, so the reader does not have to reverse-engineer the condition.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs — comment on where the section block ends in `nested_line`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 The text-scan comment sits on the wrong helper

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:278-281"
evidence: |
  /// A text scan, only to point at the key: a miss falls back to line 1.
  fn line_or_first(line: Option<usize>) -> u32 {
recommendation: Document `line_or_first` for what it does (an absent or oversized line becomes line 1) and move the "text scan" explanation to `top_level_line`, which performs the scan and has no comment.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs — `line_or_first` and `top_level_line` documented for what each does"
  - status: fixed
    by: newcomer
```
