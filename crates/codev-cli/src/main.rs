//! The `codev` binary: the imperative shell.
//!
//! No decisions here — argument parsing, calling a command, and rendering.
//! Two outputs for the same result: text for a human, a JSON document for a
//! skill.

mod commands;
mod contract;
mod docs;
mod init_prompts;
mod render;

use clap::{Parser, Subcommand, ValueEnum};
use codev_engine::{RealFileSystem, SystemClock, SystemEnv};
use serde::Serialize;
use serde_json::json;

use commands::{Ctx, Failure};
use contract::{
    ArchiveReportV1, ChangesV1, DecisionCreatedV1, DecisionDeviatedV1, DecisionListReportV1,
    DecisionPromotedV1, DecisionSealedV1, DecisionShowReportV1, DecisionSupersededV1, DecisionV1,
    InstructionsV1, NewChangeV1, PinChangeV1, SchemaV1, SchemasV1, SealEntryV1, SetupV1,
    SourceDetailV1, SourceStateV1, SourcesListReportV1, SourcesUpdateReportV1, SpecsV1, StatusV1,
    SyncReportV1, ValidateReportV1,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(
    name = "codev",
    version,
    about = "Spec-driven development for Claude Code",
    long_about = "codev adds a thin layer of specs to a repository so that you and your agent \
                  agree on what must be built before a single line of code is written.\n\n\
                  The commands below run in your terminal. The workflows are invoked in the \
                  Claude Code chat: /codev-propose, /codev-explore."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Workflow preset for `codev init --preset`.
#[derive(Clone, Copy, Debug, ValueEnum)]
enum PresetArg {
    /// All 8 workflows
    Full,
    /// propose, explore, onboard, configure
    Minimal,
    /// Pick the workflows one by one
    Custom,
}

impl PresetArg {
    fn to_preset(self) -> init_prompts::Preset {
        match self {
            PresetArg::Full => init_prompts::Preset::Full,
            PresetArg::Minimal => init_prompts::Preset::Minimal,
            PresetArg::Custom => init_prompts::Preset::Custom,
        }
    }
}

#[derive(Subcommand)]
enum Command {
    /// Initialize codev in a project and install the Claude Code skills
    Init {
        /// Project folder (default: the current folder)
        path: Option<String>,
        /// Rewrite skills even if they were edited by hand
        #[arg(long)]
        force: bool,
        /// Accept all defaults without prompting (defaults + detected values)
        #[arg(long, short = 'y')]
        yes: bool,
        /// Disable the environment probe (useful for reproducible tests)
        #[arg(long)]
        no_detect: bool,
        /// Preselect the answer to the workflows question
        #[arg(long, value_enum)]
        preset: Option<PresetArg>,
        #[arg(long)]
        json: bool,
    },

    /// Regenerate the skills after a codev upgrade
    Update {
        /// Rewrite skills even if they were edited by hand
        #[arg(long)]
        force: bool,
        #[arg(long)]
        json: bool,
    },

    /// Create a new item
    New {
        #[command(subcommand)]
        what: NewCommand,
    },

    /// List active changes, or specified capabilities with --specs
    List {
        #[arg(long)]
        specs: bool,
        #[arg(long)]
        json: bool,
    },

    /// Show the state of a change's artifacts
    Status {
        /// Change name (inferred when there is only one)
        #[arg(long)]
        change: Option<String>,
        #[arg(long)]
        json: bool,
    },

    /// Print everything needed to write an artifact
    Instructions {
        /// Artifact identifier (default: the next one to write)
        artifact: Option<String>,
        #[arg(long)]
        change: Option<String>,
        #[arg(long)]
        json: bool,
    },

    /// List the available workflow schemas
    Schemas {
        #[arg(long)]
        json: bool,
    },

    /// Create, inspect and supersede architecture decisions
    Decision {
        #[command(subcommand)]
        what: DecisionCommand,
    },

    /// Manage inherited sources (local paths and remote git repositories)
    Sources {
        #[command(subcommand)]
        what: SourcesCommand,
    },

    /// Merge a change's deltas into the main specs without archiving
    Sync {
        /// Change name (inferred when there is only one)
        #[arg(long)]
        change: Option<String>,
        #[arg(long)]
        json: bool,
    },

    /// Merge, then move a change to the dated archive
    Archive {
        /// Change name (inferred when there is only one)
        #[arg(long)]
        change: Option<String>,
        #[arg(long)]
        json: bool,
    },

    /// Open the codev documentation in the browser
    ///
    /// Three forms:
    ///
    /// - default: writes `codev-docs-<version>.html` to the system temp
    ///   folder and opens it in the browser;
    /// - `--write <PATH>`: writes the HTML to the given path, opens
    ///   nothing (for sharing — email, Confluence, shared drive);
    /// - `--print`: prints the markdown source to stdout (to pipe into
    ///   `less`, `bat` or an LLM).
    Docs {
        /// Print the markdown source to stdout (does not open anything)
        #[arg(long, conflicts_with = "write")]
        print: bool,
        /// Write the HTML to the given path (does not open anything)
        #[arg(long, value_name = "PATH")]
        write: Option<std::path::PathBuf>,
    },

    /// Generate a shell completion script for local installation
    ///
    /// The output goes to stdout — redirect it to your shell's completions
    /// folder. Typical setups:
    ///
    /// - bash: `codev completions bash > ~/.local/share/bash-completion/completions/codev`
    /// - zsh: `codev completions zsh > "${fpath[1]}/_codev"`, then `compinit`
    /// - fish: `codev completions fish > ~/.config/fish/completions/codev.fish`
    /// - powershell: `codev completions powershell | Out-String | Invoke-Expression`
    ///
    /// The command touches no file; it does not read `_codev/` either and
    /// works in any directory.
    Completions {
        /// Target shell: bash, zsh, fish, powershell, elvish
        shell: clap_complete::Shell,
    },

    /// Check changes and specs for structural errors and consistency
    Validate {
        /// Name of a specific change or spec capability
        item: Option<String>,
        /// Validate all active changes and all main specs
        #[arg(long, conflicts_with_all = ["changes", "specs", "item"])]
        all: bool,
        /// Validate all active changes
        #[arg(long, conflicts_with_all = ["all", "specs", "item"])]
        changes: bool,
        /// Validate all main specs
        #[arg(long, conflicts_with_all = ["all", "changes", "item"])]
        specs: bool,
        /// Treat any finding (warnings included) as a reason for a non-zero
        /// exit code. Useful for CI and automation.
        #[arg(long)]
        strict: bool,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum SourcesCommand {
    /// List all declared sources with their state
    List {
        #[arg(long)]
        json: bool,
    },
    /// Resolve refs, download, and update `_codev/codev.lock`
    Update {
        #[arg(long)]
        json: bool,
    },
    /// Show the details of a specific source
    Show {
        /// URL (`git:` source) or path (`path:` source)
        target: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum DecisionCommand {
    /// List local and inherited decisions
    List {
        #[arg(long)]
        json: bool,
    },
    /// Show a specific decision
    Show {
        /// Short (`0007`) or qualified (`path:~/shared/0100`) identifier
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Create a new local decision
    New {
        /// Free-form title — slugified for the file name
        title: String,
        /// Initial status
        #[arg(long, default_value = "accepted")]
        status: String,
        #[arg(long)]
        json: bool,
    },
    /// Supersede a decision: mark it `superseded` and create a new one
    Supersede {
        /// Identifier of the decision to supersede (short or qualified)
        old_id: String,
        /// Title of the new decision
        new_title: String,
        #[arg(long)]
        json: bool,
    },
    /// Add or rewrite the seal of a local decision
    ///
    /// Without `--force`: refuses if a seal exists and the body has changed.
    /// With `--force`: rewrites the seal (use after a deliberate edit of the
    /// body).
    Seal {
        /// Short identifier (`0007`) — inherited ones (`path:` / `git:`) are refused
        id: String,
        /// Rewrite an existing seal even if the body has changed
        #[arg(long)]
        force: bool,
        #[arg(long)]
        json: bool,
    },
    /// Record a local deviation from an inherited decision
    ///
    /// Creates a local `accepted` ADR that explicitly references the
    /// inherited decision being departed from. The inherited decision stays
    /// visible in `codev decision list`, but disappears from the
    /// instructions injected into the `design` artifact. To deviate from a
    /// local decision, use `codev decision supersede`.
    Deviate {
        /// Qualified identifier of the inherited decision to set aside
        ///
        /// Example: `path:~/shared/0100` or
        /// `git:git@github.com:acme/shared.git/0100`.
        target: String,
        /// Free-form title of the local deviation — slugified
        new_title: String,
        #[arg(long)]
        json: bool,
    },
    /// Promote a `### Decision: <title>` block of a `design.md` to an ADR
    ///
    /// Extracts the block's content, creates a sealed local ADR, and
    /// replaces the block's body with a textual reference to the new ADR.
    /// Refuses an archived change.
    Promote {
        /// Name of the active change whose `design.md` holds the block
        change: String,
        /// Exact title of the block to promote (what follows `Decision: `)
        title: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum NewCommand {
    /// Create a change
    Change {
        /// Name in kebab-case (`add-user-auth`)
        name: String,
        /// Workflow schema to use
        #[arg(long)]
        schema: Option<String>,
        /// Goal, kept in the change's metadata
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
                    eprintln!("error: {e}");
                    return 1;
                }
                return 0;
            }
            if let Some(path) = write {
                match docs::write_to(&path, VERSION) {
                    Ok(()) => {
                        eprintln!("Wrote {}", path.display());
                        0
                    }
                    Err(e) => {
                        eprintln!("error: cannot write the file: {e}");
                        1
                    }
                }
            } else {
                match docs::open_default(VERSION) {
                    Ok(path) => {
                        eprintln!("Opened {}", path.display());
                        0
                    }
                    Err(e) => {
                        eprintln!(
                            "error: cannot open the browser: {e}\n\
                             help: try `codev docs --print` or \
                             `codev docs --write <PATH>`"
                        );
                        1
                    }
                }
            }
        }

        Command::Completions { shell } => {
            // Script output on stdout; no project state is read.
            // An exception to the global JSON contract: what comes out here
            // is not JSON, but a shell script.
            use clap::CommandFactory;
            let mut cmd = Cli::command();
            clap_complete::generate(shell, &mut cmd, "codev", &mut std::io::stdout());
            0
        }

        Command::Init {
            path,
            force,
            yes,
            no_detect,
            preset,
            json,
        } => {
            let path = path.unwrap_or_else(|| ".".to_string());
            let init_opts = init_prompts::InitOptions {
                yes,
                no_detect,
                preset: preset.map(PresetArg::to_preset),
            };
            match commands::init(&ctx, &path, force, &init_opts) {
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
                                changes: outcome.changes.iter().map(ToString::to_string).collect(),
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
            Err(err) => {
                let exit = fail(json, status_shape(), &err);
                // Conditional hint: on `no_active_change` in human output,
                // if the config is thin, suggest `/codev-configure`. The JSON
                // stays strictly unchanged.
                if !json && err.code == "no_active_change" && config_is_thin(&ctx) {
                    eprintln!("hint: the config is sparse — /codev-configure can enrich it");
                }
                exit
            }
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
                emit(json, SyncReportV1::from(&outcome), || {
                    render::sync(&outcome)
                });
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
                // No flag and no name: validate everything, like `--all` —
                // the most useful case for pre-commit hooks and dogfooding.
                commands::ValidateArgs::All
            };

            match commands::validate(&ctx, args) {
                Ok(report) => {
                    emit(json, ValidateReportV1::from(&report), || {
                        render::validate(&report)
                    });
                    // Without `--strict`, only `Error` flips the exit code —
                    // the historical behavior. With `--strict`, any finding
                    // (warnings included) exits with 1: a documented
                    // contract for automated callers (CI, hooks, future MCP
                    // workflows).
                    let has_fail = if strict {
                        report.has_errors() || report.has_warnings()
                    } else {
                        report.has_errors()
                    };
                    if has_fail { 1 } else { 0 }
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

/// Emits the result: a JSON document, or text.
fn emit<T: Serialize>(json: bool, payload: T, human: impl FnOnce() -> String) {
    if json {
        // A deliberate `unwrap`: these types are ours and have no field
        // whose serialization can fail.
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).expect("the JSON contract is serializable")
        );
    } else {
        print!("{}", human());
    }
}

/// Silently looks at the current project's `_codev/config.yaml` and tells
/// whether it is thin (no `rules:` entry). Returns `false` if the project is
/// not initialized or if the config is unreadable — the hint is only shown
/// when we are sure to be facing a real, under-configured codev project.
fn config_is_thin(ctx: &commands::Ctx) -> bool {
    let Ok(layout) = codev_engine::root::discover_from_cwd(ctx.fs, ctx.env) else {
        return false;
    };
    let Ok(cfg) = codev_engine::config::resolve(ctx.fs, ctx.env, &layout) else {
        return false;
    };
    codev_core::config::is_config_thin(cfg.rules.is_empty())
}

/// Emits a failure, honoring the contract's invariant: in JSON mode, stdout
/// carries exactly one document, in the shape of the command.
fn fail(json: bool, shape: serde_json::Value, err: &Failure) -> i32 {
    if json {
        let payload = contract::failure(shape, &err.code, &err.message);
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).expect("the failure shape is serializable")
        );
    } else {
        eprintln!("error: {err}");
        if let Some(fix) = &err.fix {
            eprintln!("help: {fix}");
        }
    }
    1
}

#[cfg(test)]
mod completions_tests {
    use clap::CommandFactory;
    use clap_complete::Shell;

    use super::Cli;

    /// Every supported shell must produce non-empty output that names the
    /// binary — a tracer that adding a subcommand does not break
    /// generation, and that the `codev` name is still injected.
    #[test]
    fn generation_for_each_shell_is_non_empty_and_names_codev() {
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
            let script = String::from_utf8(buf).expect("the script is UTF-8");
            assert!(
                script.len() > 200,
                "output for {shell:?} is suspiciously short: {} bytes",
                script.len()
            );
            assert!(
                script.contains("codev"),
                "output for {shell:?} does not name the binary"
            );
        }
    }
}
