//! Les deux règles qui dépendent du disque : présence de deltas et cohérence
//! avec le marqueur `skip_specs`. Elles ne sont pas dans `codev-core::validate`
//! parce qu'elles ont besoin de compter des fichiers, pas seulement d'un AST.

use std::path::PathBuf;

use codev_core::parser::ast::{Finding, Severity};
use codev_core::validate::codes;
use codev_core::Layout;

use crate::change::ChangeContext;
use crate::error::{EngineError, Result};
use crate::ports::FileSystem;

use super::report::LocatedFinding;

/// Vérifie les métadonnées d'un change vis-à-vis de son contenu.
///
/// Deux cas rejetés :
///
/// - `zero_delta_without_marker` : aucun delta sous `specs/` et
///   `skip_specs: true` non déclaré. Le validateur exige une décision
///   explicite ; sans elle, l'utilisateur pourrait oublier une capacité.
/// - `skip_specs_conflict` : `skip_specs: true` déclaré mais des `.md`
///   existent quand même sous `specs/`. C'est contradictoire ; l'archive
///   ne saurait pas si elle doit les fusionner ou les ignorer.
pub fn check_change_metadata(
    fs: &dyn FileSystem,
    layout: &Layout,
    ctx: &ChangeContext,
) -> Result<Vec<LocatedFinding>> {
    let change_dir = layout.change_dir(&ctx.change);
    let specs_dir = change_dir.join("specs");
    let metadata_path = layout.change_metadata(&ctx.change);
    let metadata_relative = metadata_path
        .strip_prefix(layout.project_root())
        .unwrap_or(&metadata_path)
        .to_path_buf();

    let deltas = list_delta_files(fs, &specs_dir)?;
    let has_deltas = !deltas.is_empty();

    let mut out = Vec::new();

    if !has_deltas && !ctx.metadata.skip_specs {
        out.push(zero_delta_finding(metadata_relative.clone()));
    }
    if has_deltas && ctx.metadata.skip_specs {
        out.push(skip_specs_conflict_finding(metadata_relative, &deltas, layout));
    }

    Ok(out)
}

fn zero_delta_finding(metadata_relative: PathBuf) -> LocatedFinding {
    LocatedFinding::from_finding(
        Finding {
            severity: Severity::Error,
            code: codes::ZERO_DELTA_WITHOUT_MARKER,
            // Ligne 1 par défaut : le finding porte sur le change dans son
            // ensemble, pas sur un endroit précis de `change.yaml`.
            line: 1,
            message: "aucun fichier de delta sous `specs/` et `skip_specs: true` n'est pas déclaré ; \
                      ajoute un delta ou pose `skip_specs: true` dans change.yaml"
                .into(),
        },
        metadata_relative,
    )
}

fn skip_specs_conflict_finding(
    metadata_relative: PathBuf,
    deltas: &[PathBuf],
    layout: &Layout,
) -> LocatedFinding {
    let apercus: Vec<String> = deltas
        .iter()
        .take(3)
        .map(|p| {
            p.strip_prefix(layout.project_root())
                .unwrap_or(p)
                .display()
                .to_string()
        })
        .collect();
    let suffix = if deltas.len() > 3 {
        format!(", … ({} au total)", deltas.len())
    } else {
        String::new()
    };
    LocatedFinding::from_finding(
        Finding {
            severity: Severity::Error,
            code: codes::SKIP_SPECS_CONFLICT,
            line: 1,
            message: format!(
                "`skip_specs: true` est déclaré mais des fichiers de delta existent : {}{}. \
                 Retire `skip_specs` ou supprime les fichiers du dossier `specs/`",
                apercus.join(", "),
                suffix
            ),
        },
        metadata_relative,
    )
}

fn list_delta_files(fs: &dyn FileSystem, specs_dir: &std::path::Path) -> Result<Vec<PathBuf>> {
    let files = fs.walk_files(specs_dir).map_err(|e| EngineError::Unreadable {
        path: specs_dir.to_path_buf(),
        reason: e.to_string(),
    })?;
    let mut out: Vec<PathBuf> = files
        .into_iter()
        .filter(|p| p.ends_with(".md"))
        .map(|p| specs_dir.join(p))
        .collect();
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::change;
    use crate::config;
    use crate::ports::{FixedEnv, MemoryFileSystem};
    use codev_core::ChangeId;

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn charger_change(fs: &dyn FileSystem, nom: &str) -> ChangeContext {
        let layout = Layout::new("/p");
        let cfg = config::resolve(fs, &env(), &layout).unwrap();
        change::load(fs, &layout, &cfg, ChangeId::parse(nom).unwrap()).unwrap()
    }

    #[test]
    fn zero_delta_sans_marqueur_echoue() {
        // Change actif, aucun delta, `skip_specs` non déclaré : rejet.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/changes/refactor/change.yaml", "schema: spec-driven")
            .with_file("/p/_codev/changes/refactor/proposal.md", "x");

        let ctx = charger_change(&fs, "refactor");
        let findings = check_change_metadata(&fs, &Layout::new("/p"), &ctx).unwrap();

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].finding.code, codes::ZERO_DELTA_WITHOUT_MARKER);
        assert!(findings[0].finding.message.contains("skip_specs"), "{}", findings[0].finding.message);
    }

    #[test]
    fn zero_delta_avec_marqueur_est_accepte() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/refactor/change.yaml",
                "schema: spec-driven\nskip_specs: true\n",
            )
            .with_file("/p/_codev/changes/refactor/proposal.md", "x");

        let ctx = charger_change(&fs, "refactor");
        let findings = check_change_metadata(&fs, &Layout::new("/p"), &ctx).unwrap();
        assert!(findings.is_empty(), "aucun finding attendu : {findings:?}");
    }

    #[test]
    fn skip_specs_avec_delta_est_un_conflit() {
        // Le marqueur est déclaré ET un delta existe : contradictoire.
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/changes/refactor/change.yaml",
                "schema: spec-driven\nskip_specs: true\n",
            )
            .with_file(
                "/p/_codev/changes/refactor/specs/x/spec.md",
                "## Purpose\n\nune spec\n",
            );

        let ctx = charger_change(&fs, "refactor");
        let findings = check_change_metadata(&fs, &Layout::new("/p"), &ctx).unwrap();

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].finding.code, codes::SKIP_SPECS_CONFLICT);
        assert!(findings[0].finding.message.contains("Retire"), "{}", findings[0].finding.message);
    }
}
