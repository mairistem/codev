# Tasks

## 1. Core — `seal` module and hash

- [x] 1.1 Create `crates/codev-core/src/decisions/seal.rs` — types:
      `Seal { id: DecisionId, body_sha256: String, sealed_at: NaiveDate }`,
      `SealFile { version: u32, seals: Vec<Seal> }`. Frontmatter attributed
      `#[serde(deny_unknown_fields)]` on both. `version` fixed at `1`
      for this delivery.
- [x] 1.2 Function `body_hash(source: &str) -> Result<String,
      SealError>` — finds the `\n---\n` (or `\n---\r\n`) closing the
      frontmatter, computes SHA-256 over everything that follows **byte
      for byte**, returns the string `"sha256:<hex>"`. Typed error if the
      separator cannot be found. Tests: standard ADR, ADR with no
      content after the frontmatter (empty body), ADR with CRLF, ADR
      without frontmatter (error).
- [x] 1.3 Function `parse_seal_file(source: &str) -> Result<SealFile,
      SealError>` via `serde_norway`; returns `SealFile { version: 1,
      seals: vec![] }` on empty source. Tests: canonical form, unknown
      field rejected, version other than 1 rejected with stable code
      `seal_version_unsupported`.
- [x] 1.4 Function `render_seal_file(seal: &SealFile) -> String` —
      canonical YAML with `version:` first, `seals:` as a list, entries
      ordered by ascending `id` (determinism for git). Round-trip test
      parse→render→parse.
- [x] 1.5 Pure function `plan_seal_new(existing: &SealFile, id:
      DecisionId, body_hash: String, today: NaiveDate) -> SealFile` —
      inserts the new entry preserving order by id. Test: insertion in
      the middle, refuses an already sealed id (returns
      `Err(SealError::AlreadySealed)`).
- [x] 1.6 Pure function `plan_seal_force(existing: &SealFile, id:
      DecisionId, body_hash: String, today: NaiveDate) -> SealFile` —
      replaces the existing entry, updates `sealed_at`. Test: replaces;
      refuses an absent id (returns `Err(SealError::Unknown)`).
- [x] 1.7 Pure function `verify(seal: &SealFile, present_ids:
      &[DecisionId], body_hashes: &HashMap<DecisionId, String>) ->
      Vec<Finding>` — emits `decision_unsealed`, `decision_seal_mismatch`,
      `decision_orphan_seal` depending on the case. Tests: each case in
      isolation, combined cases, supersession chain (both ADRs of the
      chain must be sealed).

## 2. Core — integration into existing plans

- [x] 2.1 Extend `plan_new_decision`: the signature now receives
      `existing_seal: SealFile` and `today: NaiveDate` (already an
      argument). Returns a `Plan` that writes the ADR **and** the
      updated `seal.yaml`. Existing tests adapted — the plan now has 2
      writes instead of 1.
- [x] 2.2 Extend `plan_supersede`: adds a write for `seal.yaml`
      carrying the new entry for the new ADR; the old one's entry stays
      identical. Dedicated test: after `plan_supersede`, `seal.yaml`
      contains N+1 entries and the old one is bit-identical.
- [x] 2.3 New function `plan_seal(existing_adr: &Decision,
      existing_seal: &SealFile, force: bool, today: NaiveDate) ->
      Result<Plan, SealError>` — models both modes. Tests: fresh
      addition, refusal without force on change, rewrite with force,
      no-op if the seal is already correct.

## 3. Shell — engine and coordination

- [x] 3.1 `crates/codev-engine/src/decisions_actions.rs`: the function
      that builds the `new` plan must first read `seal.yaml` via the
      `FileSystem` port, call `plan_new_decision` passing it the parsed
      content, then execute the plan. Same for `supersede`.
- [x] 3.2 New action `seal(id: DecisionId, force: bool)` in the engine,
      which composes ADR read + seal read + hash computation + call to
      `plan_seal`, then execution. Errors remapped to the stable codes
      `seal_conflict`, `cannot_seal_inherited`,
      `unknown_decision_id`.
- [x] 3.3 Extension of `crates/codev-engine/src/validate.rs`: after
      loading the decision index, read `seal.yaml` via the `FileSystem`
      port, call `verify`, add its findings to the validation report.
      Tests: project where seal.yaml is absent → every ADR surfaces
      `decision_unsealed`; project where seal.yaml was filled by hand
      but an ADR was modified → `decision_seal_mismatch`.
- [x] 3.4 `_codev/decisions/seal.yaml` must be **created in the layout**
      on the engine side — list it as a known source-of-truth file,
      avoid it being mistakenly interpreted as an ADR (`.yaml` extension,
      so ignored anyway by the ADR reader which filters `.md`, but
      better to assert it with a test).

## 4. CLI — `decision seal` command

- [x] 4.1 New subcommand `codev decision seal <id> [--force]
      [--json]`. Routes to the engine action. `--json` produces
      `{ "decision": { "id": …, "bodySha256": …, "sealedAt": … },
      "status": [] }` on success, `{ "decision": null, "status": [{code,
      message}] }` on failure.
- [x] 4.2 Human rendering: fresh success → `Sealed 0001 (sha256:abcd…)`;
      re-seal → `Resealed 0001 (sha256:…)`; no-op → `Already up to date:
      0001`; conflict without force → message with the code
      `seal_conflict` recalling the use of `--force`.
- [x] 4.3 `codev decision new --json` gains a `bodySha256` field in its
      `decision` entry. The field also appears in the human output on
      its own line (`Hash: sha256:…`) — useful for copy-paste during
      migration.
- [x] 4.4 CLI integration tests: `decision seal` in three modes (fresh,
      no-op, conflict with/without force) on a test repository.

## 5. JSON contract

- [x] 5.1 Add the struct `SealEntryV1 { id, bodySha256, sealedAt }`
      in `codev-cli::contract::v1`. The struct `DecisionEntryV1` (used
      by `decision new`) gains an optional `bodySha256: Option<String>`
      — additive, never breaking.
- [x] 5.2 Document the new finding codes in the module
      `codev-cli::contract::v1::status`: `decision_unsealed`,
      `decision_seal_mismatch`, `decision_orphan_seal`, `seal_conflict`,
      `cannot_seal_inherited`, `seal_version_unsupported`,
      `unknown_decision_id` (already exists).

## 6. Migration of the repository itself

- [x] 6.1 After implementation and `cargo install`, run `codev
      validate` — check that the 6 `decision_unsealed` warnings
      surface.
- [x] 6.2 Loop `codev decision seal <id>` over the 6 ADRs; single
      commit carrying `_codev/decisions/seal.yaml`.
- [x] 6.3 Check that a post-migration `codev validate` is completely
      green on the decisions side.
- [x] 6.4 Deliberately edit the body of an ADR (for example add
      "TEST-TO-DELETE" in the context), rerun `codev validate` —
      check that `decision_seal_mismatch` is indeed emitted with a
      non-zero exit code. Remove the test edit before the commit.

## 7. Workspace integration

- [x] 7.1 `cargo test --workspace` stays green, gains at least 15 new
      tests (seal module + integrations).
- [x] 7.2 `cargo clippy --workspace --all-targets` stays free of
      warnings.
- [x] 7.3 `codev validate --all` stays green (once the migration of
      item 6 is done).
