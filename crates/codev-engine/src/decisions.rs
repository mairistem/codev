//! Index des décisions d'architecture — projet + sources héritées.
//!
//! Le parseur pur (`codev-core::decisions`) transforme un fichier en
//! `Decision`. Ici on coordonne la lecture du disque, la fusion avec les
//! sources héritées `path:`, et le calcul des décisions « en vigueur » —
//! celles qui ne sont supersedées par aucune autre.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use codev_core::decisions::{parse_decision, Decision, DecisionStatus};
use codev_core::parser::ast::{Finding, Severity};
use codev_core::parser::codes;
use codev_core::Layout;

use crate::config::ResolvedConfig;
use crate::error::{EngineError, Result};
use crate::ports::{Env, FileSystem};

/// L'origine d'une décision — projet ou source héritée.
///
/// Le format des sources héritées `git:` est réservé — le lot actuel ne
/// gère que `path:`. Une nouvelle variante suivra quand `git:` sera
/// implémenté.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Origin {
    Project,
    Path(String),
    /// URL d'un dépôt `git:` — le contenu vient du cache local, mais
    /// l'`origin` porte l'adresse d'origine pour rester lisible côté agent.
    Git(String),
}

impl Origin {
    pub fn as_str(&self) -> String {
        match self {
            Self::Project => "projet".into(),
            Self::Path(raw) => format!("path:{raw}"),
            Self::Git(url) => format!("git:{url}"),
        }
    }
}

/// Un identifiant qualifié — évite les collisions inter-sources.
///
/// La forme sérialisée `"projet/0007"` ou `"path:~/partage/0100"` est
/// exposée telle quelle dans le contrat JSON.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QualifiedId {
    pub origin: Origin,
    pub id: String,
}

impl QualifiedId {
    pub fn as_str(&self) -> String {
        format!("{}/{}", self.origin.as_str(), self.id)
    }
}

/// Une entrée de l'index.
#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub qualified_id: QualifiedId,
    pub decision: Decision,
    /// Chemin absolu du fichier, sur le disque tel qu'il a été lu.
    pub path: PathBuf,
    /// Si cette décision est héritée et écartée par un ADR local via
    /// `deviates_from`, l'identifiant qualifié de l'ADR qui la remplace.
    /// Calculé par l'index, jamais persisté sur disque.
    pub deviated_by: Option<QualifiedId>,
}

/// L'index complet.
#[derive(Debug, Clone, Default)]
pub struct DecisionIndex {
    pub entries: Vec<IndexEntry>,
    /// Sous-ensemble des `entries` qui restent en vigueur : `Accepted` et
    /// non supersedées par une autre décision `Accepted`.
    pub in_effect: Vec<QualifiedId>,
    pub findings: Vec<Finding>,
}

/// Construit l'index à partir du projet et de ses sources héritées.
///
/// `env` est nécessaire pour développer `~` dans les chemins des sources
/// `inherits: path:` — le port utilisé doit être le même que celui de
/// `config::resolve`, sinon les warnings de résolution et les décisions
/// indexées divergent.
pub fn index(
    fs: &dyn FileSystem,
    env: &dyn Env,
    layout: &Layout,
    config: &ResolvedConfig,
) -> Result<DecisionIndex> {
    let mut entries: Vec<IndexEntry> = Vec::new();
    let mut findings: Vec<Finding> = Vec::new();

    // 1. Décisions du projet.
    let projet_dir = layout.decisions_dir();
    collect_from(fs, &projet_dir, Origin::Project, &mut entries, &mut findings)?;

    // 2. Décisions des sources héritées — path ET git verrouillées.
    let states = crate::sources::list_source_states(fs, env, layout)?;
    for state in states {
        let Some(resolved_path) = state.resolved_path else {
            continue;
        };
        let origin = match state.kind {
            crate::sources::SourceKind::Path => Origin::Path(state.address.clone()),
            crate::sources::SourceKind::Git => Origin::Git(state.address.clone()),
        };
        let decisions_dir = resolved_path.join("_codev").join("decisions");
        collect_from(fs, &decisions_dir, origin, &mut entries, &mut findings)?;
    }
    let _ = config; // conservé pour compatibilité de signature

    // 3. Détection des collisions d'id — le projet gagne, l'hérité est
    //    signalé.
    detect_id_collisions(&mut entries, &mut findings);

    // 4. Résolution des dérives locales — chaque héritée référencée par
    //    un ADR local `accepted` via `deviates_from` reçoit son
    //    `deviated_by`, et disparaîtra du calcul `in_effect`.
    resolve_deviations(&mut entries, &mut findings);

    // 5. Résolution des supersessions et calcul de `in_effect`.
    let in_effect = resolve_in_effect(&entries, &mut findings);

    Ok(DecisionIndex {
        entries,
        in_effect,
        findings,
    })
}

