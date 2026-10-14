//! What a run observed for a case (`observed.json`), and the run's own
//! metadata (`run.json`). Both carry a format marker: a newer file is
//! refused by name rather than misread.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The format of `observed.json`, `run.json` and `report.json`. Format 1
/// is still read: it lacks the consumption and the repetition count, which
/// are then absent.
pub const FORMAT: u32 = 2;

/// One case's observation, written by `run`, read by `score`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observed {
    pub format: u32,
    pub case: String,
    /// The reason the run stopped, if it did.
    pub error: Option<String>,
    pub cost_usd: Option<f64>,
    pub duration_ms: Option<u64>,
    /// What the run consumed beyond its cost and duration; `None` when
    /// Claude Code printed no result, or in format 1.
    #[serde(default)]
    pub consumption: Option<Consumption>,
    /// Files the run modified outside what its step allows.
    #[serde(default)]
    pub strayed: Vec<String>,
    /// `codev review --json` as codev printed it; `null` when not run.
    pub review: Option<serde_json::Value>,
    /// The checked files after the run; `null` for a file that is gone.
    #[serde(default)]
    pub files: BTreeMap<String, Option<String>>,
}

/// What Claude Code reports a run consumed, besides its cost and duration.
/// Each measure is `None` when Claude Code did not print it: absent, never
/// zero, so that a missing figure never reads as a free run.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consumption {
    pub duration_api_ms: Option<u64>,
    pub turns: Option<u64>,
    pub permission_denials: Option<u64>,
    /// The models the run used, sorted: the model asked for, and the small
    /// one Claude Code calls for housekeeping.
    #[serde(default)]
    pub models: Vec<String>,
    pub tokens: Option<Tokens>,
}

/// Tokens summed over every model the run used.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
    /// Input tokens read without the cache.
    pub input: u64,
    pub cache_creation: u64,
    pub cache_read: u64,
    pub output: u64,
}

/// The run's metadata, written once when a run starts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInfo {
    pub format: u32,
    /// The model asked for; `null` for Claude Code's default.
    pub model: Option<String>,
    pub date: String,
    pub claude_version: Option<String>,
    pub codev_version: Option<String>,
    /// The repetitions asked for; `None` in format 1, which ran each case
    /// once.
    #[serde(default)]
    pub repeat: Option<u32>,
    /// The runs made at the same time, which decides how much of the
    /// prompt cache they share; `None` when not recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jobs: Option<u32>,
}

/// Reads a JSON file carrying a format marker, refusing a newer one.
pub(crate) fn check_format(text: &str, what: &str) -> Result<serde_json::Value, String> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("{what}: not valid JSON ({e})"))?;
    let format = value.get("format").and_then(|f| f.as_u64()).unwrap_or(0);
    if format == 0 {
        return Err(format!("{what}: no `format` marker"));
    }
    if format > u64::from(FORMAT) {
        return Err(format!(
            "{what}: format {format} is newer than this runner reads ({FORMAT}); update codev-evals"
        ));
    }
    Ok(value)
}

/// Reads `observed.json` of the case `case`.
pub fn parse_observed(text: &str, case: &str) -> Result<Observed, String> {
    let what = format!("case `{case}`, observed.json");
    let value = check_format(text, &what)?;
    serde_json::from_value(value).map_err(|e| format!("{what}: {e}"))
}

/// Reads `run.json`.
pub fn parse_run_info(text: &str) -> Result<RunInfo, String> {
    let value = check_format(text, "run.json")?;
    serde_json::from_value(value).map_err(|e| format!("run.json: {e}"))
}

/// The parts of codev's `ReviewV1` the scoring reads. Lenient: the contract
/// is additive, unknown fields are ignored.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    #[serde(default)]
    pub exists: bool,
    #[serde(default)]
    pub steps: Vec<ReviewStep>,
    #[serde(default)]
    pub findings: Vec<ReviewFinding>,
    #[serde(default)]
    pub issues: Vec<ReviewIssue>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewStep {
    pub step: String,
    #[serde(default)]
    pub roles: Vec<ReviewRole>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewRole {
    pub role: String,
    pub state: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewFinding {
    pub id: String,
    #[serde(default)]
    pub title: String,
    pub role: String,
    pub step: String,
    #[serde(default)]
    pub severity: String,
    #[serde(default)]
    pub confidence: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub recommendation: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewIssue {
    #[serde(default)]
    pub severity: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
}

impl Observed {
    /// The review document, read leniently; `None` when absent or not the
    /// expected shape.
    pub fn review(&self) -> Option<Review> {
        self.review
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_newer_observation_is_refused_with_its_case() {
        let err = parse_observed(r#"{"format": 3, "case": "x"}"#, "ops-code-timeout").unwrap_err();
        assert!(
            err.contains("ops-code-timeout") && err.contains("format 3"),
            "{err}"
        );
    }

    #[test]
    fn unknown_review_fields_are_ignored() {
        let text = r#"{"format": 1, "case": "c", "error": null, "costUsd": 0.1,
            "durationMs": 5, "review": {"exists": true, "future": [1],
            "steps": [{"step": "code.review", "recorded": true,
                       "roles": [{"role": "security", "state": "findings", "new": 1}]}],
            "findings": [{"id": "R-security-1", "role": "security", "step": "code.review",
                          "severity": "high", "confidence": "high", "location": "a.ts:1",
                          "status": "open", "awaiting": "author", "extra": {}}],
            "issues": []}}"#;
        let observed = parse_observed(text, "c").unwrap();
        let review = observed.review().unwrap();
        assert_eq!(review.findings[0].id, "R-security-1");
        assert_eq!(review.steps[0].roles[0].state, "findings");
    }

    #[test]
    fn a_format_1_observation_reads_without_consumption() {
        let text = r#"{"format": 1, "case": "c", "error": null, "costUsd": 0.35,
            "durationMs": 28004, "strayed": [], "review": null}"#;
        let observed = parse_observed(text, "c").unwrap();
        assert_eq!(observed.cost_usd, Some(0.35));
        assert_eq!(observed.consumption, None);
        let info = parse_run_info(
            r#"{"format": 1, "model": null, "date": "2026-10-13",
            "claudeVersion": null, "codevVersion": null}"#,
        )
        .unwrap();
        assert_eq!(info.repeat, None);
    }

    #[test]
    fn run_info_round_trips() {
        let info = RunInfo {
            format: FORMAT,
            model: None,
            date: "2026-10-13".into(),
            claude_version: Some("2.1.289".into()),
            codev_version: Some("0.6.1".into()),
            repeat: Some(5),
            jobs: None,
        };
        let text = serde_json::to_string(&info).unwrap();
        assert_eq!(parse_run_info(&text).unwrap(), info);
        assert!(parse_run_info("{}").unwrap_err().contains("format"));
    }
}
