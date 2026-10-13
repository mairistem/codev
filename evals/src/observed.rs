//! What a run observed for a case (`observed.json`), and the run's own
//! metadata (`run.json`). Both carry a format marker: a newer file is
//! refused by name rather than misread.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The format of `observed.json`, `run.json` and `report.json`.
pub const FORMAT: u32 = 1;

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
    /// Files the run modified outside what its step allows.
    #[serde(default)]
    pub strayed: Vec<String>,
    /// `codev review --json` as codev printed it; `null` when not run.
    pub review: Option<serde_json::Value>,
    /// The checked files after the run; `null` for a file that is gone.
    #[serde(default)]
    pub files: BTreeMap<String, Option<String>>,
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
}

/// Reads a JSON file carrying a format marker, refusing a newer one.
fn check_format(text: &str, what: &str) -> Result<serde_json::Value, String> {
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
        let err = parse_observed(r#"{"format": 2, "case": "x"}"#, "ops-code-timeout").unwrap_err();
        assert!(
            err.contains("ops-code-timeout") && err.contains("format 2"),
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
    fn run_info_round_trips() {
        let info = RunInfo {
            format: FORMAT,
            model: None,
            date: "2026-10-13".into(),
            claude_version: Some("2.1.289".into()),
            codev_version: Some("0.6.1".into()),
        };
        let text = serde_json::to_string(&info).unwrap();
        assert_eq!(parse_run_info(&text).unwrap(), info);
        assert!(parse_run_info("{}").unwrap_err().contains("format"));
    }
}
