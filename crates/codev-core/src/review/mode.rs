//! How the reviewing roles run: in the conversation that wrote the work
//! (`inline`), or each alone, given only its role and the material
//! (`isolated`). The project sets it under `review:`, a change may
//! override it in its `change.yaml`; the skills read the result from
//! `codev roles`.

/// The review mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReviewMode {
    /// Every role reviews in the conversation, as before the mode existed.
    #[default]
    Inline,
    /// Every role reviews as a separate sub-agent that never sees the
    /// conversation; the skill then merges their findings.
    Isolated,
}

impl ReviewMode {
    pub const ALL: [Self; 2] = [Self::Inline, Self::Isolated];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Isolated => "isolated",
        }
    }

    /// The mode named `text`, or a message naming the allowed values.
    pub fn parse(text: &str) -> Result<Self, String> {
        Self::ALL
            .into_iter()
            .find(|mode| mode.as_str() == text)
            .ok_or_else(|| format!("`{text}` is not a review mode; use {}", Self::allowed()))
    }

    /// `inline or isolated`, for messages.
    pub fn allowed() -> String {
        let names: Vec<&str> = Self::ALL.iter().map(|m| m.as_str()).collect();
        names.join(" or ")
    }
}

/// Where the effective mode comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeOrigin {
    Default,
    Project,
    Change,
}

impl ModeOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Project => "project",
            Self::Change => "change",
        }
    }
}

/// The change's mode when it sets one, otherwise the project's, otherwise
/// `inline` — the precedence reads in that order below.
pub fn resolve(
    project: Option<ReviewMode>,
    change: Option<ReviewMode>,
) -> (ReviewMode, ModeOrigin) {
    change
        .map(|mode| (mode, ModeOrigin::Change))
        .or(project.map(|mode| (mode, ModeOrigin::Project)))
        .unwrap_or((ReviewMode::Inline, ModeOrigin::Default))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_mode_is_inline() {
        assert_eq!(
            resolve(None, None),
            (ReviewMode::Inline, ModeOrigin::Default)
        );
    }

    #[test]
    fn the_project_mode_applies_without_an_override() {
        assert_eq!(
            resolve(Some(ReviewMode::Isolated), None),
            (ReviewMode::Isolated, ModeOrigin::Project)
        );
    }

    #[test]
    fn a_change_overrides_the_project_mode() {
        assert_eq!(
            resolve(Some(ReviewMode::Isolated), Some(ReviewMode::Inline)),
            (ReviewMode::Inline, ModeOrigin::Change)
        );
        assert_eq!(
            resolve(None, Some(ReviewMode::Isolated)),
            (ReviewMode::Isolated, ModeOrigin::Change)
        );
    }

    #[test]
    fn an_unknown_mode_is_refused_with_the_allowed_values() {
        assert_eq!(ReviewMode::parse("isolated"), Ok(ReviewMode::Isolated));
        assert_eq!(ReviewMode::parse("inline"), Ok(ReviewMode::Inline));
        let message = ReviewMode::parse("parallel").unwrap_err();
        assert!(message.contains("`parallel`"), "{message}");
        assert!(message.contains("inline or isolated"), "{message}");
        assert!(
            ReviewMode::parse("Isolated").is_err(),
            "values are lower case"
        );
    }
}
