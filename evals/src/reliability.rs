//! The documentation's reliability chapter, built from the committed
//! baseline: what each role found, missed and invented, in figures, two
//! charts and a table, in English and French.
//!
//! Pure: [`Baseline::parse`] reads the text of `report.json` and
//! `run.json`, [`page`] returns the chapter's markdown. The command
//! `codev-evals docs` reads and writes the files. The charts are inline
//! SVG with no script, so that the same page works on the website and in
//! the single-page `codev docs`; their colors are CSS custom properties
//! the documentation themes define (`--rel-series`, `--rel-track`,
//! `--rel-over`, `--rel-ink`, `--rel-muted`, `--rel-grid`), each with the
//! light value as a fallback.

use std::fmt::Write as _;

use serde::Deserialize;

use crate::observed::{RunInfo, check_format, parse_run_info};
use crate::score::{
    EvidenceCounts, Measures, SourceShare, Spread, SuiteFigures, Thresholds, number,
};

/// The repository's baseline, on GitHub: the page links to it.
const BASELINE_URL: &str = "https://github.com/mairistem/codev/tree/main/evals/baseline";
const CONTRIBUTING_URL: &str =
    "https://github.com/mairistem/codev/blob/main/CONTRIBUTING.md#changing-a-role";
const CONTRIBUTING_FR_URL: &str =
    "https://github.com/mairistem/codev/blob/main/CONTRIBUTING.fr.md#modifier-un-rôle";

/// Light values, written as presentation attributes so that a chart stays
/// legible where the theme's stylesheet is not loaded. The theme's class
/// rules override them, in light and in dark.
const SERIES: &str = "#076c92";
const TRACK: &str = "#dbebf3";
const OVER: &str = "#a46e0b";
const INK: &str = "#40565e";
const MUTED: &str = "#5f767e";
const GRID: &str = "#e7edef";

/// A no-break space, before French `:` and `;` and between a number and
/// its unit.
const NBSP: char = '\u{a0}';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    Fr,
}

impl Lang {
    pub const ALL: [Lang; 2] = [Lang::En, Lang::Fr];

    /// The book directory under `docs/`.
    pub fn tag(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Fr => "fr",
        }
    }

    fn pick(self, en: &'static str, fr: &'static str) -> &'static str {
        match self {
            Lang::En => en,
            Lang::Fr => fr,
        }
    }

    /// A ratio with two decimals, with a decimal comma in French.
    fn ratio(self, value: f64) -> String {
        self.decimal(format!("{value:.2}"))
    }

    fn decimal(self, text: String) -> String {
        match self {
            Lang::En => text,
            Lang::Fr => text.replace('.', ","),
        }
    }

    fn usd(self, value: f64) -> String {
        match self {
            Lang::En => format!("{value:.2} USD"),
            Lang::Fr => format!("{}{NBSP}USD", self.ratio(value)),
        }
    }

    /// `: ` in English, ` : ` with a no-break space in French.
    fn colon(self) -> String {
        match self {
            Lang::En => ": ".into(),
            Lang::Fr => format!("{NBSP}: "),
        }
    }
}

/// One role's line of `report.json`, the fields the page shows.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleRow {
    pub role: String,
    pub cases: u32,
    pub seeded: u32,
    pub controls: u32,
    pub expected: f64,
    pub true_positives: f64,
    pub misses: f64,
    pub false_positives: f64,
    pub severity_agreement: f64,
    pub evidence: EvidenceCounts,
    pub recall: Option<f64>,
    pub evidence_validity: Option<f64>,
}

impl RoleRow {
    fn checked(&self) -> f64 {
        self.evidence.found + self.evidence.moved + self.evidence.invalid
    }

    fn valid(&self) -> f64 {
        self.evidence.found + self.evidence.moved
    }

    /// The measures this role fails, in the report's order.
    fn fails(&self, t: &Thresholds) -> Vec<Measure> {
        let mut out = Vec::new();
        if self.recall.is_some_and(|r| r < t.min_recall) {
            out.push(Measure::Recall);
        }
        if self.false_positives > f64::from(t.max_false_positives) {
            out.push(Measure::FalsePositives);
        }
        if self.evidence_validity.is_some_and(|v| v < t.min_evidence) {
            out.push(Measure::Evidence);
        }
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Measure {
    Recall,
    FalsePositives,
    Evidence,
}

impl Measure {
    fn name(self, lang: Lang) -> &'static str {
        match self {
            Measure::Recall => lang.pick("recall", "rappel"),
            Measure::FalsePositives => lang.pick("false positives", "faux positifs"),
            Measure::Evidence => lang.pick("evidence validity", "validité des preuves"),
        }
    }
}

/// The planning figures of `report.json`: medians and ranges over the
/// repetitions.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanningFigures {
    pub cases: u32,
    pub primary_recall: Option<Spread>,
    pub collateral_recall: Option<Spread>,
    pub duplicates: Option<Spread>,
    pub decisions_ignored: Option<Spread>,
    pub invalid: Option<Spread>,
}

/// What the page is built from: the run, the thresholds and the roles.
#[derive(Debug, Clone, PartialEq)]
pub struct Baseline {
    pub run: RunInfo,
    pub thresholds: Thresholds,
    /// What one run of the suite costs: the sum of the cases' median costs.
    pub cost_usd: f64,
    /// What the whole baseline cost, every repetition included.
    pub total_cost_usd: f64,
    /// The distinct cases.
    pub cases: usize,
    pub repetitions: u32,
    /// The suite-level medians; `None` in a report of format 1, whose
    /// headline figures are then summed over the roles.
    pub figures: Option<SuiteFigures>,
    pub roles: Vec<RoleRow>,
    /// The planning step, when the baseline holds planning cases.
    pub planning: Option<PlanningFigures>,
    /// The sums of the cases' median consumption.
    pub consumption: Option<Measures>,
    /// Where the tokens go, ranked by cost; empty when no run recorded its
    /// calls.
    pub sources: Vec<SourceShare>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReportFile {
    run: RunInfo,
    thresholds: Thresholds,
    cost_usd: f64,
    #[serde(default)]
    total_cost_usd: Option<f64>,
    #[serde(default)]
    repetitions: Option<u32>,
    #[serde(default)]
    figures: Option<SuiteFigures>,
    roles: Vec<RoleRow>,
    cases: Vec<CaseId>,
    #[serde(default)]
    planning: Option<PlanningFigures>,
    #[serde(default)]
    consumption: Option<Measures>,
    #[serde(default)]
    sources: Vec<SourceShare>,
}

#[derive(Deserialize)]
struct CaseId {
    id: String,
}

/// The headline figures of the page.
struct Headline {
    expected: f64,
    found: f64,
    valid: f64,
    checked: f64,
    false_positives: f64,
    recall: Option<f64>,
    validity: Option<f64>,
    per_case: Option<f64>,
    recall_range: Option<Spread>,
}

impl Baseline {
    /// Reads `report.json` and `run.json`. A newer format is refused,
    /// naming the file; so is a report scored from another run than the
    /// one `run.json` describes. Fields the page does not show are
    /// ignored: the report only ever gains fields.
    pub fn parse(report_json: &str, run_json: &str) -> Result<Baseline, String> {
        let value = check_format(report_json, "report.json")?;
        let report: ReportFile =
            serde_json::from_value(value).map_err(|e| format!("report.json: {e}"))?;
        let run = parse_run_info(run_json)?;
        if report.run != run {
            return Err(format!(
                "report.json records the run of {} ({}), run.json the run of {} ({}); \
                 score the baseline again with `cargo run -p codev-evals -- score <dir>`",
                report.run.date,
                report.run.claude_version.as_deref().unwrap_or("unknown"),
                run.date,
                run.claude_version.as_deref().unwrap_or("unknown"),
            ));
        }
        let mut ids: Vec<String> = report.cases.into_iter().map(|c| c.id).collect();
        ids.sort();
        ids.dedup();
        Ok(Baseline {
            run,
            thresholds: report.thresholds,
            cost_usd: report.cost_usd,
            total_cost_usd: report.total_cost_usd.unwrap_or(report.cost_usd),
            cases: ids.len(),
            repetitions: report.repetitions.unwrap_or(1),
            figures: report.figures,
            roles: report.roles,
            planning: report.planning,
            consumption: report.consumption,
            sources: report.sources,
        })
    }

    fn sum(&self, f: impl Fn(&RoleRow) -> f64) -> f64 {
        self.roles.iter().map(f).sum()
    }

    fn sum_u32(&self, f: impl Fn(&RoleRow) -> u32) -> u32 {
        self.roles.iter().map(f).sum()
    }

