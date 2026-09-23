//! Parseur d'ADR : frontmatter YAML + sections markdown.
//!
//! Deux petites hospitalités du format à traiter :
//!
//! - `id: 0001` est un entier au sens YAML — on le tolère et on
//!   sérialise en `String` toujours, pour que les consommateurs n'aient
//!   qu'une forme à connaître ;
//! - `supersedes` peut être un identifiant unique ou une liste — on
//!   normalise en `Vec<String>`.

use serde::Deserialize;

use crate::parser::ast::{Finding, Parsed, Span};
use crate::parser::codes;

use super::ast::{Decision, DecisionStatus, Section};

/// Valide qu'un `deviates_from` est bien une liste de chaînes.
///
/// Séparé du reste du parsing pour émettre un finding **spécifique**
/// (`DECISION_FIELD_TYPE_MISMATCH`) plutôt que la générique
/// `DECISION_MISSING_FIELD` que serde produirait par défaut.
fn extract_deviates_from(
    value: &serde_norway::Value,
    findings: &mut Vec<Finding>,
) -> Vec<String> {
    let seq = match value {
        serde_norway::Value::Sequence(seq) => seq,
        serde_norway::Value::Null => return Vec::new(),
        _ => {
            findings.push(Finding::error(
                codes::DECISION_FIELD_TYPE_MISMATCH,
                1,
                "le champ `deviates_from` doit être une liste d'identifiants qualifiés \
                 (par exemple `deviates_from: [path:~/partage/0100]`)",
            ));
            return Vec::new();
        }
    };

    let mut out = Vec::with_capacity(seq.len());
    for (i, item) in seq.iter().enumerate() {
        match item {
            serde_norway::Value::String(s) => out.push(s.clone()),
            _ => {
                findings.push(Finding::error(
                    codes::DECISION_FIELD_TYPE_MISMATCH,
                    1,
                    format!(
                        "l'entrée #{i} de `deviates_from` doit être une chaîne \
                         `<origin>/<id>` (par exemple `path:~/partage/0100`)"
                    ),
                ));
            }
        }
    }
    out
}

/// Analyse un fichier ADR.
///
/// Rend un `Parsed<Option<Decision>>` : si le frontmatter manque ou est
/// invalide, la valeur est `None` mais les findings pointent l'endroit du
/// problème — même mécanique que le parseur de spec.
pub fn parse_decision(source: &str) -> Parsed<Option<Decision>> {
    let mut findings = Vec::new();

    let Some((frontmatter_raw, body_offset)) = extract_frontmatter(source) else {
        findings.push(Finding::error(
            codes::DECISION_MISSING_FRONTMATTER,
            1,
            "ce fichier n'a pas de frontmatter YAML délimité par `---` en tête et en fin",
        ));
        return Parsed {
            value: None,
            findings,
        };
    };

    let frontmatter: RawFrontmatter = match serde_norway::from_str(frontmatter_raw) {
        Ok(v) => v,
        Err(err) => {
            findings.push(Finding::error(
                codes::DECISION_MISSING_FIELD,
                1,
                format!("frontmatter YAML invalide : {err}"),
            ));
            return Parsed {
                value: None,
                findings,
            };
        }
    };

    // `deviates_from` est validé à la main pour émettre un finding
    // spécifique quand sa forme n'est pas une liste de chaînes.
    let deviates_from = match &frontmatter.deviates_from {
        Some(raw) => extract_deviates_from(raw, &mut findings),
        None => Vec::new(),
    };

    let status = DecisionStatus::from_raw(&frontmatter.status);
    if let DecisionStatus::Unknown(raw) = &status {
        findings.push(Finding::error(
            codes::DECISION_UNKNOWN_STATUS,
            1,
            format!(
                "statut « {raw} » inconnu ; utilise l'un de : accepted, superseded, proposed, deprecated, rejected"
            ),
        ));
    }

    // Le frontmatter va des premiers `---` (ligne 1) au deuxième `---`
    // (juste avant `body_offset`). On calcule sa ligne de fin depuis
    // `body_offset`.
    let frontmatter_span = Span::new(
        0..body_offset,
        1..line_at_byte(source, body_offset),
    );
    let sections = split_sections(source, body_offset);

    Parsed {
        value: Some(Decision {
            id: frontmatter.id.into_string(),
            title: frontmatter.title,
            status,
            date: frontmatter.date,
            tags: frontmatter.tags,
            supersedes: frontmatter.supersedes.into_vec(),
            deviates_from,
            sections,
            frontmatter_span,
        }),
        findings,
    }
}

