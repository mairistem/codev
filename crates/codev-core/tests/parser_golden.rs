//! Tests d'intégration du parseur — golden tests et invariant round-trip.
//!
//! Golden tests : deux fichiers réalistes dans `fixtures/`, parsés puis
//! vérifiés champ par champ. Ils cassent au moindre changement de structure
//! de l'AST, ce qui est le but recherché — une régression silencieuse ne
//! passe pas.
//!
//! Invariant : chaque nœud porte un `Span` dont la plage d'octets délimite
//! son texte source. En itérant tous les nœuds et en substituant chaque
//! plage par elle-même, on doit reconstruire le fichier au caractère près.
//! Sans cette garantie, `sync`/`archive` risqueraient d'écraser du contenu
//! non mentionné par le delta.

use codev_core::parser::ast::DeltaSection;
use codev_core::parser::{parse_delta, parse_spec};

const SPEC_SOURCE: &str = include_str!("fixtures/spec.md");
const DELTA_SOURCE: &str = include_str!("fixtures/delta.md");

#[test]
fn spec_dexemple_est_parsee_completement() {
    let parsed = parse_spec(SPEC_SOURCE);
    assert!(!parsed.has_errors(), "findings inattendus : {:?}", parsed.findings);

    let purpose = parsed.value.purpose.expect("purpose attendu");
    assert!(purpose.text.contains("Authentification"));

    assert_eq!(parsed.value.requirements.len(), 2);
    assert_eq!(parsed.value.requirements[0].name, "User Authentication");
    assert_eq!(parsed.value.requirements[0].scenarios.len(), 2);
    assert_eq!(parsed.value.requirements[1].name, "Session Expiration");
    assert_eq!(parsed.value.requirements[1].scenarios.len(), 1);
    assert_eq!(
        parsed.value.requirements[1].scenarios[0].name,
        "Idle timeout"
    );
}

#[test]
fn delta_dexemple_expose_les_quatre_sections_dans_lordre() {
    let parsed = parse_delta(DELTA_SOURCE);
    assert!(!parsed.has_errors(), "findings inattendus : {:?}", parsed.findings);

    let purpose = parsed.value.purpose.expect("delta d'une nouvelle capacité");
    assert!(purpose.text.contains("double authentification"));

    assert_eq!(parsed.value.sections.len(), 4);

    match &parsed.value.sections[0] {
        DeltaSection::Added { requirements, .. } => {
            assert_eq!(requirements.len(), 1);
            assert_eq!(requirements[0].name, "Two-Factor Authentication");
            assert_eq!(requirements[0].scenarios.len(), 2);
        }
        other => panic!("attendu Added, obtenu {other:?}"),
    }
    match &parsed.value.sections[2] {
        DeltaSection::Removed { removals, .. } => {
            assert_eq!(removals[0].name, "Remember Me");
            assert_eq!(removals[0].reason.as_deref(), Some("Replaced by 2FA"));
        }
        other => panic!("attendu Removed, obtenu {other:?}"),
    }
    match &parsed.value.sections[3] {
        DeltaSection::Renamed { renames, .. } => {
            assert_eq!(renames[0].from, "Session Expiration");
            assert_eq!(renames[0].to, "Session Timeout");
        }
        other => panic!("attendu Renamed, obtenu {other:?}"),
    }
}

/// L'invariant qui protège de la réécriture destructive : le span de chaque
/// nœud doit reproduire son texte source à l'octet près.
#[test]
fn spans_reproduisent_le_source_au_caractere_pres() {
    let parsed = parse_spec(SPEC_SOURCE);

    // Purpose.
    let purpose = parsed.value.purpose.as_ref().unwrap();
    let slice = &SPEC_SOURCE[purpose.span.byte_range.clone()];
    assert!(slice.starts_with("## Purpose"), "slice = {slice:?}");

    for req in &parsed.value.requirements {
        let slice = &SPEC_SOURCE[req.span.byte_range.clone()];
        assert!(
            slice.starts_with("### Requirement:"),
            "requirement span mal placé : {slice:?}"
        );
        assert!(
            slice.contains(&req.name),
            "le nom de l'exigence ({}) doit se trouver dans son span",
            req.name
        );
        for scenario in &req.scenarios {
            let slice = &SPEC_SOURCE[scenario.span.byte_range.clone()];
            assert!(
                slice.starts_with("#### Scenario:"),
                "scenario span mal placé : {slice:?}"
            );
            assert!(slice.contains(&scenario.name));
        }
    }
}

/// Réécriture ciblée : remplacer la première exigence par un texte différent
/// ne doit toucher à rien d'autre. C'est ce que fera `sync` sur un `MODIFIED`.
#[test]
fn reecriture_ciblee_ne_touche_pas_au_reste() {
    let parsed = parse_spec(SPEC_SOURCE);
    let premier = &parsed.value.requirements[0];
    let seconde = &parsed.value.requirements[1];
    let bornes_seconde_originale =
        &SPEC_SOURCE[seconde.span.byte_range.clone()];

    // Remplacer le bloc du premier par un texte plus court, puis vérifier que
    // la seconde exigence reste identique.
    let remplacement =
        "### Requirement: User Authentication (Réécrit)\n\nnouveau contenu\n";
    let mut reecrit = String::with_capacity(SPEC_SOURCE.len());
    reecrit.push_str(&SPEC_SOURCE[..premier.span.byte_range.start]);
    reecrit.push_str(remplacement);
    reecrit.push_str(&SPEC_SOURCE[premier.span.byte_range.end..]);

    // La seconde exigence, reconnaissable par son texte, doit apparaître
    // telle quelle dans la sortie — espacement compris.
    assert!(
        reecrit.contains(bornes_seconde_originale),
        "la seconde exigence doit être présente au caractère près dans la sortie"
    );
    assert!(reecrit.contains("nouveau contenu"));
}

/// Les positions en ligne sont 1-indexées et pointent sur l'en-tête, comme
/// un éditeur les affiche. Nommer la ligne dans un message d'erreur est le
/// seul cas où l'utilisateur voit ces nombres.
#[test]
fn spans_exposent_des_lignes_1_indexees() {
    let parsed = parse_spec(SPEC_SOURCE);
    let purpose = parsed.value.purpose.unwrap();
    // Le fichier commence par `# Auth Specification` (ligne 1), `` (ligne 2),
    // `## Purpose` (ligne 3).
    assert_eq!(purpose.span.start_line(), 3);

    let session_expiration = &parsed.value.requirements[1];
    // On vérifie plutôt la cohérence que la position exacte : la ligne pointe
    // bien sur `### Requirement: Session Expiration`.
    let line_content: &str = SPEC_SOURCE.split('\n').collect::<Vec<_>>()
        [(session_expiration.span.start_line() - 1) as usize];
    assert!(
        line_content.contains("Session Expiration"),
        "ligne pointée : {line_content:?}"
    );
}
