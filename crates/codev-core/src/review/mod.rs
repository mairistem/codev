//! The review record of a change, `review.md`: what the reviewing roles
//! found at `plan.challenge` and `code.review`, the evidence they quoted,
//! and the answer each finding received.
//!
//! Pure (decision 0001): the record and the cited files come in as
//! strings, located findings and the archive gate come out. Reading the
//! files and resolving the active roles is `codev-engine::review`'s job.
//!
//! The severity and confidence definitions below are the single source the
//! skills receive (substituted at rendering) and the docs test compares the
//! Roles chapter with: change them here, and both follow.

mod evidence;
mod gate;
mod history;
pub mod material;
pub mod mode;
mod parse;

pub use evidence::{EvidenceMatch, line_count, match_code, match_plan};
pub use gate::{Blocker, StepRoles, check_roles, gate};
pub use mode::{ModeOrigin, ReviewMode};
pub use parse::parse;

/// The record's file name, in the change's directory.
pub const FILE: &str = "review.md";

/// The format this codev reads and writes. A record marked with a greater
/// number comes from a newer codev and is not checked.
pub const FORMAT: u32 = 1;

/// The steps a record holds a section for, in workflow order.
pub const STEPS: &[&str] = &["plan.challenge", "code.review"];

/// How far, in lines, the evidence may have drifted from the cited lines
/// and still count as found there: an import added above the code, not
/// another function.
pub const TOLERANCE: u32 = 5;

/// Above this size a cited file is not compared: reading it whole to look
/// for a quote is not worth it, and such a file is rarely hand-written.
pub const MAX_FILE_BYTES: usize = 4 * 1024 * 1024;

/// Stable codes of the record's findings and of the archive gate.
pub mod codes {
    /// A structural defect: layout, a missing or malformed field, an id,
    /// a location, a status or a transition.
    pub const REVIEW_INVALID: &str = "review_invalid";
    /// A history entry without what its status requires (`ref`, `note`,
    /// `to`), or decided by someone who may not decide it.
    pub const REVIEW_DECISION_INCOMPLETE: &str = "review_decision_incomplete";
    /// The cited file, line or heading does not exist.
    pub const REVIEW_LOCATION_MISSING: &str = "review_location_missing";
    /// The quoted evidence is nowhere in the cited file or section.
    pub const REVIEW_EVIDENCE_NOT_FOUND: &str = "review_evidence_not_found";
    /// A role active at a recorded step has no section, or says it is
    /// disabled.
    pub const REVIEW_ROLE_MISSING: &str = "review_role_missing";
    /// A step or a role this codev does not know (warning).
    pub const REVIEW_UNKNOWN_SECTION: &str = "review_unknown_section";
    /// A field this codev does not know (warning).
    pub const REVIEW_UNKNOWN_FIELD: &str = "review_unknown_field";
    /// The evidence is in the file, but away from the cited lines
    /// (warning).
    pub const REVIEW_EVIDENCE_MOVED: &str = "review_evidence_moved";
    /// The cited file is binary or too large to compare (warning).
    pub const REVIEW_EVIDENCE_UNCHECKED: &str = "review_evidence_unchecked";
    /// The record comes from a newer codev (warning; blocks archiving).
    pub const REVIEW_FORMAT_UNSUPPORTED: &str = "review_format_unsupported";

    /// The codes a validation report can carry, for the uniqueness test.
    pub const ALL: &[&str] = &[
        REVIEW_INVALID,
        REVIEW_DECISION_INCOMPLETE,
        REVIEW_LOCATION_MISSING,
        REVIEW_EVIDENCE_NOT_FOUND,
        REVIEW_ROLE_MISSING,
        REVIEW_UNKNOWN_SECTION,
        REVIEW_UNKNOWN_FIELD,
        REVIEW_EVIDENCE_MOVED,
        REVIEW_EVIDENCE_UNCHECKED,
        REVIEW_FORMAT_UNSUPPORTED,
    ];

    /// Reasons the gate gives for refusing to archive. Not validation
    /// codes: they describe the state of the review, not a defect of the
    /// file.
    pub const REVIEW_RECORD_MISSING: &str = "review_record_missing";
    pub const REVIEW_STEP_MISSING: &str = "review_step_missing";
    pub const REVIEW_UNRESOLVED: &str = "review_unresolved";
}

