//! Le binaire `codev` : la coquille impérative.
//!
//! Aucune décision ici — l'analyse des arguments, l'appel d'une commande, et le
//! rendu. Deux sorties pour un même résultat : un texte pour un humain, un
//! document JSON pour une skill.

mod commands;
mod contract;
mod docs;
mod render;

use clap::{Parser, Subcommand};
use codev_engine::{RealFileSystem, SystemClock, SystemEnv};
use serde::Serialize;
use serde_json::json;

use commands::{Ctx, Failure};
use contract::{
    ArchiveReportV1, ChangesV1, DecisionCreatedV1, DecisionDeviatedV1, DecisionListReportV1,
    DecisionPromotedV1, DecisionSealedV1, DecisionShowReportV1, DecisionSupersededV1, DecisionV1,
    InstructionsV1, NewChangeV1, PinChangeV1, SchemaV1, SealEntryV1,
    SchemasV1, SetupV1, SourceDetailV1, SourceStateV1, SourcesListReportV1, SourcesUpdateReportV1,
    SpecsV1, StatusV1, SyncReportV1, ValidateReportV1,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "codev",
    version,
    about = "Développement piloté par les specs, pour Claude Code",
    long_about = "codev ajoute à un dépôt une fine couche de specs pour que toi et ton agent \
                  soyez d'accord sur ce qui doit être construit avant qu'une ligne de code ne \
                  soit écrite.\n\nLes commandes ci-dessous s'exécutent dans ton terminal. Les \
                  workflows, eux, s'invoquent dans le chat de Claude Code : /codev-propose, \
                  /codev-explore."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialise codev dans un projet et installe les skills Claude Code
    Init {
        /// Dossier du projet (par défaut : le dossier courant)
        path: Option<String>,
        /// Réécrit les skills même modifiées à la main
        #[arg(long)]
        force: bool,
        #[arg(long)]
        json: bool,
    },

    /// Régénère les skills après une mise à jour de codev
    Update {
        /// Réécrit les skills même modifiées à la main
        #[arg(long)]
        force: bool,
        #[arg(long)]
        json: bool,
    },

    /// Crée un nouvel élément
    New {
        #[command(subcommand)]
        what: NewCommand,
    },

    /// Liste les changes actifs, ou les capacités spécifiées avec --specs
    List {
        #[arg(long)]
        specs: bool,
        #[arg(long)]
        json: bool,
    },

    /// Affiche l'état des artefacts d'un change
    Status {
        /// Nom du change (déduit s'il n'y en a qu'un)
        #[arg(long)]
        change: Option<String>,
        #[arg(long)]
        json: bool,
    },

    /// Donne tout ce qu'il faut pour écrire un artefact
    Instructions {
        /// Identifiant de l'artefact (par défaut : le prochain à écrire)
        artifact: Option<String>,
        #[arg(long)]
        change: Option<String>,
        #[arg(long)]
        json: bool,
    },

    /// Liste les schémas de workflow disponibles
    Schemas {
        #[arg(long)]
        json: bool,
    },

    /// Crée, consulte et supersède les décisions d'architecture
    Decision {
        #[command(subcommand)]
        what: DecisionCommand,
    },

    /// Gère les sources héritées (path locales et git distantes)
    Sources {
        #[command(subcommand)]
        what: SourcesCommand,
    },

    /// Fusionne les deltas d'un change dans les specs principales sans archiver
    Sync {
        /// Nom du change (déduit s'il n'y en a qu'un)
        #[arg(long)]
        change: Option<String>,
        #[arg(long)]
        json: bool,
    },

    /// Fusionne puis déplace un change vers l'archive datée
    Archive {
        /// Nom du change (déduit s'il n'y en a qu'un)
        #[arg(long)]
        change: Option<String>,
        #[arg(long)]
        json: bool,
    },

    /// Ouvre la documentation codev dans le navigateur
    ///
    /// Trois formes :
    ///
    /// - défaut : écrit `codev-docs-<version>.html` dans le dossier
    ///   temporaire système et l'ouvre dans le navigateur ;
    /// - `--write <PATH>` : écrit le HTML au chemin donné, n'ouvre
    ///   rien (mode diffusion — email, Confluence, share drive) ;
    /// - `--print` : imprime le markdown source sur stdout (pour
    ///   pipeliner vers `less`, `bat` ou un LLM).
    Docs {
        /// Imprime le markdown source sur stdout (pas d'ouverture)
        #[arg(long, conflicts_with = "write")]
        print: bool,
        /// Écrit le HTML au chemin donné (pas d'ouverture)
        #[arg(long, value_name = "PATH")]
        write: Option<std::path::PathBuf>,
    },

    /// Génère un script de complétion shell pour l'installation locale
    ///
    /// La sortie va sur stdout — redirige-la vers ton dossier de
    /// complétions selon ton shell. Procédures typiques :
    ///
    /// - bash    : `codev completions bash > ~/.local/share/bash-completion/completions/codev`
    /// - zsh     : `codev completions zsh > "${fpath[1]}/_codev"` puis `compinit`
    /// - fish    : `codev completions fish > ~/.config/fish/completions/codev.fish`
    /// - powershell : `codev completions powershell | Out-String | Invoke-Expression`
    ///
    /// La commande ne touche à aucun fichier ; elle ne lit pas non plus
    /// `_codev/` et fonctionne dans n'importe quel répertoire.
    Completions {
        /// Shell cible : bash, zsh, fish, powershell, elvish
        shell: clap_complete::Shell,
    },

    /// Vérifie changes et specs pour erreurs structurelles et cohérence
    Validate {
        /// Nom d'un change ou d'une capacité de spec, précis
        item: Option<String>,
        /// Valider tous les changes actifs et toutes les specs principales
        #[arg(long, conflicts_with_all = ["changes", "specs", "item"])]
        all: bool,
        /// Valider tous les changes actifs
        #[arg(long, conflicts_with_all = ["all", "specs", "item"])]
        changes: bool,
        /// Valider toutes les specs principales
        #[arg(long, conflicts_with_all = ["all", "changes", "item"])]
        specs: bool,
        /// Traite tout finding (Warning inclus) comme un motif d'exit code
        /// non-nul. Utile pour la CI et l'automation.
        #[arg(long)]
        strict: bool,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum SourcesCommand {
    /// Liste toutes les sources déclarées avec leur état
    List {
        #[arg(long)]
        json: bool,
    },
    /// Résout les refs, télécharge, met à jour `_codev/codev.lock`
    Update {
        #[arg(long)]
        json: bool,
    },
    /// Affiche les détails d'une source précise
    Show {
        /// URL (source `git:`) ou chemin (source `path:`)
        target: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum DecisionCommand {
    /// Liste les décisions locales et héritées
    List {
        #[arg(long)]
        json: bool,
    },
    /// Affiche une décision précise
    Show {
        /// Identifiant court (`0007`) ou qualifié (`path:~/partage/0100`)
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Crée une nouvelle décision locale
    New {
        /// Titre libre — sera slugifié pour le nom de fichier
        title: String,
        /// Statut initial (accepted par défaut)
        #[arg(long, default_value = "accepted")]
        status: String,
        #[arg(long)]
        json: bool,
    },
    /// Supersède une décision : la marque `superseded` et en crée une nouvelle
    Supersede {
        /// Identifiant de la décision à superseder (court ou qualifié)
        old_id: String,
        /// Titre de la nouvelle décision
        new_title: String,
        #[arg(long)]
        json: bool,
    },
    /// Ajoute ou réécrit le sceau d'une décision locale
    ///
    /// Sans `--force` : refuse si un sceau existe et que le corps a changé.
    /// Avec `--force` : réécrit le sceau (à utiliser après une édition
    /// délibérée du corps).
    Seal {
        /// Identifiant court (`0007`) — l'hérité (`path:` / `git:`) est refusé
        id: String,
        /// Réécrit un sceau existant même si le corps a changé
        #[arg(long)]
        force: bool,
        #[arg(long)]
        json: bool,
    },
    /// Enregistre une dérive locale d'une décision héritée
    ///
    /// Crée un ADR local `accepted` qui référence explicitement l'héritée
    /// dont on choisit de s'écarter. La décision héritée reste visible
    /// dans `codev decision list`, mais disparaît des instructions
    /// injectées à l'artefact `design`. Pour dévier d'une décision
    /// locale, utilise `codev decision supersede`.
    Deviate {
        /// Identifiant qualifié de la décision héritée à écarter
        ///
        /// Exemple : `path:~/partage/0100` ou
        /// `git:git@github.com:acme/shared.git/0100`.
        target: String,
        /// Titre libre de la dérive locale — sera slugifié
        new_title: String,
        #[arg(long)]
        json: bool,
    },
    /// Promeut un bloc `### Décision : <titre>` d'un `design.md` en ADR
    ///
    /// Extrait le contenu du bloc, crée un ADR local scellé par K3, et
    /// remplace le corps du bloc par une référence textuelle vers le
    /// nouvel ADR. Refuse un change archivé.
    Promote {
        /// Nom du change actif dont le `design.md` porte le bloc
        change: String,
        /// Titre exact du bloc à promouvoir (ce qui suit `Décision : `)
        title: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum NewCommand {
    /// Crée un change
    Change {
        /// Nom en kebab-case (`add-user-auth`)
        name: String,
        /// Schéma de workflow à utiliser
        #[arg(long)]
        schema: Option<String>,
        /// Objectif, conservé dans les métadonnées du change
        #[arg(long)]
        goal: Option<String>,
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    std::process::exit(run(Cli::parse()));
}

fn run(cli: Cli) -> i32 {
    let fs = RealFileSystem;
    let env = SystemEnv;
    let clock = SystemClock;
    let ctx = Ctx {
        fs: &fs,
        env: &env,
        clock: &clock,
        version: VERSION,
    };

    match cli.command {
        Command::Docs { print, write } => {
            if print {
                use std::io::Write as _;
                if let Err(e) = std::io::stdout().write_all(docs::MARKDOWN_SOURCE.as_bytes()) {
                    eprintln!("Erreur : {e}");
                    return 1;
                }
                return 0;
            }
            if let Some(path) = write {
                match docs::write_to(&path, VERSION) {
                    Ok(()) => {
                        eprintln!("Écrit : {}", path.display());
                        0
                    }
                    Err(e) => {
                        eprintln!("Erreur : écriture impossible : {e}");
                        1
                    }
                }
            } else {
                match docs::open_default(VERSION) {
                    Ok(path) => {
                        eprintln!("Ouvert : {}", path.display());
                        0
                    }
                    Err(e) => {
                        eprintln!(
                            "Erreur : impossible d'ouvrir le navigateur : {e}\n\
                             Correction : essaie `codev docs --print` ou \
                             `codev docs --write <PATH>`."
                        );
                        1
                    }
                }
            }
        }

        Command::Completions { shell } => {
            // Sortie du script sur stdout ; aucun état projet n'est lu.
            // Cas d'exception au contrat JSON global : ce n'est pas du
            // JSON qui sort ici, mais un script shell.
            use clap::CommandFactory;
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "codev", &mut std::io::stdout());
            0
        }

        Command::Init { path, force, json } => {
            let path = path.unwrap_or_else(|| ".".to_string());
            match commands::init(&ctx, &path, force) {
                Ok(outcome) => {
                    emit(json, setup_v1(&outcome), || render::setup(&outcome, true));
                    if !json {
                        render::warnings(&outcome.warnings);
                    }
                    0
                }
                Err(err) => fail(json, setup_shape(), &err),
            }
        }

        Command::Update { force, json } => match commands::update(&ctx, force) {
            Ok(outcome) => {
                emit(json, setup_v1(&outcome), || render::setup(&outcome, false));
                if !json {
                    render::warnings(&outcome.warnings);
                }
                0
            }
            Err(err) => fail(json, setup_shape(), &err),
        },

        Command::New {
            what:
                NewCommand::Change {
                    name,
                    schema,
                    goal,
                    json,
                },
        } => match commands::new_change(&ctx, &name, schema.as_deref(), goal) {
            Ok(outcome) => {
                emit(
                    json,
                    NewChangeV1 {
                        change_name: outcome.change.to_string(),
                        schema_name: outcome.schema_name.clone(),
                        change_root: outcome.change_root.display().to_string(),
                        created: paths(&outcome.created),
                        status: contract::statuses(&outcome.warnings),
                    },
                    || render::new_change(&outcome),
                );
                if !json {
                    render::warnings(&outcome.warnings);
                }
                0
            }
            Err(err) => fail(
                json,
                json!({
                    "changeName": null,
                    "schemaName": null,
                    "changeRoot": null,
                    "created": [],
                }),
                &err,
            ),
        },

        Command::List { specs, json } => {
            if specs {
                match commands::list_specs(&ctx) {
                    Ok(outcome) => {
                        emit(
                            json,
                            SpecsV1 {
                                specs: outcome.specs.clone(),
                                root: Some(outcome.root.display().to_string()),
                                status: Vec::new(),
                            },
                            || render::specs(&outcome),
                        );
                        0
                    }
                    Err(err) => fail(json, json!({ "specs": [], "root": null }), &err),
                }
            } else {
                match commands::list_changes(&ctx) {
                    Ok(outcome) => {
                        emit(
                            json,
                            ChangesV1 {
                                changes: outcome
                                    .changes
                                    .iter()
                                    .map(ToString::to_string)
                                    .collect(),
                                root: Some(outcome.root.display().to_string()),
                                status: Vec::new(),
                            },
                            || render::changes(&outcome),
                        );
                        0
                    }
                    Err(err) => fail(json, json!({ "changes": [], "root": null }), &err),
                }
            }
        }

        Command::Status { change, json } => match commands::status(&ctx, change.as_deref()) {
            Ok(outcome) => {
                emit(
                    json,
                    StatusV1::new(
                        &outcome.status,
                        outcome.planning_home.display().to_string(),
                        outcome.change_root.display().to_string(),
                        &outcome.warnings,
                    ),
                    || render::status(&outcome.status),
                );
                if !json {
                    render::warnings(&outcome.warnings);
                }
                0
            }
            Err(err) => fail(json, status_shape(), &err),
        },

        Command::Instructions {
            artifact,
            change,
            json,
        } => match commands::artifact_instructions(&ctx, artifact.as_deref(), change.as_deref()) {
            Ok(instructions) => {
                emit(json, InstructionsV1::from(&instructions), || {
                    render::instructions(&instructions)
                });
                if !json {
                    render::warnings(&instructions.warnings);
                }
                0
            }
            Err(err) => fail(json, instructions_shape(), &err),
        },

        Command::Sync { change, json } => match commands::sync(&ctx, change.as_deref()) {
            Ok(outcome) => {
                emit(json, SyncReportV1::from(&outcome), || render::sync(&outcome));
                0
            }
            Err(err) => fail(json, sync_shape(), &err),
        },

        Command::Archive { change, json } => match commands::archive(&ctx, change.as_deref()) {
            Ok(outcome) => {
                emit(json, ArchiveReportV1::from(&outcome), || {
                    render::archive(&outcome)
                });
                0
            }
            Err(err) => fail(json, archive_shape(), &err),
        },

        Command::Validate {
            item,
            all,
            changes,
            specs,
            strict,
            json,
        } => {
            let args = if all {
                commands::ValidateArgs::All
            } else if changes {
                commands::ValidateArgs::Changes
            } else if specs {
                commands::ValidateArgs::Specs
            } else if let Some(name) = item {
                commands::ValidateArgs::Item(name)
            } else {
                // Sans flag ni nom : on valide tout, comme `--all` — c'est le
                // cas le plus utile en pre-commit et pour le dogfooding.
                commands::ValidateArgs::All
            };

            match commands::validate(&ctx, args) {
                Ok(report) => {
                    emit(json, ValidateReportV1::from(&report), || {
                        render::validate(&report)
                    });
                    // Sans `--strict`, seul `Error` bascule l'exit code —
                    // comportement historique. Avec `--strict`, tout
                    // finding (Warning inclus) fait sortir en 1 : contrat
                    // documenté pour les callers automatisés (CI, hooks,
                    // futurs workflows MCP).
                    let has_fail = if strict {
                        report.has_errors() || report.has_warnings()
                    } else {
                        report.has_errors()
                    };
                    if has_fail {
                        1
                    } else {
                        0
                    }
                }
                Err(err) => fail(json, validate_shape(), &err),
            }
        }

        Command::Sources { what } => match what {
            SourcesCommand::List { json } => match commands::sources_list(&ctx) {
                Ok(outcome) => {
                    emit(
                        json,
                        SourcesListReportV1 {
                            root: outcome.root.display().to_string(),
                            sources: outcome.sources.iter().map(SourceStateV1::from).collect(),
                            status: Vec::new(),
                        },
                        || render::sources_list(&outcome),
                    );
                    0
                }
                Err(err) => fail(json, json!({ "root": null, "sources": [] }), &err),
            },
            SourcesCommand::Update { json } => match commands::sources_update(&ctx) {
                Ok(outcome) => {
                    emit(
                        json,
                        SourcesUpdateReportV1 {
                            root: outcome.root.display().to_string(),
                            changes: outcome.diff.iter().map(PinChangeV1::from).collect(),
                            lock_written: outcome.lock_written,
                            status: Vec::new(),
                        },
                        || render::sources_update(&outcome),
                    );
                    0
                }
                Err(err) => fail(
                    json,
                    json!({ "root": null, "changes": [], "lockWritten": false }),
                    &err,
                ),
            },
            SourcesCommand::Show { target, json } => match commands::sources_show(&ctx, &target) {
                Ok(outcome) => {
                    emit(
                        json,
                        SourceDetailV1 {
                            source: SourceStateV1::from(&outcome.source),
                            files_exposed: outcome.files_exposed.clone(),
                            root: outcome.root.display().to_string(),
                            status: Vec::new(),
                        },
                        || render::sources_show(&outcome),
                    );
                    0
                }
                Err(err) => fail(
                    json,
                    json!({ "source": null, "filesExposed": [], "root": null }),
                    &err,
                ),
            },
        },

        Command::Decision { what } => match what {
            DecisionCommand::List { json } => match commands::decision_list(&ctx) {
                Ok(outcome) => {
                    emit(
                        json,
                        DecisionListReportV1 {
                            root: outcome.root.display().to_string(),
                            decisions: outcome.decisions.iter().map(decision_v1).collect(),
                            status: Vec::new(),
                        },
                        || render::decision_list(&outcome),
                    );
                    0
                }
                Err(err) => fail(json, decision_list_shape(), &err),
            },
            DecisionCommand::Show { id, json } => match commands::decision_show(&ctx, &id) {
                Ok(outcome) => {
                    emit(
                        json,
                        DecisionShowReportV1 {
                            root: outcome.root.display().to_string(),
                            decision: Some(decision_v1(&outcome.decision)),
                            content: Some(outcome.content.clone()),
                            status: Vec::new(),
                        },
                        || render::decision_show(&outcome),
                    );
                    0
                }
                Err(err) => fail(json, decision_show_shape(), &err),
            },
            DecisionCommand::New {
                title,
                status,
                json,
            } => match commands::decision_new(&ctx, &title, &status) {
                Ok(outcome) => {
                    emit(
                        json,
                        DecisionCreatedV1 {
                            root: outcome.root.display().to_string(),
                            decision: Some(decision_v1(&outcome.decision)),
                            path: Some(outcome.path.display().to_string()),
                            body_sha256: outcome.body_sha256.clone(),
                            status: Vec::new(),
                        },
                        || render::decision_created(&outcome),
                    );
                    0
                }
                Err(err) => fail(json, decision_created_shape(), &err),
            },
            DecisionCommand::Supersede {
                old_id,
                new_title,
                json,
            } => match commands::decision_supersede(&ctx, &old_id, &new_title) {
                Ok(outcome) => {
                    emit(
                        json,
                        DecisionSupersededV1 {
                            root: outcome.root.display().to_string(),
                            new_decision: Some(decision_v1(&outcome.new_decision)),
                            new_path: Some(outcome.new_path.display().to_string()),
                            old_id: Some(outcome.old_id.clone()),
                            old_qualified_id: Some(outcome.old_qualified_id.clone()),
                            old_path: Some(outcome.old_path.display().to_string()),
                            status: Vec::new(),
                        },
                        || render::decision_superseded(&outcome),
                    );
                    0
                }
                Err(err) => fail(json, decision_superseded_shape(), &err),
            },
            DecisionCommand::Promote {
                change,
                title,
                json,
            } => match commands::decision_promote(&ctx, &change, &title) {
                Ok(outcome) => {
                    emit(
                        json,
                        DecisionPromotedV1 {
                            root: outcome.root.display().to_string(),
                            decision: Some(decision_v1(&outcome.decision)),
                            path: Some(outcome.path.display().to_string()),
                            body_sha256: Some(outcome.body_sha256.clone()),
                            source_change: Some(outcome.source_change.clone()),
                            design_path: Some(outcome.design_path.display().to_string()),
                            status: Vec::new(),
                        },
                        || render::decision_promoted(&outcome),
                    );
                    0
                }
                Err(err) => fail(json, decision_promoted_shape(), &err),
            },
            DecisionCommand::Deviate {
                target,
                new_title,
                json,
            } => match commands::decision_deviate(&ctx, &target, &new_title) {
                Ok(outcome) => {
                    emit(
                        json,
                        DecisionDeviatedV1 {
                            root: outcome.root.display().to_string(),
                            decision: Some(decision_v1(&outcome.decision)),
                            path: Some(outcome.path.display().to_string()),
                            target_qualified_id: Some(outcome.target_qualified_id.clone()),
                            body_sha256: Some(outcome.body_sha256.clone()),
                            status: Vec::new(),
                        },
                        || render::decision_deviated(&outcome),
                    );
                    0
                }
                Err(err) => fail(json, decision_deviated_shape(), &err),
            },
            DecisionCommand::Seal { id, force, json } => {
                match commands::decision_seal(&ctx, &id, force) {
                    Ok(outcome) => {
                        emit(
                            json,
                            DecisionSealedV1 {
                                root: outcome.root.display().to_string(),
                                seal: Some(SealEntryV1 {
                                    id: outcome.id.clone(),
                                    body_sha256: outcome.body_sha256.clone(),
                                    sealed_at: outcome.sealed_at.clone(),
                                }),
                                was_noop: outcome.was_noop,
                                status: Vec::new(),
                            },
                            || render::decision_sealed(&outcome),
                        );
                        0
                    }
                    Err(err) => fail(json, decision_sealed_shape(), &err),
                }
            }
        },

        Command::Schemas { json } => match commands::list_schemas(&ctx) {
            Ok(outcome) => {
                emit(
                    json,
                    SchemasV1 {
                        schemas: outcome
                            .schemas
                            .iter()
                            .map(|s| SchemaV1 {
                                name: s.name.clone(),
                                origin: s.origin,
                                flow: s.flow.clone(),
                            })
                            .collect(),
                        root: Some(outcome.root.display().to_string()),
                        status: contract::statuses(&outcome.warnings),
                    },
                    || render::schemas(&outcome),
                );
                if !json {
                    render::warnings(&outcome.warnings);
                }
                0
            }
            Err(err) => fail(json, json!({ "schemas": [], "root": null }), &err),
        },
    }
}

fn setup_v1(outcome: &commands::SetupOutcome) -> SetupV1 {
    SetupV1 {
        root: outcome.root.display().to_string(),
        created: paths(&outcome.created),
        updated: paths(&outcome.updated),
        untouched: paths(&outcome.untouched),
        preserved: paths(&outcome.preserved),
        skills: outcome.skills.clone(),
        status: contract::statuses(&outcome.warnings),
    }
}

fn paths(paths: &[std::path::PathBuf]) -> Vec<String> {
    paths.iter().map(|p| p.display().to_string()).collect()
}

fn setup_shape() -> serde_json::Value {
    json!({
        "root": null,
        "created": [],
        "updated": [],
        "untouched": [],
        "preserved": [],
        "skills": [],
    })
}

fn status_shape() -> serde_json::Value {
    json!({
        "changeName": null,
        "schemaName": null,
        "planningHome": null,
        "changeRoot": null,
        "applyRequires": [],
        "isPlanningComplete": false,
        "artifacts": [],
    })
}

fn validate_shape() -> serde_json::Value {
    json!({ "root": null, "items": [], "hasWarnings": false })
}

fn decision_v1(s: &commands::DecisionSummary) -> DecisionV1 {
    DecisionV1 {
        id: s.id.clone(),
        qualified_id: s.qualified_id.clone(),
        title: s.title.clone(),
        status: s.status.clone(),
        date: s.date.clone(),
        tags: s.tags.clone(),
        supersedes: s.supersedes.clone(),
        deviates_from: s.deviates_from.clone(),
        path: s.path.display().to_string(),
        origin: s.origin.clone(),
        in_effect: s.in_effect,
        superseded_by: s.superseded_by.clone(),
        deviated_by: s.deviated_by.clone(),
    }
}

fn decision_list_shape() -> serde_json::Value {
    json!({ "root": null, "decisions": [] })
}

fn decision_show_shape() -> serde_json::Value {
    json!({ "root": null, "decision": null, "content": null })
}

fn decision_created_shape() -> serde_json::Value {
    json!({ "root": null, "decision": null, "path": null })
}

fn decision_superseded_shape() -> serde_json::Value {
    json!({
        "root": null,
        "newDecision": null,
        "newPath": null,
        "oldId": null,
        "oldQualifiedId": null,
        "oldPath": null,
    })
}

fn decision_sealed_shape() -> serde_json::Value {
    json!({
        "root": null,
        "seal": null,
        "wasNoop": null,
    })
}

fn decision_deviated_shape() -> serde_json::Value {
    json!({
        "root": null,
        "decision": null,
        "path": null,
        "targetQualifiedId": null,
    })
}

fn decision_promoted_shape() -> serde_json::Value {
    json!({
        "root": null,
        "decision": null,
        "path": null,
        "sourceChange": null,
        "designPath": null,
    })
}

fn sync_shape() -> serde_json::Value {
    json!({
        "changeName": null,
        "root": null,
        "updated": [],
        "created": [],
        "unchanged": [],
        "deleted": [],
    })
}

fn archive_shape() -> serde_json::Value {
    json!({
        "changeName": null,
        "root": null,
        "updated": [],
        "created": [],
        "unchanged": [],
        "deleted": [],
        "movedTo": null,
    })
}

fn instructions_shape() -> serde_json::Value {
    json!({
        "changeName": null,
        "schemaName": null,
        "artifact": null,
        "description": null,
        "resolvedOutputPath": null,
        "instruction": null,
        "template": null,
        "context": [],
        "rules": [],
        "dependencies": [],
        "unlocks": [],
        "decisions": [],
        "skipped": false,
    })
}

/// Émet le résultat : un document JSON, ou du texte.
fn emit<T: Serialize>(json: bool, payload: T, human: impl FnOnce() -> String) {
    if json {
        // Un `unwrap` assumé : ces types sont les nôtres et n'ont aucun champ
        // dont la sérialisation puisse échouer.
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).expect("le contrat JSON est sérialisable")
        );
    } else {
        print!("{}", human());
    }
}

/// Émet un échec, en respectant l'invariant du contrat : en mode JSON, stdout
/// porte exactement un document, de la forme de la commande.
fn fail(json: bool, shape: serde_json::Value, err: &Failure) -> i32 {
    if json {
        let payload = contract::failure(shape, &err.code, &err.message);
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).expect("la forme d'échec est sérialisable")
        );
    } else {
        eprintln!("Erreur : {err}");
        if let Some(fix) = &err.fix {
            eprintln!("Correction : {fix}");
        }
    }
    1
}

#[cfg(test)]
mod completions_tests {
    use clap::CommandFactory;
    use clap_complete::Shell;

    use super::Cli;

    /// Chaque shell supporté doit produire une sortie non vide qui cite
    /// le binaire — traceur qu'un ajout de sous-commande ne casse pas
    /// la génération, et que le nom `codev` reste bien injecté.
    #[test]
    fn generation_pour_chaque_shell_est_non_vide_et_cite_codev() {
        let shells = [
            Shell::Bash,
            Shell::Zsh,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Elvish,
        ];
        for shell in shells {
            let mut cmd = Cli::command();
            let mut buf = Vec::new();
            clap_complete::generate(shell, &mut cmd, "codev", &mut buf);
            let script = String::from_utf8(buf).expect("le script est de l'UTF-8");
            assert!(
                script.len() > 200,
                "sortie pour {shell:?} suspicieusement courte : {} octets",
                script.len()
            );
            assert!(
                script.contains("codev"),
                "sortie pour {shell:?} ne cite pas le nom du binaire"
            );
        }
    }
}
