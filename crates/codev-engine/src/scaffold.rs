use codev_core::{ChangeId, Layout, Plan, WriteMode};

use crate::metadata::ChangeMetadata;

const DEFAULT_CONFIG: &str = r#"# Configuration de codev pour ce projet.
schema: spec-driven

# Les workflows installés comme skills Claude Code par `codev init` et
# `codev update`. Absent, le catalogue par défaut s'applique.
# workflows:
#   - propose
#   - explore
#   - onboard
#   - apply
#   - sync
#   - archive
#   - update

# Contexte injecté dans les instructions de TOUS les artefacts : ce que l'agent
# doit savoir de ce projet avant d'écrire quoi que ce soit.
# context: |
#   Pile technique : ...
#   Conventions d'API : ...
#   Tests : ...

# Règles par artefact, injectées uniquement pour l'artefact concerné.
# rules:
#   specs:
#     - Décrire un comportement observable, jamais une implémentation.
#   design:
#     - Citer les décisions de _codev/decisions/ qui contraignent l'approche.

# Sources héritées, en lecture seule, du plus général au plus spécifique.
# inherits:
#   - path: ~/codev/partage

# Nom des outils MCP à utiliser dans les skills. Le nom dépend de la config
# Claude Code de ton utilisateur — décommenter et remplacer par le nom exact
# du MCP disponible dans ta session. Sans cette clef, la détection de tickets
# dans `/codev-propose` est inactive (fallback silencieux).
# mcp:
#   jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
"#;

/// Marqueur déposé dans les dossiers encore vides.
///
/// Git ne versionne pas les dossiers vides : sans ce fichier, un `codev init`
/// suivi d'un commit ne transmettrait pas la structure à l'équipe, et le
/// collègue suivant se demanderait où sont les dossiers.
const KEEP_FILE: &str = ".gitkeep";

const KEEP_CONTENT: &str =
    "# Ce fichier garde le dossier dans git tant qu'il est vide. Supprime-le quand il ne l'est plus.\n";

/// Le plan de `codev init` : la structure `_codev/` et sa configuration.
///
/// Les skills n'en font pas partie — c'est `codev-agents` qui les planifie, et
/// le CLI qui réunit les deux plans. Un crate, une responsabilité.
///
/// Tout est en [`WriteMode::CreateOnly`] : relancer `init` sur un projet déjà
/// initialisé ne doit rien écraser, et donc ne rien risquer.
pub fn plan_init(layout: &Layout) -> Plan {
    let mut plan = Plan::new();

    plan.dir(layout.planning_dir());
    for dir in [
        layout.specs_dir(),
        layout.decisions_dir(),
        layout.changes_dir(),
        layout.archive_dir(),
        layout.schemas_dir(),
    ] {
        plan.dir(&dir);
        plan.write(dir.join(KEEP_FILE), KEEP_CONTENT, WriteMode::CreateOnly);
    }

    plan.write(layout.config_file(), DEFAULT_CONFIG, WriteMode::CreateOnly);
    plan
}

/// Le plan de `codev new change`.
pub fn plan_new_change(layout: &Layout, change: &ChangeId, metadata: &ChangeMetadata) -> Plan {
    let mut plan = Plan::new();
    plan.dir(layout.change_dir(change));
    plan.write(
        layout.change_metadata(change),
        metadata.to_yaml(),
        WriteMode::CreateOnly,
    );
    plan
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use std::path::{Path, PathBuf};

    #[test]
    fn init_prepare_la_structure_complete() {
        let plan = plan_init(&Layout::new("/p"));

        for attendu in [
            "/p/_codev",
            "/p/_codev/specs",
            "/p/_codev/decisions",
            "/p/_codev/changes",
            "/p/_codev/changes/archive",
            "/p/_codev/schemas",
        ] {
            assert!(
                plan.dirs.contains(&PathBuf::from(attendu)),
                "{attendu} devrait être planifié"
            );
        }
        assert!(plan
            .writes
            .iter()
            .any(|w| w.path == Path::new("/p/_codev/config.yaml")));
    }

    #[test]
    fn init_necrase_jamais_rien() {
        let plan = plan_init(&Layout::new("/p"));
        assert!(
            plan.writes.iter().all(|w| w.mode == WriteMode::CreateOnly),
            "aucune écriture de `init` ne doit pouvoir écraser un fichier"
        );
    }

    #[test]
    fn la_config_par_defaut_est_valide_et_relisible() {
        // Le fichier que l'on écrit doit passer notre propre lecteur, sans quoi
        // le premier `codev status` après un `init` échouerait.
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", DEFAULT_CONFIG);
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        let resolved = config::resolve(&fs, &env, &Layout::new("/p")).unwrap();
        assert_eq!(resolved.schema, "spec-driven");
        assert!(resolved.warnings.is_empty());
    }

    #[test]
    fn new_change_ecrit_les_metadonnees() {
        let change = ChangeId::parse("add-auth").unwrap();
        let metadata = ChangeMetadata::new("spec-driven", "2026-09-08");
        let plan = plan_new_change(&Layout::new("/p"), &change, &metadata);

        assert_eq!(plan.dirs, [PathBuf::from("/p/_codev/changes/add-auth")]);
        assert_eq!(plan.writes.len(), 1);
        let write = &plan.writes[0];
        assert_eq!(
            write.path,
            PathBuf::from("/p/_codev/changes/add-auth/change.yaml")
        );
        assert!(write.contents.contains("schema: spec-driven"));
        assert_eq!(write.mode, WriteMode::CreateOnly);
    }
}