    /// The report's suite-level medians, or, for a report of format 1,
    /// the sums over the roles.
    fn headline(&self) -> Headline {
        match &self.figures {
            Some(f) => Headline {
                expected: f.expected,
                found: f.true_positives.map_or(0.0, |s| s.median),
                valid: f.valid,
                checked: f.checked,
                false_positives: f.false_positives.map_or(0.0, |s| s.median),
                recall: f.recall.map(|s| s.median),
                validity: f.evidence_validity.map(|s| s.median),
                per_case: f.false_positives_per_case.map(|s| s.median),
                // A range only when the runs disagreed, as in the report.
                recall_range: f.recall.filter(|s| self.repetitions > 1 && s.min != s.max),
            },
            None => {
                let expected = self.sum(|r| r.expected);
                let found = self.sum(|r| r.true_positives);
                let valid = self.sum(RoleRow::valid);
                let checked = self.sum(RoleRow::checked);
                let fps = self.sum(|r| r.false_positives);
                Headline {
                    expected,
                    found,
                    valid,
                    checked,
                    false_positives: fps,
                    recall: (expected > 0.0).then(|| found / expected),
                    validity: (checked > 0.0).then(|| valid / checked),
                    per_case: (self.cases > 0).then(|| fps / self.cases as f64),
                    recall_range: None,
                }
            }
        }
    }
}

/// The cost line: the whole run, and one run of the suite when the cases
/// were repeated.
fn cost(b: &Baseline, lang: Lang) -> String {
    if b.repetitions > 1 {
        match lang {
            Lang::En => format!(
                "{} for the {} runs of each case ({} for one run of the suite)",
                lang.usd(b.total_cost_usd),
                b.repetitions,
                lang.usd(b.cost_usd)
            ),
            Lang::Fr => format!(
                "{} pour les {} exécutions de chaque cas ({} pour une exécution de la suite)",
                lang.usd(b.total_cost_usd),
                b.repetitions,
                lang.usd(b.cost_usd)
            ),
        }
    } else {
        lang.usd(b.total_cost_usd)
    }
}

/// Escapes text for HTML and SVG.
fn esc(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// `2.1.289 (Claude Code)` → `2.1.289`; `codev 0.6.1` → `0.6.1`.
fn version(raw: Option<&str>, name: &str) -> String {
    let Some(raw) = raw else {
        return "—".into();
    };
    raw.trim()
        .trim_end_matches(format!("({name})").as_str())
        .trim()
        .trim_start_matches(format!("{name} ").as_str())
        .trim()
        .to_string()
}

/// A fraction of medians, or `—` when the denominator is zero.
fn fraction(num: f64, den: f64) -> String {
    if den == 0.0 {
        "—".into()
    } else {
        format!("{}/{}", number(num), number(den))
    }
}

fn opt_ratio(lang: Lang, value: Option<f64>) -> String {
    value.map_or_else(|| "—".into(), |v| lang.ratio(v))
}

/// The chapter, in markdown, for one language.
pub fn page(b: &Baseline, lang: Lang) -> String {
    let mut out = String::new();
    let c = lang.colon();
    let t = &b.thresholds;
    let seeded = b.sum_u32(|r| r.seeded);
    let controls = b.sum_u32(|r| r.controls);
    let h = b.headline();
    let (expected, found, fps, valid, checked) = (
        number(h.expected),
        number(h.found),
        number(h.false_positives),
        number(h.valid),
        number(h.checked),
    );
    let failing: Vec<&RoleRow> = b.roles.iter().filter(|r| !r.fails(t).is_empty()).collect();
    let roles = b.roles.len();

    let _ = writeln!(
        out,
        "# {}\n",
        lang.pick("Reliability of the roles", "Fiabilité des rôles")
    );
    let _ = writeln!(
        out,
        "<!-- Generated by `cargo run -p codev-evals -- docs` from evals/baseline/report.json and run.json. Do not edit by hand. -->\n"
    );
    let _ = writeln!(
        out,
        "{}\n",
        lang.pick(
            "codev's reviewing roles are measured, not assumed. This page shows how they \
             scored the last time the evaluation suite ran, on the run committed in the \
             repository as its baseline. It is generated from that baseline and changes with it.",
            "Les rôles de revue de codev sont mesurés, pas supposés. Cette page montre leurs \
             scores lors de la dernière exécution de la suite d'évaluation, celle que le dépôt \
             conserve comme ligne de base. Elle est générée à partir de cette ligne de base et \
             change avec elle."
        )
    );

    // What it is, what it is not.
    let _ = writeln!(
        out,
        "## {}\n",
        lang.pick("What the suite measures", "Ce que la suite mesure")
    );
    let _ = writeln!(
        out,
        "{}\n",
        match lang {
            Lang::En => "The suite is a set of small projects, each with a planned change. A \
                 **seeded** case hides defects a role should find, each with its severity, its \
                 location and its category of issue; a **clean control** hides none. Claude Code \
                 runs one skill step on each case with a single role, and the suite scores the \
                 review record the role writes: the defects found and missed, the findings that \
                 match no defect (**false positives**), whether each quoted evidence is really at \
                 the lines it cites, and whether the severity agrees with the expected one. \
                 [Measuring the roles](roles.md#measuring-the-roles) gives the rules."
                .to_string(),
            Lang::Fr => format!(
                "La suite est un ensemble de petits projets, chacun avec un change planifié. Un \
                 cas **ensemencé** cache des défauts qu'un rôle devrait trouver, chacun avec sa \
                 gravité, son emplacement et sa catégorie de problème{NBSP}; un **témoin propre** \
                 n'en cache aucun. Claude Code exécute une étape de skill sur chaque cas avec un \
                 seul rôle, et la suite note le compte rendu de revue que le rôle écrit{c}les \
                 défauts trouvés et manqués, les constats qui ne correspondent à aucun défaut \
                 (**faux positifs**), si chaque preuve citée se trouve vraiment aux lignes \
                 indiquées, et si la gravité concorde avec celle attendue. \
                 [Mesurer les rôles](roles.md#mesurer-les-rôles) en donne les règles."
            ),
        }
    );
    let _ = writeln!(
        out,
        "## {}\n",
        lang.pick("What it does not measure", "Ce qu'elle ne mesure pas")
    );
    if b.planning.is_some() {
        let _ = writeln!(
            out,
            "{}\n",
            lang.pick(
                "It is not a benchmark of models, and not a promise that a role finds every \
                 defect of your project. It does not measure how useful a correct finding is to \
                 you. The plan is measured only as a whole, on the planning cases below, by what \
                 it modifies, not role by role. A score describes these cases, on this model, on \
                 the day of the run.",
                "Ce n'est ni un banc d'essai de modèles, ni la promesse qu'un rôle trouve chaque \
                 défaut de votre projet. Elle ne mesure pas l'utilité pour vous d'un constat \
                 juste. Le plan n'est mesuré que dans son ensemble, sur les cas de planification \
                 ci-dessous, par ce qu'il modifie, pas rôle par rôle. Un score décrit ces cas, \
                 avec ce modèle, le jour de l'exécution."
            )
        );
    } else {
        let _ = writeln!(
            out,
            "{}\n",
            lang.pick(
                "It is not a benchmark of models, and not a promise that a role finds every defect \
                 of your project. It does not measure how useful a correct finding is to you, nor \
                 the roles that write the plan (the analyst, the architect, the tech lead…): only \
                 the roles that review. A score describes these cases, on this model, on the day \
                 of the run.",
                "Ce n'est ni un banc d'essai de modèles, ni la promesse qu'un rôle trouve chaque \
                 défaut de votre projet. Elle ne mesure pas l'utilité pour vous d'un constat juste, \
                 ni les rôles qui écrivent le plan (l'analyste, l'architecte, le tech lead…)\u{a0}: \
                 seulement les rôles qui relisent. Un score décrit ces cas, avec ce modèle, le jour \
                 de l'exécution."
            )
        );
    }

    // The run.
    let _ = writeln!(out, "## {}\n", lang.pick("The run", "L'exécution"));
    // The planning cases are counted with the others: name them, so that
    // seeded and controls add up to the total.
    let planning = b.planning.as_ref().map_or_else(String::new, |p| {
        format!(
            ", {} {}",
            p.cases,
            lang.pick("planning", "de planification")
        )
    });
    let model = b.run.model.as_deref().map_or_else(
        || {
            lang.pick("Claude Code's default", "celui de Claude Code par défaut")
                .to_string()
        },
        |m| format!("`{m}`"),
    );
    let _ = writeln!(
        out,
        "- {}{c}{}\n- {}{c}{model}\n- Claude Code{c}{}\n- codev{c}{}\n- {}{c}{} ({} {}, {} {}{planning})\n- {}{c}{}\n- {}{c}{}\n",
        lang.pick("Date", "Date"),
        b.run.date,
        lang.pick("Model", "Modèle"),
        version(b.run.claude_version.as_deref(), "Claude Code"),
        version(b.run.codev_version.as_deref(), "codev"),
        lang.pick("Cases", "Cas"),
        b.cases,
        seeded,
        lang.pick("seeded", "ensemencés"),
        controls,
        lang.pick("clean controls", "témoins propres"),
        lang.pick("Repetitions of each case", "Répétitions de chaque cas"),
        b.repetitions,
        lang.pick("Cost", "Coût"),
        cost(b, lang),
    );

    // At a glance.
    let _ = writeln!(out, "## {}\n", lang.pick("At a glance", "En bref"));
    let per_case = h
        .per_case
        .map_or_else(|| "—".to_string(), |v| lang.decimal(format!("{v:.1}")));
    let recall_all = h.recall;
    let validity_all = h.validity;
    let range = h.recall_range.map_or_else(String::new, |s| match lang {
        Lang::En => format!(
            "; from {} to {} over the {} runs",
            lang.ratio(s.min),
            lang.ratio(s.max),
            b.repetitions
        ),
        Lang::Fr => format!(
            "{NBSP}; de {} à {} sur les {} exécutions",
            lang.ratio(s.min),
            lang.ratio(s.max),
            b.repetitions
        ),
    });
    let within = roles - failing.len();
    let tiles = [
        Tile {
            label: lang.pick("Recall", "Rappel"),
            value: opt_ratio(lang, recall_all),
            note: match lang {
                Lang::En => format!("{found} of {expected} seeded defects found{range}"),
                Lang::Fr => format!("{found} défauts ensemencés trouvés sur {expected}{range}"),
            },
            flagged: false,
        },
        Tile {
            label: lang.pick("Evidence validity", "Validité des preuves"),
            value: opt_ratio(lang, validity_all),
            note: match lang {
                Lang::En => format!("{valid} of {checked} quotes found at the cited lines"),
                Lang::Fr => format!("{valid} citations sur {checked} retrouvées aux lignes citées"),
            },
            flagged: false,
        },
        Tile {
            label: lang.pick("False positives", "Faux positifs"),
            value: fps.clone(),
            note: match lang {
                Lang::En => format!(
                    "{per_case} per case; a role fails above {}",
                    t.max_false_positives
                ),
                Lang::Fr => format!(
                    "{per_case} par cas{NBSP}; un rôle échoue au-delà de {}",
                    t.max_false_positives
                ),
            },
            flagged: false,
        },
        Tile {
            label: lang.pick("Roles within the thresholds", "Rôles dans les seuils"),
            value: match lang {
                Lang::En => format!("{within} of {roles}"),
                Lang::Fr => format!("{within} sur {roles}"),
            },
            note: match (lang, failing.len()) {
                (Lang::En, 0) => "every role meets every threshold".to_string(),
                (Lang::Fr, 0) => "chaque rôle respecte chaque seuil".to_string(),
                (Lang::En, n) => format!("{n} fail at least one"),
                (Lang::Fr, n) => format!("{n} en dépassent au moins un"),
            },
            flagged: !failing.is_empty(),
        },
    ];
    let mut dl = String::from("<dl class=\"rel-tiles\">");
    for Tile {
        label,
        value,
        note,
        flagged,
    } in &tiles
    {
        let icon = if *flagged {
            format!("{} ", over_icon())
        } else {
            String::new()
        };
        let _ = write!(
            dl,
            "<div class=\"rel-tile\"><dt>{}</dt><dd><span class=\"rel-tile-value\">{}</span><span class=\"rel-tile-note\">{icon}{}</span></dd></div>",
            esc(label),
            esc(value),
            esc(note)
        );
    }
    dl.push_str("</dl>");
    let _ = writeln!(out, "{dl}\n");
    let _ = writeln!(out, "{}\n", verdict(b, lang));

    // Charts.
    let _ = writeln!(
        out,
        "## {}\n",
        lang.pick("False positives per role", "Faux positifs par rôle")
    );
    let _ = writeln!(out, "{}\n", false_positive_chart(b, lang));
    let _ = writeln!(
        out,
        "## {}\n",
        lang.pick(
            "Recall and severity agreement per role",
            "Rappel et accord de gravité par rôle"
        )
    );
    let _ = writeln!(out, "{}\n", meter_chart(b, lang));

    // The table.
    let _ = writeln!(out, "## {}\n", lang.pick("Every role", "Tous les rôles"));
    let _ = writeln!(
        out,
        "{}\n",
        lang.pick(
            "The table holds every value the charts draw. **Cases**: seeded and clean \
             controls; **found**: seeded defects found over those expected; **evidence**: the \
             share of checked quotes found at the lines they cite; **severity**: found defects \
             whose severity is the expected one.",
            "Le tableau donne toutes les valeurs que les graphiques dessinent. **Cas**\u{a0}: \
             ensemencés et témoins propres\u{a0}; **trouvés**\u{a0}: défauts ensemencés trouvés \
             sur ceux attendus\u{a0}; **preuves**\u{a0}: part des citations vérifiées retrouvées \
             aux lignes citées\u{a0}; **gravité**\u{a0}: défauts trouvés dont la gravité est \
             celle attendue."
        )
    );
    let _ = writeln!(
        out,
        "| {} |\n|---|---:|---:|---:|---:|---:|---:|---|",
        lang.pick(
            "Role | Cases | Found | Misses | False positives | Evidence | Severity | Thresholds",
            "Rôle | Cas | Trouvés | Manqués | Faux positifs | Preuves | Gravité | Seuils"
        )
    );
    for r in &b.roles {
        let fails = r.fails(t);
        let status = if fails.is_empty() {
            format!(
                "<span class=\"rel-icon-ok\" aria-hidden=\"true\">✓</span> {}",
                lang.pick("Met", "Respectés")
            )
        } else {
            let names: Vec<&str> = fails.iter().map(|m| m.name(lang)).collect();
            format!(
                "{} {}{c}{}",
                over_icon(),
                lang.pick("Failed", "Dépassés"),
                names.join(", ")
            )
        };
        let evidence = match r.evidence_validity {
            None => "—".to_string(),
            Some(v) => lang.ratio(v),
        };
        let _ = writeln!(
            out,
            "| {} | {}{NBSP}({}/{}) | {} | {} | {} | {} | {} | {} |",
            r.role,
            r.cases,
            r.seeded,
            r.controls,
            fraction(r.true_positives, r.expected),
            number(r.misses),
            number(r.false_positives),
            evidence,
            fraction(r.severity_agreement, r.true_positives),
            status
        );
    }
    out.push('\n');

    if let Some(p) = &b.planning {
        planning_section(&mut out, p, lang);
    }
    if let Some(m) = &b.consumption {
        consumption_section(&mut out, b, m, lang);
    }

    // Thresholds.
    let _ = writeln!(out, "## {}\n", lang.pick("The thresholds", "Les seuils"));
    let _ = writeln!(
        out,
        "{}\n",
        match lang {
            Lang::En => format!(
                "A role fails when its recall is under {}, when it reports more than {} false \
                 positives, or when less than {} of its checked evidence is valid — the suite's \
                 defaults, `--min-recall`, `--max-false-positives` and `--min-evidence`. A role \
                 with no seeded case has no recall, and one with no checked finding no evidence \
                 validity: neither fails on it.",
                lang.ratio(t.min_recall),
                t.max_false_positives,
                lang.ratio(t.min_evidence)
            ),
            Lang::Fr => format!(
                "Un rôle échoue quand son rappel est inférieur à {}, quand il signale plus de {} \
                 faux positifs, ou quand moins de {} de ses preuves vérifiées sont valides — les \
                 valeurs par défaut de la suite, `--min-recall`, `--max-false-positives` et \
                 `--min-evidence`. Un rôle sans cas ensemencé n'a pas de rappel, et un rôle sans \
                 constat vérifié pas de validité des preuves{c}aucun des deux n'échoue sur cette \
                 mesure.",
                lang.ratio(t.min_recall),
                t.max_false_positives,
                lang.ratio(t.min_evidence)
            ),
        }
    );
    if failing.is_empty() {
        let _ = writeln!(
            out,
            "{}\n",
            lang.pick(
                "Every role meets every threshold.",
                "Chaque rôle respecte chaque seuil."
            )
        );
    } else {
        for r in &failing {
            let details: Vec<String> = r
                .fails(t)
                .into_iter()
                .map(|m| match m {
                    Measure::Recall => format!(
                        "{} {} < {}",
                        m.name(lang),
                        opt_ratio(lang, r.recall),
                        lang.ratio(t.min_recall)
                    ),
                    Measure::FalsePositives => format!(
                        "{} {} > {}",
                        m.name(lang),
                        number(r.false_positives),
                        t.max_false_positives
                    ),
                    Measure::Evidence => format!(
                        "{} {} < {}",
                        m.name(lang),
                        opt_ratio(lang, r.evidence_validity),
                        lang.ratio(t.min_evidence)
                    ),
                })
                .collect();
            let _ = writeln!(
                out,
                "- {} **{}** {}{c}{}",
                over_icon(),
                r.role,
                lang.pick("fails", "dépasse"),
                details.join(", ")
            );
        }
        let ok: Vec<String> = b
            .roles
            .iter()
            .filter(|r| r.fails(t).is_empty())
            .map(|r| format!("**{}**", r.role))
            .collect();
        if !ok.is_empty() {
            let _ = writeln!(
                out,
                "- <span class=\"rel-icon-ok\" aria-hidden=\"true\">✓</span> {}{c}{}",
                lang.pick("Within every threshold", "Dans tous les seuils"),
                ok.join(", ")
            );
        }
        out.push('\n');
    }

    // Limits.
    let _ = writeln!(out, "## {}\n", lang.pick("The limits", "Les limites"));
    let limits = match lang {
        Lang::En => vec![
            if b.repetitions > 1 {
                format!(
                    "**{} runs per case.** A model does not answer the same way twice: each \
                     score is the median of {} runs of every case, and a change is compared \
                     with the range they span, on the same model, not across days or models.",
                    b.repetitions, b.repetitions
                )
            } else {
                "**One run per case.** A model does not answer the same way twice: each score is \
                 one noisy point. A role is compared before and after a change on the same model, \
                 not across days or models."
                    .to_string()
            },
            format!(
                "**Only what the cases cover.** {} cases, {seeded} of them seeded with {expected} \
                 defects. A kind of defect that no case seeds is not measured: a recall of 1.00 \
                 means these defects were found, not every defect.",
                b.cases
            ),
            "**A controlled context.** Each case is a small project reviewed by one role alone, \
             within a fixed budget and time. Your project is larger, and the roles review it \
             together."
                .to_string(),
            "**Not a guarantee.** The scores describe the roles on this model and this version of \
             Claude Code. Another model may score differently. Read what the roles report \
             before acting on it."
                .to_string(),
        ],
        Lang::Fr => vec![
            if b.repetitions > 1 {
                format!(
                    "**{} exécutions par cas.** Un modèle ne répond pas deux fois de la même \
                     façon{c}chaque score est la médiane de {} exécutions de chaque cas, et une \
                     modification se compare à l'écart qu'elles couvrent, avec le même modèle, \
                     pas d'un jour ou d'un modèle à l'autre.",
                    b.repetitions, b.repetitions
                )
            } else {
                format!(
                    "**Une exécution par cas.** Un modèle ne répond pas deux fois de la même \
                     façon{c}chaque score est un point bruité. Un rôle se compare avant et après une \
                     modification avec le même modèle, pas d'un jour ou d'un modèle à l'autre."
                )
            },
            format!(
                "**Seulement ce que les cas couvrent.** {} cas, dont {seeded} ensemencés de \
                 {expected} défauts. Un type de défaut qu'aucun cas n'ensemence n'est pas \
                 mesuré{c}un rappel de 1,00 signifie que ces défauts-là ont été trouvés, pas tous \
                 les défauts.",
                b.cases
            ),
            "**Un contexte maîtrisé.** Chaque cas est un petit projet relu par un seul rôle, avec \
             un budget et un temps fixés. Votre projet est plus grand, et les rôles le relisent \
             ensemble."
                .to_string(),
            "**Pas une garantie.** Les scores décrivent les rôles avec ce modèle et cette version \
             de Claude Code. Un autre modèle peut obtenir d'autres scores. Lisez ce que les rôles \
             signalent avant d'agir."
                .to_string(),
        ],
    };
    let mut limits = limits;
    if b.planning.is_some() {
        limits.push(
            lang.pick(
                "**Planning replays a public history.** The planning cases replay requests \
                 against codev's own past states, which the model may have read: their figures \
                 compare two versions of the skills on the same cases, they do not tell how well \
                 codev plans a project it has never seen.",
                "**La planification rejoue un historique public.** Les cas de planification \
                 rejouent des demandes sur des états passés de codev, que le modèle a pu lire\u{a0}: \
                 leurs chiffres comparent deux versions des skills sur les mêmes cas, ils ne disent \
                 pas comment codev planifie un projet qu'il n'a jamais vu.",
            )
            .to_string(),
        );
    }
    for l in limits {
        let _ = writeln!(out, "- {l}");
    }
    out.push('\n');

    // Run it yourself.
    let _ = writeln!(
        out,
        "## {}\n",
        lang.pick("Run it yourself", "L'exécuter vous-même")
    );
    let _ = writeln!(
        out,
        "{}\n",
        match lang {
            Lang::En => format!(
                "From a clone of the repository, with Claude Code installed and signed in. A run \
                 calls a model: this one cost {}.",
                cost(b, lang)
            ),
            Lang::Fr => format!(
                "Depuis un clone du dépôt, avec Claude Code installé et connecté. Une exécution \
                 appelle un modèle{c}celle-ci a coûté {}.",
                cost(b, lang)
            ),
        }
    );
    let _ = writeln!(
        out,
        "```bash\ncargo build --bin codev\ncargo run -p codev-evals -- run --out target/evals/mine\ncargo run -p codev-evals -- score target/evals/mine\n```\n"
    );
    let _ = writeln!(
        out,
        "{}",
        match lang {
            Lang::En => format!(
                "The observations, the reports and the run's metadata are in \
                 [`evals/baseline`]({BASELINE_URL}). The [contributing guide]({CONTRIBUTING_URL}) \
                 explains how a change to a role is measured."
            ),
            Lang::Fr => format!(
                "Les observations, les rapports et les métadonnées de l'exécution sont dans \
                 [`evals/baseline`]({BASELINE_URL}). Le [guide de contribution]({CONTRIBUTING_FR_URL}) \
                 explique comment se mesure une modification d'un rôle."
            ),
        }
    );
    out
}

/// A median with its range: `0.67 (0.60–0.75)`, the range only when the
/// repetitions disagreed.
fn spread_text(lang: Lang, s: Option<Spread>, ratio: bool) -> String {
    let show = |v: f64| {
        if ratio {
            lang.ratio(v)
        } else {
            lang.decimal(number(v))
        }
    };
    match s {
        None => "—".into(),
        Some(s) if s.min != s.max => {
            format!("{} ({}–{})", show(s.median), show(s.min), show(s.max))
        }
        Some(s) => show(s.median),
    }
}

/// The planning step: its five measures, median and range.
fn planning_section(out: &mut String, p: &PlanningFigures, lang: Lang) {
    let _ = writeln!(out, "## {}\n", lang.pick("Planning", "La planification"));
    let _ = writeln!(
        out,
        "{}\n",
        match lang {
            Lang::En => format!(
                "{} planning cases replay a request against the repository as it was before an \
                 archived change, and score the plan `/codev-propose` writes against what that \
                 change really did: the **primary recall** is the share of the requirements the \
                 request is about that the plan modifies, the **collateral recall** the share of \
                 those the real change modified in passing; a **probable duplicate** is a \
                 requirement added where one should have been modified. Each figure is the median \
                 over the repetitions, with its range.",
                p.cases
            ),
            Lang::Fr => format!(
                "{} cas de planification rejouent une demande sur le dépôt tel qu'il était avant un \
                 change archivé, et notent le plan qu'écrit `/codev-propose` face à ce que ce change \
                 a vraiment fait{c}le **rappel principal** est la part des exigences visées par la \
                 demande que le plan modifie, le **rappel collatéral** la part de celles que le vrai \
                 change a modifiées au passage{NBSP}; un **doublon probable** est une exigence ajoutée \
                 là où il fallait en modifier une. Chaque chiffre est la médiane sur les \
                 répétitions, avec son écart.",
                p.cases,
                c = lang.colon()
            ),
        }
    );
    let _ = writeln!(
        out,
        "| {} |\n|---|---:|",
        lang.pick("Measure | Median (range)", "Mesure | Médiane (écart)")
    );
    for (label, s, ratio) in [
        (
            lang.pick("Primary recall", "Rappel principal"),
            p.primary_recall,
            true,
        ),
        (
            lang.pick("Collateral recall", "Rappel collatéral"),
            p.collateral_recall,
            true,
        ),
        (
            lang.pick("Probable duplicates", "Doublons probables"),
            p.duplicates,
            false,
        ),
        (
            lang.pick("Decisions ignored", "Décisions ignorées"),
            p.decisions_ignored,
            false,
        ),
        (
            lang.pick(
                "Plans that do not validate",
                "Plans qui ne passent pas la validation",
            ),
            p.invalid,
            false,
        ),
    ] {
        let _ = writeln!(out, "| {label} | {} |", spread_text(lang, s, ratio));
    }
    out.push('\n');
}

/// What a run of the suite consumes, and where its tokens go.
fn consumption_section(out: &mut String, b: &Baseline, m: &Measures, lang: Lang) {
    let c = lang.colon();
    let _ = writeln!(
        out,
        "## {}\n",
        lang.pick("What a run consumes", "Ce que consomme une exécution")
    );
    let tokens = |v: Option<f64>| v.map_or_else(|| "—".to_string(), |v| number(v.round()));
    let _ = writeln!(
        out,
        "- {}{c}{}\n- {}{c}{}\n- {}{c}{}\n",
        lang.pick("One run of the suite", "Une exécution de la suite"),
        lang.usd(b.cost_usd),
        lang.pick("Input per call, in tokens", "Entrée par appel, en tokens"),
        tokens(m.input_per_call),
        lang.pick("Output per call, in tokens", "Sortie par appel, en tokens"),
        tokens(m.output_per_call),
    );
    if b.sources.is_empty() {
        return;
    }
    let _ = writeln!(
        out,
        "{}\n",
        lang.pick(
            "Every call to the model is recorded. The tokens a call writes to the cache — what \
             entered its context since the previous call, the bulk of a run's cost — are shared \
             among what brought them in: the skill read, each `codev` subcommand, the files, the \
             searches, the model's own output; `prompt` is Claude Code's own prompt and the \
             request. Per case the median over its runs, summed over the cases.",
            "Chaque appel au modèle est enregistré. Les tokens qu'un appel écrit dans le cache — ce \
             qui est entré dans son contexte depuis l'appel précédent, l'essentiel du coût d'une \
             exécution — sont répartis entre ce qui les a apportés\u{a0}: la lecture de la skill, \
             chaque sous-commande `codev`, les fichiers, les recherches, la sortie du modèle \
             lui-même\u{a0}; `prompt` est le prompt propre de Claude Code et la demande. Par cas la \
             médiane sur ses exécutions, sommée sur les cas."
        )
    );
    let _ = writeln!(out, "{}\n", sources_chart(&b.sources, lang));
    let _ = writeln!(
        out,
        "| {} |\n|---|---:|---:|",
        lang.pick(
            "Source | Tokens | Share of the cost",
            "Source | Tokens | Part du coût"
        )
    );
    for s in &b.sources {
        let _ = writeln!(
            out,
            "| {} | {} | {} |",
            s.source,
            number(s.tokens),
            percent(lang, s.share)
        );
    }
    out.push('\n');
}

/// `60 %`, with a no-break space in French.
fn percent(lang: Lang, share: f64) -> String {
    match lang {
        Lang::En => format!("{:.0} %", share * 100.0),
        Lang::Fr => format!("{:.0}{NBSP}%", share * 100.0),
    }
}

/// Where the tokens go: one horizontal bar per source, its share of the
/// cost, ranked from the most costly.
fn sources_chart(sources: &[SourceShare], lang: Lang) -> String {
    // A wider label column than the role charts': source names such as
    // `codev instructions` are longer than a role's name.
    let (label_end, x0, plot) = (142.0, 150.0, 200.0);
    let (row_h, bar_h, first) = (26.0, 14.0, 12.0);
    let bottom = first + row_h * sources.len() as f64;
    let height = bottom + 22.0;
    let c = lang.colon();
    let title = lang.pick("Where the tokens go", "Où vont les tokens");
    let mut desc = lang
        .pick(
            "Horizontal bars, one per source, ranked by their share of the cost. ",
            "Barres horizontales, une par source, classées par part du coût. ",
        )
        .to_string();
    let mut marks = String::new();
    for (i, s) in sources.iter().enumerate() {
        let y = first + row_h * i as f64;
        let cy = y + row_h / 2.0;
        let share = percent(lang, s.share);
        let tip = match lang {
            Lang::En => format!(
                "{}: {} tokens, {share} of the cost",
                s.source,
                number(s.tokens)
            ),
            Lang::Fr => format!(
                "{}{c}{} tokens, {share} du coût",
                s.source,
                number(s.tokens)
            ),
        };
        let _ = write!(desc, "{tip}. ");
        let _ = write!(
            marks,
            "<g class=\"rel-row\"><title>{}</title><rect class=\"rel-hit\" x=\"0\" y=\"{y:.1}\" width=\"{VIEW_WIDTH:.1}\" height=\"{row_h:.1}\" fill=\"transparent\"/>",
            esc(&tip)
        );
        marks.push_str(&text(
            "rel-label",
            INK,
            label_end,
            cy + 4.0,
            "end",
            &s.source,
        ));
        let end = x0 + plot * s.share;
        if s.share > 0.0 {
            let _ = write!(
                marks,
                "<path class=\"rel-bar\" fill=\"{SERIES}\" d=\"{}\"/>",
                bar_path(x0, cy - bar_h / 2.0, end - x0, bar_h)
            );
        }
        marks.push_str(&text(
            "rel-value",
            INK,
            end + 6.0,
            cy + 4.0,
            "start",
            &share,
        ));
        marks.push_str("</g>");
    }
    let mut svg = format!(
        "<svg class=\"rel-chart\" viewBox=\"0 0 {VIEW_WIDTH:.0} {height:.0}\" role=\"img\" aria-labelledby=\"rel-src-title-{tag}\" aria-describedby=\"rel-src-desc-{tag}\" font-family=\"sans-serif\" font-size=\"12\"><title id=\"rel-src-title-{tag}\">{}</title><desc id=\"rel-src-desc-{tag}\">{}</desc>",
        esc(title),
        esc(desc.trim_end()),
        tag = lang.tag()
    );
    for v in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let gx = x0 + plot * v;
        let _ = write!(
            svg,
            "<line class=\"rel-grid\" stroke=\"{GRID}\" x1=\"{gx:.1}\" y1=\"{first:.1}\" x2=\"{gx:.1}\" y2=\"{bottom:.1}\"/>"
        );
        svg.push_str(&text(
            "rel-tick",
            MUTED,
            gx,
            bottom + 16.0,
            "middle",
            &percent(lang, v),
        ));
    }
    let _ = write!(
        svg,
        "<line class=\"rel-axis\" stroke=\"{MUTED}\" x1=\"{x0:.1}\" y1=\"{first:.1}\" x2=\"{x0:.1}\" y2=\"{bottom:.1}\"/>"
    );
    svg.push_str(&marks);
    svg.push_str("</svg>");
    figure(
        &svg,
        lang.pick(
            "The share of the cost of the tokens written to the cache, per source, over every case \
             of the suite.",
            "La part du coût des tokens écrits dans le cache, par source, sur tous les cas de la \
             suite.",
        ),
    )
}

/// A headline figure: its label, its value, a line of context, and
/// whether it reports a failure (then the line starts with the icon).
struct Tile {
    label: &'static str,
    value: String,
    note: String,
    flagged: bool,
}

/// The status icon of a failing role: a shape, never color alone, always
/// followed by words; hidden from screen readers, which read the words.
fn over_icon() -> String {
    "<span class=\"rel-icon-over\" aria-hidden=\"true\">▲</span>".into()
}

/// The judgement of the run, computed so that it stays true.
fn verdict(b: &Baseline, lang: Lang) -> String {
    let t = &b.thresholds;
    let roles = b.roles.len();
    let mut sentences = Vec::new();
    let by = |m: Measure| -> Vec<&RoleRow> {
        b.roles.iter().filter(|r| r.fails(t).contains(&m)).collect()
    };
    let any_fail = b.roles.iter().any(|r| !r.fails(t).is_empty());
    if any_fail {
        sentences.push(
            lang.pick(
                "**The run fails its own thresholds.**",
                "**L'exécution ne respecte pas ses propres seuils.**",
            )
            .to_string(),
        );
    } else {
        sentences.push(
            lang.pick(
                "**Every role meets every threshold.**",
                "**Chaque rôle respecte chaque seuil.**",
            )
            .to_string(),
        );
    }
    let list = |rs: &[&RoleRow]| {
        rs.iter()
            .map(|r| r.role.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let c = lang.colon();
    let fp = by(Measure::FalsePositives);
    if !fp.is_empty() {
        sentences.push(match lang {
            Lang::En => format!(
                "{} of {roles} roles report more than {} false positives{c}{}.",
                fp.len(),
                t.max_false_positives,
                list(&fp)
            ),
            Lang::Fr => format!(
                "{} rôles sur {roles} signalent plus de {} faux positifs{c}{}.",
                fp.len(),
                t.max_false_positives,
                list(&fp)
            ),
        });
    }
    let recall = by(Measure::Recall);
    if !recall.is_empty() {
        sentences.push(match lang {
            Lang::En => format!(
                "{} of {roles} roles find less than {} of their seeded defects{c}{}.",
                recall.len(),
                lang.ratio(t.min_recall),
                list(&recall)
            ),
            Lang::Fr => format!(
                "{} rôles sur {roles} trouvent moins de {} de leurs défauts ensemencés{c}{}.",
                recall.len(),
                lang.ratio(t.min_recall),
                list(&recall)
            ),
        });
    }
    let evidence = by(Measure::Evidence);
    if !evidence.is_empty() {
        sentences.push(match lang {
            Lang::En => format!(
                "{} of {roles} roles quote valid evidence less than {} of the time{c}{}.",
                evidence.len(),
                lang.ratio(t.min_evidence),
                list(&evidence)
            ),
            Lang::Fr => format!(
                "{} rôles sur {roles} citent une preuve valide moins de {} du temps{c}{}.",
                evidence.len(),
                lang.ratio(t.min_evidence),
                list(&evidence)
            ),
        });
    }
    if !fp.is_empty() {
        sentences.push(
            lang.pick(
                "Precision is the roles' weak point: they report plausible issues that the case \
                 does not hold. Reducing these false positives without losing what the roles \
                 find is the next work planned on the roles.",
                "La précision est le point faible des rôles\u{a0}: ils signalent des problèmes \
                 plausibles que le cas ne contient pas. Réduire ces faux positifs sans perdre ce \
                 que les rôles trouvent est le prochain travail prévu sur les rôles.",
            )
            .to_string(),
        );
    }
    let recalls: Vec<f64> = b.roles.iter().filter_map(|r| r.recall).collect();
    if !recalls.is_empty() && recalls.iter().all(|r| *r >= 1.0) {
        sentences.push(
            lang.pick(
                "Every seeded defect was found: on this model, recall does not tell the roles \
                 apart, and the differences show in false positives and severity agreement.",
                "Chaque défaut ensemencé a été trouvé\u{a0}: avec ce modèle, le rappel ne \
                 distingue pas les rôles, et les différences se lisent dans les faux positifs et \
                 l'accord de gravité.",
            )
            .to_string(),
        );
    }
    sentences.join(" ")
}

/// The smallest round step that splits `max` into at most six intervals.
fn tick_step(max: u32) -> u32 {
    let mut base = 1;
    loop {
        for m in [1, 2, 5] {
            let step = m * base;
            if max.div_ceil(step) <= 6 {
                return step;
            }
        }
        base *= 10;
    }
}

/// A horizontal bar from `x` to `x + w`, square at the baseline, its data
/// end rounded by 4 units.
fn bar_path(x: f64, y: f64, w: f64, h: f64) -> String {
    let r = 4f64.min(w).min(h / 2.0);
    format!(
        "M{x:.1} {y:.1}h{:.1}a{r:.1} {r:.1} 0 0 1 {r:.1} {r:.1}v{:.1}a{r:.1} {r:.1} 0 0 1 -{r:.1} {r:.1}h-{:.1}z",
        w - r,
        h - 2.0 * r,
        w - r
    )
}

/// The warning shape drawn in a chart, centered on `(cx, cy)`.
fn over_triangle(cx: f64, cy: f64) -> String {
    format!(
        "<path class=\"rel-over\" fill=\"{OVER}\" d=\"M{:.1} {:.1}L{:.1} {:.1}L{:.1} {:.1}z\"/>",
        cx,
        cy - 6.0,
        cx + 6.5,
        cy + 5.0,
        cx - 6.5,
        cy + 5.0
    )
}

fn text(class: &str, fill: &str, x: f64, y: f64, anchor: &str, content: &str) -> String {
    format!(
        "<text class=\"{class}\" fill=\"{fill}\" x=\"{x:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\">{}</text>",
        esc(content)
    )
}

// Both charts share one coordinate system, 400 units wide, scaled to the
// column by the stylesheet. Columns, left to right: role labels ending at
// `LABEL_END`; the plot from x = 100; then the status column (the icon
// and its word). False positives: plot 100–300, status at 340. Meters:
// recall 100–166, status at 210, severity agreement 282–342, each meter
// followed by its fraction. The gaps leave room for the longest French
// word ("au-delà", "en deçà") and a two-digit fraction.
const VIEW_WIDTH: f64 = 400.0;
const LABEL_END: f64 = 92.0;

/// False positives per role: horizontal bars sorted by count, the
/// threshold as a labeled reference line, each role over it marked by an
/// icon and a word.
fn false_positive_chart(b: &Baseline, lang: Lang) -> String {
    let t = &b.thresholds;
    let mut rows: Vec<&RoleRow> = b.roles.iter().collect();
    rows.sort_by(|a, z| {
        z.false_positives
            .total_cmp(&a.false_positives)
            .then(a.role.cmp(&z.role))
    });
    let max = (rows
        .iter()
        .map(|r| r.false_positives)
        .fold(0.0, f64::max)
        .ceil() as u32)
        .max(t.max_false_positives)
        .max(1);
    let step = tick_step(max);
    let top = step * max.div_ceil(step);
    let (x0, plot) = (100.0, 200.0);
    let x = |v: f64| x0 + plot * v / f64::from(top);
    let (row_h, bar_h, first) = (26.0, 14.0, 24.0);
    let bottom = first + row_h * rows.len() as f64;
    let height = bottom + 22.0;
    let c = lang.colon();
    let over_word = lang.pick("over", "au-delà");

    let title = lang.pick("False positives per role", "Faux positifs par rôle");
    let mut desc = match lang {
        Lang::En => format!(
            "Horizontal bars, one per role, sorted by count. The threshold is {}. ",
            t.max_false_positives
        ),
        Lang::Fr => format!(
            "Barres horizontales, une par rôle, triées par nombre. Le seuil est {}. ",
            t.max_false_positives
        ),
    };
    let mut marks = String::new();
    for (i, r) in rows.iter().enumerate() {
        let y = first + row_h * i as f64;
        let cy = y + row_h / 2.0;
        let over = r.false_positives > f64::from(t.max_false_positives);
        let fp = number(r.false_positives);
        let tip = match (lang, over) {
            (Lang::En, true) => format!(
                "{}: {} false positives, over the threshold of {}",
                r.role, fp, t.max_false_positives
            ),
            (Lang::En, false) => format!(
                "{}: {} false positives, within the threshold of {}",
                r.role, fp, t.max_false_positives
            ),
            (Lang::Fr, true) => format!(
                "{}{c}{} faux positifs, au-delà du seuil de {}",
                r.role, fp, t.max_false_positives
            ),
            (Lang::Fr, false) => format!(
                "{}{c}{} faux positifs, dans le seuil de {}",
                r.role, fp, t.max_false_positives
            ),
        };
        let _ = write!(desc, "{tip}. ");
        let _ = write!(
            marks,
            "<g class=\"rel-row\"><title>{}</title><rect class=\"rel-hit\" x=\"0\" y=\"{y:.1}\" width=\"{VIEW_WIDTH:.1}\" height=\"{row_h:.1}\" fill=\"transparent\"/>",
            esc(&tip)
        );
        marks.push_str(&text("rel-label", INK, LABEL_END, cy + 4.0, "end", &r.role));
        let end = x(r.false_positives);
        if r.false_positives > 0.0 {
            let _ = write!(
                marks,
                "<path class=\"rel-bar\" fill=\"{SERIES}\" d=\"{}\"/>",
                bar_path(x0, cy - bar_h / 2.0, end - x0, bar_h)
            );
        }
        marks.push_str(&text("rel-value", INK, end + 6.0, cy + 4.0, "start", &fp));
        if over {
            marks.push_str(&over_triangle(340.0, cy));
            marks.push_str(&text("rel-label", INK, 352.0, cy + 4.0, "start", over_word));
        }
        marks.push_str("</g>");
    }

    let mut svg = format!(
        "<svg class=\"rel-chart\" viewBox=\"0 0 {VIEW_WIDTH:.0} {height:.0}\" role=\"img\" aria-labelledby=\"rel-fp-title-{tag}\" aria-describedby=\"rel-fp-desc-{tag}\" font-family=\"sans-serif\" font-size=\"12\"><title id=\"rel-fp-title-{tag}\">{}</title><desc id=\"rel-fp-desc-{tag}\">{}</desc>",
        esc(title),
        esc(desc.trim_end()),
        tag = lang.tag()
    );
    // Gridlines and their round ticks, then the baseline.
    let mut v = 0;
    while v <= top {
        let gx = x(f64::from(v));
        let _ = write!(
            svg,
            "<line class=\"rel-grid\" stroke=\"{GRID}\" x1=\"{gx:.1}\" y1=\"{first:.1}\" x2=\"{gx:.1}\" y2=\"{bottom:.1}\"/>"
        );
        svg.push_str(&text(
            "rel-tick",
            MUTED,
            gx,
            bottom + 16.0,
            "middle",
            &v.to_string(),
        ));
        v += step;
    }
    let _ = write!(
        svg,
        "<line class=\"rel-axis\" stroke=\"{MUTED}\" x1=\"{x0:.1}\" y1=\"{first:.1}\" x2=\"{x0:.1}\" y2=\"{bottom:.1}\"/>"
    );
    svg.push_str(&marks);
    // The threshold, over the bars, labeled above the plot.
    let tx = x(f64::from(t.max_false_positives));
    let _ = write!(
        svg,
        "<line class=\"rel-threshold\" stroke=\"{INK}\" stroke-width=\"1.5\" stroke-dasharray=\"4 3\" x1=\"{tx:.1}\" y1=\"{:.1}\" x2=\"{tx:.1}\" y2=\"{bottom:.1}\"/>",
        first - 8.0
    );
    svg.push_str(&text(
        "rel-label",
        INK,
        tx + 4.0,
        first - 10.0,
        "start",
        &format!(
            "{}{c}{}",
            lang.pick("Threshold", "Seuil"),
            t.max_false_positives
        ),
    ));
    svg.push_str("</svg>");

    figure(
        &svg,
        &match lang {
            Lang::En => format!(
                "High-confidence findings that match no seeded defect, per role, over every case \
                 of the role. The dashed line is the threshold; {} marks a role over it.",
                "▲ over"
            ),
            Lang::Fr => format!(
                "Constats de confiance haute qui ne correspondent à aucun défaut ensemencé, par \
                 rôle, sur tous les cas du rôle. La ligne pointillée est le seuil{NBSP}; {} marque \
                 un rôle qui le dépasse.",
                "▲ au-delà"
            ),
        },
    )
}

/// Recall and severity agreement per role: two panels of meters from 0
/// to 1 sharing the role rows, the recall threshold drawn, each role under
/// it marked by an icon and a word. A measure without value draws nothing.
fn meter_chart(b: &Baseline, lang: Lang) -> String {
    let t = &b.thresholds;
    let (row_h, meter_h, first) = (24.0, 10.0, 26.0);
    let bottom = first + row_h * b.roles.len() as f64;
    let height = bottom + 8.0;
    let (ax, aw) = (100.0, 66.0);
    let (bx, bw) = (282.0, 60.0);
    let c = lang.colon();
    let under_word = lang.pick("under", "en deçà");
    let title = lang.pick(
        "Recall and severity agreement per role",
        "Rappel et accord de gravité par rôle",
    );
    let mut desc = match lang {
        Lang::En => format!(
            "Two meters per role, from 0 to 1. Recall is the share of seeded defects found, with \
             a threshold of {}; severity agreement is the share of found defects whose severity \
             is the expected one. ",
            lang.ratio(t.min_recall)
        ),
        Lang::Fr => format!(
            "Deux jauges par rôle, de 0 à 1. Le rappel est la part des défauts ensemencés \
             trouvés, avec un seuil de {}{NBSP}; l'accord de gravité est la part des défauts \
             trouvés dont la gravité est celle attendue. ",
            lang.ratio(t.min_recall)
        ),
    };
    let mut svg_rows = String::new();
    for (i, r) in b.roles.iter().enumerate() {
        let y = first + row_h * i as f64;
        let cy = y + row_h / 2.0;
        let my = cy - meter_h / 2.0;
        let severity = (r.true_positives > 0.0).then(|| r.severity_agreement / r.true_positives);
        let under = r.recall.is_some_and(|v| v < t.min_recall);
        let recall_text = match r.recall {
            None => lang
                .pick("no seeded case", "aucun cas ensemencé")
                .to_string(),
            Some(v) => format!(
                "{} ({} {})",
                lang.ratio(v),
                fraction(r.true_positives, r.expected),
                lang.pick("found", "trouvés")
            ),
        };
        let severity_text = match severity {
            None => "—".to_string(),
            Some(v) => format!(
                "{} ({})",
                lang.ratio(v),
                fraction(r.severity_agreement, r.true_positives)
            ),
        };
        let tip = format!(
            "{}{c}{} {recall_text}{}, {} {severity_text}",
            r.role,
            lang.pick("recall", "rappel"),
            if under {
                match lang {
                    Lang::En => format!(", under the threshold of {}", lang.ratio(t.min_recall)),
                    Lang::Fr => format!(", en deçà du seuil de {}", lang.ratio(t.min_recall)),
                }
            } else {
                String::new()
            },
            lang.pick("severity agreement", "accord de gravité"),
        );
        let _ = write!(desc, "{tip}. ");
        let _ = write!(
            svg_rows,
            "<g class=\"rel-row\"><title>{}</title><rect class=\"rel-hit\" x=\"0\" y=\"{y:.1}\" width=\"{VIEW_WIDTH:.1}\" height=\"{row_h:.1}\" fill=\"transparent\"/>",
            esc(&tip)
        );
        svg_rows.push_str(&text("rel-label", INK, LABEL_END, cy + 4.0, "end", &r.role));
        for (px, pw, value, label) in [
            (ax, aw, r.recall, fraction(r.true_positives, r.expected)),
            (
                bx,
                bw,
                severity,
                fraction(r.severity_agreement, r.true_positives),
            ),
        ] {
            if let Some(v) = value {
                let _ = write!(
                    svg_rows,
                    "<rect class=\"rel-track\" fill=\"{TRACK}\" x=\"{px:.1}\" y=\"{my:.1}\" width=\"{pw:.1}\" height=\"{meter_h:.1}\"/>"
                );
                let w = pw * v.clamp(0.0, 1.0);
                if w > 0.0 {
                    let _ = write!(
                        svg_rows,
                        "<path class=\"rel-bar\" fill=\"{SERIES}\" d=\"{}\"/>",
                        bar_path(px, my, w, meter_h)
                    );
                }
            }
            svg_rows.push_str(&text(
                "rel-value",
                INK,
                px + pw + 6.0,
                cy + 4.0,
                "start",
                &label,
            ));
        }
        if under {
            svg_rows.push_str(&over_triangle(210.0, cy));
            svg_rows.push_str(&text(
                "rel-label",
                INK,
                219.0,
                cy + 4.0,
                "start",
                under_word,
            ));
        }
        svg_rows.push_str("</g>");
    }

    let mut svg = format!(
        "<svg class=\"rel-chart\" viewBox=\"0 0 {VIEW_WIDTH:.0} {height:.0}\" role=\"img\" aria-labelledby=\"rel-meter-title-{tag}\" aria-describedby=\"rel-meter-desc-{tag}\" font-family=\"sans-serif\" font-size=\"12\"><title id=\"rel-meter-title-{tag}\">{}</title><desc id=\"rel-meter-desc-{tag}\">{}</desc>",
        esc(title),
        esc(desc.trim_end()),
        tag = lang.tag()
    );
    svg.push_str(&text(
        "rel-head",
        INK,
        ax,
        12.0,
        "start",
        &format!(
            "{} ({} {})",
            lang.pick("Recall", "Rappel"),
            lang.pick("threshold", "seuil"),
            lang.ratio(t.min_recall)
        ),
    ));
    svg.push_str(&text(
        "rel-head",
        INK,
        bx,
        12.0,
        "start",
        lang.pick("Severity agreement", "Accord de gravité"),
    ));
    svg.push_str(&svg_rows);
    let tx = ax + aw * t.min_recall.clamp(0.0, 1.0);
    let _ = write!(
        svg,
        "<line class=\"rel-threshold\" stroke=\"{INK}\" stroke-width=\"1.5\" stroke-dasharray=\"4 3\" x1=\"{tx:.1}\" y1=\"{:.1}\" x2=\"{tx:.1}\" y2=\"{bottom:.1}\"/>",
        first - 4.0
    );
    svg.push_str("</svg>");

    figure(
        &svg,
        &match lang {
            Lang::En => format!(
                "Each meter runs from 0 to 1 and ends with its fraction. The dashed line on recall \
                 is the threshold; {} marks a role under it. A role without a seeded case has no \
                 recall (—).",
                "▲ under"
            ),
            Lang::Fr => format!(
                "Chaque jauge va de 0 à 1 et se termine par sa fraction. La ligne pointillée du \
                 rappel est le seuil{NBSP}; {} marque un rôle en deçà. Un rôle sans cas ensemencé \
                 n'a pas de rappel (—).",
                "▲ en deçà"
            ),
        },
    )
}

/// One HTML block: no blank line inside, so that mdBook and `codev docs`
/// pass it through as it is.
fn figure(svg: &str, caption: &str) -> String {
    format!(
        "<figure class=\"rel-figure\">\n{svg}\n<figcaption>{}</figcaption>\n</figure>",
        esc(caption)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observed::FORMAT;

    fn role(name: &str, expected: u32, found: u32, fps: u32) -> RoleRow {
        RoleRow {
            role: name.into(),
            cases: 3,
            seeded: u32::from(expected > 0) * 2,
            controls: 1,
            expected: f64::from(expected),
            true_positives: f64::from(found),
            misses: f64::from(expected - found),
            false_positives: f64::from(fps),
            severity_agreement: f64::from(found),
            evidence: EvidenceCounts {
                found: f64::from(found + fps),
                moved: 0.0,
                invalid: 0.0,
                unchecked: 0.0,
            },
            recall: (expected > 0).then(|| f64::from(found) / f64::from(expected)),
            evidence_validity: (found + fps > 0).then_some(1.0),
        }
    }

    fn baseline(roles: Vec<RoleRow>) -> Baseline {
        Baseline {
            run: RunInfo {
                format: FORMAT,
                model: None,
                date: "2026-10-13".into(),
                claude_version: Some("2.1.289 (Claude Code)".into()),
                codev_version: Some("codev 0.6.1".into()),
                repeat: None,
                jobs: None,
            },
            thresholds: Thresholds::default(),
            cost_usd: 15.1836,
            total_cost_usd: 15.1836,
            cases: 9,
            repetitions: 1,
            figures: None,
            roles,
            planning: None,
            consumption: None,
            sources: vec![],
        }
    }

    /// The SVG `<g>` of a role in the chart that starts with `title`.
    fn chart_row<'a>(page: &'a str, chart: &str, role: &str) -> &'a str {
        let start = page.find(chart).expect("the chart");
        let chart = &page[start..];
        let chart = &chart[..chart.find("</svg>").expect("its end")];
        let at = chart
            .find(format!("<title>{role}").as_str())
            .unwrap_or_else(|| panic!("a row for {role}"));
        let row = &chart[at..];
        &row[..row.find("</g>").expect("the row's end")]
    }

    fn report_json(date: &str, format: u32) -> String {
        format!(
            r#"{{"format":{format},"run":{{"format":1,"model":null,"date":"{date}","claudeVersion":"2.1.289 (Claude Code)","codevVersion":"codev 0.6.1"}},
            "thresholds":{{"minRecall":0.5,"maxFalsePositives":2,"minEvidence":0.8}},"pass":false,"costUsd":1.5,"failures":[],
            "roles":[{{"role":"ops","cases":2,"seeded":1,"controls":1,"expected":4,"truePositives":1,"misses":3,"falsePositives":0,
            "unreported":0,"duplicates":0,"crossRole":0,"severityAgreement":1,"evidence":{{"found":1,"moved":0,"invalid":0,"unchecked":0}},
            "recall":0.25,"evidenceValidity":1.0,"costUsd":1.5,"failures":[],"aFieldAddedLater":true}}],"cases":[{{"id":"a"}},{{"id":"b"}},{{"id":"b"}}]}}"#
        )
    }

