//! The rules of a finding's history: which answer may follow which, what
//! each must carry, and who may give it.
//!
//! codev checks who decided, not whether the decision was right: that is
//! judgement, and it stays with the people and the skills.

use crate::parser::ast::Finding;

use super::codes::{REVIEW_DECISION_INCOMPLETE, REVIEW_INVALID};
use super::{Entry, ReviewFinding, Severity, Status, is_human};

/// The defects of `finding`'s history, reported at `line`.
pub fn check(finding: &ReviewFinding, line: u32) -> Vec<Finding> {
    let id = &finding.id;
    let role = &finding.role;
    let mut out = Vec::new();
    let Some(first) = finding.history.first() else {
        out.push(Finding::error(
            REVIEW_INVALID,
            line,
            format!("{id}: `history` is empty; its first entry is `status: open`, `by: {role}`"),
        ));
        return out;
    };
    if first.status != Status::Open || first.by != *role {
        out.push(Finding::error(
            REVIEW_INVALID,
            line,
            format!(
                "{id}: the history starts with `status: open`, `by: {role}` — the role that raised it"
            ),
        ));
    }
    for pair in finding.history.windows(2) {
        let (previous, entry) = (&pair[0], &pair[1]);
        if let Some(defect) = transition(previous.status, entry, role) {
            out.push(Finding::error(
                REVIEW_INVALID,
                line,
                format!("{id}: {defect}"),
            ));
            continue;
        }
        if let Some(missing) = requirement(previous.status, entry, role, finding.severity) {
            out.push(Finding::error(
                REVIEW_DECISION_INCOMPLETE,
                line,
                format!("{id}: {missing}"),
            ));
        }
    }
    out
}

/// Whether `entry` may follow a `previous` status; the reason when not.
fn transition(previous: Status, entry: &Entry, role: &str) -> Option<String> {
    use Status::*;
    let to = entry.status;
    let allowed = match previous {
        Open => matches!(to, Fixed | Disputed | AcceptedRisk | Deferred),
        Disputed => matches!(to, Fixed | Disputed | AcceptedRisk | Deferred | Rejected),
        Fixed => matches!(to, Fixed | Open),
        AcceptedRisk | Deferred | Rejected => false,
    };
    if !allowed {
        return Some(format!(
            "`{}` cannot follow `{}`; raise a new finding to reopen a settled one",
            to.as_str(),
            previous.as_str()
        ));
    }
    let reviewer_or_human = entry.by == role || is_human(&entry.by);
    match (previous, to) {
        (Disputed, Rejected) if !reviewer_or_human => Some(format!(
            "only the reviewer (`{role}`) or a human rejects a finding, not `{}`",
            entry.by
        )),
        (Fixed, Fixed | Open) if entry.by != role => Some(format!(
            "only the reviewer (`{role}`) verifies or reopens a fix, not `{}`",
            entry.by
        )),
        _ => None,
    }
}

/// What `entry` lacks for its status; `None` when complete.
fn requirement(
    previous: Status,
    entry: &Entry,
    role: &str,
    severity: Option<Severity>,
) -> Option<String> {
    let has = |field: &Option<String>| field.as_deref().is_some_and(|v| !v.trim().is_empty());
    let status = entry.status.as_str();
    match entry.status {
        Status::Fixed if previous != Status::Fixed && !has(&entry.reference) => Some(format!(
            "`{status}` needs `ref`: the commit or the file change that fixes it"
        )),
        Status::Disputed if !has(&entry.note) => Some(format!(
            "`{status}` needs `note`: the argument, grounded in the material"
        )),
        Status::AcceptedRisk if !has(&entry.note) => {
            Some(format!("`{status}` needs `note`: the justification"))
        }
        Status::AcceptedRisk if !is_human(&entry.by) => Some(format!(
            "an accepted risk is decided by a human (`by: human` or `human:<name>`), not `{}`",
            entry.by
        )),
        Status::Deferred if !has(&entry.to) => Some(format!(
            "`{status}` needs `to`: the change or the issue it goes to"
        )),
        Status::Deferred if severity == Some(Severity::High) && !is_human(&entry.by) => {
            Some(format!(
                "a high finding is deferred by a human (`by: human` or `human:<name>`), not `{}`",
                entry.by
            ))
        }
        Status::Open if previous == Status::Fixed && entry.by == role && !has(&entry.note) => Some(
            format!("`{status}` after a fix needs `note`: why the fix does not hold"),
        ),
        _ => None,
    }
}