/// Renvoie `(contenu du frontmatter, offset du premier octet du corps)`.
fn extract_frontmatter(source: &str) -> Option<(&str, usize)> {
    // Doit commencer par `---\n` — on tolère `\r\n` en normalisant à `\n`
    // au moment du split.
    let after_open = source.strip_prefix("---\n")?;
    // Chercher la fermeture : une ligne `---` seule.
    let close_relative = find_close_fence(after_open)?;
    let frontmatter = &after_open[..close_relative];
    // `body_offset` en octets dans le source d'origine : `---\n` (4) +
    // frontmatter + `---\n` (4) — sauf si le fichier se termine juste
    // après `---`, auquel cas on prend la longueur totale.
    let open_len = 4;
    let close_len = "---\n".len();
    let raw_offset = open_len + close_relative + close_len;
    let body_offset = raw_offset.min(source.len());
    Some((frontmatter, body_offset))
}

/// Cherche une ligne `---` (fermeture) dans le contenu qui suit
/// l'ouverture. Rend l'offset relatif du début de cette ligne, ou `None`.
fn find_close_fence(after_open: &str) -> Option<usize> {
    let mut cursor = 0;
    for line in after_open.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        if trimmed == "---" {
            return Some(cursor);
        }
        cursor += line.len();
    }
    None
}

fn line_at_byte(source: &str, byte: usize) -> u32 {
    let clamped = byte.min(source.len());
    1 + source[..clamped].bytes().filter(|&b| b == b'\n').count() as u32
}

/// Découpe le corps en sections `## <titre>` de premier niveau.
///
/// Le préambule (avant la première section) est ignoré — nos ADR n'en
/// portent pas. Un futur change pourrait l'exposer si le besoin apparaît.
fn split_sections(source: &str, body_offset: usize) -> Vec<Section> {
    let body = &source[body_offset..];
    let base_line = line_at_byte(source, body_offset);

    // On repère les débuts de section (`## `, exactement deux dièses suivis
    // d'un espace) et leur position en octets absolue.
    let mut heads: Vec<(usize, usize, String)> = Vec::new(); // (abs_start, rel_start, name)
    let mut rel = 0usize;
    for line in body.split_inclusive('\n') {
        let stripped = line.trim_end_matches(['\n', '\r']);
        if let Some(name) = section_name(stripped) {
            heads.push((body_offset + rel, rel, name));
        }
        rel += line.len();
    }

    let total = source.len();
    let mut sections = Vec::with_capacity(heads.len());
    for (i, (abs_start, rel_start, name)) in heads.iter().enumerate() {
        let end = heads
            .get(i + 1)
            .map(|(abs, _, _)| *abs)
            .unwrap_or(total);
        let head_line_offset = body[..*rel_start].bytes().filter(|&b| b == b'\n').count() as u32;
        let start_line = base_line + head_line_offset;
        let inner_body = source[*abs_start..end]
            .lines()
            .skip(1) // sauter l'en-tête `## …`
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
        let end_line = line_at_byte(source, end);
        sections.push(Section {
            name: name.clone(),
            body: inner_body,
            span: Span::new(*abs_start..end, start_line..end_line),
        });
    }
    sections
}

fn section_name(line: &str) -> Option<String> {
    let after = line.strip_prefix("## ")?;
    if after.starts_with('#') {
        return None; // c'est `###` ou plus profond
    }
    let trimmed = after.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
}

// ─────────────────────────── frontmatter deserialisation ───────────────────────────

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFrontmatter {
    id: IdValue,
    title: String,
    status: String,
    date: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    supersedes: SupersedesValue,
    /// Le champ est passé en `Value` puis validé à la main — cela permet
    /// d'émettre `DECISION_FIELD_TYPE_MISMATCH` avec un message ciblé
    /// quand la forme n'est pas la bonne, plutôt que la générique
    /// `DECISION_MISSING_FIELD` que serde produirait.
    #[serde(default)]
    deviates_from: Option<serde_norway::Value>,
}