/// Ouvre chaque `*.md` du dossier `dir`, appelle `parse_decision`, agrège.
fn collect_from(
    fs: &dyn FileSystem,
    dir: &std::path::Path,
    origin: Origin,
    entries: &mut Vec<IndexEntry>,
    findings: &mut Vec<Finding>,
) -> Result<()> {
    let files = fs.walk_files(dir).map_err(|e| EngineError::Unreadable {
        path: dir.to_path_buf(),
        reason: e.to_string(),
    })?;

    for relative in files.into_iter().filter(|p| p.ends_with(".md")) {
        let path = dir.join(&relative);
        let source = fs.read_to_string(&path).map_err(|e| EngineError::Unreadable {
            path: path.clone(),
            reason: e.to_string(),
        })?;
        let parsed = parse_decision(&source);
        findings.extend(parsed.findings);
        if let Some(decision) = parsed.value {
            entries.push(IndexEntry {
                qualified_id: QualifiedId {
                    origin: origin.clone(),
                    id: decision.id.clone(),
                },
                decision,
                path,
                deviated_by: None,
            });
        }
    }
    Ok(())
}

fn detect_id_collisions(entries: &mut [IndexEntry], findings: &mut Vec<Finding>) {
    // Grouper par `id` seul (pas par qualifié). Une collision est un id qui
    // apparaît à la fois côté `Project` et côté `Path(_)`.
    let mut par_id: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, entry) in entries.iter().enumerate() {
        par_id.entry(entry.decision.id.as_str()).or_default().push(i);
    }
    for (id, indices) in par_id {
        if indices.len() < 2 {
            continue;
        }
        let a_projet = indices
            .iter()
            .any(|&i| entries[i].qualified_id.origin == Origin::Project);
        let a_source = indices
            .iter()
            .any(|&i| entries[i].qualified_id.origin != Origin::Project);
        if a_projet && a_source {
            findings.push(Finding {
                severity: Severity::Warning,
                code: codes::DECISION_ID_COLLISION,
                line: 1,
                message: format!(
                    "l'identifiant « {id} » existe à la fois dans le projet et dans une source \
                     héritée ; la version du projet est retenue comme en vigueur"
                ),
            });
        }
    }
}

