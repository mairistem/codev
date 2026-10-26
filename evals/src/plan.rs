//! Scoring a planning run, without a model: which expected requirements its
//! deltas modify, what it adds where it should have modified, which
//! expected decisions it ignores, and whether it validates. Pure: the shell
//! (`suite.rs`) reads the specs and decisions at the case's `ref` and hands
//! their names and titles in.

use std::collections::BTreeMap;

use codev_core::parser::{DeltaSection, parse_delta};
use serde::Serialize;

use crate::case::{DecisionExpect, PlanCase, RequirementId, normalize_heading};

const CHANGES_DIR: &str = "_codev/changes/";

/// The repository at a planning case's `ref`, as far as scoring needs it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AtRef {
    /// The requirement names of each capability's main spec.
    pub requirements: BTreeMap<String, Vec<String>>,
    /// Each decision's title, by its id (`0007`).
    pub decision_titles: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlanOutcome {
    Planned,
    /// The run created no change.
    NoPlan,
}

/// What a planning run did against what its case expects.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanScore {
    pub outcome: PlanOutcome,
    pub primary_found: Vec<String>,
    pub primary_missed: Vec<String>,
    pub collateral_found: Vec<String>,
    pub collateral_missed: Vec<String>,
    /// Requirements added where an expected one was left unmodified, or
    /// under the name of a requirement the capability already had.
    pub duplicates: Vec<String>,
    /// The ids of the expected decisions the plan ignores.
    pub decisions_ignored: Vec<String>,
    /// `codev validate` reports no error on any change the run created.
    pub valid: bool,
    /// For a case with `keyword_language`: the structural keyword lines
    /// written in another language, as `<file>: <line>`; `None` when the
    /// case expects no language.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords_off: Option<Vec<String>>,
}

impl PlanScore {
    /// The share of the primary requirements modified; `None` when the
    /// case expects none.
    pub fn primary_recall(&self) -> Option<f64> {
        recall(self.primary_found.len(), self.primary_missed.len())
    }

    /// Whether the plan's keywords are in the case's language; `None`
    /// when the case expects none.
    pub fn keywords_in_language(&self) -> Option<bool> {
        self.keywords_off.as_ref().map(Vec::is_empty)
    }

    /// The share of the collateral requirements modified, apart.
    pub fn collateral_recall(&self) -> Option<f64> {
        recall(self.collateral_found.len(), self.collateral_missed.len())
    }
}

fn recall(found: usize, missed: usize) -> Option<f64> {
    let total = found + missed;
    (total > 0).then(|| found as f64 / total as f64)
}

/// What the deltas of a plan do, by capability: the names it modifies,
/// normalized, and the names it adds, as written.
#[derive(Default)]
struct Touched {
    modified: BTreeMap<String, Vec<String>>,
    added: BTreeMap<String, Vec<String>>,
}

