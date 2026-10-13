# Review record: rename-listen-setting

<!-- codev-review: 1 -->

## code.review

### migration

#### R-migration-1 A file holding both `listen` and `bind_address` no longer loads

```yaml
severity: medium
confidence: high
location: "src/config.rs:10-11"
evidence: |
  #[serde(alias = "listen")]
  pub bind_address: String,
recommendation: Decide and document the behavior when both keys are present (serde's derive rejects it with "duplicate field `bind_address`"); either say in the release note to replace `listen` rather than add `bind_address` beside it, or read both keys explicitly and prefer `bind_address` with a warning. Cover the case with a test.
history:
  - status: open
    by: migration
```

An operator who migrates by adding `bind_address` while leaving `listen` in place — a common step when editing a config by hand or merging with configuration management — gets a start-up failure, which the release note does not warn about.

#### R-migration-2 The old-key migration path has no automated test

```yaml
severity: medium
confidence: high
location: "src/config.rs:14-16"
evidence: |
  pub fn load(text: &str) -> Result<Config, toml::de::Error> {
      toml::from_str(text)
  }
recommendation: Add unit tests calling `load` with a pre-4.2 file using `listen`, a new file using `bind_address`, and a file with both keys; and a test asserting `OrderResponse` still serializes `customer_id`.
history:
  - status: open
    by: migration
```

The only verification of the compatibility shim is the manual staging check in tasks 1.1 and 1.2 (`verified by starting the service on staging with an old and a new file`); the crate has no test module, so a later removal of the alias or of the `rename` would break existing files and partners without any test failing.

#### R-migration-3 The deprecated `listen` key is read silently, with no warning or removal plan

```yaml
severity: low
confidence: high
location: "src/config.rs:8-10"
evidence: |
  /// The address the HTTP server binds to. Files written before 4.2 call
  /// it `listen`, which is still read.
  #[serde(alias = "listen")]
recommendation: Log a deprecation warning at start-up when the file uses `listen`, and state in the release note in which version the alias will be removed.
history:
  - status: open
    by: migration
```

The release note marks `listen` as deprecated, but nothing tells an operator whose file still uses it; when the alias is eventually dropped, those files fail with a missing-field error and nobody was warned.

### newcomer

Disabled by configuration.
