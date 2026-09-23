use std::path::PathBuf;

use codev_core::decisions::DecisionStatus;
use codev_core::{ArtifactState, ChangeId, CoreError, Layout};

use crate::change::{self, ChangeContext};
use crate::config::{Block, ResolvedConfig};
use crate::decisions::{self, Origin};
use crate::error::{EngineError, Result, Warning};
use crate::ports::{Env, FileSystem};

/// Un artefact déjà écrit, à relire pour se situer avant d'écrire le suivant.
#[derive(Debug, Clone)]
pub struct Dependency {
    pub id: String,
    pub path: PathBuf,
    pub done: bool,
}

/// Une référence à une décision d'architecture en vigueur.
///
/// Volontairement une **référence** — pas le contenu complet : la skill qui
/// consomme cette liste va lire le fichier via `path` si elle en a besoin,
/// comme elle le fait déjà pour les dépendances. Dupliquer le corps ici
/// ferait grossir chaque instruction sans utilité.
#[derive(Debug, Clone)]
pub struct DecisionRef {
    /// L'`id` court, tel qu'écrit dans le frontmatter.
    pub id: String,
    /// L'identifiant qualifié `origin/id` — évite les collisions entre
    /// projet et sources héritées.
    pub qualified_id: String,
    pub title: String,
    /// Toujours l'un des cinq statuts reconnus, sérialisé en minuscules.
    /// Pour une décision « en vigueur » c'est `accepted` — l'exposer garde
    /// le consommateur honnête si le calcul change plus tard.
    pub status: String,
    pub tags: Vec<String>,
    /// Chemin relatif au projet.
    pub path: PathBuf,
    /// `"projet"` ou `"path:<chemin déclaré>"`.
    pub origin: String,
}

/// Tout ce qu'un agent doit savoir pour écrire un artefact.
///
/// C'est la pièce centrale du système : le CLI ne rédige rien, il assemble le
/// contexte. `instruction` vient du schéma, `template` de ses fichiers,
/// `context` et `rules` de la configuration et des sources héritées.
#[derive(Debug, Clone)]
pub struct Instructions {
    pub change: ChangeId,
    pub schema_name: String,
    pub artifact_id: String,
    pub description: Option<String>,
    /// Où écrire. Peut être un motif glob : `instruction` dit alors comment
    /// choisir le chemin concret.
    pub resolved_output_path: PathBuf,
    pub instruction: Option<String>,
    pub template: Option<String>,
    /// Contraintes pour l'agent, jamais du contenu à recopier dans le fichier.
    pub context: Vec<Block>,
    pub rules: Vec<Block>,
    pub dependencies: Vec<Dependency>,
    pub unlocks: Vec<String>,
    /// Les décisions d'architecture **en vigueur** — rempli pour l'artefact
    /// `design`, laissé vide pour les autres. Une skill qui produit un
    /// design lit ces références et ouvre les fichiers pointés avant de
    /// rédiger. Le champ est toujours présent, éventuellement vide, pour
    /// que le consommateur teste `decisions.length` sans branche
    /// conditionnelle.
    pub decisions: Vec<DecisionRef>,
    /// Vrai quand le change a neutralisé cet artefact : il ne doit **pas** être
    /// créé.
    pub skipped: bool,
    pub warnings: Vec<Warning>,
}