    const RUN_JSON: &str = r#"{"format":1,"model":null,"date":"2026-10-13","claudeVersion":"2.1.289 (Claude Code)","codevVersion":"codev 0.6.1"}"#;

    #[test]
    fn the_baseline_reads_a_report_and_ignores_unknown_fields() {
        let b = Baseline::parse(&report_json("2026-10-13", 1), RUN_JSON).unwrap();
        assert_eq!(b.cases, 2);
        assert_eq!(b.roles[0].role, "ops");
        assert_eq!(b.roles[0].recall, Some(0.25));
        assert_eq!(b.thresholds, Thresholds::default());
    }

    #[test]
    fn a_report_of_format_3_is_refused() {
        let e = Baseline::parse(&report_json("2026-10-13", 3), RUN_JSON).unwrap_err();
        assert!(e.contains("report.json") && e.contains("format 3"), "{e}");
    }

    #[test]
    fn a_report_of_another_run_is_refused() {
        let e = Baseline::parse(&report_json("2026-11-02", 1), RUN_JSON).unwrap_err();
        assert!(e.contains("report.json") && e.contains("run.json"), "{e}");
        assert!(e.contains("2026-11-02") && e.contains("2026-10-13"), "{e}");
    }

    #[test]
    fn the_reliability_page_names_the_repetitions() {
        let mut b = baseline(vec![role("ops", 4, 4, 1)]);
        b.repetitions = 5;
        b.total_cost_usd = 75.0;
        b.figures = Some(SuiteFigures {
            expected: 39.0,
            true_positives: Some(Spread {
                median: 38.0,
                min: 36.0,
                max: 39.0,
            }),
            recall: Some(Spread {
                median: 0.974,
                min: 0.923,
                max: 1.0,
            }),
            checked: 80.0,
            valid: 80.0,
            evidence_validity: Some(Spread {
                median: 1.0,
                min: 1.0,
                max: 1.0,
            }),
            false_positives: Some(Spread {
                median: 7.0,
                min: 5.0,
                max: 9.0,
            }),
            false_positives_per_case: Some(Spread {
                median: 0.175,
                min: 0.125,
                max: 0.225,
            }),
        });
        let en = page(&b, Lang::En);
        assert!(en.contains("**5 runs per case.**"), "{en}");
        assert!(en.contains("Repetitions of each case: 5"), "{en}");
        assert!(
            en.contains("38 of 39 seeded defects found; from 0.92 to 1.00 over the 5 runs"),
            "{en}"
        );
        assert!(en.contains("75.00 USD for the 5 runs of each case"), "{en}");
        let fr = page(&b, Lang::Fr);
        assert!(fr.contains("**5 exécutions par cas.**"), "{fr}");
        assert!(fr.contains("Répétitions de chaque cas"), "{fr}");
    }

