//! Codes stables des `Finding` émis par le parseur.
//!
//! Nommer chaque code une fois — plutôt que de l'écrire en littéral aux
//! sites d'émission — sert deux objectifs :
//!
//! 1. Le module `validate` peut itérer cette liste pour vérifier que ses
//!    propres codes ne créent aucun doublon.
//! 2. Un renommage force à passer par cette constante et sa documentation,
//!    en rappelant que le code est un contrat public.

/// La spec principale ne contient pas de `## Purpose`.
pub const SPEC_PURPOSE_MISSING: &str = "spec_purpose_missing";

/// Un `### Requirement:` apparaît hors de la section `## Requirements`.
pub const REQUIREMENT_OUTSIDE_SECTION: &str = "requirement_outside_section";

/// Un en-tête de delta (`## ADDED Requirements`, etc.) apparaît dans une spec
/// principale.
pub const DELTA_HEADER_IN_MAIN_SPEC: &str = "delta_header_in_main_spec";

/// Un scénario est écrit avec trois dièses (`### Scenario:`) au lieu de
/// quatre.
pub const SCENARIO_WRONG_HEADING_LEVEL: &str = "scenario_wrong_heading_level";

/// Deux exigences de même nom dans la même section d'un delta.
pub const DUPLICATE_REQUIREMENT: &str = "duplicate_requirement";

// ─────────────────────────── codes du parseur de décisions ───────────────────────────

/// Le fichier de décision n'a pas de frontmatter YAML délimité par `---`.
pub const DECISION_MISSING_FRONTMATTER: &str = "decision_missing_frontmatter";

/// Le frontmatter n'a pas un champ obligatoire (`id`, `title`, `status`,
/// `date`), ou porte une clé inconnue.
pub const DECISION_MISSING_FIELD: &str = "decision_missing_field";

/// Le `status` porte une valeur qui n'est pas l'une des cinq reconnues.
pub const DECISION_UNKNOWN_STATUS: &str = "decision_unknown_status";

// Les codes ci-dessous sont émis par l'index côté engine, pas par le
// parseur. On les enregistre ici pour rester à un seul point d'unicité
// contrôlé par `validate::codes_de_findings_sont_uniques`.

/// Un `supersedes` pointe vers un identifiant absent de l'index.
pub const DECISION_SUPERSEDES_UNKNOWN: &str = "decision_supersedes_unknown";

/// Un même identifiant apparaît dans le projet et dans une source héritée.
pub const DECISION_ID_COLLISION: &str = "decision_id_collision";

/// Une chaîne de supersession forme un cycle — aucune décision du cycle
/// n'entre en vigueur.
pub const DECISION_SUPERSESSION_CYCLE: &str = "decision_supersession_cycle";

/// Un champ typé du frontmatter (`deviates_from`, `tags`…) porte une valeur
/// qui n'est pas de la bonne forme (par exemple une chaîne au lieu d'une
/// liste).
pub const DECISION_FIELD_TYPE_MISMATCH: &str = "decision_field_type_mismatch";

/// Un `deviates_from` d'un ADR local pointe vers un `qualified-id` qui
/// n'est plus présent dans l'index (source retirée, SHA déplacé, id changé).
pub const DECISION_DANGLING_DEVIATION: &str = "decision_dangling_deviation";

/// Deux ADR locaux `accepted` référencent la même cible dans leur
/// `deviates_from` — l'outil ne tranche pas et refuse.
pub const DECISION_CONFLICTING_DEVIATIONS: &str = "decision_conflicting_deviations";

/// Tous les codes du parseur, dans l'ordre de leur première apparition.
///
/// Le module `validate` en fait un contrôle d'unicité au démarrage des tests ;
/// une nouvelle règle dont le code entrerait en collision est refusée avant
/// même d'être écrite.
pub const ALL: &[&str] = &[
    SPEC_PURPOSE_MISSING,
    REQUIREMENT_OUTSIDE_SECTION,
    DELTA_HEADER_IN_MAIN_SPEC,
    SCENARIO_WRONG_HEADING_LEVEL,
    DUPLICATE_REQUIREMENT,
    DECISION_MISSING_FRONTMATTER,
    DECISION_MISSING_FIELD,
    DECISION_UNKNOWN_STATUS,
    DECISION_SUPERSEDES_UNKNOWN,
    DECISION_ID_COLLISION,
    DECISION_SUPERSESSION_CYCLE,
    DECISION_FIELD_TYPE_MISMATCH,
    DECISION_DANGLING_DEVIATION,
    DECISION_CONFLICTING_DEVIATIONS,
];
