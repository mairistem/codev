use std::path::PathBuf;

use codev_core::parser::ast::{Finding, Severity};

/// A finding, augmented with the path of the file it applies to.
///
/// The type separates the "where" concern (here, in the engine) from the
/// "what" concern (in the parser or in a rule). Breaking the core's
/// `Finding` to add a `path` to it would force every call site to carry a
/// path that only makes sense at the scale of a project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatedFinding {
    pub path: PathBuf,
    pub finding: Finding,
}

impl LocatedFinding {
    pub fn from_finding(finding: Finding, path: PathBuf) -> Self {
        Self { path, finding }
    }

    pub fn is_error(&self) -> bool {
        self.finding.severity == Severity::Error
    }
}

/// What an item is: an active change, or a main spec.
///
/// An `enum` rather than a `String`: both the human rendering and the JSON
/// contract use it, and we want the compiler to warn us when a third kind
/// of item appears (decisions, for example).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Change,
    Spec,
    /// All of the project's decisions, seen as a single item — seal
    /// findings point to the ADR or to `seal.yaml`, depending on the
    /// case.
    Decisions,
}

impl ItemKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Change => "change",
            Self::Spec => "spec",
            Self::Decisions => "decisions",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemReport {
    pub kind: ItemKind,
    pub name: String,
    pub path: PathBuf,
    pub findings: Vec<LocatedFinding>,
}

impl ItemReport {
    pub fn has_errors(&self) -> bool {
        self.findings.iter().any(LocatedFinding::is_error)
    }

    /// `true` as soon as at least one finding has `Warning` severity.
    /// Symmetric to `has_errors` — useful for the CLI's strict mode.
    pub fn has_warnings(&self) -> bool {
        self.findings
            .iter()
            .any(|f| f.finding.severity == Severity::Warning)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateReport {
    pub root: PathBuf,
    pub items: Vec<ItemReport>,
}

impl ValidateReport {
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(ItemReport::has_errors)
    }

    /// `true` as soon as at least one `ItemReport` contains a `Warning`
    /// finding. Used by the strict mode of `codev validate` — the CLI does
    /// not change the severity, it combines this boolean with the flag.
    pub fn has_warnings(&self) -> bool {
        self.items.iter().any(ItemReport::has_warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codev_core::parser::ast::Span;

    #[test]
    fn located_keeps_code_line_severity() {
        let finding = Finding {
            severity: Severity::Error,
            code: "some_code",
            line: 42,
            message: "a message".into(),
        };
        let located = LocatedFinding::from_finding(finding, PathBuf::from("_codev/x.md"));
        assert!(located.is_error());
        assert_eq!(located.finding.code, "some_code");
        assert_eq!(located.finding.line, 42);
        assert_eq!(located.path, PathBuf::from("_codev/x.md"));
    }

    #[test]
    fn has_warnings_is_true_for_a_single_warning() {
        let warning = Finding {
            severity: Severity::Warning,
            code: "warn",
            line: 1,
            message: "m".into(),
        };
        let report = ItemReport {
            kind: ItemKind::Change,
            name: "x".into(),
            path: PathBuf::from("x"),
            findings: vec![LocatedFinding::from_finding(warning, PathBuf::from("f"))],
        };
        assert!(report.has_warnings());
        assert!(!report.has_errors());
    }

    #[test]
    fn has_warnings_is_false_for_an_error_alone() {
        // Errors are not warnings — strict mode tells them apart.
        let err = Finding {
            severity: Severity::Error,
            code: "e",
            line: 1,
            message: "m".into(),
        };
        let report = ItemReport {
            kind: ItemKind::Spec,
            name: "s".into(),
            path: PathBuf::from("s"),
            findings: vec![LocatedFinding::from_finding(err, PathBuf::from("f"))],
        };
        assert!(!report.has_warnings());
        assert!(report.has_errors());
    }

    #[test]
    fn has_warnings_on_empty_report_is_false() {
        let report = ValidateReport {
            root: PathBuf::from("/p"),
            items: vec![],
        };
        assert!(!report.has_warnings());
    }

    #[test]
    fn item_report_flags_an_error_even_alone() {
        let finding = Finding {
            severity: Severity::Error,
            code: "err",
            line: 1,
            message: "m".into(),
        };
        let report = ItemReport {
            kind: ItemKind::Change,
            name: "add-auth".into(),
            path: PathBuf::from("_codev/changes/add-auth"),
            findings: vec![LocatedFinding::from_finding(finding, PathBuf::from("x"))],
        };
        assert!(report.has_errors());

        // A warning does not tip it over.
        let warning = Finding {
            severity: Severity::Warning,
            code: "warn",
            line: 1,
            message: "m".into(),
        };
        let mut only_warn = report.clone();
        only_warn.findings = vec![LocatedFinding::from_finding(warning, PathBuf::from("x"))];
        assert!(!only_warn.has_errors());
    }

    // Compilation test: `Span` stays importable the same way from the
    // core — guarantees that a future parser refactor cannot break this
    // entry point without failing a test here.
    #[test]
    fn span_stays_importable() {
        let _ = Span::new(0..1, 1..2);
    }
}
