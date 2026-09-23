//! Sceau du corps d'un ADR.
//!
//! Une décision `accepted` est censée être immuable : hasher son **corps**
//! (tout ce qui suit le frontmatter) au moment de son acceptation, puis
//! comparer plus tard, permet de détecter mécaniquement une édition en
//! place. Le frontmatter, lui, peut évoluer légitimement (transition
//! `accepted` → `superseded` par exemple) — c'est pour ça que le hash ne
//! porte pas sur le fichier entier.
//!
//! Ce module est pur (aucune I/O) — voir la décision
//! `_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md`. Le
//! parsing et le rendu YAML passent par `serde_norway` conformément à la
//! décision `_codev/decisions/0006-serde-norway-pour-yaml.md`.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// La version du schéma du fichier `seal.yaml`. Un fichier lu avec une
/// version différente est refusé — un consommateur plus ancien ne peut
/// pas deviner ce qu'ajoute une version plus récente.
pub const SEAL_FILE_VERSION: u32 = 1;

/// Erreurs produites par les fonctions de ce module.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SealError {
    /// Le fichier ADR ne contient pas de séparateur `---` fermant son
    /// frontmatter : impossible de calculer un hash de corps.
    #[error("frontmatter fermant introuvable — le hash du corps ne peut être calculé")]
    MissingFrontmatterCloser,

    /// Le fichier `seal.yaml` a une `version` que ce binaire ne connaît pas.
    /// Code stable pour l'agent : `seal_version_unsupported`.
    #[error("version « {actual} » non supportée pour seal.yaml (attendu : {expected})")]
    UnsupportedVersion { actual: u32, expected: u32 },

    /// Le YAML de `seal.yaml` n'est pas parsable. Message brut de
    /// `serde_norway` gardé pour aider au debug.
    #[error("seal.yaml illisible : {0}")]
    Invalid(String),

    /// `plan_seal_new` refuse un id déjà scellé — cette voie est celle de
    /// la migration ou du geste initial ; pour ré-approuver un sceau, il
    /// faut `plan_seal_force`.
    #[error("l'id « {id} » est déjà scellé")]
    AlreadySealed { id: String },

    /// `plan_seal_force` refuse un id absent du sceau — force n'a de sens
    /// que quand une entrée existante doit être remplacée.
    #[error("l'id « {id} » n'a pas de sceau existant à réécrire")]
    Unknown { id: String },
}

/// Une entrée du fichier de sceau.
///
/// Les noms sérialisés sont en camelCase pour rester cohérents avec le
/// contrat JSON exposé par le CLI, même si le fichier reste YAML.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Seal {
    /// Identifiant local de la décision — le même `id` que celui du
    /// frontmatter de l'ADR.
    pub id: String,
    /// Hash SHA-256 du corps, préfixé `sha256:` — voir `body_hash`.
    pub body_sha256: String,
    /// Date à laquelle le sceau a été apposé, `AAAA-MM-JJ`.
    pub sealed_at: String,
}

/// Le fichier `seal.yaml` dans son ensemble.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SealFile {
    pub version: u32,
    pub seals: Vec<Seal>,
}

impl SealFile {
    /// Un sceau vide — utilisé quand le fichier n'existe pas encore.
    pub fn empty() -> Self {
        Self {
            version: SEAL_FILE_VERSION,
            seals: Vec::new(),
        }
    }

    /// Recherche l'entrée d'un id — lecture linéaire, la liste est petite
    /// (une entrée par ADR local, quelques dizaines au grand maximum).
    pub fn find(&self, id: &str) -> Option<&Seal> {
        self.seals.iter().find(|s| s.id == id)
    }
}

/// Calcule le hash SHA-256 du **corps** d'un ADR — soit tout ce qui suit
/// le séparateur `---` fermant le frontmatter, byte pour byte, sans
/// normalisation.
///
/// Retour : chaîne préfixée `sha256:`, hex lowercase.
///
/// Erreur : `MissingFrontmatterCloser` si le fichier n'a pas de
/// séparateur fermant. Le cas « aucun frontmatter du tout » est
/// considéré comme un fichier non-ADR — c'est au parseur d'ADR
/// (`parse_decision`) de le rejeter, pas ici.
pub fn body_hash(adr_source: &str) -> Result<String, SealError> {
    let body = body_slice(adr_source)?;
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    let digest = hasher.finalize();
    Ok(format!("sha256:{:x}", digest))
}

