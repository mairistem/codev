//! Seal of an ADR body.
//!
//! An `accepted` decision is meant to be immutable: hashing its **body**
//! (everything after the frontmatter) when it is accepted, then comparing
//! later, makes it possible to mechanically detect an in-place edit. The
//! frontmatter, on the other hand, may legitimately evolve (an `accepted`
//! → `superseded` transition, for example) — which is why the hash does
//! not cover the whole file.
//!
//! This module is pure (no I/O) — see decision
//! `_codev/decisions/0001-functional-core-imperative-shell.md`. YAML
//! parsing and rendering go through `serde_norway`, in accordance with
//! decision `_codev/decisions/0006-serde-norway-for-yaml.md`.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// The schema version of the `seal.yaml` file. A file read with a
/// different version is rejected — an older consumer cannot guess what a
/// newer version adds.
pub const SEAL_FILE_VERSION: u32 = 1;

/// Errors produced by the functions of this module.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SealError {
    /// The ADR file has no `---` separator closing its frontmatter: the
    /// body hash cannot be computed.
    #[error("closing frontmatter delimiter not found — the body hash cannot be computed")]
    MissingFrontmatterCloser,

    /// The `seal.yaml` file has a `version` this binary does not know.
    /// Stable code for the agent: `seal_version_unsupported`.
    #[error("unsupported seal.yaml version `{actual}` (expected: {expected})")]
    UnsupportedVersion { actual: u32, expected: u32 },

    /// The YAML of `seal.yaml` cannot be parsed. The raw `serde_norway`
    /// message is kept to help debugging.
    #[error("unreadable seal.yaml: {0}")]
    Invalid(String),

    /// `plan_seal_new` rejects an already sealed id — this path is for
    /// migration or the initial sealing; re-approving a seal requires
    /// `plan_seal_force`.
    #[error("id `{id}` is already sealed")]
    AlreadySealed { id: String },

    /// `plan_seal_force` rejects an id missing from the seal — force only
    /// makes sense when an existing entry must be replaced.
    #[error("id `{id}` has no existing seal to rewrite")]
    Unknown { id: String },
}

/// An entry of the seal file.
///
/// Serialized names are camelCase to stay consistent with the JSON
/// contract exposed by the CLI, even though the file is YAML.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Seal {
    /// Local identifier of the decision — the same `id` as in the ADR
    /// frontmatter.
    pub id: String,
    /// SHA-256 hash of the body, prefixed with `sha256:` — see `body_hash`.
    pub body_sha256: String,
    /// Date on which the seal was applied, `YYYY-MM-DD`.
    pub sealed_at: String,
}

/// The `seal.yaml` file as a whole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SealFile {
    pub version: u32,
    pub seals: Vec<Seal>,
}

impl SealFile {
    /// An empty seal — used when the file does not exist yet.
    pub fn empty() -> Self {
        Self {
            version: SEAL_FILE_VERSION,
            seals: Vec::new(),
        }
    }

    /// Looks up the entry for an id — linear scan, the list is small (one
    /// entry per local ADR, a few dozen at most).
    pub fn find(&self, id: &str) -> Option<&Seal> {
        self.seals.iter().find(|s| s.id == id)
    }
}

/// Computes the SHA-256 hash of an ADR **body** — that is, everything
/// after the `---` separator closing the frontmatter, byte for byte,
/// without normalization.
///
/// Returns: a string prefixed with `sha256:`, lowercase hex.
///
/// Error: `MissingFrontmatterCloser` if the file has no closing
/// separator. The "no frontmatter at all" case is considered a non-ADR
/// file — it is up to the ADR parser (`parse_decision`) to reject it,
/// not here.
pub fn body_hash(adr_source: &str) -> Result<String, SealError> {
    let body = body_slice(adr_source)?;
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    let digest = hasher.finalize();
    Ok(format!("sha256:{:x}", digest))
}

