//! `codev-evals`: list, run and score the role evaluation suite.
//!
//! Exit codes: 0 every role passes, 1 a role fails or a case stopped,
//! 2 usage (invalid case, threshold or observation, missing tool).

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::{Args, Parser, Subcommand};

use codev_evals::observed::{self, FORMAT, RunInfo};
use codev_evals::reliability::{self, Baseline, Lang};
use codev_evals::runner::{self, ClaudeSettings, RunOptions, Tools};
use codev_evals::score::{self, SuiteScore, ThresholdFlags, Thresholds};
use codev_evals::{case, compare, report, suite};

#[derive(Parser)]
#[command(
    name = "codev-evals",
    about = "Measure what codev's reviewing roles find, miss and invent"
)]
struct Cli {
    /// The codev repository (default: the one this binary was built from)
    #[arg(long, global = true)]
    repo: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List the cases of the suite
    List,
    /// Run cases with Claude Code, then score them
    Run(RunArgs),
    /// Score the observations of an output directory again, without a model
    Score(ScoreArgs),
    /// Write the documentation's reliability chapter, in English and
    /// French, from a baseline's report
    Docs(DocsArgs),
    /// Compare a candidate run with a baseline: quality within the
    /// baseline's range, cost under its floor
    Compare(CompareArgs),
}

#[derive(Args)]
struct CompareArgs {
    /// The baseline's output directory
    baseline: PathBuf,
    /// The candidate's output directory
    candidate: PathBuf,
    /// Leave the consumption out of the verdict (a role change), still
    /// reporting it
    #[arg(long)]
    quality_only: bool,
}

#[derive(Args)]
struct DocsArgs {
    /// The baseline to read `report.json` and `run.json` from
    /// (default: evals/baseline of the repository)
    #[arg(long)]
    baseline: Option<PathBuf>,
}

#[derive(Args)]
struct ThresholdArgs {
    /// Minimum recall per role, 0 to 1 [env: CODEV_EVAL_MIN_RECALL] [default: 0.5]
    #[arg(long)]
    min_recall: Option<String>,
    /// Maximum false positives per role [env: CODEV_EVAL_MAX_FALSE_POSITIVES] [default: 2]
    #[arg(long)]
    max_false_positives: Option<String>,
    /// Minimum evidence validity per role, 0 to 1 [env: CODEV_EVAL_MIN_EVIDENCE] [default: 0.8]
    #[arg(long)]
    min_evidence: Option<String>,
}

#[derive(Args)]
struct RunArgs {
    /// Run only this case (repeatable)
    #[arg(long = "case")]
    cases: Vec<String>,
    /// Run only the cases of this role (repeatable)
    #[arg(long = "role")]
    roles: Vec<String>,
    /// Run only the cases of this step (repeatable): `review` for the three
    /// review steps, or `plan.challenge`, `code.review`, `plan.trace`,
    /// `plan.write`
    #[arg(long = "step")]
    steps: Vec<String>,
    /// Output directory (default: target/evals/run-<timestamp>); a directory
    /// that already holds observations is resumed
    #[arg(long)]
    out: Option<PathBuf>,
    /// Where fixtures are set up (default: the system's temporary directory)
    #[arg(long)]
    work: Option<PathBuf>,
    /// Keep the fixture directories after the run
    #[arg(long)]
    keep: bool,
    /// Run again the cases already observed in the output directory
    #[arg(long)]
    rerun: bool,
    /// Run each case this many times, each repetition recorded apart
    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..))]
    repeat: u32,
    /// Cases run at the same time
    #[arg(long, default_value_t = 2)]
    jobs: usize,
    /// The codev binary under test (default: target/debug/codev of the repository)
    #[arg(long)]
    codev: Option<PathBuf>,
    /// The Claude Code command [env: CODEV_EVAL_CLAUDE] [default: claude]
    #[arg(long)]
    claude: Option<PathBuf>,
    #[command(flatten)]
    thresholds: ThresholdArgs,
}

#[derive(Args)]
struct ScoreArgs {
    /// The output directory of a run
    dir: PathBuf,
    /// Where to write the reports (default: the directory itself)
    #[arg(long)]
    out: Option<PathBuf>,
    #[command(flatten)]
    thresholds: ThresholdArgs,
}

fn env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

fn thresholds(args: &ThresholdArgs) -> Result<Thresholds, String> {
    let flags = ThresholdFlags {
        min_recall: args.min_recall.clone(),
        max_false_positives: args.max_false_positives.clone(),
        min_evidence: args.min_evidence.clone(),
    };
    Thresholds::resolve(&flags, &env)
}

