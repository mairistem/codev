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
use serde::de::IgnoredAny;

use crate::observed::{RunInfo, check_format, parse_run_info};
use crate::score::{EvidenceCounts, Thresholds};

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
    pub expected: u32,
    pub true_positives: u32,
    pub misses: u32,
    pub false_positives: u32,
    pub severity_agreement: u32,
    pub evidence: EvidenceCounts,
    pub recall: Option<f64>,
    pub evidence_validity: Option<f64>,
}

impl RoleRow {
    fn checked(&self) -> u32 {
        self.evidence.found + self.evidence.moved + self.evidence.invalid
    }

    fn valid(&self) -> u32 {
        self.evidence.found + self.evidence.moved
    }

    /// The measures this role fails, in the report's order.
    fn fails(&self, t: &Thresholds) -> Vec<Measure> {
        let mut out = Vec::new();
        if self.recall.is_some_and(|r| r < t.min_recall) {
            out.push(Measure::Recall);
        }
        if self.false_positives > t.max_false_positives {
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

/// What the page is built from: the run, the thresholds and the roles.
#[derive(Debug, Clone, PartialEq)]
pub struct Baseline {
    pub run: RunInfo,
    pub thresholds: Thresholds,
    pub cost_usd: f64,
    pub cases: usize,
    pub roles: Vec<RoleRow>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReportFile {
    run: RunInfo,
    thresholds: Thresholds,
    cost_usd: f64,
    roles: Vec<RoleRow>,
    cases: Vec<IgnoredAny>,
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
        Ok(Baseline {
            run,
            thresholds: report.thresholds,
            cost_usd: report.cost_usd,
            cases: report.cases.len(),
            roles: report.roles,
        })
    }

    fn sum(&self, f: impl Fn(&RoleRow) -> u32) -> u32 {
        self.roles.iter().map(f).sum()
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

/// A fraction, or `—` when the denominator is zero.
fn fraction(num: u32, den: u32) -> String {
    if den == 0 {
        "—".into()
    } else {
        format!("{num}/{den}")
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
    let seeded = b.sum(|r| r.seeded);
    let controls = b.sum(|r| r.controls);
    let expected = b.sum(|r| r.expected);
    let found = b.sum(|r| r.true_positives);
    let fps = b.sum(|r| r.false_positives);
    let valid = b.sum(RoleRow::valid);
    let checked = b.sum(RoleRow::checked);
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

    // The run.
    let _ = writeln!(out, "## {}\n", lang.pick("The run", "L'exécution"));
    let model = b.run.model.as_deref().map_or_else(
        || {
            lang.pick("Claude Code's default", "celui de Claude Code par défaut")
                .to_string()
        },
        |m| format!("`{m}`"),
    );
    let _ = writeln!(
        out,
        "- {}{c}{}\n- {}{c}{model}\n- Claude Code{c}{}\n- codev{c}{}\n- {}{c}{} ({} {}, {} {})\n- {}{c}{}\n",
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
        lang.pick("Cost", "Coût"),
        lang.usd(b.cost_usd),
    );

    // At a glance.
    let _ = writeln!(out, "## {}\n", lang.pick("At a glance", "En bref"));
    let per_case = if b.cases == 0 {
        "—".to_string()
    } else {
        lang.decimal(format!("{:.1}", f64::from(fps) / b.cases as f64))
    };
    let recall_all = (expected > 0).then(|| f64::from(found) / f64::from(expected));
    let validity_all = (checked > 0).then(|| f64::from(valid) / f64::from(checked));
    let within = roles - failing.len();
    let tiles = [
        Tile {
            label: lang.pick("Recall", "Rappel"),
            value: opt_ratio(lang, recall_all),
            note: match lang {
                Lang::En => format!("{found} of {expected} seeded defects found"),
                Lang::Fr => format!("{found} défauts ensemencés trouvés sur {expected}"),
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
            value: fps.to_string(),
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
            r.misses,
            r.false_positives,
            evidence,
            fraction(r.severity_agreement, r.true_positives),
            status
        );
    }
    out.push('\n');

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
                        r.false_positives,
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
            "**One run per case.** A model does not answer the same way twice: each score is \
             one noisy point. A role is compared before and after a change on the same model, \
             not across days or models."
                .to_string(),
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
            format!(
                "**Une exécution par cas.** Un modèle ne répond pas deux fois de la même \
                 façon{c}chaque score est un point bruité. Un rôle se compare avant et après une \
                 modification avec le même modèle, pas d'un jour ou d'un modèle à l'autre."
            ),
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
                lang.usd(b.cost_usd)
            ),
            Lang::Fr => format!(
                "Depuis un clone du dépôt, avec Claude Code installé et connecté. Une exécution \
                 appelle un modèle{c}celle-ci a coûté {}.",
                lang.usd(b.cost_usd)
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
            .cmp(&a.false_positives)
            .then(a.role.cmp(&z.role))
    });
    let max = rows
        .iter()
        .map(|r| r.false_positives)
        .max()
        .unwrap_or(0)
        .max(t.max_false_positives)
        .max(1);
    let step = tick_step(max);
    let top = step * max.div_ceil(step);
    let (x0, plot) = (100.0, 200.0);
    let x = |v: u32| x0 + plot * f64::from(v) / f64::from(top);
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
        let over = r.false_positives > t.max_false_positives;
        let tip = match (lang, over) {
            (Lang::En, true) => format!(
                "{}: {} false positives, over the threshold of {}",
                r.role, r.false_positives, t.max_false_positives
            ),
            (Lang::En, false) => format!(
                "{}: {} false positives, within the threshold of {}",
                r.role, r.false_positives, t.max_false_positives
            ),
            (Lang::Fr, true) => format!(
                "{}{c}{} faux positifs, au-delà du seuil de {}",
                r.role, r.false_positives, t.max_false_positives
            ),
            (Lang::Fr, false) => format!(
                "{}{c}{} faux positifs, dans le seuil de {}",
                r.role, r.false_positives, t.max_false_positives
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
        if r.false_positives > 0 {
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
            &r.false_positives.to_string(),
        ));
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
        let gx = x(v);
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
    let tx = x(t.max_false_positives);
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
        let severity = (r.true_positives > 0)
            .then(|| f64::from(r.severity_agreement) / f64::from(r.true_positives));
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
            expected,
            true_positives: found,
            misses: expected - found,
            false_positives: fps,
            severity_agreement: found,
            evidence: EvidenceCounts {
                found: found + fps,
                moved: 0,
                invalid: 0,
                unchecked: 0,
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
            },
            thresholds: Thresholds::default(),
            cost_usd: 15.1836,
            cases: 9,
            roles,
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
            "recall":0.25,"evidenceValidity":1.0,"costUsd":1.5,"failures":[],"aFieldAddedLater":true}}],"cases":[{{}},{{}}]}}"#
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
    fn a_newer_report_format_is_refused() {
        let e = Baseline::parse(&report_json("2026-10-13", 2), RUN_JSON).unwrap_err();
        assert!(e.contains("report.json") && e.contains("format 2"), "{e}");
    }

    #[test]
    fn a_report_of_another_run_is_refused() {
        let e = Baseline::parse(&report_json("2026-11-02", 1), RUN_JSON).unwrap_err();
        assert!(e.contains("report.json") && e.contains("run.json"), "{e}");
        assert!(e.contains("2026-11-02") && e.contains("2026-10-13"), "{e}");
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
}
