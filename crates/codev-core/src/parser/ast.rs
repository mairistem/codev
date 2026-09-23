use std::ops::Range;

/// L'intervalle qu'occupe un nœud dans le texte source.
///
/// Deux vues du même intervalle, et c'est délibéré : `byte_range` sert aux
/// réécritures au caractère près, `line_range` sert aux messages. Les deux se
/// déduisent l'un de l'autre à la construction, une seule fois, plutôt que le
/// consommateur le refasse à chaque diagnostic.
///
/// Les lignes sont **1-indexées** (comme un éditeur les affiche), les octets
/// sont 0-indexés (comme `str::get`). Le fait qu'ils diffèrent est le prix à
/// payer pour ne pas mentir aux deux côtés.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub byte_range: Range<usize>,
    pub line_range: Range<u32>,
}

impl Span {
    pub fn new(byte_range: Range<usize>, line_range: Range<u32>) -> Self {
        Self { byte_range, line_range }
    }

    pub fn start_line(&self) -> u32 {
        self.line_range.start
    }
}

/// Le bloc `## Purpose` d'une spec principale, ou d'un delta de nouvelle
/// capacité.
///
/// Le texte est fourni **nettoyé** — sans son en-tête, sans blanc de fin —
/// pour qu'un consommateur puisse l'insérer directement. Le span, lui, couvre
/// l'en-tête inclus, pour qu'une réécriture remplace le bloc en entier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurposeBlock {
    pub text: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Requirement {
    pub name: String,
    /// Le paragraphe descriptif entre l'en-tête `### Requirement:` et le
    /// premier `#### Scenario:`.
    pub description: String,
    pub scenarios: Vec<Scenario>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scenario {
    pub name: String,
    /// Les lignes du corps du scénario, sans l'en-tête.
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

/// L'opération que déclare un en-tête `## <OP> Requirements`.
///
/// Un `enum` fermé plutôt qu'une `String` : ajouter une cinquième opération
/// est une décision structurante — cf. l'avertissement dans `design.md` — pas
/// un simple ajout de variante à faire distraitement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltaOp {
    Added,
    Modified,
    Removed,
    Renamed,
}

/// Une section du delta, portant sa charge utile spécifique.
///
/// Un enum plutôt qu'une structure aux champs conditionnels : le compilateur
/// interdit alors « `Removed` avec des scénarios », qui n'a aucun sens.
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

/// Une spec principale lue depuis `_codev/specs/<capability>/spec.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spec {
    pub purpose: Option<PurposeBlock>,
    pub requirements: Vec<Requirement>,
    /// Point d'insertion des ADDED : offset en octets juste avant la prochaine
    /// section `##` de premier niveau après `## Requirements`, ou la longueur
    /// du source si aucune section ne suit.
    ///
    /// `None` quand la spec n'a pas de section `## Requirements` — dans ce
    /// cas, tout `ADDED` doit être refusé en amont par le validateur, et le
    /// merge ne devrait jamais être appelé.
    pub requirements_section_end: Option<usize>,
}

/// Un delta lu depuis `_codev/changes/<name>/specs/<capability>/spec.md`.
///
/// `purpose` n'a de sens que pour une nouvelle capacité — un delta d'une
/// capacité existante qui en porterait un doit être signalé par le validateur
/// (hors périmètre du parseur : ici on l'extrait tel qu'il est).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delta {
    pub purpose: Option<PurposeBlock>,
    pub sections: Vec<DeltaSection>,
}

/// Sévérité d'un défaut structurel.
///
/// `Error` interdit à `sync` et `archive` d'écrire, `Warning` et `Info` sont
/// des observations. La distinction se fait ici, dans les données ; les
/// consommateurs ne l'interprètent pas différemment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Un défaut structurel localisé.
///
/// `code` est stable — un consommateur peut s'y fier ; `message` est libre de
/// reformulation. Mêmes règles que le contrat JSON du CLI, pour les mêmes
/// raisons.
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
}

/// Ce que rend un parseur : la valeur reconstruite — même partielle — et la
/// liste des défauts rencontrés.
///
/// Pas de `Result` : un fichier catastrophiquement illisible se distingue mal
/// d'un fichier partiellement récupérable, et le second est le cas courant.
/// Un consommateur qui refuse la moindre erreur teste `has_errors()`.
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
    fn les_types_dast_se_construisent_a_la_main() {
        // Le test « ça compile » : si une variante manque un champ ou change
        // de forme, ce test tombe le premier, dans un contexte qui rend
        // évident le contrat public.
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
                        reason: Some("obsolète".into()),
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
    fn parsed_distingue_les_erreurs_des_avertissements() {
        let mut parsed = Parsed::new(0u32);
        parsed
            .findings
            .push(Finding::error("x", 1, "message"));
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