    #[test]
    fn the_page_is_identical_twice() {
        let b = baseline(vec![role("ops", 4, 4, 11), role("tracer", 1, 1, 0)]);
        for lang in Lang::ALL {
            assert_eq!(page(&b, lang), page(&b, lang));
        }
    }

    #[test]
    fn a_role_over_the_false_positive_threshold_is_marked() {
        let b = baseline(vec![role("ops", 4, 4, 11), role("security", 2, 2, 1)]);
        let en = page(&b, Lang::En);
        let ops = chart_row(&en, "rel-fp-title-en", "ops");
        assert!(ops.contains("over the threshold of 2"), "{ops}");
        assert!(ops.contains("class=\"rel-over\""), "an icon: {ops}");
        assert!(ops.contains(">over</text>"), "a label: {ops}");
        let security = chart_row(&en, "rel-fp-title-en", "security");
        assert!(!security.contains("rel-over"), "{security}");
        assert!(
            en.contains("| ops | 3\u{a0}(2/1) | 4/4 | 0 | 11 |"),
            "the table: {en}"
        );
        assert!(en.contains("Failed: false positives |"), "{en}");
        assert!(
            en.contains("1 of 2 roles report more than 2 false positives: ops."),
            "{en}"
        );
        let fr = page(&b, Lang::Fr);
        let ops = chart_row(&fr, "rel-fp-title-fr", "ops");
        assert!(ops.contains(">au-delà</text>"), "{ops}");
        assert!(fr.contains("Dépassés\u{a0}: faux positifs |"), "{fr}");
    }