fn usage(message: impl std::fmt::Display) -> ExitCode {
    eprintln!("codev-evals: {message}");
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let repo = cli.repo.unwrap_or_else(|| {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("evals/ has a parent")
            .to_path_buf()
    });
    if let Commands::Docs(args) = &cli.command {
        return docs(&repo, args);
    }
    let cases = match suite::load_cases(&repo) {
        Ok(cases) => cases,
        Err(e) => return usage(e),
    };
    match cli.command {
        Commands::List => {
            for c in &cases {
                println!(
                    "{:<45} {:<14} {:<15} {:<8} {}",
                    c.id,
                    c.role,
                    c.step.as_str(),
                    c.kind.as_str(),
                    c.summary
                );
            }
            for gap in case::coverage_gaps(&cases) {
                eprintln!("coverage: {gap}");
            }
            ExitCode::SUCCESS
        }
        Commands::Score(args) => {
            let thresholds = match thresholds(&args.thresholds) {
                Ok(t) => t,
                Err(e) => return usage(e),
            };
            let out = args.out.unwrap_or_else(|| args.dir.clone());
            match suite::score_dir(&cases, &args.dir, thresholds) {
                Ok(score) => finish(&score, &out),
                Err(e) => usage(e),
            }
        }
        Commands::Run(args) => run(&repo, &cases, args),
        Commands::Compare(args) => compare_runs(&cases, &args),
        Commands::Docs(_) => unreachable!("handled before the cases are loaded"),
    }
}

/// The reliability chapter of each book, from a baseline. Both pages are
/// built before either is written, so a bad report writes nothing.
fn docs(repo: &Path, args: &DocsArgs) -> ExitCode {
    let dir = args
        .baseline
        .clone()
        .unwrap_or_else(|| repo.join("evals").join("baseline"));
    let read = |name: &str| {
        fs::read_to_string(dir.join(name)).map_err(|e| format!("{}: {e}", dir.join(name).display()))
    };
    let baseline = match read("report.json")
        .and_then(|report| Ok((report, read("run.json")?)))
        .and_then(|(report, run)| Baseline::parse(&report, &run))
    {
        Ok(b) => b,
        Err(e) => return usage(e),
    };
    let pages: Vec<(PathBuf, String)> = Lang::ALL
        .into_iter()
        .map(|lang| {
            (
                repo.join("docs")
                    .join(lang.tag())
                    .join("src")
                    .join("reliability.md"),
                reliability::page(&baseline, lang),
            )
        })
        .collect();
    for (path, text) in &pages {
        if let Err(e) = fs::write(path, text) {
            return usage(format!("writing {}: {e}", path.display()));
        }
        println!("wrote {}", path.display());
    }
    ExitCode::SUCCESS
}

fn now() -> (u64, String) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    (secs, runner::civil_date((secs / 86_400) as i64))
}