/// Calcule les `deviated_by` sur les entrées héritées, en s'appuyant sur
/// le champ `deviates_from` des ADR locaux `accepted`.
///
/// Émet :
/// - `decision_dangling_deviation` (warning) quand la cible n'existe pas.
/// - `decision_conflicting_deviations` (erreur) quand deux ADR locaux
///   `accepted` dévient de la même cible.
///
/// Ne fait rien pour un `deviates_from` porté par un ADR non `accepted`
/// (un `proposed` n'engage pas encore).
fn resolve_deviations(entries: &mut [IndexEntry], findings: &mut Vec<Finding>) {
    // Étape 1 : construire l'index qualified → position dans `entries`.
    let by_qualified: BTreeMap<String, usize> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| (e.qualified_id.as_str(), i))
        .collect();

    // Étape 2 : pour chaque ADR local `accepted`, collecter ses cibles.
    // `deviations_par_cible[target_qualified] = liste des <ADR local> qui la référencent`.
    let mut deviations_par_cible: BTreeMap<String, Vec<QualifiedId>> = BTreeMap::new();
    for entry in entries.iter() {
        if entry.qualified_id.origin != Origin::Project {
            continue;
        }
        if !matches!(entry.decision.status, DecisionStatus::Accepted) {
            continue;
        }
        for target in &entry.decision.deviates_from {
            deviations_par_cible
                .entry(target.clone())
                .or_default()
                .push(entry.qualified_id.clone());
        }
    }

    // Étape 3 : pour chaque cible, résoudre.
    for (target, sources) in deviations_par_cible {
        // Cible inexistante ?
        let Some(&target_idx) = by_qualified.get(&target) else {
            // Chaque ADR local qui la référence remonte un warning.
            for source in &sources {
                findings.push(Finding {
                    severity: Severity::Warning,
                    code: codes::DECISION_DANGLING_DEVIATION,
                    line: 1,
                    message: format!(
                        "la décision locale « {source_qid} » dévie de \
                         « {target} », qui n'est pas indexée (source retirée, \
                         SHA déplacé, ou id changé)",
                        source_qid = source.as_str()
                    ),
                });
            }
            continue;
        };

        // Cible locale ? Cas normalement refusé par `plan_deviate` en
        // amont, mais un utilisateur pourrait avoir écrit un ADR à la
        // main. Pas de finding dédié — la sémantique est simplement que
        // dévier d'une locale n'a pas d'effet côté index (pas d'occultation).
        if entries[target_idx].qualified_id.origin == Origin::Project {
            continue;
        }

        // Deux ou plus ADR locaux qui dévient de la même cible → conflit.
        if sources.len() > 1 {
            let noms = sources
                .iter()
                .map(|q| q.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            findings.push(Finding {
                severity: Severity::Error,
                code: codes::DECISION_CONFLICTING_DEVIATIONS,
                line: 1,
                message: format!(
                    "les décisions locales {noms} dévient toutes de « {target} » — \
                     l'outil ne tranche pas ; retire les ADR en trop ou remplace-les \
                     par des supersessions locales"
                ),
            });
            // On marque quand même la cible du premier trouvé (ordre
            // stable de `BTreeMap`) pour éviter que l'héritée reste
            // en_effect si l'utilisateur ignore le finding.
        }

        // Marquer la cible héritée.
        entries[target_idx].deviated_by = Some(sources[0].clone());
    }
}

fn resolve_in_effect(entries: &[IndexEntry], findings: &mut Vec<Finding>) -> Vec<QualifiedId> {
    // Détection de cycles : un DFS coloré par état. Toute décision touchée
    // par un cycle est exclue de `in_effect`.
    let (cycle_members, cycle_finding) = detect_cycles(entries);
    if let Some(f) = cycle_finding {
        findings.push(f);
    }

    // Pour chaque décision `Accepted` non-projet en collision avec le
    // projet, la version projet gagne — on marque l'héritée comme masquée.
    let projet_ids: BTreeSet<&str> = entries
        .iter()
        .filter(|e| e.qualified_id.origin == Origin::Project)
        .map(|e| e.decision.id.as_str())
        .collect();

    // Ensemble des ids supersedés par au moins une décision Accepted.
    let mut supersedes_source = BTreeSet::new();
    for entry in entries.iter().filter(|e| {
        matches!(e.decision.status, DecisionStatus::Accepted) && !cycle_members.contains(&index_of(entries, e))
    }) {
        for target in &entry.decision.supersedes {
            if !entries.iter().any(|e| e.decision.id == *target) {
                findings.push(Finding {
                    severity: Severity::Warning,
                    code: codes::DECISION_SUPERSEDES_UNKNOWN,
                    line: 1,
                    message: format!(
                        "la décision « {} » supersede « {target} » qui n'existe pas dans l'index",
                        entry.decision.id
                    ),
                });
                continue;
            }
            supersedes_source.insert(target.clone());
        }
    }

    let mut in_effect = Vec::new();
    for (idx, entry) in entries.iter().enumerate() {
        // Statut candidat ?
        if !entry.decision.status.is_candidate_for_effect() {
            continue;
        }
        // Dans un cycle ?
        if cycle_members.contains(&idx) {
            continue;
        }
        // Supersedée ?
        if supersedes_source.contains(entry.decision.id.as_str()) {
            continue;
        }
        // Masquée par une version projet ?
        if entry.qualified_id.origin != Origin::Project
            && projet_ids.contains(entry.decision.id.as_str())
        {
            continue;
        }
        // Déviée localement ? (K6)
        if entry.deviated_by.is_some() {
            continue;
        }
        in_effect.push(entry.qualified_id.clone());
    }
    in_effect
}