/// Extracts the ADR body: the substring starting right after the
/// `\n---\n` (or `\n---\r\n`) closing the frontmatter.
///
/// Kept separate from the hash logic — useful for a future consumer that
/// would want to display a diff rather than a match/mismatch boolean.
pub fn body_slice(adr_source: &str) -> Result<&str, SealError> {
    // An ADR starts with `---\n` (or `---\r\n`); skip that line.
    let after_opener_idx = if let Some(rest) = adr_source.strip_prefix("---\n") {
        adr_source.len() - rest.len()
    } else if let Some(rest) = adr_source.strip_prefix("---\r\n") {
        adr_source.len() - rest.len()
    } else {
        // No frontmatter at all: by convention, "everything is body"; but
        // the ADR parser will already have rejected this case, so in
        // practice we never get here.
        return Err(SealError::MissingFrontmatterCloser);
    };

    // Look for the next `\n---\n` or `\n---\r\n`.
    let rest = &adr_source[after_opener_idx..];
    if let Some(pos) = rest.find("\n---\n") {
        let end = after_opener_idx + pos + "\n---\n".len();
        Ok(&adr_source[end..])
    } else if let Some(pos) = rest.find("\n---\r\n") {
        let end = after_opener_idx + pos + "\n---\r\n".len();
        Ok(&adr_source[end..])
    } else {
        Err(SealError::MissingFrontmatterCloser)
    }
}

/// Parses a `seal.yaml` file. On an empty source, returns an empty seal
/// (the initial state of a project).
pub fn parse_seal_file(source: &str) -> Result<SealFile, SealError> {
    if source.trim().is_empty() {
        return Ok(SealFile::empty());
    }
    let parsed: SealFile =
        serde_norway::from_str(source).map_err(|e| SealError::Invalid(e.to_string()))?;
    if parsed.version != SEAL_FILE_VERSION {
        return Err(SealError::UnsupportedVersion {
            actual: parsed.version,
            expected: SEAL_FILE_VERSION,
        });
    }
    Ok(parsed)
}

/// Renders a `SealFile` as canonical YAML, with entries sorted by
/// ascending `id`. Determinism is essential so that `git diff` doesn't
/// churn when nothing has changed.
pub fn render_seal_file(seal: &SealFile) -> String {
    let mut sorted = seal.clone();
    sorted.seals.sort_by(|a, b| a.id.cmp(&b.id));
    // Prefix the header with a short comment — the CLI writes this file,
    // and humans need to know they are not supposed to edit it by hand.
    let body = serde_norway::to_string(&sorted).unwrap_or_default();
    format!(
        "# File maintained by codev — see `codev decision seal`.\n\
         # The hash of an ADR body is recorded here when it is accepted;\n\
         # `codev validate` compares this hash with the current body.\n\
         {body}"
    )
}

/// Prepares the addition of a seal entry to an existing file.
///
/// Rejects if the id is already sealed — this path is for the first
/// sealing (ADR creation or migration). To rewrite an existing seal, use
/// `plan_seal_force`.
pub fn plan_seal_new(
    existing: &SealFile,
    id: String,
    body_hash: String,
    today: String,
) -> Result<SealFile, SealError> {
    if existing.find(&id).is_some() {
        return Err(SealError::AlreadySealed { id });
    }
    let mut next = existing.clone();
    next.seals.push(Seal {
        id,
        body_sha256: body_hash,
        sealed_at: today,
    });
    // Stable sort by id for a clean git diff.
    next.seals.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(next)
}

/// Prepares the replacement of an existing seal entry.
///
/// Rejects if the id has no seal — force makes no sense on an empty
/// file; use `plan_seal_new` instead.
pub fn plan_seal_force(
    existing: &SealFile,
    id: String,
    body_hash: String,
    today: String,
) -> Result<SealFile, SealError> {
    if existing.find(&id).is_none() {
        return Err(SealError::Unknown { id });
    }
    let mut next = existing.clone();
    for s in &mut next.seals {
        if s.id == id {
            s.body_sha256 = body_hash.clone();
            s.sealed_at = today.clone();
        }
    }
    Ok(next)
}