    #[test]
    fn a_role_under_the_recall_threshold_is_marked() {
        let b = baseline(vec![role("ops", 4, 1, 0), role("privacy", 2, 2, 0)]);
        let en = page(&b, Lang::En);
        let ops = chart_row(&en, "rel-meter-title-en", "ops");
        assert!(ops.contains("under the threshold of 0.50"), "{ops}");
        assert!(ops.contains("class=\"rel-over\"") && ops.contains(">under</text>"));
        // A quarter of the 66-unit recall scale.
        assert!(ops.contains("M100.0 "), "{ops}");
        assert!(ops.contains("h12.5a4.0"), "a 16.5-unit fill: {ops}");
        assert!(ops.contains(">1/4</text>"), "{ops}");
        let privacy = chart_row(&en, "rel-meter-title-en", "privacy");
        assert!(!privacy.contains("rel-over"), "{privacy}");
        assert!(
            en.contains("<span class=\"rel-tile-value\">0.50</span><span class=\"rel-tile-note\">3 of 6 seeded defects found"),
            "the headline counts the misses: {en}"
        );
        assert!(en.contains("Failed: recall |"), "{en}");
        assert!(!en.contains("Every seeded defect was found"), "{en}");
    }

    #[test]
    fn a_role_without_seeded_case_shows_no_recall() {
        let b = baseline(vec![role("tracer", 0, 0, 0)]);
        let en = page(&b, Lang::En);
        let tracer = chart_row(&en, "rel-meter-title-en", "tracer");
        assert!(tracer.contains("no seeded case"), "{tracer}");
        assert!(!tracer.contains("rel-bar") && !tracer.contains("rel-track"));
        assert!(tracer.contains(">—</text>"), "{tracer}");
        assert!(
            en.contains("| tracer | 3\u{a0}(0/1) | — | 0 | 0 | — | — |"),
            "{en}"
        );
    }