/// How bad a finding is, if the reviewer is right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    High,
    Medium,
    Low,
}

impl Severity {
    pub const ALL: [Self; 3] = [Self::High, Self::Medium, Self::Low];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.as_str() == text)
    }

    /// One sentence, the same in the skills and in the docs.
    pub fn definition(self) -> &'static str {
        match self {
            Self::High => {
                "The change would break a requirement, lose or expose data, or open a \
                 vulnerability, and must not be archived until it is fixed or a human \
                 accepts the risk."
            }
            Self::Medium => {
                "A real defect or risk with a limited impact or a workaround, to fix in \
                 this change or to defer deliberately."
            }
            Self::Low => {
                "An improvement, such as readability, consistency or a minor cost, that \
                 does not decide whether the change is correct."
            }
        }
    }
}

/// How sure the reviewer is that the finding is real.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    High,
    Low,
}

impl Confidence {
    pub const ALL: [Self; 2] = [Self::High, Self::Low];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Low => "low",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.as_str() == text)
    }

    pub fn definition(self) -> &'static str {
        match self {
            Self::High => "The evidence shows the problem directly.",
            Self::Low => "The problem depends on something the reviewer could not see or verify.",
        }
    }
}

/// The scales as markdown bullets: what the skills receive in place of
/// `{{REVIEW_SCALE}}`.
pub fn scale_markdown() -> String {
    let mut out = String::from("Severity:\n\n");
    for severity in Severity::ALL {
        out.push_str(&format!(
            "- `{}` — {}\n",
            severity.as_str(),
            severity.definition()
        ));
    }
    out.push_str("\nConfidence:\n\n");
    for confidence in Confidence::ALL {
        out.push_str(&format!(
            "- `{}` — {}\n",
            confidence.as_str(),
            confidence.definition()
        ));
    }
    out
}

/// Where a finding stands, after its last answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Open,
    Fixed,
    Disputed,
    AcceptedRisk,
    Deferred,
    Rejected,
}

impl Status {
    pub const ALL: [Self; 6] = [
        Self::Open,
        Self::Fixed,
        Self::Disputed,
        Self::AcceptedRisk,
        Self::Deferred,
        Self::Rejected,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Fixed => "fixed",
            Self::Disputed => "disputed",
            Self::AcceptedRisk => "accepted-risk",
            Self::Deferred => "deferred",
            Self::Rejected => "rejected",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.as_str() == text)
    }

    /// The finding still describes the tree as it is: its location and
    /// evidence are checked. A fix changed the code; a rejected finding
    /// no longer matters.
    pub fn describes_current_tree(self) -> bool {
        matches!(
            self,
            Self::Open | Self::Disputed | Self::AcceptedRisk | Self::Deferred
        )
    }

    /// Unresolved: a `high` finding in this state blocks archiving.
    pub fn is_unresolved(self) -> bool {
        matches!(self, Self::Open | Self::Disputed)
    }
}

/// Who must act next on a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Awaiting {
    /// `open`: the author fixes or disputes.
    Author,
    /// A fix not yet verified, or a dispute not yet answered.
    Reviewer,
    /// A dispute the reviewer maintained.
    Human,
}

impl Awaiting {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Author => "author",
            Self::Reviewer => "reviewer",
            Self::Human => "human",
        }
    }
}

/// What a finding points at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location {
    /// `<path>:<line>` or `<path>:<start>-<end>`, relative to the project
    /// root, `/`-separated.
    Code { path: String, start: u32, end: u32 },
    /// `<file>#<heading>`, relative to the change's directory.
    Plan { file: String, heading: String },
}

/// One answer in a finding's history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub status: Status,
    pub by: String,
    pub note: Option<String>,
    pub reference: Option<String>,
    pub to: Option<String>,
}

/// `human` or `human:<name>`: a decision a person made.
pub fn is_human(by: &str) -> bool {
    by == "human"
        || by
            .strip_prefix("human:")
            .is_some_and(|name| !name.trim().is_empty())
}