/// The result of a verification.
///
/// Parser `Finding`s are not returned directly — the `seal` module stays
/// pure and independent of `codev-core::parser`. The shell (validate on
/// the engine side) translates these cases into `Finding`s with the
/// right stable codes and severities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationCase {
    /// A local ADR with no seal entry — a warning in validate, migration
    /// expected.
    Unsealed { id: String },
    /// An ADR whose body hash no longer matches the seal — an error in
    /// validate.
    Mismatch {
        id: String,
        recorded: String,
        actual: String,
    },
    /// A seal entry that references a nonexistent ADR — warning.
    OrphanSeal { id: String },
}

/// Checks the consistency of a `SealFile` against the current state of
/// the directory.
///
/// `present` lists the local `accepted`/`superseded` ADRs — the other
/// statuses (proposed, deprecated, rejected) are not sealed (they carry
/// no immutability commitment). `body_hashes` provides, for each present
/// `id`, the hash of its current body.
///
/// Cases are returned in a stable order so that the validate report
/// stays deterministic.
pub fn verify(seal: &SealFile, present: &[(String, String)]) -> Vec<VerificationCase> {
    use std::collections::HashSet;

    let mut cases = Vec::new();
    let ids_present: HashSet<&str> = present.iter().map(|(id, _)| id.as_str()).collect();

    // 1) each present ADR: sealed? matching?
    let mut present_sorted = present.to_vec();
    present_sorted.sort_by(|a, b| a.0.cmp(&b.0));
    for (id, actual_hash) in &present_sorted {
        match seal.find(id) {
            None => cases.push(VerificationCase::Unsealed { id: id.clone() }),
            Some(entry) => {
                if entry.body_sha256 != *actual_hash {
                    cases.push(VerificationCase::Mismatch {
                        id: id.clone(),
                        recorded: entry.body_sha256.clone(),
                        actual: actual_hash.clone(),
                    });
                }
            }
        }
    }

    // 2) each seal: does its ADR still exist?
    let mut orphans: Vec<&Seal> = seal
        .seals
        .iter()
        .filter(|s| !ids_present.contains(s.id.as_str()))
        .collect();
    orphans.sort_by(|a, b| a.id.cmp(&b.id));
    for s in orphans {
        cases.push(VerificationCase::OrphanSeal { id: s.id.clone() });
    }

    cases
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─────────────── body_hash ───────────────

    #[test]
    fn hash_of_a_standard_body() {
        let adr = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Context\n\nx\n";
        let hash = body_hash(adr).unwrap();
        assert!(hash.starts_with("sha256:"));
        // Checks reproducibility (same input → same hash).
        assert_eq!(body_hash(adr).unwrap(), hash);
    }

    #[test]
    fn hash_changes_when_the_body_changes() {
        let adr_a = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Context\n\nx\n";
        let adr_b = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Context\n\ny\n";
        assert_ne!(body_hash(adr_a).unwrap(), body_hash(adr_b).unwrap());
    }

    #[test]
    fn hash_ignores_the_frontmatter() {
        // Two ADRs with the same body but a different frontmatter → same
        // hash. That is the whole point of separating them.
        let adr_a = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Context\n\nIdentical body.\n";
        let adr_b = "---\nid: \"0001\"\ntitle: T\nstatus: superseded\ndate: 2026-09-09\n---\n\n## Context\n\nIdentical body.\n";
        assert_eq!(body_hash(adr_a).unwrap(), body_hash(adr_b).unwrap());
    }

    #[test]
    fn hash_with_crlf_is_supported() {
        let adr = "---\r\nid: \"0001\"\r\ntitle: T\r\nstatus: accepted\r\ndate: 2026-09-09\r\n---\r\n\r\n## Context\r\n\r\nx\r\n";
        // Does not panic; the hash differs from a pure LF one (byte for
        // byte, without normalization) — this is documented as a risk.
        let h = body_hash(adr).unwrap();
        assert!(h.starts_with("sha256:"));
    }

    #[test]
    fn hash_of_empty_body_is_valid() {
        // The sha256 hash of the empty string is well known.
        let adr = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n";
        let h = body_hash(adr).unwrap();
        assert_eq!(
            h,
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn hash_errors_without_closing_frontmatter() {
        let adr = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\n";
        let err = body_hash(adr).unwrap_err();
        assert!(matches!(err, SealError::MissingFrontmatterCloser));
    }

    // ─────────────── parse / render ───────────────

    #[test]
    fn parse_empty_source_gives_empty_seal() {
        let seal = parse_seal_file("").unwrap();
        assert_eq!(seal.version, 1);
        assert!(seal.seals.is_empty());
    }

    #[test]
    fn parse_canonical_form() {
        let source = "version: 1\nseals:\n  - id: \"0001\"\n    bodySha256: \"sha256:abc\"\n    sealedAt: 2026-09-09\n";
        let seal = parse_seal_file(source).unwrap();
        assert_eq!(seal.seals.len(), 1);
        assert_eq!(seal.seals[0].id, "0001");
        assert_eq!(seal.seals[0].body_sha256, "sha256:abc");
        assert_eq!(seal.seals[0].sealed_at, "2026-09-09");
    }

    #[test]
    fn parse_unknown_field_is_rejected() {
        // deny_unknown_fields: an undeclared field makes the parse fail.
        let source = "version: 1\nseals:\n  - id: \"0001\"\n    bodySha256: \"sha256:abc\"\n    sealedAt: 2026-09-09\n    extra: wrong\n";
        let err = parse_seal_file(source).unwrap_err();
        assert!(matches!(err, SealError::Invalid(_)));
    }

    #[test]
    fn parse_different_version_is_rejected() {
        let source = "version: 999\nseals: []\n";
        let err = parse_seal_file(source).unwrap_err();
        assert!(matches!(
            err,
            SealError::UnsupportedVersion {
                actual: 999,
                expected: 1
            }
        ));
    }

    #[test]
    fn render_is_deterministic_even_when_unordered() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0003".into(),
            body_sha256: "sha256:c".into(),
            sealed_at: "2026-09-09".into(),
        });
        seal.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:a".into(),
            sealed_at: "2026-09-09".into(),
        });
        let rendered = render_seal_file(&seal);
        // Sorting puts 0001 before 0003. serde_norway may render the id
        // with or without quotes; we only look for the matching sha,
        // which is distinctive.
        let idx_a = rendered.find("sha256:a").unwrap();
        let idx_c = rendered.find("sha256:c").unwrap();
        assert!(
            idx_a < idx_c,
            "0001 (sha256:a) must appear before 0003 (sha256:c):\n{rendered}"
        );
    }

    #[test]
    fn round_trip_parse_render_parse() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:aaa".into(),
            sealed_at: "2026-09-09".into(),
        });
        let rendered = render_seal_file(&seal);
        let reparsed = parse_seal_file(&rendered).unwrap();
        assert_eq!(reparsed.seals, seal.seals);
        assert_eq!(reparsed.version, seal.version);
    }

    // ─────────────── plan_seal_new / plan_seal_force ───────────────

    #[test]
    fn plan_seal_new_inserts_then_sorts() {
        let mut existing = SealFile::empty();
        existing.seals.push(Seal {
            id: "0002".into(),
            body_sha256: "sha256:b".into(),
            sealed_at: "2026-09-09".into(),
        });
        let next = plan_seal_new(
            &existing,
            "0001".into(),
            "sha256:a".into(),
            "2026-09-09".into(),
        )
        .unwrap();
        assert_eq!(next.seals.len(), 2);
        assert_eq!(next.seals[0].id, "0001");
        assert_eq!(next.seals[1].id, "0002");
    }

    #[test]
    fn plan_seal_new_rejects_already_sealed_id() {
        let mut existing = SealFile::empty();
        existing.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:a".into(),
            sealed_at: "2026-09-09".into(),
        });
        let err = plan_seal_new(
            &existing,
            "0001".into(),
            "sha256:different".into(),
            "2026-09-10".into(),
        )
        .unwrap_err();
        assert!(matches!(err, SealError::AlreadySealed { .. }));
    }

    #[test]
    fn plan_seal_force_replaces_the_existing_entry() {
        let mut existing = SealFile::empty();
        existing.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:old".into(),
            sealed_at: "2026-09-08".into(),
        });
        let next = plan_seal_force(
            &existing,
            "0001".into(),
            "sha256:new".into(),
            "2026-09-09".into(),
        )
        .unwrap();
        assert_eq!(next.seals.len(), 1);
        assert_eq!(next.seals[0].body_sha256, "sha256:new");
        assert_eq!(next.seals[0].sealed_at, "2026-09-09");
    }

    #[test]
    fn plan_seal_force_rejects_missing_id() {
        let existing = SealFile::empty();
        let err = plan_seal_force(
            &existing,
            "9999".into(),
            "sha256:x".into(),
            "2026-09-09".into(),
        )
        .unwrap_err();
        assert!(matches!(err, SealError::Unknown { .. }));
    }

    // ─────────────── verify ───────────────

    #[test]
    fn verify_reports_unsealed_adrs() {
        let seal = SealFile::empty();
        let cases = verify(
            &seal,
            &[
                ("0001".into(), "sha256:a".into()),
                ("0002".into(), "sha256:b".into()),
            ],
        );
        assert_eq!(cases.len(), 2);
        assert!(matches!(&cases[0], VerificationCase::Unsealed { id } if id == "0001"));
        assert!(matches!(&cases[1], VerificationCase::Unsealed { id } if id == "0002"));
    }

    #[test]
    fn verify_reports_a_mismatch() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:recorded".into(),
            sealed_at: "2026-09-09".into(),
        });
        let cases = verify(&seal, &[("0001".into(), "sha256:current".into())]);
        assert_eq!(cases.len(), 1);
        match &cases[0] {
            VerificationCase::Mismatch {
                id,
                recorded,
                actual,
            } => {
                assert_eq!(id, "0001");
                assert_eq!(recorded, "sha256:recorded");
                assert_eq!(actual, "sha256:current");
            }
            _ => panic!("expected Mismatch, got {:?}", cases[0]),
        }
    }

    #[test]
    fn verify_reports_an_orphan() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0004".into(),
            body_sha256: "sha256:x".into(),
            sealed_at: "2026-09-09".into(),
        });
        let cases = verify(&seal, &[]);
        assert_eq!(cases.len(), 1);
        assert!(matches!(&cases[0], VerificationCase::OrphanSeal { id } if id == "0004"));
    }

    #[test]
    fn verify_stable_order() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0003".into(),
            body_sha256: "sha256:c".into(),
            sealed_at: "2026-09-09".into(),
        });
        seal.seals.push(Seal {
            id: "0002".into(),
            body_sha256: "sha256:b_wrong".into(),
            sealed_at: "2026-09-09".into(),
        });
        // Present: 0001 (unsealed) + 0002 (mismatch); 0003 orphaned.
        let cases = verify(
            &seal,
            &[
                ("0002".into(), "sha256:b_actual".into()),
                ("0001".into(), "sha256:a".into()),
            ],
        );
        assert_eq!(cases.len(), 3);
        // Order: Unsealed(0001), Mismatch(0002), OrphanSeal(0003).
        assert!(matches!(&cases[0], VerificationCase::Unsealed { id } if id == "0001"));
        assert!(matches!(&cases[1], VerificationCase::Mismatch { id, .. } if id == "0002"));
        assert!(matches!(&cases[2], VerificationCase::OrphanSeal { id } if id == "0003"));
    }

    #[test]
    fn find_present_or_absent() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:a".into(),
            sealed_at: "2026-09-09".into(),
        });
        assert!(seal.find("0001").is_some());
        assert!(seal.find("0002").is_none());
    }
}