    #[test]
    fn every_role_within_the_thresholds_is_said() {
        let b = baseline(vec![role("ops", 4, 3, 1), role("security", 2, 2, 2)]);
        let en = page(&b, Lang::En);
        assert!(!en.contains("rel-over"), "no role marked: {en}");
        assert!(en.contains("**Every role meets every threshold.**"), "{en}");
        assert!(!en.contains("Precision is the roles' weak point"), "{en}");
        let fr = page(&b, Lang::Fr);
        assert!(
            fr.contains("**Chaque rôle respecte chaque seuil.**"),
            "{fr}"
        );
    }

    #[test]
    fn a_figure_holds_no_blank_line() {
        let b = baseline(vec![role("ops", 4, 1, 11), role("tracer", 0, 0, 0)]);
        for lang in Lang::ALL {
            let p = page(&b, lang);
            let mut rest = p.as_str();
            let mut figures = 0;
            while let Some(start) = rest.find("<figure") {
                let end = rest[start..].find("</figure>").expect("closed") + start;
                assert!(!rest[start..end].contains("\n\n"), "{}", &rest[start..end]);
                figures += 1;
                rest = &rest[end..];
            }
            assert_eq!(figures, 2);
            assert!(p.contains("role=\"img\"") && p.contains("<desc id="));
        }
    }