/// Scores both directories on the cases they share and prints the
/// comparison; refuses two models, an unreadable directory or no case in
/// common, printing no report.
fn compare_runs(cases: &[case::Case], args: &CompareArgs) -> ExitCode {
    let ids = |dir: &Path| -> Result<BTreeSet<String>, String> {
        suite::read_run(dir)?;
        Ok(suite::load_observations(dir)?
            .into_keys()
            .map(|(id, _)| id)
            .collect())
    };
    let (base_ids, cand_ids) =
        match ids(&args.baseline).and_then(|b| Ok((b, ids(&args.candidate)?))) {
            Ok(pair) => pair,
            Err(e) => return usage(e),
        };
    let common: BTreeSet<String> = base_ids.intersection(&cand_ids).cloned().collect();
    if common.is_empty() {
        return usage("the baseline and the candidate have no case in common");
    }
    let not_compared: Vec<String> = base_ids.symmetric_difference(&cand_ids).cloned().collect();
    let score =
        |dir: &Path| suite::score_dir_only(cases, dir, score::Thresholds::default(), Some(&common));
    let (baseline, candidate) =
        match score(&args.baseline).and_then(|b| Ok((b, score(&args.candidate)?))) {
            Ok(pair) => pair,
            Err(e) => return usage(e),
        };
    if let Some(e) = compare::models_differ(
        &baseline.run,
        &compare::models_used(&baseline),
        &candidate.run,
        &compare::models_used(&candidate),
    ) {
        return usage(e);
    }
    let mode = if args.quality_only {
        compare::Mode::QualityOnly
    } else {
        compare::Mode::QualityAndCost
    };
    let result = compare::compare(&baseline, &candidate, &not_compared, mode);
    print!("{}", result.report);
    for f in &result.failures {
        eprintln!("fail: {f}");
    }
    if result.pass {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn run(repo: &Path, cases: &[case::Case], args: RunArgs) -> ExitCode {
    let thresholds = match thresholds(&args.thresholds) {
        Ok(t) => t,
        Err(e) => return usage(e),
    };
    let repeat = args.repeat;
    let settings = match ClaudeSettings::from_env(&env) {
        Ok(s) => s,
        Err(e) => return usage(e),
    };
    for id in &args.cases {
        if !cases.iter().any(|c| &c.id == id) {
            return usage(format!("no case `{id}`"));
        }
    }
    let by_step = match runner::select_steps(cases, &args.steps) {
        Ok(selected) => selected,
        Err(e) => return usage(e),
    };
    let selected: Vec<case::Case> = by_step
        .into_iter()
        .filter(|c| args.cases.is_empty() || args.cases.contains(&c.id))
        .filter(|c| args.roles.is_empty() || args.roles.contains(&c.role))
        .cloned()
        .collect();
    let (secs, date) = now();
    let out = args.out.unwrap_or_else(|| {
        repo.join("target")
            .join("evals")
            .join(format!("run-{secs}"))
    });
    let tools = Tools {
        claude: args
            .claude
            .or_else(|| env("CODEV_EVAL_CLAUDE").map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("claude")),
        codev: args
            .codev
            .unwrap_or_else(|| repo.join("target").join("debug").join("codev")),
        git: PathBuf::from("git"),
    };
    let (claude_version, codev_version) = match runner::preflight(&tools) {
        Ok(v) => v,
        Err(e) => return usage(e),
    };
    let tools = Tools {
        codev: fs::canonicalize(&tools.codev).unwrap_or(tools.codev),
        ..tools
    };
    let info = RunInfo {
        format: FORMAT,
        model: settings.model.clone(),
        date,
        claude_version: Some(claude_version),
        codev_version: Some(codev_version),
        repeat: Some(repeat),
        jobs: Some(args.jobs as u32),
    };
    if let Ok(text) = fs::read_to_string(out.join("run.json")) {
        let resumed = match observed::parse_run_info(&text) {
            Ok(r) => r,
            Err(e) => return usage(e),
        };
        if let Err(e) = runner::check_resume(
            &resumed,
            info.model.as_deref(),
            info.claude_version.as_deref(),
        ) {
            return usage(e);
        }
    }
    if let Err(e) = runner::ensure_run_info(&out, &info) {
        return usage(format!("{}: {e}", out.display()));
    }
    let recorded = suite::recorded(&out);
    let pending = runner::pending(&selected, &recorded, repeat, args.rerun);
    eprintln!(
        "[codev-evals] {} run(s) to make, {} already recorded in {}",
        pending.len(),
        selected.len() * repeat as usize - pending.len(),
        out.display()
    );
    let options = RunOptions {
        repo: repo.to_path_buf(),
        out: out.clone(),
        work: args
            .work
            .unwrap_or_else(|| std::env::temp_dir().join(format!("codev-evals-{secs}"))),
        keep: args.keep,
        jobs: args.jobs,
        tools,
        settings,
    };
    runner::run_cases(&pending, &options, &info.date);
    if !options.keep {
        let _ = fs::remove_dir(&options.work);
    }
    match suite::score_dir(cases, &out, thresholds) {
        Ok(score) => finish(&score, &out),
        Err(e) => usage(e),
    }
}

fn finish(score: &SuiteScore, out: &Path) -> ExitCode {
    let write = fs::create_dir_all(out)
        .and_then(|()| fs::write(out.join("report.json"), report::json(score)))
        .and_then(|()| fs::write(out.join("report.md"), report::markdown(score)));
    if let Err(e) = write {
        return usage(format!("writing the reports in {}: {e}", out.display()));
    }
    for r in &score.roles {
        println!(
            "{:<14} recall {:>5}  false positives {:>2}  evidence {:>5}",
            r.role,
            r.recall.map_or("—".into(), |v| format!("{v:.2}")),
            r.false_positives,
            r.evidence_validity
                .map_or("—".into(), |v| format!("{v:.2}")),
        );
    }
    for f in &score.failures {
        eprintln!("fail: {f}");
    }
    println!("report: {}", out.join("report.md").display());
    ExitCode::from(score::exit_code(score) as u8)
}