/// Assemble les instructions d'un artefact.
///
/// `artifact_id` à `None` demande « le prochain à écrire », c'est-à-dire le
/// premier `Ready` dans l'ordre topologique.
pub fn for_artifact(
    fs: &dyn FileSystem,
    env: &dyn Env,
    layout: &Layout,
    ctx: &ChangeContext,
    config: &ResolvedConfig,
    artifact_id: Option<&str>,
) -> Result<Instructions> {
    let status = change::status(fs, layout, ctx)?;

    let artifact_id = match artifact_id {
        Some(id) => {
            if ctx.schema.graph.artifact(id).is_none() {
                return Err(CoreError::UnknownArtifact {
                    schema: ctx.schema.name().to_string(),
                    artifact: id.to_string(),
                }
                .into());
            }
            id.to_string()
        }
        None => status
            .artifacts
            .iter()
            .find(|a| a.state == ArtifactState::Ready)
            .map(|a| a.id.clone())
            .ok_or_else(|| EngineError::NoArtifactReady {
                change: ctx.change.to_string(),
            })?,
    };

    let artifact = ctx
        .schema
        .graph
        .artifact(&artifact_id)
        .expect("l'identifiant vient d'être validé contre le graphe");
    let state = status
        .artifacts
        .iter()
        .find(|a| a.id == artifact_id)
        .map(|a| a.state)
        .expect("tout artefact du graphe figure dans le statut");

    let change_dir = layout.change_dir(&ctx.change);
    let mut warnings = config.warnings.clone();
    if state == ArtifactState::Skipped {
        warnings.push(Warning::new(
            "artifact_skipped",
            format!(
                "l'artefact « {artifact_id} » est neutralisé par `skip_specs` dans le change.yaml : \
                 ses fichiers ne doivent pas être créés"
            ),
        ));
    }

    let dependencies = artifact
        .requires
        .iter()
        .map(|dep_id| {
            let done = status
                .artifacts
                .iter()
                .find(|a| &a.id == dep_id)
                .is_some_and(|a| a.state.satisfies_dependency());
            let generates = ctx
                .schema
                .graph
                .artifact(dep_id)
                .map(|a| a.generates.clone())
                .unwrap_or_default();
            Dependency {
                id: dep_id.clone(),
                path: change_dir.join(generates),
                done,
            }
        })
        .collect();

    // Décisions injectées : uniquement pour `design`. Pour les autres
    // artefacts on renvoie une liste vide plutôt que de charger l'index
    // pour rien.
    let decisions = if artifact.id == "design" {
        let index = decisions::index(fs, env, layout, config)?;
        warnings.extend(
            index
                .findings
                .iter()
                .map(|f| Warning::new(f.code, f.message.clone())),
        );
        build_decision_refs(&index, layout)
    } else {
        Vec::new()
    };

    Ok(Instructions {
        change: ctx.change.clone(),
        schema_name: ctx.schema.name().to_string(),
        artifact_id: artifact.id.clone(),
        description: artifact.description.clone(),
        resolved_output_path: change_dir.join(&artifact.generates),
        instruction: artifact.instruction.clone(),
        template: ctx.schema.template(fs, artifact)?,
        context: config.context.clone(),
        rules: config.rules_for(&artifact.id).to_vec(),
        dependencies,
        unlocks: ctx
            .schema
            .graph
            .unlocked_by(&artifact.id)
            .iter()
            .map(|id| id.to_string())
            .collect(),
        decisions,
        skipped: state == ArtifactState::Skipped,
        warnings,
    })
}

