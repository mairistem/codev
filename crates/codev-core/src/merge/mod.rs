//! Fusion sémantique d'un delta dans une spec principale, sans aucune I/O.
//!
//! Deux entrées publiques :
//!
//! - [`merge_into_existing`] pour une capacité qui a déjà une spec principale ;
//! - [`build_new_spec`] pour une nouvelle capacité (crée le fichier à partir
//!   du `## Purpose` du delta et de ses `ADDED`).
//!
//! Les deux fonctions ne touchent pas au disque : la première produit un
//! [`MergePlan`] (liste d'édits sur la source existante), la seconde rend le
//! contenu complet du fichier à créer. La coquille (`codev-engine::sync`)
//! s'occupe d'exécuter.

pub mod edits;
pub mod render;

pub use edits::{apply_edits, Edit};

use std::collections::BTreeMap;

use crate::parser::ast::{Delta, DeltaSection, Spec};

/// Le plan de fusion pour **une** spec principale existante.
///
/// Une fois calculé, l'appelant applique les édits sur la source de la spec ;
/// le résultat est le nouveau contenu à écrire (ou à comparer pour
/// idempotence).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergePlan {
    pub edits: Vec<Edit>,
    /// La spec entière doit-elle être supprimée du disque ?
    ///
    /// Vaut `true` quand le change porte `retire_capabilities: true` et
    /// qu'après application des REMOVED, la spec principale n'a plus
    /// aucune exigence. La coquille route cette information : le cœur
    /// ne connaît pas le chemin absolu du fichier.
    pub should_delete_spec: bool,
}

impl MergePlan {
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty() && !self.should_delete_spec
    }
}

/// Ce qui peut mal tourner pendant la fusion.
///
/// Les variantes portent un `code` stable exposable dans le contrat JSON,
/// mêmes règles que dans le validateur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeError {
    /// Un `MODIFIED` cible une exigence qui n'existe pas dans la spec principale.
    ModifiedTargetMissing { name: String },
    /// Un `REMOVED` viderait la spec de toute exigence.
    ///
    /// Le geste demande `retire_capabilities: true` dans le `change.yaml`,
    /// et la suppression du fichier de spec — deux choses que ce lot ne
    /// livre pas encore. On refuse plutôt que d'écrire une spec dégénérée.
    WouldLeaveSpecWithoutRequirement { name: String },
    /// Un delta cible une capacité qui n'a pas de spec principale, mais ne
    /// porte pas de `## Purpose` — obligatoire pour créer le fichier.
    NewCapabilityWithoutPurpose,
    /// La spec principale existante n'a pas de section `## Requirements` où
    /// insérer un `ADDED`. Cas rare — indique une spec malformée qu'il faut
    /// corriger à la main avant sync.
    NoRequirementsSection,
}

impl MergeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ModifiedTargetMissing { .. } => "modified_target_missing",
            Self::WouldLeaveSpecWithoutRequirement { .. } => "would_leave_spec_without_requirement",
            Self::NewCapabilityWithoutPurpose => "new_capability_without_purpose",
            Self::NoRequirementsSection => "no_requirements_section",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::ModifiedTargetMissing { name } => format!(
                "MODIFIED : l'exigence « {name} » n'existe pas dans la spec principale — \
                 utilise ADDED si elle est nouvelle, ou corrige le nom"
            ),
            Self::WouldLeaveSpecWithoutRequirement { name } => format!(
                "REMOVED : supprimer « {name} » laisserait la spec sans aucune exigence ; \
                 utilise `retire_capabilities: true` dans `change.yaml` pour retirer la capacité"
            ),
            Self::NewCapabilityWithoutPurpose => "nouvelle capacité : le delta doit porter `## Purpose` pour créer la spec principale".into(),
            Self::NoRequirementsSection => "la spec principale n'a pas de section `## Requirements` où insérer un ADDED".into(),
        }
    }
}

impl std::fmt::Display for MergeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message())
    }
}

impl std::error::Error for MergeError {}