    #[test]
    fn ticks_are_round() {
        assert_eq!(tick_step(2), 1);
        assert_eq!(tick_step(11), 2);
        assert_eq!(tick_step(29), 5);
        assert_eq!(tick_step(130), 50);
    }

    #[test]
    fn the_page_gives_the_planning_figures() {
        let mut b = baseline(vec![role("security", 2, 2, 0)]);
        let s = |median, min, max| Some(Spread { median, min, max });
        b.planning = Some(PlanningFigures {
            cases: 25,
            primary_recall: s(0.67, 0.6, 0.75),
            collateral_recall: s(0.4, 0.4, 0.4),
            duplicates: s(2.0, 1.0, 3.0),
            decisions_ignored: s(1.0, 0.0, 1.0),
            invalid: s(0.0, 0.0, 0.0),
        });
        let en = page(&b, Lang::En);
        assert!(en.contains("## Planning"), "{en}");
        assert!(en.contains("| Primary recall | 0.67 (0.60–0.75) |"), "{en}");
        assert!(en.contains("| Collateral recall | 0.40 |"), "{en}");
        assert!(
            en.contains("**Planning replays a public history.**"),
            "{en}"
        );
        let fr = page(&b, Lang::Fr);
        assert!(fr.contains("## La planification"), "{fr}");
        assert!(
            fr.contains("| Rappel principal | 0,67 (0,60–0,75) |"),
            "{fr}"
        );
        // The run line counts the planning cases with the others.
        assert!(en.contains("controls, 25 planning)"), "{en}");
        assert!(fr.contains("témoins propres, 25 de planification)"), "{fr}");
        assert!(
            !page(&baseline(vec![role("security", 2, 2, 0)]), Lang::En).contains("## Planning")
        );
    }

