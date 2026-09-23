use std::path::PathBuf;

use codev_core::parser::ast::{Finding, Severity};

/// Un finding, augmenté du chemin du fichier auquel il s'applique.
///
/// Le type sépare la préoccupation « où » (ici, dans l'engine) de la
/// préoccupation « quoi » (dans le parseur ou dans une règle). Casser le
/// `Finding` du cœur pour y ajouter un `path` obligerait chaque call site à
/// porter un chemin qui n'a de sens qu'à l'échelle d'un projet.
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

/// Ce qu'est un item : un change actif, ou une spec principale.
///
/// Un `enum` plutôt qu'une `String` : le rendu humain et le contrat JSON
/// s'en servent, et on veut que le compilateur nous prévienne quand un
/// troisième type d'item apparaîtra (les décisions, par exemple).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Change,
    Spec,
    /// L'ensemble des décisions du projet, vu comme un item unique — les
    /// findings de scellement pointent vers l'ADR ou vers `seal.yaml`
    /// selon le cas.
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

    /// `true` dès qu'au moins un finding porte la sévérité `Warning`.
    /// Symétrique de `has_errors` — utile pour le mode strict de la CLI.
    pub fn has_warnings(&self) -> bool {
        self.findings.iter().any(|f| f.finding.severity == Severity::Warning)
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

    /// `true` dès qu'au moins un `ItemReport` contient un finding
    /// `Warning`. Sert au mode strict de `codev validate` — le CLI ne
    /// modifie pas la sévérité, il compose ce booléen avec le flag.
    pub fn has_warnings(&self) -> bool {
        self.items.iter().any(ItemReport::has_warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codev_core::parser::ast::Span;

    #[test]
    fn located_conserve_code_line_severite() {
        let finding = Finding {
            severity: Severity::Error,
            code: "un_code",
            line: 42,
            message: "un message".into(),
        };
        let located = LocatedFinding::from_finding(finding, PathBuf::from("_codev/x.md"));
        assert!(located.is_error());
        assert_eq!(located.finding.code, "un_code");
        assert_eq!(located.finding.line, 42);
        assert_eq!(located.path, PathBuf::from("_codev/x.md"));
    }

    #[test]
    fn has_warnings_est_vrai_sur_un_warning_isole() {
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
    fn has_warnings_est_faux_sur_une_erreur_seule() {
        // Les erreurs ne sont pas des warnings — le mode strict distinguera.
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
    fn has_warnings_sur_report_vide_est_faux() {
        let report = ValidateReport {
            root: PathBuf::from("/p"),
            items: vec![],
        };
        assert!(!report.has_warnings());
    }

    #[test]
    fn item_report_signale_un_error_meme_isole() {
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

        // Un warning ne fait pas basculer.
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

    // Test de compilation : `Span` reste importable de la même façon depuis
    // le cœur — garantit qu'un futur refactor du parseur ne casse pas ce
    // point d'entrée sans faire échouer un test ici.
    #[test]
    fn span_reste_importable() {
        let _ = Span::new(0..1, 1..2);
    }
}