/// Construit le plan de fusion d'un delta dans une spec principale existante.
///
/// Ne touche pas au disque. Aucune écriture n'est faite si la fonction rend
/// `Err` — l'atomicité annoncée dans `_codev/decisions/0001` en dépend.
pub fn merge_into_existing(
    spec_source: &str,
    spec: &Spec,
    delta: &Delta,
    retire_capabilities: bool,
) -> Result<MergePlan, MergeError> {
    let _ = spec_source; // conservé pour usage futur (validation cross-section) ; l'AST porte déjà les spans
    let mut edits = Vec::new();

    // Indexer les exigences de la spec par nom, pour retrouver MODIFIED et REMOVED
    // en O(log n).
    let by_name: BTreeMap<&str, usize> = spec
        .requirements
        .iter()
        .enumerate()
        .map(|(idx, r)| (r.name.as_str(), idx))
        .collect();

    // Compter les REMOVED pour détecter le vidage total avant d'écrire.
    let mut removed_count = 0usize;

    for section in &delta.sections {
        match section {
            DeltaSection::Modified { requirements, .. } => {
                for new_req in requirements {
                    let Some(&idx) = by_name.get(new_req.name.as_str()) else {
                        return Err(MergeError::ModifiedTargetMissing {
                            name: new_req.name.clone(),
                        });
                    };
                    let target = &spec.requirements[idx];
                    edits.push(Edit::new(
                        target.span.byte_range.clone(),
                        render::requirement(new_req),
                    ));
                }
            }
            DeltaSection::Removed { removals, .. } => {
                for removal in removals {
                    let Some(&idx) = by_name.get(removal.name.as_str()) else {
                        return Err(MergeError::ModifiedTargetMissing {
                            name: removal.name.clone(),
                        });
                    };
                    let target = &spec.requirements[idx];
                    edits.push(Edit::new(
                        target.span.byte_range.clone(),
                        String::new(),
                    ));
                    removed_count += 1;
                }
            }
            DeltaSection::Renamed { renames, .. } => {
                for rename in renames {
                    // Trouver l'exigence par son ancien nom ; retitrer *seulement*
                    // la ligne d'en-tête, pour laisser corps et scénarios intacts.
                    let Some(&idx) = by_name.get(rename.from.as_str()) else {
                        // Renommer une exigence absente est un no-op silencieux —
                        // le validateur (E6, lot 2) le remontera à part.
                        continue;
                    };
                    let target = &spec.requirements[idx];
                    let start = target.span.byte_range.start;
                    // Fin de la ligne d'en-tête : premier `\n` après `start`.
                    let header_end = spec_source[start..]
                        .find('\n')
                        .map(|off| start + off)
                        .unwrap_or(target.span.byte_range.end);
                    let new_header = format!("### Requirement: {}", rename.to);
                    edits.push(Edit::new(start..header_end, new_header));
                }
            }
            DeltaSection::Added { requirements, .. } => {
                // Idempotence : un ADDED dont le nom existe déjà dans la spec
                // principale est traité comme un no-op silencieux.
                // Ré-appliquer un même sync ne doit rien changer, sans quoi
                // l'utilisateur qui relance en cas de doute écrit un doublon.
                // Un ADDED voulu-mais-en-conflit est ce que le validateur
                // (E4/E6, lot 2) remontera en amont, avec un finding
                // dédié — pas ici.
                let a_ajouter: Vec<_> = requirements
                    .iter()
                    .filter(|r| !by_name.contains_key(r.name.as_str()))
                    .collect();
                if a_ajouter.is_empty() {
                    continue;
                }
                let end = spec
                    .requirements_section_end
                    .ok_or(MergeError::NoRequirementsSection)?;
                // On insère avant `end`, précédé d'une ligne blanche si le
                // caractère qui précède n'en est pas déjà une — pour garantir
                // la même densité qu'entre deux exigences voisines.
                let mut inserted = String::new();
                let needs_leading_blank = spec_source[..end]
                    .chars()
                    .last()
                    .map(|c| c != '\n')
                    .unwrap_or(false);
                if needs_leading_blank {
                    inserted.push('\n');
                }
                let already_blank_before = spec_source[..end].ends_with("\n\n");
                if !already_blank_before {
                    inserted.push('\n');
                }
                for (i, new_req) in a_ajouter.iter().enumerate() {
                    if i > 0 {
                        inserted.push('\n');
                    }
                    inserted.push_str(&render::requirement(new_req));
                }
                edits.push(Edit::new(end..end, inserted));
            }
        }
    }

    let would_empty = removed_count > 0 && removed_count >= spec.requirements.len();
    if would_empty && !retire_capabilities {
        // Nom du premier REMOVED comme accroche du message — l'utilisateur
        // sait alors par où commencer.
        let first = delta
            .sections
            .iter()
            .find_map(|s| match s {
                DeltaSection::Removed { removals, .. } => removals.first(),
                _ => None,
            })
            .map(|r| r.name.clone())
            .unwrap_or_default();
        return Err(MergeError::WouldLeaveSpecWithoutRequirement { name: first });
    }

    Ok(MergePlan {
        edits,
        should_delete_spec: would_empty && retire_capabilities,
    })
}