/// Scores the plan made of `files` (the observation's files, by path) and
/// `validate` (`codev validate --json` on each change created).
pub fn score_plan(
    case: &PlanCase,
    files: &BTreeMap<String, Option<String>>,
    validate: &BTreeMap<String, serde_json::Value>,
    at_ref: &AtRef,
) -> PlanScore {
    let plan: Vec<(&str, &str)> = files
        .iter()
        .filter_map(|(path, text)| {
            let text = text.as_deref()?;
            path.strip_prefix(CHANGES_DIR).map(|rest| (rest, text))
        })
        .collect();
    let mut touched = Touched::default();
    let mut prose = String::new();
    for (rest, text) in &plan {
        let parts: Vec<&str> = rest.split('/').collect();
        match parts.as_slice() {
            [_, "specs", capability, "spec.md"] => read_delta(capability, text, &mut touched),
            [_, "proposal.md"] | [_, "design.md"] => {
                prose.push_str(text);
                prose.push_str("\n\n");
            }
            _ => {}
        }
    }
    let is_modified = |id: &RequirementId| {
        touched
            .modified
            .get(&id.capability)
            .is_some_and(|names| names.contains(&normalize_heading(&id.name)))
    };
    let split = |ids: &[RequirementId]| -> (Vec<String>, Vec<String>) {
        let (found, missed): (Vec<&RequirementId>, Vec<&RequirementId>) =
            ids.iter().partition(|id| is_modified(id));
        let names = |v: Vec<&RequirementId>| v.into_iter().map(|id| id.to_string()).collect();
        (names(found), names(missed))
    };
    let (primary_found, primary_missed) = split(&case.modified);
    let (collateral_found, collateral_missed) = split(&case.collateral);

    let mut duplicates = Vec::new();
    for (capability, added) in &touched.added {
        let expected_left = case
            .modified
            .iter()
            .chain(&case.collateral)
            .any(|id| &id.capability == capability && !is_modified(id));
        let existing: Vec<String> = at_ref
            .requirements
            .get(capability)
            .map(|names| names.iter().map(|n| normalize_heading(n)).collect())
            .unwrap_or_default();
        for name in added {
            if expected_left || existing.contains(&normalize_heading(name)) {
                duplicates.push(format!("{capability}::{name}"));
            }
        }
    }

    let decisions_ignored = case
        .decisions
        .iter()
        .filter(|d| ignores(&prose, &d.id, at_ref.decision_titles.get(&d.id), d.expect))
        .map(|d| d.id.clone())
        .collect();

    let outcome = if plan.is_empty() {
        PlanOutcome::NoPlan
    } else {
        PlanOutcome::Planned
    };
    let valid = outcome == PlanOutcome::Planned
        && !validate.is_empty()
        && validate.values().all(|report| errors(report) == 0);
    let keywords_off = case
        .keyword_language
        .as_deref()
        .map(|code| keyword_lines_off(&plan, code));
    PlanScore {
        outcome,
        primary_found,
        primary_missed,
        collateral_found,
        collateral_missed,
        duplicates,
        decisions_ignored,
        valid,
        keywords_off,
    }
}

/// The language of the structural keyword a heading line holds, as the
/// The artifacts that hold structural keywords; the tasks hold none.
const KEYWORD_ARTIFACTS: [&str; 3] = ["/proposal.md", "/design.md", "/spec.md"];

/// Each keyword heading of the plan's artifacts written in another
/// language than `code`, as `<path>: <line>`.
fn keyword_lines_off(plan: &[(&str, &str)], code: &str) -> Vec<String> {
    let mut off = Vec::new();
    let checked = plan
        .iter()
        .filter(|(path, _)| KEYWORD_ARTIFACTS.iter().any(|end| path.ends_with(end)));
    for (path, text) in checked {
        let lines = text
            .lines()
            .filter(|line| keyword_language_of(line).is_some_and(|lang| lang.code() != code));
        off.extend(lines.map(|line| format!("{path}: {line}")));
    }
    // A run that wrote nothing has no keyword in its language.
    if plan.is_empty() {
        off.push("no artifact written".to_string());
    }
    off
}

/// keyword catalog recognizes it; `None` for any other line.
pub fn keyword_language_of(line: &str) -> Option<codev_core::keywords::Lang> {
    codev_core::keywords::heading_language(line)
}

fn read_delta(capability: &str, text: &str, touched: &mut Touched) {
    let delta = parse_delta(text).value;
    let modified = touched.modified.entry(capability.to_string()).or_default();
    let mut added = Vec::new();
    for section in &delta.sections {
        match section {
            DeltaSection::Added { requirements, .. } => {
                added.extend(requirements.iter().map(|r| r.name.clone()))
            }
            DeltaSection::Modified { requirements, .. } => {
                modified.extend(requirements.iter().map(|r| normalize_heading(&r.name)))
            }
            DeltaSection::Removed { removals, .. } => {
                modified.extend(removals.iter().map(|r| normalize_heading(&r.name)))
            }
            DeltaSection::Renamed { renames, .. } => {
                modified.extend(renames.iter().map(|r| normalize_heading(&r.from)))
            }
        }
    }
    if !added.is_empty() {
        touched
            .added
            .entry(capability.to_string())
            .or_default()
            .extend(added);
    }
}

/// Words that say a plan supersedes or deviates from a decision, in the
/// languages the plans are written in.
const SUPERSEDES: [&str; 8] = [
    "supersed",
    "deviat",
    "replace",
    "overturn",
    "remplac",
    "dérog",
    "s'écarte",
    "s’écarte",
];

