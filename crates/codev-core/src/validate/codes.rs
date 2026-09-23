//! Codes stables émis par les règles de `validate`.
//!
//! Séparé des codes du parseur pour rendre la provenance évidente : un
//! `code` en `spec_purpose_missing` vient du parseur, un `code` en
//! `requirement_no_shall` vient d'ici.

/// Une exigence dont la description ne contient ni `SHALL` ni `MUST`.
pub const REQUIREMENT_NO_SHALL: &str = "requirement_no_shall";

/// Une exigence sans aucun scénario.
pub const REQUIREMENT_NO_SCENARIO: &str = "requirement_no_scenario";

/// Une spec principale sans aucune exigence extractible.
pub const SPEC_NO_REQUIREMENT: &str = "spec_no_requirement";

/// Une même exigence figure dans deux sections `ADDED`/`MODIFIED`/`REMOVED`
/// d'un même delta.
pub const CROSS_SECTION_CONFLICT: &str = "cross_section_conflict";

/// Un `RENAMED.TO` coïncide avec un `ADDED` de même nom dans le même delta.
pub const RENAME_TARGET_COLLISION: &str = "rename_target_collision";

/// Un `MODIFIED` référence un `RENAMED.FROM` — le nouveau nom doit être
/// utilisé à la place.
pub const MODIFIED_USES_OLD_NAME: &str = "modified_uses_old_name";

/// Un change sans aucun delta et sans `skip_specs: true` en métadonnée.
pub const ZERO_DELTA_WITHOUT_MARKER: &str = "zero_delta_without_marker";

/// `skip_specs: true` déclaré, mais des fichiers de delta existent.
pub const SKIP_SPECS_CONFLICT: &str = "skip_specs_conflict";
