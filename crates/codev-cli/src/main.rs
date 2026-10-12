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
use codev_engine::{
    CurlHttp, RealExecutable, RealFileSystem, RealProcessRunner, SystemClock, SystemEnv,
};
use serde::Serialize;
use serde_json::json;

use commands::{Ctx, Failure};
use contract::{
    ArchiveReportV1, ChangesV1, DecisionAcceptedV1, DecisionCreatedV1, DecisionDeviatedV1,
    DecisionListReportV1, DecisionPromotedV1, DecisionSealedV1, DecisionShowReportV1,
    DecisionSupersededV1, DecisionV1, DoctorReportV1, InstructionsV1, NewChangeV1, PinChangeV1,
    SchemaV1, SchemasV1, SealEntryV1, SetupV1, SourceDetailV1, SourceStateV1, SourcesListReportV1,
    SourcesUpdateReportV1, SpecsV1, StatusV1, SupersededDecisionV1, SyncReportV1, ValidateReportV1,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The target triple this binary was built for, set by `build.rs`.
const TARGET: &str = env!("CODEV_TARGET");

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

/// Documentation language for `codev docs --lang`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum DocsLang {
    /// English
    En,
    /// French
    Fr,
}

impl DocsLang {
    fn to_lang(self) -> docs::Lang {
        match self {
            DocsLang::En => docs::Lang::En,
            DocsLang::Fr => docs::Lang::Fr,
        }
    }
}

/// Output format of `codev changelog --format`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum ChangelogFormatArg {
    /// `## [Unreleased]` (or the release) and `### <Category>` sections
    KeepAChangelog,
    /// The same without the version heading, for a release body
    Markdown,
    /// The JSON contract v1
    Json,
}