/// Rend le contenu complet d'une **nouvelle** spec principale à partir d'un
/// delta de capacité nouvelle.
///
/// Exige un `## Purpose` — sans lui, on refuserait d'écrire un fichier avec
/// un placeholder que personne n'a validé.
pub fn build_new_spec(capability_path: &str, delta: &Delta) -> Result<String, MergeError> {
    let purpose = delta
        .purpose
        .as_ref()
        .ok_or(MergeError::NewCapabilityWithoutPurpose)?;

    let title = render::spec_title_from_capability(capability_path);
    let mut out = String::new();
    out.push_str("# ");
    out.push_str(&title);
    out.push_str(" Specification\n\n");
    out.push_str("## Purpose\n\n");
    out.push_str(purpose.text.trim());
    out.push_str("\n\n");
    out.push_str("## Requirements\n");

    // Les ADDED (et seulement eux) constituent le corps initial : REMOVED,
    // MODIFIED et RENAMED n'ont pas de sens pour une capacité vierge, et
    // sont ignorés silencieusement — le validateur les aurait signalés.
    for section in &delta.sections {
        if let DeltaSection::Added { requirements, .. } = section {
            for req in requirements {
                out.push('\n');
                out.push_str(&render::requirement(req));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_delta, parse_spec};

    fn parse_pair(spec_source: &str, delta_source: &str) -> (Spec, Delta) {
        let spec = parse_spec(spec_source);
        assert!(
            !spec.has_errors(),
            "spec source doit être valide : {:?}",
            spec.findings
        );
        let delta = parse_delta(delta_source);
        assert!(
            !delta.has_errors(),
            "delta source doit être valide : {:?}",
            delta.findings
        );
        (spec.value, delta.value)
    }

    fn merged(spec_source: &str, delta_source: &str) -> String {
        let (spec, delta) = parse_pair(spec_source, delta_source);
        let plan = merge_into_existing(spec_source, &spec, &delta, false).unwrap();
        apply_edits(spec_source, &plan.edits)
    }

    // ─────────────── MODIFIED ───────────────

    #[test]
    fn modified_remplace_le_bloc() {
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL emit a token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** token\n";
        let delta_source = "## MODIFIED Requirements\n\n### Requirement: Login\nThe system MUST emit a JWT token.\n\n#### Scenario: OK\n- **WHEN** login\n- **THEN** JWT\n";
        let out = merged(spec_source, delta_source);
        assert!(out.contains("The system MUST emit a JWT token"));
        assert!(!out.contains("SHALL emit a token"));
        // Purpose intact.
        assert!(out.contains("## Purpose\n\nx.\n"));
    }

    #[test]
    fn modified_sans_cible_echoue() {
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## MODIFIED Requirements\n\n### Requirement: Fantome\nThe system SHALL y.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let (spec, delta) = parse_pair(spec_source, delta_source);
        let err = merge_into_existing(spec_source, &spec, &delta, false).unwrap_err();
        assert_eq!(err.code(), "modified_target_missing");
        assert!(err.to_string().contains("Fantome"));
    }

    // ─────────────── REMOVED ───────────────

    #[test]
    fn removed_supprime_le_bloc_et_son_espacement() {
        // Deux exigences : on retire la première, la seconde reste au caractère
        // près puisque le span de la première inclut l'espacement qui la sépare
        // de la seconde.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: Session\nThe system SHALL y.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let delta_source = "## REMOVED Requirements\n\n### Requirement: Login\n**Reason**: obsolete\n**Migration**: none\n";
        let out = merged(spec_source, delta_source);
        assert!(!out.contains("Login"));
        assert!(out.contains("### Requirement: Session"));
    }

    #[test]
    fn removed_de_derniere_exigence_echoue_sans_flag() {
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Solo\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## REMOVED Requirements\n\n### Requirement: Solo\n**Reason**: retire\n**Migration**: none\n";
        let (spec, delta) = parse_pair(spec_source, delta_source);
        let err = merge_into_existing(spec_source, &spec, &delta, false).unwrap_err();
        assert_eq!(err.code(), "would_leave_spec_without_requirement");
        // Message met à jour — pointe vers retire_capabilities.
        assert!(err.to_string().contains("retire_capabilities"));
    }

    #[test]
    fn removed_total_avec_flag_retire_la_capacite() {
        // Même contexte que le refus, mais avec `retire_capabilities: true`
        // → plan `should_delete_spec` à `true`, edits appliqués.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Solo\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## REMOVED Requirements\n\n### Requirement: Solo\n**Reason**: retire\n**Migration**: none\n";
        let (spec, delta) = parse_pair(spec_source, delta_source);
        let plan = merge_into_existing(spec_source, &spec, &delta, true).unwrap();
        assert!(plan.should_delete_spec);
    }

    #[test]
    fn removed_partiel_avec_flag_ne_supprime_pas() {
        // Une exigence retirée sur deux : le flag est un no-op silencieux.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n### Requirement: Logout\nThe system SHALL y.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let delta_source = "## REMOVED Requirements\n\n### Requirement: Login\n**Reason**: obsolete\n**Migration**: none\n";
        let (spec, delta) = parse_pair(spec_source, delta_source);
        let plan = merge_into_existing(spec_source, &spec, &delta, true).unwrap();
        assert!(
            !plan.should_delete_spec,
            "il reste Logout, la capacité n'est pas retirée"
        );
        assert!(!plan.edits.is_empty(), "le REMOVED Login est bien appliqué");
    }

    // ─────────────── RENAMED ───────────────

    #[test]
    fn renamed_ne_touche_qu_a_l_entete() {
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Session Expiration\nThe system MUST expire.\n\n#### Scenario: Idle\n- **WHEN** idle\n- **THEN** expire\n";
        let delta_source = "## RENAMED Requirements\n\n- FROM: Session Expiration\n- TO: Session Timeout\n";
        let out = merged(spec_source, delta_source);
        assert!(out.contains("### Requirement: Session Timeout"));
        assert!(!out.contains("### Requirement: Session Expiration"));
        // Le corps est intact.
        assert!(out.contains("The system MUST expire."));
        assert!(out.contains("#### Scenario: Idle"));
    }

    // ─────────────── ADDED ───────────────

    #[test]
    fn added_est_insere_apres_la_derniere_exigence() {
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## ADDED Requirements\n\n### Requirement: Session\nThe system SHALL y.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let out = merged(spec_source, delta_source);
        let login_pos = out.find("### Requirement: Login").unwrap();
        let session_pos = out.find("### Requirement: Session").unwrap();
        assert!(login_pos < session_pos, "l'ADDED vient après l'existant");
    }

    #[test]
    fn added_precede_une_section_libre_qui_suit() {
        // ## Notes après ## Requirements : l'ADDED doit s'insérer AVANT.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n\n## Notes\n\nBonus.\n";
        let delta_source = "## ADDED Requirements\n\n### Requirement: Session\nThe system SHALL y.\n\n#### Scenario: T\n- **WHEN** c\n- **THEN** d\n";
        let out = merged(spec_source, delta_source);
        let session_pos = out.find("### Requirement: Session").unwrap();
        let notes_pos = out.find("## Notes").unwrap();
        assert!(
            session_pos < notes_pos,
            "l'ADDED s'insère AVANT la section libre suivante"
        );
        // Notes reste intacte.
        assert!(out.contains("## Notes\n\nBonus.\n"));
    }

    // ─────────────── build_new_spec ───────────────

    #[test]
    fn build_new_spec_avec_purpose_et_added() {
        let delta_source = "## Purpose\n\nGère l'authentification.\n\n## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta = parse_delta(delta_source).value;
        let content = build_new_spec("user-auth", &delta).unwrap();

        assert!(content.starts_with("# User Auth Specification\n\n"));
        assert!(content.contains("## Purpose\n\nGère l'authentification.\n"));
        assert!(content.contains("## Requirements\n"));
        assert!(content.contains("### Requirement: Login"));
    }

    #[test]
    fn build_new_spec_sans_purpose_echoue() {
        let delta_source = "## ADDED Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta = parse_delta(delta_source).value;
        let err = build_new_spec("x", &delta).unwrap_err();
        assert_eq!(err.code(), "new_capability_without_purpose");
    }

    // ─────────────── invariants ───────────────

    #[test]
    fn added_dun_nom_deja_present_est_silencieusement_ignore() {
        // Cas de la ré-application : la spec contient déjà `Login`, le delta
        // demande `ADDED: Login`. On skip pour préserver l'idempotence.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## ADDED Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let out = merged(spec_source, delta_source);
        assert_eq!(out, spec_source, "ADDED déjà présent = no-op");
    }

    #[test]
    fn sync_est_idempotent_apres_deux_passages() {
        // Appliquer le même delta deux fois — la deuxième passe doit rendre
        // exactement le même contenu que la première.
        let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
        let delta_source = "## MODIFIED Requirements\n\n### Requirement: Login\nThe system MUST y.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";

        let apres_un = merged(spec_source, delta_source);
        let apres_deux = merged(&apres_un, delta_source);
        assert_eq!(apres_un, apres_deux, "le sync doit être idempotent");
    }

    #[test]
    fn edit_construction_est_stable() {
        let edit = Edit::new(0..5, "abc");
        assert_eq!(edit.byte_range, 0..5);
        assert_eq!(edit.replacement, "abc");
    }
}