fn index_of(entries: &[IndexEntry], entry: &IndexEntry) -> usize {
    entries
        .iter()
        .position(|e| std::ptr::eq(e, entry))
        .expect("l'entrée vient du même slice")
}

fn detect_cycles(entries: &[IndexEntry]) -> (BTreeSet<usize>, Option<Finding>) {
    // Nous ne considérons pour le cycle que les chaînes de supersession
    // entre décisions présentes dans l'index — cible manquante = pas de
    // cycle, c'est signalé ailleurs.
    let by_id: BTreeMap<&str, usize> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| (e.decision.id.as_str(), i))
        .collect();

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum State {
        White,
        Gray,
        Black,
    }
    let mut state = vec![State::White; entries.len()];
    let mut members = BTreeSet::new();

    fn visit(
        node: usize,
        entries: &[IndexEntry],
        by_id: &BTreeMap<&str, usize>,
        state: &mut Vec<State>,
        stack: &mut Vec<usize>,
        members: &mut BTreeSet<usize>,
    ) {
        state[node] = State::Gray;
        stack.push(node);
        for target in &entries[node].decision.supersedes {
            let Some(&next) = by_id.get(target.as_str()) else {
                continue;
            };
            match state[next] {
                State::White => visit(next, entries, by_id, state, stack, members),
                State::Gray => {
                    // Cycle : tout ce qui est sur la pile depuis `next` en
                    // fait partie.
                    let start = stack.iter().position(|&n| n == next).unwrap_or(0);
                    for &n in &stack[start..] {
                        members.insert(n);
                    }
                }
                State::Black => {}
            }
        }
        stack.pop();
        state[node] = State::Black;
    }

    for start in 0..entries.len() {
        if state[start] == State::White {
            visit(start, entries, &by_id, &mut state, &mut Vec::new(), &mut members);
        }
    }

    let finding = if members.is_empty() {
        None
    } else {
        let noms: Vec<String> = members
            .iter()
            .map(|&i| entries[i].decision.id.clone())
            .collect();
        Some(Finding {
            severity: Severity::Warning,
            code: codes::DECISION_SUPERSESSION_CYCLE,
            line: 1,
            message: format!(
                "cycle de supersession détecté : {} — aucune de ces décisions n'entre en vigueur",
                noms.join(", ")
            ),
        })
    };
    (members, finding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::ports::{FixedEnv, MemoryFileSystem};

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn resolved(fs: &dyn FileSystem) -> ResolvedConfig {
        config::resolve(fs, &env(), &Layout::new("/p")).unwrap()
    }

    fn adr(id: &str, status: &str, supersedes: &[&str]) -> String {
        let sup = if supersedes.is_empty() {
            String::new()
        } else {
            format!(
                "supersedes: [{}]\n",
                supersedes
                    .iter()
                    .map(|s| format!("\"{s}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        format!(
            "---\nid: \"{id}\"\ntitle: \"ADR {id}\"\nstatus: {status}\ndate: 2026-09-08\n{sup}---\n\n## Contexte\n\nx\n"
        )
    }

    #[test]
    fn index_vide_sur_projet_sans_adr() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(idx.entries.is_empty());
        assert!(idx.in_effect.is_empty());
        assert!(idx.findings.is_empty());
    }

    #[test]
    fn adr_du_projet_et_dune_source_apparaissent_avec_leur_origin() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file(
                "/p/_codev/decisions/0001-local.md",
                adr("0001", "accepted", &[]),            )
            .with_file(
                "/home/partage/_codev/decisions/0100-partage.md",
                adr("0100", "accepted", &[]),            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        assert_eq!(idx.entries.len(), 2);
        assert!(idx
            .entries
            .iter()
            .any(|e| e.qualified_id.as_str() == "projet/0001"));
        assert!(idx
            .entries
            .iter()
            .any(|e| e.qualified_id.as_str() == "path:~/partage/0100"));
    }

    #[test]
    fn supersession_directe_masque_la_source() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0003.md",
                adr("0003", "accepted", &[]),            )
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr("0007", "accepted", &["0003"]),            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        let en_vigueur: Vec<String> = idx.in_effect.iter().map(|q| q.id.clone()).collect();
        assert_eq!(en_vigueur, ["0007"]);
    }

    #[test]
    fn chaine_a_trois_maillons_laisse_le_dernier() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/A.md",
                adr("A", "accepted", &[]),            )
            .with_file(
                "/p/_codev/decisions/B.md",
                adr("B", "accepted", &["A"]),            )
            .with_file(
                "/p/_codev/decisions/C.md",
                adr("C", "accepted", &["B"]),            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        let en_vigueur: Vec<String> = idx.in_effect.iter().map(|q| q.id.clone()).collect();
        assert_eq!(en_vigueur, ["C"]);
    }

    #[test]
    fn supersedes_vers_absent_est_signale() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr("0007", "accepted", &["9999"]),            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(idx
            .findings
            .iter()
            .any(|f| f.code == codes::DECISION_SUPERSEDES_UNKNOWN
                && f.message.contains("9999")));
        // 0007 reste en vigueur — la cible perdue ne le disqualifie pas.
        assert_eq!(
            idx.in_effect.iter().map(|q| q.id.clone()).collect::<Vec<_>>(),
            ["0007"]
        );
    }

    #[test]
    fn collision_projet_source_projet_gagne() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr("0007", "accepted", &[]),            )
            .with_file(
                "/home/partage/_codev/decisions/0007-doublon.md",
                adr("0007", "accepted", &[]),            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        assert!(idx
            .findings
            .iter()
            .any(|f| f.code == codes::DECISION_ID_COLLISION));

        // Une seule entrée en vigueur : celle du projet.
        assert_eq!(idx.in_effect.len(), 1);
        assert_eq!(idx.in_effect[0].origin, Origin::Project);
    }

    #[test]
    fn cycle_de_supersession_est_signale() {
        // A supersede B, B supersede A → cycle. Ni l'un ni l'autre en vigueur.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/A.md",
                adr("A", "accepted", &["B"]),            )
            .with_file(
                "/p/_codev/decisions/B.md",
                adr("B", "accepted", &["A"]),            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        assert!(idx
            .findings
            .iter()
            .any(|f| f.code == codes::DECISION_SUPERSESSION_CYCLE));
        assert!(idx.in_effect.is_empty());
    }

    #[test]
    fn statut_proposed_nentre_pas_en_vigueur() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0001.md",
                adr("0001", "proposed", &[]),            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert_eq!(idx.entries.len(), 1);
        assert!(idx.in_effect.is_empty());
    }

    // ─────────────── déviations locales (K6) ───────────────

    fn adr_deviates(id: &str, status: &str, deviates_from: &[&str]) -> String {
        let dev = if deviates_from.is_empty() {
            String::new()
        } else {
            format!(
                "deviates_from: [{}]\n",
                deviates_from
                    .iter()
                    .map(|s| format!("\"{s}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        format!(
            "---\nid: \"{id}\"\ntitle: \"ADR {id}\"\nstatus: {status}\ndate: 2026-09-08\n{dev}---\n\n## Contexte\n\nx\n"
        )
    }

    #[test]
    fn deviation_marque_heritée_et_la_retire_du_in_effect() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file(
                "/home/partage/_codev/decisions/0100.md",
                adr("0100", "accepted", &[]),
            )
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr_deviates("0007", "accepted", &["path:~/partage/0100"]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();

        // 2 entrées, une locale une héritée.
        assert_eq!(idx.entries.len(), 2);
        let heritee = idx
            .entries
            .iter()
            .find(|e| e.qualified_id.origin != Origin::Project)
            .unwrap();
        assert!(heritee.deviated_by.is_some());
        assert_eq!(
            heritee.deviated_by.as_ref().unwrap().as_str(),
            "projet/0007"
        );
        // L'héritée est retirée du in_effect, la locale y reste.
        assert!(!idx
            .in_effect
            .iter()
            .any(|q| q.origin != Origin::Project));
        assert!(idx
            .in_effect
            .iter()
            .any(|q| q.as_str() == "projet/0007"));
    }

    #[test]
    fn deviation_dune_cible_inconnue_remonte_un_warning_et_reste_dangling() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr_deviates("0007", "accepted", &["path:~/inconnue/9999"]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        assert!(idx
            .findings
            .iter()
            .any(|f| f.code == codes::DECISION_DANGLING_DEVIATION));
        // Le finding est un warning, pas une erreur.
        assert!(idx
            .findings
            .iter()
            .filter(|f| f.code == codes::DECISION_DANGLING_DEVIATION)
            .all(|f| f.severity == Severity::Warning));
    }

    #[test]
    fn deux_deviations_sur_meme_cible_declenchent_conflit() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file(
                "/home/partage/_codev/decisions/0100.md",
                adr("0100", "accepted", &[]),
            )
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr_deviates("0007", "accepted", &["path:~/partage/0100"]),
            )
            .with_file(
                "/p/_codev/decisions/0008.md",
                adr_deviates("0008", "accepted", &["path:~/partage/0100"]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        let conflicts: Vec<_> = idx
            .findings
            .iter()
            .filter(|f| f.code == codes::DECISION_CONFLICTING_DEVIATIONS)
            .collect();
        assert_eq!(conflicts.len(), 1, "un seul finding par cible en conflit");
        assert_eq!(conflicts[0].severity, Severity::Error);
        // Les deux ADR sont nommés dans le message.
        assert!(conflicts[0].message.contains("projet/0007"));
        assert!(conflicts[0].message.contains("projet/0008"));
    }

    #[test]
    fn deviation_par_un_proposed_na_aucun_effet() {
        // Un ADR proposed n'engage pas ; sa dérive n'occulte pas la cible.
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file(
                "/home/partage/_codev/decisions/0100.md",
                adr("0100", "accepted", &[]),
            )
            .with_file(
                "/p/_codev/decisions/0007.md",
                adr_deviates("0007", "proposed", &["path:~/partage/0100"]),
            );
        let cfg = resolved(&fs);
        let idx = index(&fs, &env(), &Layout::new("/p"), &cfg).unwrap();
        let heritee = idx
            .entries
            .iter()
            .find(|e| e.qualified_id.origin != Origin::Project)
            .unwrap();
        assert!(heritee.deviated_by.is_none());
        // L'héritée reste bien in_effect.
        assert!(idx
            .in_effect
            .iter()
            .any(|q| q.as_str() == "path:~/partage/0100"));
    }
}