impl ChangelogFormatArg {
    fn to_format(self) -> commands::ChangelogFormat {
        match self {
            Self::KeepAChangelog => commands::ChangelogFormat::KeepAChangelog,
            Self::Markdown => commands::ChangelogFormat::Markdown,
            Self::Json => commands::ChangelogFormat::Json,
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
        /// Language of the prose skills write in artifacts, as an ISO 639
        /// code (`en`, `fr`, `pt-BR`…). Default: detected from the locale,
        /// otherwise `en`
        #[arg(long, value_name = "CODE", value_parser = parse_language)]
        language: Option<String>,
        /// Inherit conventions from a source, written under `inherits:`
        /// (repeatable). A git URL (`git@…`, `https://…`, `ssh://…`,
        /// `file://…`, or ending in `.git`), optionally suffixed with
        /// `@<ref>` (default `main`), or a local folder. Git sources are
        /// pinned in `_codev/codev.lock` right away
        #[arg(long, value_name = "SOURCE")]
        inherit: Vec<String>,
        /// Do not write codev's section in `CLAUDE.md`
        #[arg(long)]
        no_claude_md: bool,
        /// Do not write `.github/workflows/codev.yml`
        #[arg(long)]
        no_ci: bool,
        /// Do not write codev's section in the pull request template
        #[arg(long)]
        no_pr_template: bool,
        /// Skip all repository integration: same as `--no-claude-md --no-ci
        /// --no-pr-template`
        #[arg(long)]
        no_integration: bool,
        #[arg(long)]
        json: bool,
    },

    /// Regenerate the skills, and refresh codev's managed sections, after a
    /// codev upgrade
    Update {
        /// Rewrite skills even if they were edited by hand
        #[arg(long)]
        force: bool,
        #[arg(long)]
        json: bool,
    },

    /// Download and install the latest codev release, verified
    ///
    /// Downloads the release archive for this platform from GitHub,
    /// verifies its SHA-256 against the release's `SHA256SUMS`, replaces
    /// this binary atomically, then runs the new binary's `codev update` in
    /// the current project, if any. Never downgrades unless `--version` asks
    /// for it. A binary installed by cargo, a cargo build or a package
    /// manager is left alone: the command that upgrades it is printed
    /// instead.
    SelfUpdate {
        /// Install this version (`X.Y.Z`) instead of the latest; allows a
        /// downgrade
        #[arg(long, value_name = "X.Y.Z")]
        version: Option<String>,
        /// Only report the running and latest versions (exit code 0 whether
        /// or not an update is available)
        #[arg(long, conflicts_with = "snooze")]
        check: bool,
        /// Silence the update notice for the latest known version, for 7
        /// days
        #[arg(long, conflicts_with = "version")]
        snooze: bool,
        /// Do not ask for confirmation
        #[arg(long, short = 'y')]
        yes: bool,
        /// JSON output; implies `--yes`
        #[arg(long)]
        json: bool,
    },

    /// Check what the project's codev setup still lacks, and how to fix it
    ///
    /// Read-only and offline: configuration, skills, inherited sources, the
    /// running codev version against `codev_version` and the cached update
    /// check, specs, decisions, `CLAUDE.md`, CI and pull request template,
    /// `codev validate`, and whether `CHANGELOG.md` matches the archived
    /// changes. Each check is ok, missing or a warning, with the command or
    /// skill that fixes it.
    Doctor {
        /// Exit with code 1 when a check is missing (warnings never fail)
        #[arg(long)]
        strict: bool,
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

    /// Create, accept, inspect and supersede architecture decisions
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

    /// Generate release notes from the archived changes
    ///
    /// Lists the release-note items of the changes archived after `--since`
    /// (default: the highest `v*` tag) and up to `--until` (default: the
    /// working tree), by category, breaking items first. A change archived
    /// without a release note falls back on its What Changes bullets, with a
    /// warning. `--write` keeps the `[Unreleased]` section of `CHANGELOG.md`
    /// up to date, in a managed section; with `--release`, it becomes a
    /// dated version block.
    Changelog {
        /// Git ref the range starts after (default: the highest `v*` tag
        /// below `--until`)
        #[arg(long, value_name = "REF")]
        since: Option<String>,
        /// Git ref the range ends at (default: the working tree)
        #[arg(long, value_name = "REF")]
        until: Option<String>,
        /// Output format
        #[arg(long, value_enum, default_value_t = ChangelogFormatArg::KeepAChangelog)]
        format: ChangelogFormatArg,
        /// Title the block `## [<VERSION>] - <DATE>` instead of
        /// `## [Unreleased]`
        #[arg(long, value_name = "VERSION")]
        release: Option<String>,
        /// Release date, `YYYY-MM-DD` (default: today)
        #[arg(long, value_name = "DATE", requires = "release")]
        date: Option<String>,
        /// Update `CHANGELOG.md` (created if missing); text outside codev's
        /// managed section is kept
        #[arg(long)]
        write: bool,
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
        /// Language of the documentation
        #[arg(long, value_enum, default_value_t = DocsLang::En)]
        lang: DocsLang,
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
    ///
    /// The decision is created `proposed` and unsealed, so its body can be
    /// written freely; `codev decision accept` then accepts and seals it.
    New {
        /// Free-form title — slugified for the file name
        title: String,
        /// Initial status; `accepted` seals the body right away, `proposed`
        /// leaves it editable until `codev decision accept`
        #[arg(long, default_value = "proposed")]
        status: String,
        #[arg(long)]
        json: bool,
    },
    /// Accept a proposed decision: set its status to `accepted` and seal it
    ///
    /// Rewrites the frontmatter status of a local `proposed` decision and
    /// records the hash of its body in `seal.yaml`, in the same plan — all
    /// writes happen or none does. The body is left untouched. The decisions
    /// listed in its `supersedes` are marked `superseded` in the same plan;
    /// each must still be `accepted`. This is how a decision created by
    /// `new`, `supersede`, `deviate` or `promote` takes effect. Inherited
    /// decisions and decisions that are not `proposed` are refused.
    Accept {
        /// Short identifier (`0007`) — inherited ones (`path:` / `git:`) are refused
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Create a proposed decision that will supersede an accepted one
    ///
    /// The new decision is created `proposed` and unsealed, with the old one
    /// in its `supersedes`. The old decision is not modified and stays in
    /// effect until `codev decision accept` accepts the new one, which marks
    /// the old one `superseded` in the same step.
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
    /// Creates a local `proposed` ADR, unsealed, that explicitly references
    /// the inherited decision being departed from. The deviation takes
    /// effect once `codev decision accept` accepts it: the inherited
    /// decision then stays visible in `codev decision list`, but disappears
    /// from the instructions injected into the `design` artifact. To deviate
    /// from a local decision, use `codev decision supersede`.
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
    /// Extracts the block's content into a `proposed` local ADR, unsealed,
    /// and replaces the block's body with a textual reference to the new
    /// ADR. Review and rework the ADR, then accept and seal it with
    /// `codev decision accept`. Refuses an archived change.
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
    let runner = RealProcessRunner;
    let http = CurlHttp {
        runner: &runner,
        user_agent: format!("codev/{VERSION}"),
    };
    let exe = RealExecutable;
    let ctx = Ctx {
        fs: &fs,
        env: &env,
        clock: &clock,
        runner: &runner,
        http: &http,
        exe: &exe,
        version: VERSION,
        target: TARGET,
    };

    // A Windows update leaves the previous binary next to this one: it was
    // still running then, it is not now.
    if cfg!(windows) {
        codev_engine::update::cleanup_previous_binary(&exe);
    }

    match cli.command {
        Command::Docs { print, write, lang } => {
            let lang = lang.to_lang();
            if print {
                use std::io::Write as _;
                return match std::io::stdout().write_all(docs::markdown(lang).as_bytes()) {
                    Ok(()) => 0,
                    // `codev docs --print | head`: the reader left, which is
                    // not an error worth reporting.
                    Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => 0,
                    Err(e) => {
                        eprintln!("error: {e}");
                        1
                    }
                };
            }
            if let Some(path) = write {
                match docs::write_to(&path, lang, VERSION) {
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
                match docs::open_default(lang, VERSION) {
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
            language,
            inherit,
            no_claude_md,
            no_ci,
            no_pr_template,
            no_integration,
            json,
        } => {
            let path = path.unwrap_or_else(|| ".".to_string());
            let init_opts = init_prompts::InitOptions {
                yes,
                no_detect,
                preset: preset.map(PresetArg::to_preset),
                language,
                inherits: inherit,
                no_claude_md: no_claude_md || no_integration,
                no_ci: no_ci || no_integration,
                no_pr_template: no_pr_template || no_integration,
            };
            match commands::init(&ctx, &path, force, &init_opts) {
                Ok(outcome) => {
                    emit(json, setup_v1(&outcome), || render::setup(&outcome, true));
                    if !json {
                        render::warnings(&outcome.warnings);
                        if outcome
                            .warnings
                            .iter()
                            .any(|w| w.code == "inherit_pin_failed")
                        {
                            eprintln!("help: run codev sources update");
                        }
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
                emit(json, NewChangeV1::from(&outcome), || {
                    render::new_change(&outcome)
                });
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
                                root: Some(contract::json_path(&outcome.root)),
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
                                root: Some(contract::json_path(&outcome.root)),
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
                        &outcome.planning_home,
                        &outcome.change_root,
                        &outcome.warnings,
                    )
                    .with_update(outcome.update.as_ref(), outcome.requirement.as_ref()),
                    || render::status(&outcome.status),
                );
                if !json {
                    render::warnings(&outcome.warnings);
                    update_line(outcome.update.as_ref());
                    doctor_hint(&ctx);
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
                if !json && err.code == "no_active_change" {
                    update_line(commands::update_notice(&ctx).as_ref());
                    doctor_hint(&ctx);
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
            Err(err) => fail(
                json,
                instructions_shape(&commands::configured_language(&ctx)),
                &err,
            ),
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

        Command::Changelog {
            since,
            until,
            format,
            release,
            date,
            write,
        } => {
            let json = format == ChangelogFormatArg::Json;
            let args = commands::ChangelogArgs {
                since,
                until,
                release,
                date,
                write,
            };
            match commands::changelog(&ctx, format.to_format(), &args) {
                Ok(outcome) => {
                    emit(json, contract::ChangelogV1::from(&outcome), || {
                        render::changelog_written(&outcome)
                    });
                    if !json {
                        render::warnings(&outcome.warnings);
                    }
                    0
                }
                Err(err) => fail(json, contract::changelog_shape(), &err),
            }
        }

        Command::SelfUpdate {
            version,
            check,
            snooze,
            yes,
            json,
        } => {
            let args = commands::SelfUpdateArgs {
                version,
                check,
                snooze,
            };
            let interactive = !yes && !json && std::io::IsTerminal::is_terminal(&std::io::stdin());
            let mut confirm = |current: &str, target: &str| {
                !interactive
                    || dialoguer::Confirm::new()
                        .with_prompt(format!("Update codev {current} to {target}?"))
                        .default(true)
                        .interact()
                        .unwrap_or(false)
            };
            match commands::self_update(&ctx, &args, &mut confirm) {
                Ok(outcome) => {
                    emit(json, contract::SelfUpdateV1::from(&outcome), || {
                        render::self_update(&outcome)
                    });
                    if !json {
                        render::warnings(&outcome.warnings);
                    }
                    0
                }
                Err(err) => fail(json, contract::self_update_shape(VERSION), &err),
            }
        }

        Command::Doctor { strict, json } => match commands::doctor(&ctx) {
            Ok(outcome) => {
                emit(json, DoctorReportV1::from(&outcome), || {
                    render::doctor(&outcome)
                });
                if strict && outcome.missing() > 0 {
                    1
                } else {
                    0
                }
            }
            Err(err) => fail(json, doctor_shape(), &err),
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
                            root: contract::json_path(&outcome.root),
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
                            root: contract::json_path(&outcome.root),
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
                            root: contract::json_path(&outcome.root),
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
                            root: contract::json_path(&outcome.root),
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
                            root: contract::json_path(&outcome.root),
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
                            root: contract::json_path(&outcome.root),
                            decision: Some(decision_v1(&outcome.decision)),
                            path: Some(contract::json_path(&outcome.path)),
                            body_sha256: outcome.body_sha256.clone(),
                            status: Vec::new(),
                        },
                        || render::decision_created(&outcome),
                    );
                    0
                }
                Err(err) => fail(json, decision_created_shape(), &err),
            },
            DecisionCommand::Accept { id, json } => match commands::decision_accept(&ctx, &id) {
                Ok(outcome) => {
                    emit(
                        json,
                        DecisionAcceptedV1 {
                            root: contract::json_path(&outcome.root),
                            decision: Some(decision_v1(&outcome.decision)),
                            path: Some(contract::json_path(&outcome.path)),
                            body_sha256: Some(outcome.body_sha256.clone()),
                            superseded: outcome
                                .superseded
                                .iter()
                                .map(|p| SupersededDecisionV1 {
                                    id: p.id.clone(),
                                    qualified_id: p.qualified_id.clone(),
                                    path: contract::json_path(&p.path),
                                })
                                .collect(),
                            status: Vec::new(),
                        },
                        || render::decision_accepted(&outcome),
                    );
                    0
                }
                Err(err) => fail(json, decision_accepted_shape(), &err),
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
                            root: contract::json_path(&outcome.root),
                            new_decision: Some(decision_v1(&outcome.new_decision)),
                            new_path: Some(contract::json_path(&outcome.new_path)),
                            old_id: Some(outcome.old_id.clone()),
                            old_qualified_id: Some(outcome.old_qualified_id.clone()),
                            old_path: Some(contract::json_path(&outcome.old_path)),
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
                            root: contract::json_path(&outcome.root),
                            decision: Some(decision_v1(&outcome.decision)),
                            path: Some(contract::json_path(&outcome.path)),
                            source_change: Some(outcome.source_change.clone()),
                            design_path: Some(contract::json_path(&outcome.design_path)),
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
                            root: contract::json_path(&outcome.root),
                            decision: Some(decision_v1(&outcome.decision)),
                            path: Some(contract::json_path(&outcome.path)),
                            target_qualified_id: Some(outcome.target_qualified_id.clone()),
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
                                root: contract::json_path(&outcome.root),
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
                        root: Some(contract::json_path(&outcome.root)),
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
        root: contract::json_path(&outcome.root),
        created: contract::json_paths(&outcome.created),
        updated: contract::json_paths(&outcome.updated),
        untouched: contract::json_paths(&outcome.untouched),
        preserved: contract::json_paths(&outcome.preserved),
        skills: outcome.skills.clone(),
        integration: outcome
            .integration
            .iter()
            .map(contract::IntegrationStepV1::from)
            .collect(),
        status: contract::statuses(&outcome.warnings),
    }
}

fn setup_shape() -> serde_json::Value {
    json!({
        "root": null,
        "created": [],
        "updated": [],
        "untouched": [],
        "preserved": [],
        "skills": [],
        "integration": [],
    })
}

fn doctor_shape() -> serde_json::Value {
    json!({
        "root": null,
        "checks": [],
        "summary": { "total": 0, "missing": 0, "warnings": 0 },
    })
}

/// One line on stderr when a newer codev is available and not snoozed.
fn update_line(notice: Option<&codev_core::update::UpdateNotice>) {
    if let Some(notice) = notice
        && notice.should_announce()
    {
        eprintln!("{}", notice.line());
    }
}

/// One line on stderr when `codev doctor` would report a missing check.
/// Silent when doctor cannot run: a hint never breaks a command.
fn doctor_hint(ctx: &commands::Ctx) {
    if let Some(missing) = commands::doctor_missing_from_cwd(ctx)
        && missing > 0
    {
        eprintln!("hint: {missing} codev setup check(s) to fix — run `codev doctor`");
    }
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
        path: contract::json_path(&s.path),
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

fn decision_accepted_shape() -> serde_json::Value {
    json!({ "root": null, "decision": null, "path": null, "superseded": [] })
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

fn instructions_shape(language: &str) -> serde_json::Value {
    json!({
        "changeName": null,
        "schemaName": null,
        "artifact": null,
        "description": null,
        "resolvedOutputPath": null,
        "instruction": null,
        "template": null,
        "language": language,
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

/// Validates `--language` up front, with the same rule `config.yaml` applies.
fn parse_language(raw: &str) -> Result<String, String> {
    if codev_core::detect::locale::is_valid_language_code(raw) {
        Ok(raw.to_string())
    } else {
        Err(format!(
            "`{raw}` is not a language code; use an ISO 639 code such as `en`, `fr` or `pt-BR`"
        ))
    }
}

#[cfg(test)]
mod parsing_tests {
    use clap::Parser;

    use super::{Cli, Command, DecisionCommand};

    #[test]
    fn decision_new_status_defaults_to_proposed() {
        let cli = Cli::try_parse_from(["codev", "decision", "new", "A choice"]).unwrap();
        let Command::Decision {
            what: DecisionCommand::New { status, .. },
        } = cli.command
        else {
            panic!("expected `decision new`");
        };
        assert_eq!(status, "proposed");
    }

    #[test]
    fn decision_accept_takes_an_id_and_json() {
        let cli = Cli::try_parse_from(["codev", "decision", "accept", "0007", "--json"]).unwrap();
        let Command::Decision {
            what: DecisionCommand::Accept { id, json },
        } = cli.command
        else {
            panic!("expected `decision accept`");
        };
        assert_eq!(id, "0007");
        assert!(json);
    }

    #[test]
    fn self_update_flags_parse() {
        let cli = Cli::try_parse_from([
            "codev",
            "self-update",
            "--version",
            "0.5.0",
            "--yes",
            "--json",
        ])
        .unwrap();
        let Command::SelfUpdate {
            version,
            check,
            snooze,
            yes,
            json,
        } = cli.command
        else {
            panic!("expected `self-update`");
        };
        assert_eq!(version.as_deref(), Some("0.5.0"));
        assert!(yes && json && !check && !snooze);
        // `--check` and `--snooze` exclude each other, as do `--snooze` and
        // `--version`.
        assert!(Cli::try_parse_from(["codev", "self-update", "--check", "--snooze"]).is_err());
        assert!(
            Cli::try_parse_from(["codev", "self-update", "--snooze", "--version", "1.0.0"])
                .is_err()
        );
    }
}

#[cfg(test)]
mod doctor_tests {
    #[test]
    fn doctor_failure_keeps_the_shape() {
        let json = crate::contract::failure(super::doctor_shape(), "no_codev_root", "no project");
        assert_eq!(json["checks"], serde_json::json!([]));
        assert!(json["root"].is_null());
        assert_eq!(json["summary"]["missing"], 0);
        assert_eq!(json["status"][0]["code"], "no_codev_root");
        assert_eq!(json["status"][0]["level"], "error");
    }

    #[test]
    fn status_failure_keeps_the_shape() {
        let json = crate::contract::failure(
            super::status_shape(),
            "ambiguous_change",
            "several active changes: add-auth, fix-bug — specify which one",
        );
        assert!(json["changeName"].is_null() && json["changeRoot"].is_null());
        assert!(json["planningHome"].is_null() && json["schemaName"].is_null());
        assert_eq!(json["artifacts"], serde_json::json!([]));
        assert_eq!(json["applyRequires"], serde_json::json!([]));
        assert_eq!(json["isPlanningComplete"], false);
        assert_eq!(json["status"][0]["code"], "ambiguous_change");
    }

    #[test]
    fn instructions_failure_keeps_the_shape() {
        let json = crate::contract::failure(
            super::instructions_shape("fr"),
            "unknown_artifact",
            "unknown artifact `sketch` in schema `spec-driven`",
        );
        for field in [
            "changeName",
            "schemaName",
            "artifact",
            "description",
            "resolvedOutputPath",
            "instruction",
            "template",
        ] {
            assert!(json[field].is_null(), "`{field}` should be null: {json}");
        }
        for field in ["context", "rules", "dependencies", "unlocks", "decisions"] {
            assert_eq!(json[field], serde_json::json!([]), "`{field}`: {json}");
        }
        assert_eq!(json["skipped"], false);
        // `language` is a field of the success document: the failure keeps
        // it, with the language it was given.
        assert_eq!(json["language"], "fr");
        assert_eq!(json["status"][0]["code"], "unknown_artifact");
    }

    #[test]
    fn setup_paths_use_slashes_on_windows() {
        // Built with `\` explicitly, so the conversion is proven on every
        // platform.
        let path = |p: &str| std::path::PathBuf::from(p);
        let outcome = crate::commands::SetupOutcome {
            root: path("C:\\work\\acme"),
            created: vec![path("C:\\work\\acme\\_codev\\config.yaml")],
            updated: vec![path(
                "C:\\work\\acme\\.claude\\skills\\codev-apply\\SKILL.md",
            )],
            untouched: vec![path("C:\\work\\acme\\_codev\\specs")],
            preserved: vec![path(
                "C:\\work\\acme\\.claude\\skills\\codev-sync\\SKILL.md",
            )],
            skills: Vec::new(),
            warnings: Vec::new(),
            config_thin: false,
            integration: Vec::new(),
            pins: Vec::new(),
            doctor_missing: None,
        };
        let json = serde_json::to_value(super::setup_v1(&outcome)).unwrap();
        assert_eq!(json["root"], "C:/work/acme");
        assert_eq!(json["created"][0], "C:/work/acme/_codev/config.yaml");
        assert_eq!(
            json["updated"][0],
            "C:/work/acme/.claude/skills/codev-apply/SKILL.md"
        );
        assert_eq!(json["untouched"][0], "C:/work/acme/_codev/specs");
        assert_eq!(
            json["preserved"][0],
            "C:/work/acme/.claude/skills/codev-sync/SKILL.md"
        );
    }

    #[test]
    fn setup_failure_keeps_the_shape() {
        let json = crate::contract::failure(
            super::setup_shape(),
            "no_codev_root",
            "no codev project found from /elsewhere",
        );
        assert!(json["root"].is_null());
        for field in [
            "created",
            "updated",
            "untouched",
            "preserved",
            "skills",
            "integration",
        ] {
            assert_eq!(json[field], serde_json::json!([]), "`{field}`: {json}");
        }
        assert_eq!(json["status"][0]["code"], "no_codev_root");
    }
}

#[cfg(test)]
mod changelog_tests {
    use clap::Parser;

    use super::{ChangelogFormatArg, Cli, Command};

    #[test]
    fn changelog_json_failure_keeps_the_shape() {
        let json = crate::contract::failure(
            crate::contract::changelog_shape(),
            "unknown_ref",
            "git does not know `v9`",
        );
        assert_eq!(json["entries"], serde_json::json!([]));
        assert!(json["root"].is_null() && json["written"].is_null());
        assert_eq!(json["markdown"], "");
        assert_eq!(json["status"][0]["code"], "unknown_ref");
    }

    #[test]
    fn changelog_flags_parse_and_date_requires_release() {
        let cli = Cli::try_parse_from([
            "codev",
            "changelog",
            "--since",
            "v1.0.0",
            "--until",
            "v1.1.0",
            "--format",
            "markdown",
        ])
        .unwrap();
        let Command::Changelog { since, format, .. } = cli.command else {
            panic!("expected the changelog command");
        };
        assert_eq!(since.as_deref(), Some("v1.0.0"));
        assert_eq!(format, ChangelogFormatArg::Markdown);
        assert!(Cli::try_parse_from(["codev", "changelog", "--date", "2026-10-20"]).is_err());
    }
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