/// L'`id` peut être `0001` (entier YAML avec zéro de tête) ou `"my-decision"`
/// (chaîne). On normalise en `String`, en préservant l'aspect « 0001 » à
/// quatre chiffres pour un entier qui commence par des zéros.
#[derive(Debug)]
enum IdValue {
    Str(String),
    Int(i64),
}

impl IdValue {
    fn into_string(self) -> String {
        match self {
            Self::Str(s) => s,
            // Padding à 4 chiffres — la convention documentaire des ADR du
            // dépôt (`0001`… `0006`). Un id > 9999 déborde et s'écrit tel
            // quel, pas de troncature.
            Self::Int(n) if (0..10_000).contains(&n) => format!("{n:04}"),
            Self::Int(n) => n.to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for IdValue {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, Visitor};
        use std::fmt;

        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = IdValue;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("un identifiant sous forme d'entier ou de chaîne")
            }
            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(IdValue::Str(v.to_string()))
            }
            fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(IdValue::Str(v))
            }
            fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(IdValue::Int(v))
            }
            fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(IdValue::Int(v as i64))
            }
        }
        d.deserialize_any(V)
    }
}

#[derive(Debug, Default)]
struct SupersedesValue(Vec<String>);

impl SupersedesValue {
    fn into_vec(self) -> Vec<String> {
        self.0
    }
}

