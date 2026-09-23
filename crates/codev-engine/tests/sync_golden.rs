//! Tests d'intégration de fusion — sur la fonction pure du cœur.
//!
//! Aucun `FileSystem`, aucun engine : on parse la spec et le delta, on
//! calcule le `MergePlan`, on applique. C'est ce qui rend ces tests
//! rapides et lisibles ; les invariants qu'ils gardent sont ce qui protège
//! `sync` et `archive` d'une réécriture destructive.

use codev_core::merge::{apply_edits, merge_into_existing};
use codev_core::parser::{parse_delta, parse_spec};

const SPEC_BEFORE: &str = include_str!("fixtures/sync/spec_before.md");

fn merge(spec_source: &str, delta_source: &str) -> String {
    let spec = parse_spec(spec_source);
    assert!(
        !spec.has_errors(),
        "spec doit être valide : {:?}",
        spec.findings
    );
    let delta = parse_delta(delta_source);
    assert!(
        !delta.has_errors(),
        "delta doit être valide : {:?}",
        delta.findings
    );
    let plan =
        merge_into_existing(spec_source, &spec.value, &delta.value, false).expect("merge doit réussir");
    apply_edits(spec_source, &plan.edits)
}

#[test]
fn added_ajoute() {
    let delta = "## ADDED Requirements\n\n### Requirement: Two-Factor\nThe system MUST support 2FA.\n\n#### Scenario: Enrolment\n- **WHEN** user enables\n- **THEN** QR code shown\n";
    let out = merge(SPEC_BEFORE, delta);
    // Nouveau bloc présent.
    assert!(out.contains("### Requirement: Two-Factor"));
    // Il vient bien APRÈS l'existant, AVANT la section Notes.
    let login_pos = out.find("### Requirement: Login").unwrap();
    let twofactor_pos = out.find("### Requirement: Two-Factor").unwrap();
    let notes_pos = out.find("## Notes").unwrap();
    assert!(login_pos < twofactor_pos);
    assert!(twofactor_pos < notes_pos);
}

#[test]
fn modified_remplace() {
    let delta = "## MODIFIED Requirements\n\n### Requirement: Session Expiration\nThe system MUST expire sessions after 15 minutes.\n\n#### Scenario: Idle timeout\n- **WHEN** 15 minutes pass without activity\n- **THEN** the session is invalidated\n";
    let out = merge(SPEC_BEFORE, delta);
    assert!(out.contains("15 minutes"));
    assert!(!out.contains("30 minutes"));
    // Login intact.
    assert!(out.contains("### Requirement: Login"));
    assert!(out.contains("The system SHALL emit a token upon successful login."));
}

#[test]
fn removed_supprime() {
    let delta = "## REMOVED Requirements\n\n### Requirement: Session Expiration\n**Reason**: obsolete\n**Migration**: remplacé par 2FA\n";
    let out = merge(SPEC_BEFORE, delta);
    assert!(!out.contains("### Requirement: Session Expiration"));
    // Login reste.
    assert!(out.contains("### Requirement: Login"));
}

#[test]
fn renamed_retitle() {
    let delta = "## RENAMED Requirements\n\n- FROM: Session Expiration\n- TO: Session Timeout\n";
    let out = merge(SPEC_BEFORE, delta);
    assert!(out.contains("### Requirement: Session Timeout"));
    assert!(!out.contains("### Requirement: Session Expiration"));
    // Le corps de l'exigence est intact au caractère près.
    assert!(out.contains("The system MUST expire sessions after 30 minutes."));
    assert!(out.contains("- **WHEN** 30 minutes pass without activity"));
}

#[test]
fn multi_ops_est_deterministe() {
    // Un delta touchant plusieurs opérations : ADDED + MODIFIED. Le résultat
    // doit être stable et cohérent.
    let delta = "## ADDED Requirements\n\n### Requirement: Two-Factor\nThe system MUST support 2FA.\n\n#### Scenario: Enrolment\n- **WHEN** user enables\n- **THEN** QR code shown\n\n## MODIFIED Requirements\n\n### Requirement: Session Expiration\nThe system MUST expire sessions after 15 minutes.\n\n#### Scenario: Idle timeout\n- **WHEN** 15 minutes pass without activity\n- **THEN** the session is invalidated\n";
    let out = merge(SPEC_BEFORE, delta);
    assert!(out.contains("Two-Factor"));
    assert!(out.contains("15 minutes"));
    // Un deuxième passage donne le même résultat.
    let out2 = merge(&out, delta);
    assert_eq!(out, out2, "multi-ops doit être idempotent");
}

#[test]
fn une_section_libre_apres_requirements_survit() {
    // C'est l'invariant qui garantit la préservation du contenu non
    // mentionné : la section `## Notes` doit rester au caractère près.
    let delta = "## MODIFIED Requirements\n\n### Requirement: Login\nThe system SHALL emit a JWT.\n\n#### Scenario: Valid credentials\n- **WHEN** the user submits valid credentials\n- **THEN** a JWT is returned\n";
    let out = merge(SPEC_BEFORE, delta);
    assert!(out.contains("## Notes\n\nFree-form notes that must survive any sync.\n"));
}

#[test]
fn commentaire_html_dans_exigence_non_touchee_survit() {
    // Une exigence contient un commentaire HTML dans son texte. Si un autre
    // MODIFIED touche une autre exigence, le commentaire doit être intact.
    let spec_avec_commentaire = "## Purpose\n\nCap.\n\n## Requirements\n\n### Requirement: R1\nThe system SHALL x.\n<!-- note interne à préserver -->\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: R2\nThe system SHALL y.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
    let delta = "## MODIFIED Requirements\n\n### Requirement: R2\nThe system MUST z.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
    let out = merge(spec_avec_commentaire, delta);
    assert!(out.contains("<!-- note interne à préserver -->"));
}

#[test]
fn deux_syncs_successifs_donnent_le_meme_resultat() {
    // L'invariant d'idempotence, sur un cas plus riche.
    let delta = "## MODIFIED Requirements\n\n### Requirement: Login\nThe system SHALL emit a JWT token.\n\n#### Scenario: Valid credentials\n- **WHEN** the user submits valid credentials\n- **THEN** a JWT token is returned\n";
    let apres_un = merge(SPEC_BEFORE, delta);
    let apres_deux = merge(&apres_un, delta);
    assert_eq!(apres_un, apres_deux);
}
