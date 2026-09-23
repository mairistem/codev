use codev_engine::Warning;

/// Un workflow : le corps d'une skill, plus ce qu'il faut pour la déclarer.
///
/// Le corps est un fichier markdown d'`assets/workflows/`, inclus à la
/// compilation. Il reste une **donnée** : l'améliorer ne demande pas de
/// toucher au code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Workflow {
    pub id: &'static str,
    /// Ce que l'agent lit pour décider si cette skill s'applique. C'est la
    /// phrase la plus importante du fichier : une description vague et la skill
    /// ne se déclenche jamais.
    pub description: &'static str,
    /// Les outils que le workflow a besoin d'utiliser.
    ///
    /// Sert aussi de garde-fou : `explore` n'obtient pas `Write`, ce qui rend
    /// sa promesse de ne rien écrire structurelle et non déclarative.
    pub allowed_tools: &'static str,
    pub body: &'static str,
}

/// Les workflows livrés par cette version.
///
/// Le catalogue ne contient que ce que le CLI sait réellement servir : un
/// workflow dont les commandes n'existent pas encore produirait une skill qui
/// échoue devant l'utilisateur. Ajouter un workflow, c'est ajouter un fichier
/// d'assets et une entrée ici.
pub const CATALOG: &[Workflow] = &[
    Workflow {
        id: "propose",
        description: "Créer un change codev et rédiger tous ses artefacts de planification \
                      en une fois — proposal, specs, design, tâches. À utiliser quand \
                      l'utilisateur décrit ce qu'il veut construire ou corriger et qu'il faut \
                      un plan prêt pour l'implémentation. Ne modifie aucun code. Détecte un \
                      identifiant de ticket (pattern [A-Z]{2,}-\\d+) mentionné dans le prompt \
                      et enrichit le proposal via le MCP Jira configuré côté projet, si \
                      disponible.",
        // `{{JIRA_MCP_TOOL}}` en fin de liste : placeholder substitué au
        // moment du rendering par le nom du tool MCP Jira déclaré dans
        // `_codev/config.yaml.mcp.jira_tool`. Sans config, le placeholder
        // (et la virgule qui le précède) sont retirés proprement — voir
        // `codev-agents::claude::substitute_jira_mcp`. La skill reste
        // lecture seule sur Jira : un seul tool déclaré, jamais d'écriture.
        allowed_tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, {{JIRA_MCP_TOOL}}",
        body: include_str!("../../../assets/workflows/propose.md"),
    },
    Workflow {
        id: "explore",
        description: "Défricher une idée, enquêter sur un problème ou clarifier un besoin \
                      avant de créer un change codev. À utiliser quand la demande est floue, \
                      qu'il faut comparer plusieurs approches, ou qu'on ne sait pas encore quoi \
                      construire. N'écrit aucun fichier.",
        allowed_tools: "Bash(codev:*), Read, Glob, Grep",
        body: include_str!("../../../assets/workflows/explore.md"),
    },
    Workflow {
        id: "apply",
        description: "Implémenter les tâches d'un change codev déjà planifié : lire tasks.md, \
                      traiter chaque case non cochée dans l'ordre, cocher au fur et à mesure. \
                      Modifie du code du projet. Ne modifie pas d'autres changes, n'archive pas \
                      et ne sync pas — ces pas restent explicites côté utilisateur.",
        // `Bash` général en plus de `Bash(codev:*)` : les tâches citent des
        // commandes de vérification (cargo, git, npm, python…) qu'il faut
        // pouvoir lancer. Le préfixe `Bash(codev:*)` reste en tête pour
        // documenter l'usage principal.
        allowed_tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash",
        body: include_str!("../../../assets/workflows/apply.md"),
    },
    Workflow {
        id: "sync",
        description: "Faire entrer les deltas d'un change codev déjà planifié dans les specs \
                      principales, sans déplacer le change. À utiliser quand une capacité \
                      nouvelle doit apparaître dans les specs avant d'être consommée par un \
                      autre change, ou pour relire le merge avant d'archiver. N'archive pas.",
        // `Read` en plus de `Bash(codev:*)` pour permettre à la skill de
        // relire `tasks.md` si l'utilisateur pose une question de contexte —
        // sans jamais écrire. Pas de `Bash` général : la seule action
        // effective passe par `codev`.
        allowed_tools: "Bash(codev:*), Read",
        body: include_str!("../../../assets/workflows/sync.md"),
    },
    Workflow {
        id: "archive",
        description: "Clore un change codev : fusionner ses deltas dans les specs principales et \
                      déplacer le dossier vers l'archive datée. Refuse d'agir si la validation \
                      remonte des erreurs, et renvoie alors vers `codev validate` pour le détail.",
        allowed_tools: "Bash(codev:*), Read",
        body: include_str!("../../../assets/workflows/archive.md"),
    },
    Workflow {
        id: "update",
        description: "Réviser un artefact de planification déjà écrit d'un change codev actif — \
                      proposal, specs, design ou tasks — en préservant la cohérence avec les \
                      autres artefacts. Ne modifie aucun code du projet, ne crée aucun artefact \
                      manquant, ne touche à aucun change archivé.",
        // Édition markdown directe via Edit/Write, pas de Bash général : la
        // skill révise du texte, elle n'exécute pas de tests. La règle
        // « seule `apply` a le Bash général » reste vraie.
        allowed_tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep",
        body: include_str!("../../../assets/workflows/update.md"),
    },
    Workflow {
        id: "onboard",
        description: "Présenter codev à un utilisateur qui le découvre : ce que fait l'outil, \
                      l'état actuel du projet, et la prochaine action recommandée. Strictement \
                      en lecture — ne modifie ni ne crée rien.",
        // Lecture pure : le rôle est de guider, jamais d'agir à la place.
        // Pas de Write, pas de Edit, pas de Grep (chemins connus), pas de
        // Bash général — la règle « seule `apply` a le Bash général »
        // reste vraie.
        allowed_tools: "Bash(codev:*), Read, Glob",
        body: include_str!("../../../assets/workflows/onboard.md"),
    },
];