/// A finding of the record. Fields the file got wrong are `None` or
/// empty; the parse findings say why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewFinding {
    pub id: String,
    pub title: String,
    pub step: String,
    pub role: String,
    /// The line of its `####` heading in `review.md`.
    pub line: u32,
    pub severity: Option<Severity>,
    pub confidence: Option<Confidence>,
    /// As written, `\` read as `/`.
    pub location_text: String,
    pub location: Option<Location>,
    pub evidence: String,
    pub recommendation: String,
    pub history: Vec<Entry>,
    /// `raised_by` as written: the roles that raised it, its own first.
    /// Empty when the field is absent — raised by its role alone.
    pub raised_by: Vec<String>,
}

impl ReviewFinding {
    /// Every role that raised it, its own first.
    pub fn raisers(&self) -> Vec<String> {
        if self.raised_by.is_empty() {
            vec![self.role.clone()]
        } else {
            self.raised_by.clone()
        }
    }

    /// The status of its last answer; `None` with an empty history.
    pub fn status(&self) -> Option<Status> {
        self.history.last().map(|entry| entry.status)
    }

    /// Presented to the human: only a confident finding is.
    pub fn reported(&self) -> bool {
        self.confidence == Some(Confidence::High)
    }

    /// Who acts next, read from the last answer and who gave it.
    pub fn awaiting(&self) -> Option<Awaiting> {
        let last = self.history.last()?;
        let by_reviewer = last.by == self.role;
        match last.status {
            Status::Open => Some(Awaiting::Author),
            Status::Fixed if by_reviewer => None,
            Status::Fixed => Some(Awaiting::Reviewer),
            Status::Disputed if by_reviewer => Some(Awaiting::Human),
            Status::Disputed => Some(Awaiting::Reviewer),
            Status::AcceptedRisk | Status::Deferred | Status::Rejected => None,
        }
    }

    /// A `high` finding still `open` or `disputed`.
    pub fn blocks_archive(&self) -> bool {
        self.severity == Some(Severity::High) && self.status().is_some_and(Status::is_unresolved)
    }
}

/// What a role's section says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleState {
    Findings,
    NoFinding,
    Disabled,
    /// Nothing usable: a `review_invalid` was reported.
    Empty,
}

impl RoleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Findings => "findings",
            Self::NoFinding => "no-finding",
            Self::Disabled => "disabled",
            Self::Empty => "empty",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleSection {
    pub role: String,
    pub line: u32,
    pub state: RoleState,
    pub findings: Vec<ReviewFinding>,
    /// Its `Fallback:` line: why the role ran inline in an isolated
    /// review.
    pub fallback: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepSection {
    pub step: String,
    pub line: u32,
    pub roles: Vec<RoleSection>,
    /// Its `Mode:` line, when written.
    pub mode: Option<ReviewMode>,
    /// Its `Summarized:` lines, in file order.
    pub summarized: Vec<String>,
}

impl StepSection {
    pub fn role(&self, id: &str) -> Option<&RoleSection> {
        self.roles.iter().find(|r| r.role == id)
    }

    /// The findings of other roles of this step that credit `role` in
    /// their `raised_by`.
    pub fn credited(&self, role: &str) -> Vec<&ReviewFinding> {
        self.roles
            .iter()
            .filter(|section| section.role != role)
            .flat_map(|section| section.findings.iter())
            .filter(|finding| finding.raised_by.iter().any(|r| r == role))
            .collect()
    }
}

/// A parsed record: the known steps, in file order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub format: u32,
    pub steps: Vec<StepSection>,
}

impl Record {
    pub fn step(&self, step: &str) -> Option<&StepSection> {
        self.steps.iter().find(|s| s.step == step)
    }

    /// Every finding, in file order.
    pub fn findings(&self) -> impl Iterator<Item = &ReviewFinding> {
        self.steps
            .iter()
            .flat_map(|s| s.roles.iter())
            .flat_map(|r| r.findings.iter())
    }

    /// A newer codev wrote it: nothing in it is checked.
    pub fn is_supported(&self) -> bool {
        self.format <= FORMAT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scale_lists_every_level_with_its_definition() {
        let scale = scale_markdown();
        for severity in Severity::ALL {
            assert!(scale.contains(severity.definition()), "{scale}");
        }
        for confidence in Confidence::ALL {
            assert!(scale.contains(confidence.definition()), "{scale}");
        }
    }

    #[test]
    fn a_human_is_named_human_or_human_with_a_name() {
        assert!(is_human("human"));
        assert!(is_human("human:Ludovic"));
        assert!(!is_human("human:"));
        assert!(!is_human("developer"));
    }
}