/// Convertit les entrées `in_effect` de l'index en `DecisionRef` prêtes
/// pour le contrat public.
///
/// L'ordre est celui de `in_effect` — qui suit l'ordre naturel des
/// entrées : projet d'abord puis sources héritées, chacun trié par id de
/// fichier. Prévisible et testable.
fn build_decision_refs(
    index: &decisions::DecisionIndex,
    layout: &Layout,
) -> Vec<DecisionRef> {
    let mut out = Vec::with_capacity(index.in_effect.len());
    for qid in &index.in_effect {
        let Some(entry) = index
            .entries
            .iter()
            .find(|e| &e.qualified_id == qid)
        else {
            continue; // ne devrait pas arriver — `in_effect` sort de `entries`
        };
        let status = match &entry.decision.status {
            DecisionStatus::Unknown(raw) => raw.clone(),
            other => other.as_str().to_string(),
        };
        // Chemin relatif au projet quand c'est possible ; sinon on garde
        // l'absolu (source héritée hors du dépôt).
        let relative_path = entry
            .path
            .strip_prefix(layout.project_root())
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| entry.path.clone());
        out.push(DecisionRef {
            id: entry.decision.id.clone(),
            qualified_id: qid.as_str(),
            title: entry.decision.title.clone(),
            status,
            tags: entry.decision.tags.clone(),
            path: relative_path,
            origin: match &entry.qualified_id.origin {
                Origin::Project => "projet".to_string(),
                Origin::Path(raw) => format!("path:{raw}"),
                Origin::Git(url) => format!("git:{url}"),
            },
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use codev_core::ChangeId;

    fn projet(fichiers: &[(&str, &str)]) -> MemoryFileSystem {
        let mut fs = MemoryFileSystem::new().with_file(
            "/p/_codev/config.yaml",
            "context: |\n  Pile : Rust\nrules:\n  specs:\n    - Comportement observable seulement\n  design:\n    - Citer les décisions\n",
        );
        for (path, contents) in fichiers {
            fs = fs.with_file(*path, *contents);
        }
        fs
    }

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn instructions(fs: &MemoryFileSystem, artefact: Option<&str>) -> Result<Instructions> {
        let layout = Layout::new("/p");
        let e = env();
        let config = crate::config::resolve(fs, &e, &layout).unwrap();
        let ctx = change::load(fs, &layout, &config, ChangeId::parse("add-auth").unwrap()).unwrap();
        for_artifact(fs, &e, &layout, &ctx, &config, artefact)
    }

    #[test]
    fn sans_artefact_nomme_donne_le_prochain_a_ecrire() {
        let fs = projet(&[
            ("/p/_codev/changes/add-auth/change.yaml", "schema: spec-driven"),
            ("/p/_codev/changes/add-auth/proposal.md", "# Proposal"),
        ]);
        let instr = instructions(&fs, None).unwrap();
        assert_eq!(instr.artifact_id, "specs");
    }

    #[test]
    fn porte_le_template_et_la_consigne_du_schema() {
        let fs = projet(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        let instr = instructions(&fs, Some("proposal")).unwrap();

        assert!(
            instr.instruction.is_some_and(|i| i.contains("POURQUOI")),
            "la consigne du schéma doit remonter"
        );
        assert!(
            instr.template.is_some_and(|t| t.contains("## Pourquoi")),
            "le template embarqué doit remonter"
        );
        assert_eq!(
            instr.resolved_output_path,
            PathBuf::from("/p/_codev/changes/add-auth/proposal.md")
        );
    }

    #[test]
    fn ninjecte_que_les_regles_de_lartefact_demande() {
        let fs = projet(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);

        let specs = instructions(&fs, Some("specs")).unwrap();
        assert_eq!(specs.rules.len(), 1);
        assert!(specs.rules[0].text.contains("observable"));

        let proposal = instructions(&fs, Some("proposal")).unwrap();
        assert!(
            proposal.rules.is_empty(),
            "proposal n'a pas de règle déclarée"
        );

        // Le contexte, lui, s'applique partout.
        assert_eq!(proposal.context.len(), 1);
        assert_eq!(proposal.context[0].origin, "projet");
    }

    #[test]
    fn dit_ce_que_lartefact_debloquera_et_ce_quil_faut_relire() {
        let fs = projet(&[
            ("/p/_codev/changes/add-auth/change.yaml", "schema: spec-driven"),
            ("/p/_codev/changes/add-auth/proposal.md", "# Proposal"),
        ]);
        let instr = instructions(&fs, Some("specs")).unwrap();

        assert_eq!(instr.unlocks, ["tasks"]);
        assert_eq!(instr.dependencies.len(), 1);
        assert_eq!(instr.dependencies[0].id, "proposal");
        assert!(instr.dependencies[0].done);
        assert_eq!(
            instr.dependencies[0].path,
            PathBuf::from("/p/_codev/changes/add-auth/proposal.md")
        );
    }

    #[test]
    fn instructions_portent_un_champ_decisions_meme_vide() {
        // Aucun ADR dans le projet — le champ existe mais reste vide.
        // Toujours présent : le consommateur teste `decisions.length` sans
        // branche.
        let fs = projet(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        let instr = instructions(&fs, Some("design")).unwrap();
        assert!(instr.decisions.is_empty(), "aucun ADR déclaré = vide");
    }

    #[test]
    fn design_recoit_les_decisions_en_vigueur() {
        let fs = projet(&[
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
            ("/p/_codev/changes/add-auth/proposal.md", "# Proposal"),
            (
                "/p/_codev/decisions/0001-fondation.md",
                "---\nid: 0001\ntitle: Fondation\nstatus: accepted\ndate: 2026-09-08\n---\n\n## Contexte\n\nx\n",
            ),
        ]);
        let instr = instructions(&fs, Some("design")).unwrap();
        assert_eq!(instr.decisions.len(), 1);
        assert_eq!(instr.decisions[0].id, "0001");
        assert_eq!(instr.decisions[0].title, "Fondation");
        assert_eq!(instr.decisions[0].status, "accepted");
        assert_eq!(instr.decisions[0].origin, "projet");
        // Chemin relatif au projet, pas absolu.
        assert_eq!(
            instr.decisions[0].path,
            PathBuf::from("_codev/decisions/0001-fondation.md")
        );
    }

    #[test]
    fn proposal_ne_recoit_pas_les_decisions() {
        // Décision présente MAIS artefact demandé n'est pas `design` : le
        // champ existe et reste vide. Un futur change pourra étendre à
        // d'autres artefacts sans casser ce test — il faudra le déplacer,
        // pas juste le supprimer, pour maintenir la limite explicite.
        let fs = projet(&[
            ("/p/_codev/changes/add-auth/change.yaml", "schema: spec-driven"),
            (
                "/p/_codev/decisions/0001.md",
                "---\nid: 0001\ntitle: X\nstatus: accepted\ndate: 2026-09-08\n---\n",
            ),
        ]);
        let instr = instructions(&fs, Some("proposal")).unwrap();
        assert!(
            instr.decisions.is_empty(),
            "seul `design` reçoit les décisions dans ce lot"
        );
    }

    #[test]
    fn un_artefact_neutralise_est_signale_et_non_a_creer() {
        let fs = projet(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven\nskip_specs: true\n",
        )]);
        let instr = instructions(&fs, Some("specs")).unwrap();
        assert!(instr.skipped);
        assert!(instr.warnings.iter().any(|w| w.code == "artifact_skipped"));
    }

    #[test]
    fn refuse_un_artefact_qui_nexiste_pas_dans_le_schema() {
        let fs = projet(&[(
            "/p/_codev/changes/add-auth/change.yaml",
            "schema: spec-driven",
        )]);
        let err = instructions(&fs, Some("croquis")).unwrap_err();
        assert_eq!(err.code(), "unknown_artifact");
    }

    #[test]
    fn quand_tout_est_ecrit_il_ny_a_plus_rien_de_pret() {
        let fs = projet(&[
            ("/p/_codev/changes/add-auth/change.yaml", "schema: spec-driven"),
            ("/p/_codev/changes/add-auth/proposal.md", "x"),
            ("/p/_codev/changes/add-auth/specs/user-auth/spec.md", "x"),
            ("/p/_codev/changes/add-auth/design.md", "x"),
            ("/p/_codev/changes/add-auth/tasks.md", "x"),
        ]);
        let err = instructions(&fs, None).unwrap_err();
        assert_eq!(err.code(), "no_artifact_ready");
    }

    #[test]
    fn instructions_design_omettent_les_decisions_deviees() {
        // Une héritée déviée par un ADR local n'apparaît PLUS dans les
        // décisions injectées à l'artefact `design` — l'agent voit la
        // dérive, pas la décision qu'elle remplace.
        let adr_locale = "---\nid: \"0007\"\ntitle: Alternative locale\nstatus: accepted\ndate: 2026-09-08\ndeviates_from: [\"path:~/partage/0100\"]\n---\n\n## Contexte\n\nx\n";
        let adr_heritee = "---\nid: \"0100\"\ntitle: Choix source\nstatus: accepted\ndate: 2026-09-08\n---\n\n## Contexte\n\ny\n";
        let fs = projet(&[
            ("/p/_codev/config.yaml", "inherits:\n  - path: ~/partage\n"),
            ("/p/_codev/decisions/0007-alternative.md", adr_locale),
            ("/home/partage/_codev/decisions/0100-choix.md", adr_heritee),
            (
                "/p/_codev/changes/add-auth/change.yaml",
                "schema: spec-driven",
            ),
            (
                "/p/_codev/changes/add-auth/proposal.md",
                "# Proposal\n\n## Pourquoi\n\nT\n\n## Ce qui change\n\n- x\n",
            ),
            (
                "/p/_codev/changes/add-auth/specs/x/spec.md",
                "## ADDED Requirements\n\n### Requirement: X\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n",
            ),
        ]);
        let instr = instructions(&fs, Some("design")).unwrap();

        let ids: Vec<&str> = instr.decisions.iter().map(|d| d.qualified_id.as_str()).collect();
        assert!(ids.contains(&"projet/0007"), "la dérive locale doit être là");
        assert!(
            !ids.contains(&"path:~/partage/0100"),
            "l'héritée déviée doit disparaître ; ids présents : {ids:?}"
        );
    }

    #[test]
    fn les_avertissements_de_configuration_suivent_jusquaux_instructions() {
        let mut fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "inherits:\n  - path: /nulle/part\n");
        fs = fs.with_file("/p/_codev/changes/add-auth/change.yaml", "schema: spec-driven");
        let instr = instructions(&fs, Some("proposal")).unwrap();
        assert!(instr.warnings.iter().any(|w| w.code == "inherit_unresolved"));
    }
}