/// Extrait le corps de l'ADR : la sous-chaîne qui commence juste après
/// le `\n---\n` (ou `\n---\r\n`) fermant le frontmatter.
///
/// Sépare la logique du hash — utile pour un futur consommateur qui
/// voudrait afficher un diff plutôt qu'un booléen match/mismatch.
pub fn body_slice(adr_source: &str) -> Result<&str, SealError> {
    // Un ADR commence par `---\n` (ou `---\r\n`) ; on saute cette ligne.
    let after_opener_idx = if let Some(rest) = adr_source.strip_prefix("---\n") {
        adr_source.len() - rest.len()
    } else if let Some(rest) = adr_source.strip_prefix("---\r\n") {
        adr_source.len() - rest.len()
    } else {
        // Pas de frontmatter du tout : par convention, on considère que
        // « tout est corps » ; mais le parseur d'ADR aura déjà rejeté ce
        // cas, donc en pratique on n'y arrive jamais.
        return Err(SealError::MissingFrontmatterCloser);
    };

    // On cherche le prochain `\n---\n` ou `\n---\r\n`.
    let rest = &adr_source[after_opener_idx..];
    if let Some(pos) = rest.find("\n---\n") {
        let end = after_opener_idx + pos + "\n---\n".len();
        Ok(&adr_source[end..])
    } else if let Some(pos) = rest.find("\n---\r\n") {
        let end = after_opener_idx + pos + "\n---\r\n".len();
        Ok(&adr_source[end..])
    } else {
        Err(SealError::MissingFrontmatterCloser)
    }
}

/// Parse un fichier `seal.yaml`. Sur source vide, retourne un sceau
/// vide (état initial d'un projet).
pub fn parse_seal_file(source: &str) -> Result<SealFile, SealError> {
    if source.trim().is_empty() {
        return Ok(SealFile::empty());
    }
    let parsed: SealFile =
        serde_norway::from_str(source).map_err(|e| SealError::Invalid(e.to_string()))?;
    if parsed.version != SEAL_FILE_VERSION {
        return Err(SealError::UnsupportedVersion {
            actual: parsed.version,
            expected: SEAL_FILE_VERSION,
        });
    }
    Ok(parsed)
}

/// Rend un `SealFile` en YAML canonique, avec les entrées ordonnées par
/// `id` croissant. Le déterminisme est indispensable pour que `git diff`
/// ne remue rien quand ce n'est pas nécessaire.
pub fn render_seal_file(seal: &SealFile) -> String {
    let mut sorted = seal.clone();
    sorted.seals.sort_by(|a, b| a.id.cmp(&b.id));
    // On préfixe l'en-tête d'un petit commentaire — c'est le CLI qui
    // écrit ce fichier, l'humain doit savoir qu'il n'est pas censé y
    // toucher à la main.
    let body = serde_norway::to_string(&sorted).unwrap_or_default();
    format!(
        "# Fichier tenu par codev — voir `codev decision seal`.\n\
         # Le hash du corps d'un ADR y est enregistré à son acceptation ;\n\
         # `codev validate` compare ce hash avec le corps courant.\n\
         {body}"
    )
}

/// Prépare l'ajout d'une entrée de sceau à un fichier existant.
///
/// Refuse si l'id est déjà scellé — cette voie est celle du premier
/// scellement (création d'ADR ou migration). Pour réécrire un sceau
/// existant, utiliser `plan_seal_force`.
pub fn plan_seal_new(
    existing: &SealFile,
    id: String,
    body_hash: String,
    today: String,
) -> Result<SealFile, SealError> {
    if existing.find(&id).is_some() {
        return Err(SealError::AlreadySealed { id });
    }
    let mut next = existing.clone();
    next.seals.push(Seal {
        id,
        body_sha256: body_hash,
        sealed_at: today,
    });
    // Tri stable par id pour un diff propre côté git.
    next.seals.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(next)
}

/// Prépare le remplacement d'une entrée de sceau existante.
///
/// Refuse si l'id n'a pas de sceau — force n'a pas de sens sur un
/// fichier vide, il faut passer par `plan_seal_new`.
pub fn plan_seal_force(
    existing: &SealFile,
    id: String,
    body_hash: String,
    today: String,
) -> Result<SealFile, SealError> {
    if existing.find(&id).is_none() {
        return Err(SealError::Unknown { id });
    }
    let mut next = existing.clone();
    for s in &mut next.seals {
        if s.id == id {
            s.body_sha256 = body_hash.clone();
            s.sealed_at = today.clone();
        }
    }
    Ok(next)
}