impl<'de> Deserialize<'de> for SupersedesValue {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, SeqAccess, Visitor};
        use std::fmt;

        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = SupersedesValue;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("un identifiant unique ou un tableau d'identifiants")
            }
            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(SupersedesValue(vec![v.to_string()]))
            }
            fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(SupersedesValue(vec![IdValue::Int(v).into_string()]))
            }
            fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(SupersedesValue(vec![IdValue::Int(v as i64).into_string()]))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut out = Vec::new();
                while let Some(id) = seq.next_element::<IdValue>()? {
                    out.push(id.into_string());
                }
                Ok(SupersedesValue(out))
            }
        }
        d.deserialize_any(V)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_adr_bien_forme() {
        let source = "---\nid: 0007\ntitle: \"Le titre\"\nstatus: accepted\ndate: 2026-09-08\ntags: [architecture]\n---\n\n## Contexte\n\nUn peu de texte.\n\n## Décision\n\nUne autre.\n";
        let parsed = parse_decision(source);
        assert!(!parsed.has_errors(), "findings : {:?}", parsed.findings);
        let d = parsed.value.expect("décision attendue");
        assert_eq!(d.id, "0007");
        assert_eq!(d.title, "Le titre");
        assert_eq!(d.status, DecisionStatus::Accepted);
        assert_eq!(d.date, "2026-09-08");
        assert_eq!(d.tags, ["architecture"]);
        assert!(d.supersedes.is_empty());
        assert_eq!(d.sections.len(), 2);
        assert_eq!(d.sections[0].name, "Contexte");
        assert!(d.sections[0].body.contains("Un peu"));
    }

    #[test]
    fn parse_sans_frontmatter_signale() {
        let source = "# Pas de frontmatter\n\ncontenu\n";
        let parsed = parse_decision(source);
        assert!(parsed.value.is_none());
        assert!(parsed
            .findings
            .iter()
            .any(|f| f.code == codes::DECISION_MISSING_FRONTMATTER));
    }

    #[test]
    fn parse_sans_titre_signale() {
        // Champ obligatoire manquant : serde émet une erreur, on la reporte
        // en `decision_missing_field`.
        let source = "---\nid: 0001\nstatus: accepted\ndate: 2026-09-08\n---\n\n## X\n";
        let parsed = parse_decision(source);
        assert!(parsed.value.is_none());
        assert!(parsed
            .findings
            .iter()
            .any(|f| f.code == codes::DECISION_MISSING_FIELD));
    }

    #[test]
    fn status_inconnu_est_signale() {
        let source = "---\nid: 0001\ntitle: T\nstatus: pending\ndate: 2026-09-08\n---\n";
        let parsed = parse_decision(source);
        let d = parsed.value.expect("décision quand même construite");
        assert_eq!(d.status, DecisionStatus::Unknown("pending".into()));
        let f = parsed
            .findings
            .iter()
            .find(|f| f.code == codes::DECISION_UNKNOWN_STATUS)
            .expect("finding attendu");
        assert!(f.message.contains("pending"), "{}", f.message);
        assert!(f.message.contains("accepted"), "{}", f.message);
    }

    #[test]
    fn supersedes_accepte_id_unique_et_tableau() {
        let source_single =
            "---\nid: 0007\ntitle: T\nstatus: accepted\ndate: 2026-09-08\nsupersedes: 0003\n---\n";
        let d = parse_decision(source_single).value.unwrap();
        assert_eq!(d.supersedes, ["0003"]);

        let source_list = "---\nid: 0008\ntitle: T\nstatus: accepted\ndate: 2026-09-08\nsupersedes: [0003, 0004]\n---\n";
        let d = parse_decision(source_list).value.unwrap();
        assert_eq!(d.supersedes, ["0003", "0004"]);
    }

    #[test]
    fn id_string_est_accepte_tel_quel() {
        let source =
            "---\nid: my-decision\ntitle: T\nstatus: accepted\ndate: 2026-09-08\n---\n";
        let d = parse_decision(source).value.unwrap();
        assert_eq!(d.id, "my-decision");
    }

    #[test]
    fn deviates_from_liste_de_chaines_est_reconnu() {
        let source = "---\nid: 0007\ntitle: T\nstatus: accepted\ndate: 2026-09-08\ndeviates_from: [\"path:~/partage/0100\"]\n---\n";
        let parsed = parse_decision(source);
        assert!(!parsed.has_errors(), "findings : {:?}", parsed.findings);
        let d = parsed.value.unwrap();
        assert_eq!(d.deviates_from, ["path:~/partage/0100"]);
    }

    #[test]
    fn deviates_from_a_plusieurs_entrees() {
        let source = "---\nid: 0007\ntitle: T\nstatus: accepted\ndate: 2026-09-08\ndeviates_from: [\"path:~/a/0001\", \"git:acme/shared/0002\"]\n---\n";
        let d = parse_decision(source).value.unwrap();
        assert_eq!(d.deviates_from.len(), 2);
    }

    #[test]
    fn deviates_from_absent_donne_liste_vide() {
        let source = "---\nid: 0001\ntitle: T\nstatus: accepted\ndate: 2026-09-08\n---\n";
        let d = parse_decision(source).value.unwrap();
        assert!(d.deviates_from.is_empty());
    }

    #[test]
    fn deviates_from_string_au_lieu_de_liste_est_signale() {
        // Chaîne unique là où une liste est attendue → finding spécifique.
        let source = "---\nid: 0007\ntitle: T\nstatus: accepted\ndate: 2026-09-08\ndeviates_from: \"pas-une-liste\"\n---\n";
        let parsed = parse_decision(source);
        // La décision est quand même construite (l'erreur porte sur un
        // champ optionnel), mais le finding est là.
        assert!(parsed
            .findings
            .iter()
            .any(|f| f.code == codes::DECISION_FIELD_TYPE_MISMATCH));
    }

    #[test]
    fn deviates_from_entree_non_string_est_signalee() {
        // Une liste dont l'un des éléments est un entier → finding
        // spécifique sur l'entrée, les autres continuent d'être exposées.
        let source = "---\nid: 0007\ntitle: T\nstatus: accepted\ndate: 2026-09-08\ndeviates_from: [\"path:~/a/0001\", 42]\n---\n";
        let parsed = parse_decision(source);
        let d = parsed.value.unwrap();
        assert_eq!(d.deviates_from, ["path:~/a/0001"]);
        assert!(parsed
            .findings
            .iter()
            .any(|f| f.code == codes::DECISION_FIELD_TYPE_MISMATCH));
    }

    #[test]
    fn cle_inconnue_dans_frontmatter_est_signalee() {
        // deny_unknown_fields — cohérent avec ChangeMetadata et ProjectConfig.
        let source = "---\nid: 0001\ntitle: T\nstatus: accepted\ndate: 2026-09-08\nauteur: X\n---\n";
        let parsed = parse_decision(source);
        assert!(parsed
            .findings
            .iter()
            .any(|f| f.code == codes::DECISION_MISSING_FIELD));
    }
}