/// Whether the plan's prose ignores a decision: never cites it, by id or
/// title; or, for one to supersede, no paragraph citing it says so.
fn ignores(prose: &str, id: &str, title: Option<&String>, expect: DecisionExpect) -> bool {
    let cites = |text: &str| {
        let lower = text.to_lowercase();
        cites_id(&lower, id)
            || title.is_some_and(|t| !t.is_empty() && lower.contains(&t.to_lowercase()))
    };
    match expect {
        DecisionExpect::Follow => !cites(prose),
        DecisionExpect::Supersede => !prose.split("\n\n").any(|paragraph| {
            let lower = paragraph.to_lowercase();
            cites(paragraph) && SUPERSEDES.iter().any(|w| lower.contains(w))
        }),
    }
}

/// `id` as a whole number in `text`: `0007` in "ADR 0007" or `0007-x.md`,
/// not in `10007`.
fn cites_id(text: &str, id: &str) -> bool {
    text.match_indices(id).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + id.len()..].chars().next();
        !before.is_some_and(|c| c.is_ascii_digit()) && !after.is_some_and(|c| c.is_ascii_digit())
    })
}

/// The errors in one `codev validate --json` report.
fn errors(report: &serde_json::Value) -> usize {
    report["items"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|item| item["findings"].as_array().into_iter().flatten())
        .filter(|f| f["severity"] == "error")
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::case::ExpectedDecision;
    use serde_json::json;

    const CREATING: &str = "Creating a decision through the CLI";

    fn case(modified: &[&str], decisions: &[(&str, DecisionExpect)]) -> PlanCase {
        PlanCase {
            git_ref: "abc".into(),
            request: "r".into(),
            modified: modified
                .iter()
                .map(|m| RequirementId::parse(m).unwrap())
                .collect(),
            collateral: Vec::new(),
            decisions: decisions
                .iter()
                .map(|(id, expect)| ExpectedDecision {
                    id: id.to_string(),
                    expect: *expect,
                })
                .collect(),
            config: None,
            keyword_language: None,
        }
    }

    fn at_ref() -> AtRef {
        AtRef {
            requirements: BTreeMap::from([(
                "decisions".to_string(),
                vec![CREATING.to_string(), "Listing decisions".to_string()],
            )]),
            decision_titles: BTreeMap::from([
                (
                    "0007".to_string(),
                    "The binary never calls a model".to_string(),
                ),
                ("0001".to_string(), "No cache on disk".to_string()),
            ]),
        }
    }

    fn valid() -> BTreeMap<String, serde_json::Value> {
        BTreeMap::from([("c".to_string(), json!({"items": [{"findings": []}]}))])
    }

    fn plan(files: &[(&str, &str)]) -> BTreeMap<String, Option<String>> {
        files
            .iter()
            .map(|(p, t)| (format!("_codev/changes/c/{p}"), Some(t.to_string())))
            .collect()
    }

    fn requirement(name: &str) -> String {
        format!(
            "### Requirement: {name}\n\nThe system SHALL do it.\n\n#### Scenario: It does\n\n- **WHEN** asked\n- **THEN** it does\n\n"
        )
    }

    #[test]
    fn a_modified_requirement_and_a_new_one_beside_it() {
        let delta = format!(
            "## MODIFIED Requirements\n\n{}## ADDED Requirements\n\n{}",
            requirement(CREATING),
            requirement("Accepting a decision through the CLI")
        );
        let files = plan(&[("proposal.md", "p"), ("specs/decisions/spec.md", &delta)]);
        let score = score_plan(
            &case(&[&format!("decisions::{CREATING}")], &[]),
            &files,
            &valid(),
            &at_ref(),
        );
        assert_eq!(score.primary_recall(), Some(1.0));
        assert!(score.duplicates.is_empty(), "{:?}", score.duplicates);
        assert!(score.valid);
    }

    #[test]
    fn an_addition_where_a_modification_was_due() {
        let delta = format!(
            "## ADDED Requirements\n\n{}",
            requirement("Creating a draft decision")
        );
        let files = plan(&[("specs/decisions/spec.md", &delta)]);
        let score = score_plan(
            &case(&[&format!("decisions::{CREATING}")], &[]),
            &files,
            &valid(),
            &at_ref(),
        );
        assert_eq!(score.primary_recall(), Some(0.0));
        assert_eq!(score.duplicates, ["decisions::Creating a draft decision"]);

        // Adding a requirement the capability already has is a duplicate too.
        let delta = format!(
            "## ADDED Requirements\n\n{}",
            requirement("listing  DECISIONS")
        );
        let files = plan(&[("specs/decisions/spec.md", &delta)]);
        let score = score_plan(&case(&[], &[]), &files, &valid(), &at_ref());
        assert_eq!(score.duplicates.len(), 1);
    }

    #[test]
    fn french_keywords_count() {
        let delta = format!(
            "## Exigences MODIFIÉES\n\n### Exigence : {CREATING}\n\nLe système DOIT le faire.\n\n#### Scénario : Il le fait\n\n- **QUAND** on le demande\n- **ALORS** il le fait\n"
        );
        let files = plan(&[("specs/decisions/spec.md", &delta)]);
        let score = score_plan(
            &case(&[&format!("decisions::{CREATING}")], &[]),
            &files,
            &valid(),
            &at_ref(),
        );
        assert_eq!(score.primary_found, [format!("decisions::{CREATING}")]);
    }

    #[test]
    fn a_decision_to_supersede_only_cited_is_ignored() {
        let expect = case(&[], &[("0007", DecisionExpect::Supersede)]);
        let cited = plan(&[("design.md", "## Decisions\n\nThis follows decision 0007.\n")]);
        let score = score_plan(&expect, &cited, &valid(), &at_ref());
        assert_eq!(score.decisions_ignored, ["0007"]);

        let superseded = plan(&[(
            "design.md",
            "## Decisions\n\nThis change supersedes decision 0007: the binary calls a model.\n",
        )]);
        let score = score_plan(&expect, &superseded, &valid(), &at_ref());
        assert!(score.decisions_ignored.is_empty());
    }

    #[test]
    fn a_followed_decision_cited_by_title() {
        let expect = case(&[], &[("0001", DecisionExpect::Follow)]);
        let by_title = plan(&[("proposal.md", "We keep to no cache on disk, as decided.\n")]);
        assert!(
            score_plan(&expect, &by_title, &valid(), &at_ref())
                .decisions_ignored
                .is_empty()
        );
        let by_id = plan(&[("design.md", "See ADR 0001.\n")]);
        assert!(
            score_plan(&expect, &by_id, &valid(), &at_ref())
                .decisions_ignored
                .is_empty()
        );
        let silent = plan(&[("design.md", "Version 10001 of nothing.\n")]);
        assert_eq!(
            score_plan(&expect, &silent, &valid(), &at_ref()).decisions_ignored,
            ["0001"]
        );
    }

    #[test]
    fn a_plan_that_does_not_validate() {
        let files = plan(&[("proposal.md", "p")]);
        let invalid = BTreeMap::from([(
            "c".to_string(),
            json!({"items": [{"findings": [{"severity": "warning"}, {"severity": "error"}]}]}),
        )]);
        assert!(!score_plan(&case(&[], &[]), &files, &invalid, &at_ref()).valid);
        let warned = BTreeMap::from([(
            "c".to_string(),
            json!({"items": [{"findings": [{"severity": "warning"}]}]}),
        )]);
        assert!(score_plan(&case(&[], &[]), &files, &warned, &at_ref()).valid);
    }

    #[test]
    fn a_run_that_plans_nothing() {
        let expect = case(&[&format!("decisions::{CREATING}")], &[]);
        let score = score_plan(&expect, &BTreeMap::new(), &BTreeMap::new(), &at_ref());
        assert_eq!(score.outcome, PlanOutcome::NoPlan);
        assert_eq!(score.primary_missed, [format!("decisions::{CREATING}")]);
        assert_eq!(score.primary_recall(), Some(0.0));
        assert!(!score.valid);
    }

    #[test]
    fn a_french_plan_with_an_english_heading() {
        let mut french = case(&[], &[("0007", DecisionExpect::Follow)]);
        french.keyword_language = Some("fr".into());
        let files = plan(&[
            (
                "proposal.md",
                "# Proposition : X\n\n## Changements\n\n- x\n\nVoir 0007.\n",
            ),
            (
                "specs/decisions/spec.md",
                "## Exigences MODIFIÉES\n\n### Requirement: Login\n\nLe système DOIT x.\n",
            ),
        ]);
        let score = score_plan(&french, &files, &valid(), &at_ref());
        assert_eq!(score.keywords_in_language(), Some(false));
        let off = score.keywords_off.unwrap();
        assert_eq!(off.len(), 1, "{off:?}");
        assert!(off[0].ends_with("### Requirement: Login"), "{off:?}");
        let english = score_plan(&case(&[], &[]), &files, &valid(), &at_ref());
        assert_eq!(english.keywords_in_language(), None, "no language expected");
    }
}
