use std::ops::Range;

/// The range a node occupies in the source text.
///
/// Two views of the same range, and that is deliberate: `byte_range` serves
/// character-exact rewrites, `line_range` serves messages. Each is derived
/// from the other at construction, once, rather than having the consumer
/// redo it for every diagnostic.
///
/// Lines are **1-indexed** (as an editor displays them), bytes are
/// 0-indexed (as with `str::get`). Their difference is the price to pay for
/// not lying to either side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub byte_range: Range<usize>,
    pub line_range: Range<u32>,
}

impl Span {
    pub fn new(byte_range: Range<usize>, line_range: Range<u32>) -> Self {
        Self {
            byte_range,
            line_range,
        }
    }

    pub fn start_line(&self) -> u32 {
        self.line_range.start
    }
}

/// The `## Purpose` block of a main spec, or of a new-capability delta.
///
/// The text is provided **cleaned** — without its heading, without trailing
/// whitespace — so that a consumer can insert it directly. The span, on the
/// other hand, includes the heading, so that a rewrite replaces the whole
/// block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurposeBlock {
    pub text: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Requirement {
    pub name: String,
    /// The descriptive paragraph between the `### Requirement:` heading and
    /// the first `#### Scenario:`.
    pub description: String,
    pub scenarios: Vec<Scenario>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scenario {
    pub name: String,
    /// The lines of the scenario body, without the heading.
    pub body: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Removal {
    pub name: String,
    pub reason: Option<String>,
    pub migration: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rename {
    pub from: String,
    pub to: String,
    pub span: Span,
}

/// The operation declared by a `## <OP> Requirements` heading.
///
/// A closed `enum` rather than a `String`: adding a fifth operation is a
/// structural decision — see the warning in `design.md` — not a mere
/// variant to add absent-mindedly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltaOp {
    Added,
    Modified,
    Removed,
    Renamed,
}

/// A delta section, carrying its specific payload.
///
/// An enum rather than a struct with conditional fields: the compiler then
/// forbids "`Removed` with scenarios", which makes no sense.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeltaSection {
    Added {
        requirements: Vec<Requirement>,
        span: Span,
    },
    Modified {
        requirements: Vec<Requirement>,
        span: Span,
    },
    Removed {
        removals: Vec<Removal>,
        span: Span,
    },
    Renamed {
        renames: Vec<Rename>,
        span: Span,
    },
}

impl DeltaSection {
    pub fn op(&self) -> DeltaOp {
        match self {
            Self::Added { .. } => DeltaOp::Added,
            Self::Modified { .. } => DeltaOp::Modified,
            Self::Removed { .. } => DeltaOp::Removed,
            Self::Renamed { .. } => DeltaOp::Renamed,
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            Self::Added { span, .. }
            | Self::Modified { span, .. }
            | Self::Removed { span, .. }
            | Self::Renamed { span, .. } => span,
        }
    }
}

/// A main spec read from `_codev/specs/<capability>/spec.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spec {
    pub purpose: Option<PurposeBlock>,
    pub requirements: Vec<Requirement>,
    /// Insertion point for ADDED entries: byte offset just before the next
    /// top-level `##` section after `## Requirements`, or the length of the
    /// source if no section follows.
    ///
    /// `None` when the spec has no `## Requirements` section — in that case,
    /// any `ADDED` must be rejected upstream by the validator, and the merge
    /// should never be called.
    pub requirements_section_end: Option<usize>,
}

/// A delta read from `_codev/changes/<name>/specs/<capability>/spec.md`.
///
/// `purpose` only makes sense for a new capability — a delta for an
/// existing capability that carries one must be reported by the validator
/// (outside the parser's scope: here it is extracted as-is).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delta {
    pub purpose: Option<PurposeBlock>,
    pub sections: Vec<DeltaSection>,
}

/// Severity of a structural defect.
///
/// `Error` forbids `sync` and `archive` from writing, `Warning` and `Info`
/// are observations. The distinction is made here, in the data; consumers
/// do not interpret it differently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// A located structural defect.
///
/// `code` is stable — a consumer can rely on it; `message` may be reworded
/// freely. Same rules as the CLI's JSON contract, for the same reasons.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub severity: Severity,
    pub code: &'static str,
    pub line: u32,
    pub message: String,
}

impl Finding {
    pub fn error(code: &'static str, line: u32, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            line,
            message: message.into(),
        }
    }

    pub fn warning(code: &'static str, line: u32, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            line,
            message: message.into(),
        }
    }
}

/// What a parser returns: the reconstructed value — even partial — and the
/// list of defects encountered.
///
/// No `Result`: a catastrophically unreadable file is hard to tell apart
/// from a partially recoverable one, and the latter is the common case.
/// A consumer that refuses any error checks `has_errors()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parsed<T> {
    pub value: T,
    pub findings: Vec<Finding>,
}

impl<T> Parsed<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            findings: Vec::new(),
        }
    }

    pub fn has_errors(&self) -> bool {
        self.findings.iter().any(|f| f.severity == Severity::Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ast_types_can_be_built_by_hand() {
        // The "it compiles" test: if a variant is missing a field or changes
        // shape, this test fails first, in a context that makes the public
        // contract obvious.
        let span = Span::new(0..10, 1..2);
        assert_eq!(span.start_line(), 1);

        let scenario = Scenario {
            name: "S".into(),
            body: String::new(),
            span: span.clone(),
        };
        let requirement = Requirement {
            name: "R".into(),
            description: String::new(),
            scenarios: vec![scenario],
            span: span.clone(),
        };
        let spec = Spec {
            purpose: Some(PurposeBlock {
                text: "P".into(),
                span: span.clone(),
            }),
            requirements: vec![requirement],
            requirements_section_end: Some(10),
        };
        assert_eq!(spec.requirements.len(), 1);

        let delta = Delta {
            purpose: None,
            sections: vec![
                DeltaSection::Added {
                    requirements: Vec::new(),
                    span: span.clone(),
                },
                DeltaSection::Removed {
                    removals: vec![Removal {
                        name: "X".into(),
                        reason: Some("obsolete".into()),
                        migration: None,
                        span: span.clone(),
                    }],
                    span: span.clone(),
                },
                DeltaSection::Renamed {
                    renames: vec![Rename {
                        from: "A".into(),
                        to: "B".into(),
                        span: span.clone(),
                    }],
                    span: span.clone(),
                },
            ],
        };
        assert_eq!(delta.sections[0].op(), DeltaOp::Added);
        assert_eq!(delta.sections[1].op(), DeltaOp::Removed);
    }

    #[test]
    fn parsed_distinguishes_errors_from_warnings() {
        let mut parsed = Parsed::new(0u32);
        parsed.findings.push(Finding::error("x", 1, "message"));
        assert!(parsed.has_errors());

        let info = Parsed {
            value: 0u32,
            findings: vec![Finding {
                severity: Severity::Info,
                code: "y",
                line: 3,
                message: "note".into(),
            }],
        };
        assert!(!info.has_errors());
    }
}
