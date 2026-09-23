//! Rendu markdown des blocs manipulés par le merge.
//!
//! Un canon strict, pour que deux syncs successifs donnent le même contenu
//! au caractère près. Les blocs sont écrits dans un format canonique — sans
//! chercher à imiter l'espacement d'une spec existante voisine, ce qui serait
//! source de dérive silencieuse.

use crate::parser::ast::{Requirement, Scenario};

/// Rendu d'une exigence complète.
///
/// Format :
///
/// ```text
/// ### Requirement: <name>
///
/// <description>
///
/// #### Scenario: <name>
///
/// <body>
///
/// #### Scenario: <name>
///
/// <body>
/// ```
///
/// La dernière ligne se termine sans blanc supplémentaire — c'est l'appelant
/// (typiquement `merge::edits`) qui ajuste l'espacement de séparation avec le
/// bloc suivant.
pub fn requirement(req: &Requirement) -> String {
    let mut out = String::new();
    out.push_str("### Requirement: ");
    out.push_str(&req.name);
    out.push('\n');

    if !req.description.trim().is_empty() {
        out.push('\n');
        out.push_str(req.description.trim_end());
        out.push('\n');
    }

    for scenario in &req.scenarios {
        out.push('\n');
        out.push_str(&scenario_str(scenario));
    }

    out
}

fn scenario_str(scenario: &Scenario) -> String {
    let mut out = String::new();
    out.push_str("#### Scenario: ");
    out.push_str(&scenario.name);
    out.push('\n');
    if !scenario.body.trim().is_empty() {
        out.push('\n');
        out.push_str(scenario.body.trim_end());
        out.push('\n');
    }
    out
}

/// Titre d'une spec principale dérivé du chemin de la capacité.
///
/// `identity/user-auth` → `User Auth`. On ne cherche pas à préserver le
/// chemin complet dans le titre — le nom du dossier suffit et le fichier vit
/// à l'endroit qui le désambiguïse.
pub fn spec_title_from_capability(capability_path: &str) -> String {
    let last = capability_path
        .rsplit('/')
        .next()
        .unwrap_or(capability_path);
    last.split('-')
        .filter(|s| !s.is_empty())
        .map(title_case)
        .collect::<Vec<_>>()
        .join(" ")
}

fn title_case(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Span;

    fn span() -> Span {
        Span::new(0..0, 1..1)
    }

    #[test]
    fn render_requirement_est_stable() {
        let req = Requirement {
            name: "Login".into(),
            description: "The system SHALL emit a token.".into(),
            scenarios: vec![Scenario {
                name: "OK".into(),
                body: "- **WHEN** login\n- **THEN** token".into(),
                span: span(),
            }],
            span: span(),
        };
        let attendu = "### Requirement: Login\n\n\
                       The system SHALL emit a token.\n\n\
                       #### Scenario: OK\n\n\
                       - **WHEN** login\n\
                       - **THEN** token\n";
        assert_eq!(requirement(&req), attendu);
    }

    #[test]
    fn render_avec_deux_scenarios() {
        let req = Requirement {
            name: "Auth".into(),
            description: "The system MUST authenticate.".into(),
            scenarios: vec![
                Scenario {
                    name: "OK".into(),
                    body: "- **WHEN** valid\n- **THEN** ok".into(),
                    span: span(),
                },
                Scenario {
                    name: "KO".into(),
                    body: "- **WHEN** invalid\n- **THEN** ko".into(),
                    span: span(),
                },
            ],
            span: span(),
        };
        let rendu = requirement(&req);
        assert!(rendu.contains("#### Scenario: OK"));
        assert!(rendu.contains("#### Scenario: KO"));
        // Une ligne blanche entre les deux scénarios.
        assert!(rendu.contains("- **THEN** ok\n\n#### Scenario: KO"));
    }

    #[test]
    fn titre_depuis_capacite_flat() {
        assert_eq!(spec_title_from_capability("user-auth"), "User Auth");
    }

    #[test]
    fn titre_depuis_capacite_imbriquee() {
        // On prend le dernier segment — le chemin complet resterait dans le
        // dossier, pas besoin de le dupliquer dans le titre.
        assert_eq!(
            spec_title_from_capability("identity/user-auth"),
            "User Auth"
        );
    }

    #[test]
    fn titre_avec_segment_vide_est_robuste() {
        assert_eq!(spec_title_from_capability("--x"), "X");
        assert_eq!(spec_title_from_capability(""), "");
    }
}