/// Les workflows installés quand la configuration n'en désigne aucun.
///
/// `onboard` est le seul workflow « d'accueil » : il fait partie du
/// catalogue par défaut parce que sa raison d'être est de guider un
/// utilisateur qui vient d'installer codev. Le cacher derrière un opt-in
/// serait absurde : celui qui aurait besoin de le découvrir ne saurait
/// pas l'activer.
pub const DEFAULT_WORKFLOWS: &[&str] = &["propose", "explore", "onboard"];

pub fn find(id: &str) -> Option<&'static Workflow> {
    CATALOG.iter().find(|w| w.id == id)
}

/// Résout la liste demandée en workflows connus.
///
/// Un identifiant inconnu produit un avertissement, pas une erreur : une faute
/// de frappe dans `config.yaml` ne doit pas empêcher l'installation des autres
/// skills, mais elle ne doit pas non plus passer inaperçue.
pub fn select(requested: Option<&[String]>) -> (Vec<&'static Workflow>, Vec<Warning>) {
    let mut warnings = Vec::new();
    let ids: Vec<String> = match requested {
        Some(ids) => ids.to_vec(),
        None => DEFAULT_WORKFLOWS.iter().map(|s| s.to_string()).collect(),
    };

    let mut workflows = Vec::new();
    for id in ids {
        match find(&id) {
            Some(workflow) if !workflows.contains(&workflow) => workflows.push(workflow),
            Some(_) => {}
            None => warnings.push(Warning::new(
                "unknown_workflow",
                format!(
                    "workflow « {id} » inconnu, ignoré — cette version fournit : {}",
                    CATALOG
                        .iter()
                        .map(|w| w.id)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )),
        }
    }
    (workflows, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_workflow_a_un_corps_et_une_description_utilisables() {
        for workflow in CATALOG {
            assert!(
                workflow.body.len() > 200,
                "le corps de « {} » est suspicieusement court",
                workflow.id
            );
            assert!(
                workflow.description.len() > 60,
                "la description de « {} » est trop vague pour déclencher la skill",
                workflow.id
            );
            assert!(
                workflow.allowed_tools.contains("Bash(codev:*)"),
                "« {} » doit pouvoir appeler le CLI",
                workflow.id
            );
        }
    }

    #[test]
    fn explore_ne_peut_pas_ecrire() {
        // Sa promesse « n'écrit aucun fichier » doit être structurelle.
        let explore = find("explore").unwrap();
        assert!(!explore.allowed_tools.contains("Write"));
        assert!(!explore.allowed_tools.contains("Edit"));
    }

    #[test]
    fn cycle_completion_skills_present_et_restreintes() {
        // Les deux skills du bouclage : présentes, avec `allowed-tools`
        // strictement `Bash(codev:*), Read` — pas de `Bash` général, pas de
        // `Write`/`Edit`. Ce qui rend la skill incapable de modifier un
        // fichier par elle-même : tout passe par le CLI.
        for id in ["sync", "archive"] {
            let workflow = find(id).unwrap_or_else(|| panic!("« {id} » doit être dans le CATALOG"));
            assert_eq!(
                workflow.allowed_tools, "Bash(codev:*), Read",
                "« {id} » doit se cantonner à Bash(codev:*), Read"
            );
            assert!(!workflow.allowed_tools.contains("Write"));
            assert!(!workflow.allowed_tools.contains("Edit"));
            assert!(!workflow.allowed_tools.ends_with(", Bash"));
        }
    }

    #[test]
    fn sync_et_archive_citent_les_champs_du_contrat() {
        // Les skills nomment explicitement les champs du contrat public
        // qu'elles consomment. Un renommage de champ dans `contract.rs` doit
        // remonter jusqu'ici via `grep`, plutôt que devenir un drame
        // silencieux à l'exécution.
        let sync = find("sync").unwrap();
        for champ in ["SyncReportV1", "changeName", "created", "updated", "unchanged"] {
            assert!(
                sync.body.contains(champ),
                "sync doit citer nommément « {champ} »"
            );
        }
        let archive = find("archive").unwrap();
        for champ in ["ArchiveReportV1", "movedTo", "validation_failed", "status"] {
            assert!(
                archive.body.contains(champ),
                "archive doit citer nommément « {champ} »"
            );
        }
    }

    #[test]
    fn update_est_dans_le_catalogue_et_a_les_bons_outils() {
        let update = find("update").expect("update doit être dans le CATALOG");
        // Édition markdown : Write et Edit obligatoires. Bash(codev:*) seul —
        // pas de Bash général, la règle « seule apply l'a » reste préservée.
        assert_eq!(
            update.allowed_tools,
            "Bash(codev:*), Read, Write, Edit, Glob, Grep"
        );
        assert!(!update.allowed_tools.ends_with(", Bash"));
    }

    #[test]
    fn update_cite_ses_frontieres() {
        // Traceur d'un renommage ou d'une suppression accidentelle des
        // garde-fous du corps de la skill.
        let update = find("update").unwrap();
        for frontiere in ["ne modifie", "archivé"] {
            assert!(
                update.body.contains(frontiere),
                "le body de `update` doit citer la frontière « {frontiere} »"
            );
        }
    }

    #[test]
    fn apply_est_dans_le_catalogue_et_a_les_bons_outils() {
        let apply = find("apply").expect("apply doit être dans le CATALOG");
        // Le préfixe `Bash(codev:*)` en premier — usage principal — et le
        // `Bash` général en dernier — pour les commandes de vérification.
        // C'est le SEUL workflow à demander ce dernier, garde-fou contre un
        // élargissement silencieux à d'autres.
        assert!(apply.allowed_tools.starts_with("Bash(codev:*)"));
        assert!(apply.allowed_tools.contains(", Bash"));
        assert!(apply.allowed_tools.contains("Edit"));

        let autres_avec_bash_general = CATALOG
            .iter()
            .filter(|w| w.id != "apply")
            .filter(|w| w.allowed_tools.ends_with(", Bash") || w.allowed_tools == "Bash")
            .count();
        assert_eq!(
            autres_avec_bash_general, 0,
            "seul `apply` doit avoir le Bash général — sinon la promesse de restriction se perd"
        );
    }

    #[test]
    fn sans_demande_installe_le_catalogue_par_defaut() {
        // `apply`, `sync`, `archive` et `update` n'entrent PAS dans
        // DEFAULT_WORKFLOWS — décision assumée : le catalogue par défaut se
        // limite à ce qui prépare le travail (`propose`, `explore`) et à
        // l'accueil (`onboard`) ; le reste est opt-in projet par projet.
        let (workflows, warnings) = select(None);
        assert_eq!(
            workflows.iter().map(|w| w.id).collect::<Vec<_>>(),
            DEFAULT_WORKFLOWS
        );
        for opt_in in ["apply", "sync", "archive", "update"] {
            assert!(
                !DEFAULT_WORKFLOWS.contains(&opt_in),
                "« {opt_in} » doit rester opt-in ; ajoute-le explicitement dans _codev/config.yaml"
            );
        }
        assert!(warnings.is_empty());
    }

    #[test]
    fn onboard_est_dans_le_catalogue_et_a_les_bons_outils() {
        let onboard = find("onboard").expect("onboard doit être dans le CATALOG");
        // Lecture pure : Bash(codev:*), Read, Glob — pas de Write/Edit,
        // pas de Bash général, pas de Grep (chemins connus).
        assert_eq!(onboard.allowed_tools, "Bash(codev:*), Read, Glob");
        assert!(!onboard.allowed_tools.contains("Write"));
        assert!(!onboard.allowed_tools.contains("Edit"));
        assert!(!onboard.allowed_tools.ends_with(", Bash"));
    }

    #[test]
    fn onboard_est_dans_le_catalogue_par_defaut() {
        // `onboard` fait partie du catalogue par défaut — c'est son
        // rôle même : accueillir un utilisateur qui n'a rien
        // configuré.
        assert!(DEFAULT_WORKFLOWS.contains(&"onboard"));
    }

    #[test]
    fn onboard_cite_ses_trois_blocs() {
        // Traceur d'une refonte accidentelle du body : la skill promet
        // trois blocs (description, état, suite/action). Ces mots-clés
        // doivent rester présents.
        let onboard = find("onboard").unwrap();
        for mot_cle in ["codev, c'est", "ici, tu as", "la suite"] {
            assert!(
                onboard.body.to_lowercase().contains(&mot_cle.to_lowercase()),
                "le body de `onboard` doit citer le bloc « {mot_cle} »"
            );
        }
    }

    #[test]
    fn un_workflow_inconnu_avertit_sans_bloquer_les_autres() {
        let demande = vec!["propose".to_string(), "teleportation".to_string()];
        let (workflows, warnings) = select(Some(&demande));
        assert_eq!(workflows.iter().map(|w| w.id).collect::<Vec<_>>(), ["propose"]);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "unknown_workflow");
        assert!(warnings[0].message.contains("propose"), "le message doit lister ce qui existe");
    }

    #[test]
    fn deduplique_une_demande_repetee() {
        let demande = vec!["propose".to_string(), "propose".to_string()];
        let (workflows, _) = select(Some(&demande));
        assert_eq!(workflows.len(), 1);
    }

    // ─────────────── MCP Atlassian sur propose (première intégration) ───────────────

    #[test]
    fn propose_utilise_un_placeholder_pour_le_mcp_jira() {
        // Le CATALOG ne cite plus aucun nom MCP en dur : c'est le
        // rendering qui substitue `{{JIRA_MCP_TOOL}}` d'après la config
        // projet. Verrouille la présence du placeholder dans les deux
        // endroits attendus : `allowed_tools` et body.
        let propose = find("propose").expect("propose doit être dans le CATALOG");
        assert!(
            propose.allowed_tools.contains("{{JIRA_MCP_TOOL}}"),
            "allowed_tools doit contenir le placeholder pour la substitution"
        );
        assert!(
            propose.body.contains("{{JIRA_MCP_TOOL}}"),
            "le body doit citer le placeholder pour que la substitution \
             injecte le nom du tool dans les instructions à l'agent"
        );
    }

    #[test]
    fn propose_ne_cite_pas_de_nom_de_mcp_en_dur() {
        // Garde-fou contre la régression du hardcodage. Aucune chaîne
        // `mcp__...` ne doit apparaître dans le CATALOG — tout passe par
        // le placeholder.
        let propose = find("propose").unwrap();
        assert!(
            !propose.allowed_tools.contains("mcp__"),
            "aucun nom MCP ne doit être hardcodé dans allowed_tools"
        );
        assert!(
            !propose.body.contains("mcp__"),
            "aucun nom MCP ne doit être hardcodé dans le body"
        );
    }

    #[test]
    fn propose_cite_la_detection_de_ticket_dans_son_body() {
        // Traceur d'une refonte accidentelle du body : les mots-clés du
        // pattern et du placeholder doivent rester présents pour que la
        // logique décrite reste visible à l'agent qui lit la skill.
        let propose = find("propose").unwrap();
        for mot_cle in ["[A-Z]{2,}-\\d+", "{{JIRA_MCP_TOOL}}"] {
            assert!(
                propose.body.contains(mot_cle),
                "le body de `propose` doit citer « {mot_cle} »"
            );
        }
    }
}