/// Le résultat d'une vérification.
///
/// On ne renvoie pas directement des `Finding` du parseur — le module
/// `seal` reste pur et sans lien avec `codev-core::parser`. La coquille
/// (validate côté engine) traduit ces cas en `Finding` avec les bons
/// codes stables et sévérités.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationCase {
    /// Un ADR local sans entrée de sceau — warning côté validate,
    /// migration attendue.
    Unsealed { id: String },
    /// Un ADR dont le hash de corps ne correspond plus au sceau — erreur
    /// côté validate.
    Mismatch {
        id: String,
        recorded: String,
        actual: String,
    },
    /// Une entrée de sceau qui référence un ADR inexistant — warning.
    OrphanSeal { id: String },
}

/// Vérifie la cohérence d'un `SealFile` avec l'état courant du dossier.
///
/// `present` liste les ADR locaux `accepted`/`superseded` — les autres
/// statuts (proposed, deprecated, rejected) ne sont pas scellés (ils ne
/// portent pas d'engagement d'immutabilité). `body_hashes` fournit, pour
/// chaque `id` présent, le hash de son corps courant.
///
/// Les cas sont retournés dans un ordre stable pour que le rapport de
/// validate reste déterministe.
pub fn verify(seal: &SealFile, present: &[(String, String)]) -> Vec<VerificationCase> {
    use std::collections::HashSet;

    let mut cases = Vec::new();
    let ids_present: HashSet<&str> = present.iter().map(|(id, _)| id.as_str()).collect();

    // 1) chaque ADR présent : sealed ? matching ?
    let mut present_sorted = present.to_vec();
    present_sorted.sort_by(|a, b| a.0.cmp(&b.0));
    for (id, actual_hash) in &present_sorted {
        match seal.find(id) {
            None => cases.push(VerificationCase::Unsealed { id: id.clone() }),
            Some(entry) => {
                if entry.body_sha256 != *actual_hash {
                    cases.push(VerificationCase::Mismatch {
                        id: id.clone(),
                        recorded: entry.body_sha256.clone(),
                        actual: actual_hash.clone(),
                    });
                }
            }
        }
    }

    // 2) chaque sceau : son ADR existe-t-il toujours ?
    let mut orphans: Vec<&Seal> = seal
        .seals
        .iter()
        .filter(|s| !ids_present.contains(s.id.as_str()))
        .collect();
    orphans.sort_by(|a, b| a.id.cmp(&b.id));
    for s in orphans {
        cases.push(VerificationCase::OrphanSeal { id: s.id.clone() });
    }

    cases
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─────────────── body_hash ───────────────

    #[test]
    fn hash_dun_corps_standard() {
        let adr = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Contexte\n\nx\n";
        let hash = body_hash(adr).unwrap();
        assert!(hash.starts_with("sha256:"));
        // Vérifie la reproductibilité (même entrée → même hash).
        assert_eq!(body_hash(adr).unwrap(), hash);
    }

    #[test]
    fn hash_change_si_le_corps_change() {
        let adr_a = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Contexte\n\nx\n";
        let adr_b = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Contexte\n\ny\n";
        assert_ne!(body_hash(adr_a).unwrap(), body_hash(adr_b).unwrap());
    }

    #[test]
    fn hash_ignore_le_frontmatter() {
        // Deux ADR avec le même corps mais un frontmatter différent →
        // même hash. C'est tout l'intérêt de séparer.
        let adr_a = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n\n## Contexte\n\nCorps identique.\n";
        let adr_b = "---\nid: \"0001\"\ntitle: T\nstatus: superseded\ndate: 2026-09-09\n---\n\n## Contexte\n\nCorps identique.\n";
        assert_eq!(body_hash(adr_a).unwrap(), body_hash(adr_b).unwrap());
    }

    #[test]
    fn hash_avec_crlf_supporte() {
        let adr = "---\r\nid: \"0001\"\r\ntitle: T\r\nstatus: accepted\r\ndate: 2026-09-09\r\n---\r\n\r\n## Contexte\r\n\r\nx\r\n";
        // Ne panique pas ; le hash est différent d'un LF pur (byte pour
        // byte, sans normalisation) — c'est documenté comme risque.
        let h = body_hash(adr).unwrap();
        assert!(h.starts_with("sha256:"));
    }

    #[test]
    fn hash_corps_vide_est_valide() {
        // Le hash sha256 de la chaîne vide est bien connu.
        let adr = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\ndate: 2026-09-09\n---\n";
        let h = body_hash(adr).unwrap();
        assert_eq!(
            h,
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn hash_erreur_si_pas_de_frontmatter_fermant() {
        let adr = "---\nid: \"0001\"\ntitle: T\nstatus: accepted\n";
        let err = body_hash(adr).unwrap_err();
        assert!(matches!(err, SealError::MissingFrontmatterCloser));
    }

    // ─────────────── parse / render ───────────────

    #[test]
    fn parse_source_vide_donne_seal_vide() {
        let seal = parse_seal_file("").unwrap();
        assert_eq!(seal.version, 1);
        assert!(seal.seals.is_empty());
    }

    #[test]
    fn parse_forme_canonique() {
        let source = "version: 1\nseals:\n  - id: \"0001\"\n    bodySha256: \"sha256:abc\"\n    sealedAt: 2026-09-09\n";
        let seal = parse_seal_file(source).unwrap();
        assert_eq!(seal.seals.len(), 1);
        assert_eq!(seal.seals[0].id, "0001");
        assert_eq!(seal.seals[0].body_sha256, "sha256:abc");
        assert_eq!(seal.seals[0].sealed_at, "2026-09-09");
    }

    #[test]
    fn parse_champ_inconnu_est_refuse() {
        // deny_unknown_fields : un champ non déclaré fait échouer le parse.
        let source = "version: 1\nseals:\n  - id: \"0001\"\n    bodySha256: \"sha256:abc\"\n    sealedAt: 2026-09-09\n    extra: mauvais\n";
        let err = parse_seal_file(source).unwrap_err();
        assert!(matches!(err, SealError::Invalid(_)));
    }

    #[test]
    fn parse_version_differente_est_refusee() {
        let source = "version: 999\nseals: []\n";
        let err = parse_seal_file(source).unwrap_err();
        assert!(matches!(
            err,
            SealError::UnsupportedVersion {
                actual: 999,
                expected: 1
            }
        ));
    }

    #[test]
    fn render_est_deterministe_meme_ordre_desordonne() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0003".into(),
            body_sha256: "sha256:c".into(),
            sealed_at: "2026-09-09".into(),
        });
        seal.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:a".into(),
            sealed_at: "2026-09-09".into(),
        });
        let rendered = render_seal_file(&seal);
        // Le tri place 0001 avant 0003. serde_norway peut rendre l'id avec
        // ou sans guillemets ; on cherche seulement le sha correspondant,
        // qui, lui, est distinctif.
        let idx_a = rendered.find("sha256:a").unwrap();
        let idx_c = rendered.find("sha256:c").unwrap();
        assert!(idx_a < idx_c, "0001 (sha256:a) doit apparaître avant 0003 (sha256:c) :\n{rendered}");
    }

    #[test]
    fn round_trip_parse_render_parse() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:aaa".into(),
            sealed_at: "2026-09-09".into(),
        });
        let rendered = render_seal_file(&seal);
        let reparsed = parse_seal_file(&rendered).unwrap();
        assert_eq!(reparsed.seals, seal.seals);
        assert_eq!(reparsed.version, seal.version);
    }

    // ─────────────── plan_seal_new / plan_seal_force ───────────────

    #[test]
    fn plan_seal_new_insere_puis_trie() {
        let mut existing = SealFile::empty();
        existing.seals.push(Seal {
            id: "0002".into(),
            body_sha256: "sha256:b".into(),
            sealed_at: "2026-09-09".into(),
        });
        let next = plan_seal_new(
            &existing,
            "0001".into(),
            "sha256:a".into(),
            "2026-09-09".into(),
        )
        .unwrap();
        assert_eq!(next.seals.len(), 2);
        assert_eq!(next.seals[0].id, "0001");
        assert_eq!(next.seals[1].id, "0002");
    }

    #[test]
    fn plan_seal_new_refuse_id_deja_scelle() {
        let mut existing = SealFile::empty();
        existing.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:a".into(),
            sealed_at: "2026-09-09".into(),
        });
        let err = plan_seal_new(
            &existing,
            "0001".into(),
            "sha256:different".into(),
            "2026-09-10".into(),
        )
        .unwrap_err();
        assert!(matches!(err, SealError::AlreadySealed { .. }));
    }

    #[test]
    fn plan_seal_force_remplace_lentree_existante() {
        let mut existing = SealFile::empty();
        existing.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:ancien".into(),
            sealed_at: "2026-09-08".into(),
        });
        let next = plan_seal_force(
            &existing,
            "0001".into(),
            "sha256:nouveau".into(),
            "2026-09-09".into(),
        )
        .unwrap();
        assert_eq!(next.seals.len(), 1);
        assert_eq!(next.seals[0].body_sha256, "sha256:nouveau");
        assert_eq!(next.seals[0].sealed_at, "2026-09-09");
    }

    #[test]
    fn plan_seal_force_refuse_id_absent() {
        let existing = SealFile::empty();
        let err = plan_seal_force(
            &existing,
            "9999".into(),
            "sha256:x".into(),
            "2026-09-09".into(),
        )
        .unwrap_err();
        assert!(matches!(err, SealError::Unknown { .. }));
    }

    // ─────────────── verify ───────────────

    #[test]
    fn verify_signale_les_adr_non_scelles() {
        let seal = SealFile::empty();
        let cases = verify(
            &seal,
            &[
                ("0001".into(), "sha256:a".into()),
                ("0002".into(), "sha256:b".into()),
            ],
        );
        assert_eq!(cases.len(), 2);
        assert!(matches!(&cases[0], VerificationCase::Unsealed { id } if id == "0001"));
        assert!(matches!(&cases[1], VerificationCase::Unsealed { id } if id == "0002"));
    }

    #[test]
    fn verify_signale_un_mismatch() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:enregistre".into(),
            sealed_at: "2026-09-09".into(),
        });
        let cases = verify(&seal, &[("0001".into(), "sha256:actuel".into())]);
        assert_eq!(cases.len(), 1);
        match &cases[0] {
            VerificationCase::Mismatch {
                id,
                recorded,
                actual,
            } => {
                assert_eq!(id, "0001");
                assert_eq!(recorded, "sha256:enregistre");
                assert_eq!(actual, "sha256:actuel");
            }
            _ => panic!("attendu Mismatch, reçu {:?}", cases[0]),
        }
    }

    #[test]
    fn verify_signale_un_orphelin() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0004".into(),
            body_sha256: "sha256:x".into(),
            sealed_at: "2026-09-09".into(),
        });
        let cases = verify(&seal, &[]);
        assert_eq!(cases.len(), 1);
        assert!(matches!(&cases[0], VerificationCase::OrphanSeal { id } if id == "0004"));
    }

    #[test]
    fn verify_ordre_stable() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0003".into(),
            body_sha256: "sha256:c".into(),
            sealed_at: "2026-09-09".into(),
        });
        seal.seals.push(Seal {
            id: "0002".into(),
            body_sha256: "sha256:b_wrong".into(),
            sealed_at: "2026-09-09".into(),
        });
        // Présents : 0001 (non scellé) + 0002 (mismatch) ; 0003 orphelin.
        let cases = verify(
            &seal,
            &[
                ("0002".into(), "sha256:b_actual".into()),
                ("0001".into(), "sha256:a".into()),
            ],
        );
        assert_eq!(cases.len(), 3);
        // Ordre : Unsealed(0001), Mismatch(0002), OrphanSeal(0003).
        assert!(matches!(&cases[0], VerificationCase::Unsealed { id } if id == "0001"));
        assert!(matches!(&cases[1], VerificationCase::Mismatch { id, .. } if id == "0002"));
        assert!(matches!(&cases[2], VerificationCase::OrphanSeal { id } if id == "0003"));
    }

    #[test]
    fn find_existe_ou_non() {
        let mut seal = SealFile::empty();
        seal.seals.push(Seal {
            id: "0001".into(),
            body_sha256: "sha256:a".into(),
            sealed_at: "2026-09-09".into(),
        });
        assert!(seal.find("0001").is_some());
        assert!(seal.find("0002").is_none());
    }
}