    #[test]
    fn the_page_ranks_the_sources_by_cost() {
        let mut b = baseline(vec![role("security", 2, 2, 0)]);
        b.consumption = Some(Measures {
            input_per_call: Some(38_000.0),
            output_per_call: Some(900.0),
            ..Measures::default()
        });
        let share = |source: &str, tokens: f64, share: f64| SourceShare {
            source: source.into(),
            tokens,
            share,
        };
        b.sources = vec![
            share("skill", 60_000.0, 0.6),
            share("codev roles", 25_000.0, 0.25),
            share("file", 15_000.0, 0.15),
        ];
        for (lang, heading, row) in [
            (
                Lang::En,
                "## What a run consumes",
                "| skill | 60000 | 60 % |",
            ),
            (
                Lang::Fr,
                "## Ce que consomme une exécution",
                "| skill | 60000 | 60\u{a0}% |",
            ),
        ] {
            let text = page(&b, lang);
            let at = |needle: &str| {
                text.find(needle)
                    .unwrap_or_else(|| panic!("{needle}: {text}"))
            };
            assert!(at(heading) < at(row));
            assert!(
                at("| skill |") < at("| codev roles |") && at("| codev roles |") < at("| file |")
            );
            assert!(text.contains(&format!("rel-src-title-{}", lang.tag())));
            // The chart draws the sources in the same order, each named on hover.
            let chart = &text[at("rel-src-title")..];
            assert!(chart.find(">skill<").unwrap() < chart.find(">file<").unwrap());
            assert!(text.contains("38000"));
            assert!(!text.contains("{NBSP}"), "a placeholder left in the page");
            // Long source names fit left of the plot.
            assert!(chart.contains("x=\"142.0\""), "{chart}");
        }
    }
}
