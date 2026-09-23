//! Plans de création et de supersession d'ADR — cœur de coquille : les
//! fonctions calculent un `Plan`, la coquille CLI l'exécute.

use std::path::PathBuf;

use codev_core::decisions::seal::{self, SealError, SealFile};
use codev_core::decisions::DecisionStatus;
use codev_core::{Layout, Plan, WriteMode};

use crate::decisions::{DecisionIndex, Origin};
use crate::ports::FileSystem;

/// Lit et parse `seal.yaml` s'il existe ; sinon retourne un sceau vide.
///
/// C'est le point d'accès unique à `seal.yaml` côté coquille — utilisé
/// avant d'appeler `plan_new`, `plan_supersede` et `plan_seal`, et par
/// `validate` pour vérifier les hashes.
pub fn read_seal_file(
    fs: &dyn FileSystem,
    layout: &Layout,
) -> Result<SealFile, ActionError> {
    let path = layout.decisions_seal_file();
    if !fs.exists(&path) {
        return Ok(SealFile::empty());
    }
    let source = fs
        .read_to_string(&path)
        .map_err(|e| ActionError::Seal(SealError::Invalid(e.to_string())))?;
    seal::parse_seal_file(&source).map_err(ActionError::from)
}

const DECISION_TEMPLATE: &str = include_str!("../../../assets/templates/decision.md");

/// Ce que peut refuser une action.
///
/// Les codes portent la même règle que le reste de l'outil : stables et
/// testables côté agent, messages libres de reformulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionError {
    EmptyTitle,
    UnknownDecisionId { id: String },
    CannotSupersedeInherited { qualified_id: String },
    CannotSealInherited { qualified_id: String },
    /// La cible passée à `decision deviate` est une décision locale — le
    /// geste propre pour ça, c'est `decision supersede`.
    CannotDeviateFromLocal { qualified_id: String },
    /// La cible d'un `decision promote` pointe vers un change archivé —
    /// un design archivé est de l'histoire, on ne le modifie pas.
    CannotPromoteFromArchived { change: String },
    /// Le change n'a pas de `design.md` — impossible de promouvoir.
    DesignMissing { change: String },
    /// Aucun bloc `### Décision : <titre>` ne correspond.
    DecisionHeadingNotFound { title: String },
    /// Plusieurs blocs partagent le même titre — l'utilisateur précise.
    AmbiguousDecisionHeading { title: String, lines: Vec<u32> },
    AmbiguousDecisionId { id: String, candidates: Vec<String> },
    /// Un sceau déjà présent, corps différent du hash enregistré, `--force`
    /// non demandé. Le code stable est `seal_conflict`.
    SealConflict { id: String },
    /// Une erreur du module `seal` — code stable dérivé.
    Seal(SealError),
}

impl ActionError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::EmptyTitle => "empty_title",
            Self::UnknownDecisionId { .. } => "unknown_decision_id",
            Self::CannotSupersedeInherited { .. } => "cannot_supersede_inherited",
            Self::CannotSealInherited { .. } => "cannot_seal_inherited",
            Self::CannotDeviateFromLocal { .. } => "cannot_deviate_from_local",
            Self::CannotPromoteFromArchived { .. } => "cannot_promote_from_archived",
            Self::DesignMissing { .. } => "design_missing",
            Self::DecisionHeadingNotFound { .. } => "decision_heading_not_found",
            Self::AmbiguousDecisionHeading { .. } => "ambiguous_decision_heading",
            Self::AmbiguousDecisionId { .. } => "ambiguous_decision_id",
            Self::SealConflict { .. } => "seal_conflict",
            Self::Seal(err) => match err {
                SealError::MissingFrontmatterCloser => "seal_body_unreadable",
                SealError::UnsupportedVersion { .. } => "seal_version_unsupported",
                SealError::Invalid(_) => "seal_invalid",
                SealError::AlreadySealed { .. } => "seal_conflict",
                SealError::Unknown { .. } => "unknown_decision_id",
            },
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::EmptyTitle => {
                "un titre non vide est requis pour créer une décision".into()
            }
            Self::UnknownDecisionId { id } => format!(
                "aucune décision d'identifiant « {id} » dans le projet ; \
                 `codev decision list` montre celles qui existent"
            ),
            Self::CannotSupersedeInherited { qualified_id } => format!(
                "la décision « {qualified_id} » est héritée, donc en lecture seule ; \
                 le geste « déviation » (à venir) permettra de s'en écarter localement"
            ),
            Self::CannotSealInherited { qualified_id } => format!(
                "la décision « {qualified_id} » est héritée : le scellement \
                 relève du projet source, pas du consommateur"
            ),
            Self::CannotDeviateFromLocal { qualified_id } => format!(
                "la décision « {qualified_id} » est locale : le geste propre \
                 pour la remplacer est `codev decision supersede`, pas \
                 `deviate` — qui est réservé aux décisions héritées"
            ),
            Self::CannotPromoteFromArchived { change } => format!(
                "le change « {change} » est archivé : un design archivé est \
                 de l'histoire, la promotion se fait avant l'archive"
            ),
            Self::DesignMissing { change } => format!(
                "le change « {change} » n'a pas de `design.md` — rien à promouvoir. \
                 Crée-le d'abord avec `codev-propose` ou en éditant à la main."
            ),
            Self::DecisionHeadingNotFound { title } => format!(
                "aucun bloc `### Décision : {title}` trouvé sous `## Décisions` \
                 du design ; vérifie le titre exact (le contrôle est sensible à \
                 la casse)"
            ),
            Self::AmbiguousDecisionHeading { title, lines } => {
                let lignes = lines
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "le titre « {title} » apparaît sur plusieurs blocs \
                     (lignes {lignes}) — édite un des titres pour rendre le \
                     choix sans ambiguïté avant de re-lancer la promotion"
                )
            }
            Self::AmbiguousDecisionId { id, candidates } => format!(
                "l'identifiant « {id} » désigne plusieurs décisions : {} — \
                 précise avec un identifiant qualifié",
                candidates.join(", ")
            ),
            Self::SealConflict { id } => format!(
                "le corps de « {id} » a changé depuis son scellement ; \
                 utilise `codev decision seal {id} --force` pour réécrire \
                 délibérément le sceau"
            ),
            Self::Seal(err) => err.to_string(),
        }
    }
}

impl From<SealError> for ActionError {
    fn from(err: SealError) -> Self {
        Self::Seal(err)
    }
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message())
    }
}

impl std::error::Error for ActionError {}

/// Le plan calculé pour `codev decision new`.
#[derive(Debug)]
pub struct CreatePlan {
    pub plan: Plan,
    pub new_id: String,
    pub new_path: PathBuf,
    /// Hash du corps du nouvel ADR — exposé pour que le CLI puisse le
    /// remonter dans son JSON de succès sans avoir à re-hasher.
    pub body_sha256: String,
}

/// Le plan calculé pour `codev decision supersede`.
#[derive(Debug)]
pub struct SupersedePlan {
    pub plan: Plan,
    pub new_id: String,
    pub new_path: PathBuf,
    pub old_qualified_id: String,
    pub old_path: PathBuf,
    /// Hash du corps du nouvel ADR — même motivation que `CreatePlan`.
    pub body_sha256: String,
}

/// Le plan calculé pour `codev decision seal`.
///
/// Deux issues côté résultat :
/// - `plan` non vide, `body_sha256` porte le nouveau hash → écriture à
///   faire côté coquille ;
/// - `plan.writes` vide → no-op silencieux (sceau déjà à jour).
#[derive(Debug)]
pub struct SealActionPlan {
    pub plan: Plan,
    pub id: String,
    pub body_sha256: String,
    pub sealed_at: String,
    pub was_noop: bool,
}

/// Le plan calculé pour `codev decision promote`.
#[derive(Debug)]
pub struct PromotePlan {
    pub plan: Plan,
    pub new_id: String,
    pub new_path: PathBuf,
    pub body_sha256: String,
    /// Le nom du change d'où vient la promotion — utile au rendu et au
    /// contrat JSON.
    pub source_change: String,
    /// Chemin du `design.md` modifié — pareillement utile.
    pub design_path: PathBuf,
}

/// Le plan calculé pour `codev decision deviate`.
#[derive(Debug)]
pub struct DeviatePlan {
    pub plan: Plan,
    pub new_id: String,
    pub new_path: PathBuf,
    /// L'identifiant qualifié qu'on écarte — normalisé (accepte un
    /// `path:...` ou `git:...` avec ou sans le `origin/` explicite tant
    /// que la résolution est sans ambiguïté).
    pub target_qualified_id: String,
    pub body_sha256: String,
}

/// Prépare la création d'un nouvel ADR local.
///
/// `existing_seal` est le contenu courant de `seal.yaml` — la coquille
/// l'a lu au préalable. Le plan produit écrit l'ADR **et** la mise à jour
/// du sceau atomiquement : soit les deux réussissent, soit rien n'est
/// écrit.
pub fn plan_new(
    index: &DecisionIndex,
    existing_seal: &SealFile,
    title: &str,
    status: DecisionStatus,
    today: &str,
    layout: &Layout,
) -> Result<CreatePlan, ActionError> {
    if title.trim().is_empty() {
        return Err(ActionError::EmptyTitle);
    }

    let next = next_local_id(index);
    let slug = slug_from_title(title);
    let filename = format!("{next}-{slug}.md");
    let new_path = layout.decisions_dir().join(&filename);

    let contents = render_new_adr(&next, title, &status, today, &[]);
    let body_sha256 = seal::body_hash(&contents)?;

    // Le sceau ne s'écrit que si l'ADR est `accepted` ou `superseded` —
    // les autres statuts (proposed, deprecated, rejected) ne portent pas
    // d'engagement d'immutabilité.
    let mut plan = Plan::new();
    plan.dir(layout.decisions_dir());
    plan.write(new_path.clone(), contents, WriteMode::CreateOnly);
    if status_needs_seal(&status) {
        let new_seal = seal::plan_seal_new(
            existing_seal,
            next.clone(),
            body_sha256.clone(),
            today.to_string(),
        )?;
        plan.write(
            layout.decisions_seal_file(),
            seal::render_seal_file(&new_seal),
            WriteMode::Overwrite,
        );
    }

    Ok(CreatePlan {
        plan,
        new_id: next,
        new_path,
        body_sha256,
    })
}

/// Prépare une supersession : réécrit le prédécesseur avec `status:
/// superseded` et crée un nouvel ADR qui le référence.
///
/// `source_content` fournit à la fonction le contenu du fichier
/// prédécesseur, sans que l'index ait à le stocker deux fois. On ne
/// paramètre pas par `&dyn FileSystem` pour rester utilisable depuis
/// d'autres consommateurs (tests, futur `--dry-run` client) sans
/// coupler à un port.
pub fn plan_supersede(
    index: &DecisionIndex,
    existing_seal: &SealFile,
    old_id: &str,
    new_title: &str,
    today: &str,
    layout: &Layout,
    source_content: impl FnOnce(&std::path::Path) -> std::io::Result<String>,
) -> Result<SupersedePlan, ActionError> {
    if new_title.trim().is_empty() {
        return Err(ActionError::EmptyTitle);
    }

    // Résolution de l'ancien : deux cas selon le format du `old_id`.
    let (old_index, is_qualified) = resolve_old_entry(index, old_id)?;
    let old_entry = &index.entries[old_index];

    // Une décision héritée reste en lecture seule.
    if old_entry.qualified_id.origin != Origin::Project {
        return Err(ActionError::CannotSupersedeInherited {
            qualified_id: old_entry.qualified_id.as_str(),
        });
    }
    let _ = is_qualified; // exposé plus tard si besoin

    // Nouveau frontmatter du prédécesseur — mêmes champs sauf `status`.
    let old_source = source_content(&old_entry.path).map_err(|e| {
        // Une erreur de lecture est une erreur d'exécution, pas un refus
        // logique — on l'expose comme `UnknownDecisionId` pour ne pas
        // fuiter l'io. En pratique c'est très rare (le fichier vient
        // d'être indexé) ; les futurs consommateurs sensibles pourront
        // pré-lire pour distinguer.
        let _ = e;
        ActionError::UnknownDecisionId {
            id: old_entry.decision.id.clone(),
        }
    })?;
    let new_old_contents = rewrite_frontmatter_status(
        &old_source,
        &old_entry.decision,
        DecisionStatus::Superseded,
    );

    // Nouveau numéro pour la décision qui supersede.
    let next = next_local_id(index);
    let new_slug = slug_from_title(new_title);
    let new_filename = format!("{next}-{new_slug}.md");
    let new_path = layout.decisions_dir().join(&new_filename);

    let old_id_ref = old_entry.decision.id.clone();
    let new_adr = render_new_adr(
        &next,
        new_title,
        &DecisionStatus::Accepted,
        today,
        &[old_id_ref],
    );

    // Hash et sceau pour le nouvel ADR uniquement — le corps de l'ancien
    // reste bit-identique (voir test dédié), donc son sceau reste valide
    // sans manipulation.
    let body_sha256 = seal::body_hash(&new_adr)?;
    let new_seal = seal::plan_seal_new(
        existing_seal,
        next.clone(),
        body_sha256.clone(),
        today.to_string(),
    )?;

    let mut plan = Plan::new();
    plan.dir(layout.decisions_dir());
    plan.write(new_path.clone(), new_adr, WriteMode::CreateOnly);
    plan.write(old_entry.path.clone(), new_old_contents, WriteMode::Overwrite);
    plan.write(
        layout.decisions_seal_file(),
        seal::render_seal_file(&new_seal),
        WriteMode::Overwrite,
    );

    Ok(SupersedePlan {
        plan,
        new_id: next,
        new_path,
        old_qualified_id: old_entry.qualified_id.as_str(),
        old_path: old_entry.path.clone(),
        body_sha256,
    })
}

/// Prépare un sceau — cas de la commande `codev decision seal <id>`.
///
/// Trois issues selon l'état :
/// - ADR non scellé → ajoute une entrée (`was_noop = false`).
/// - ADR scellé, corps inchangé → no-op silencieux (`was_noop = true`,
///   `plan.writes` vide).
/// - ADR scellé, corps changé, `force = false` → refuse avec
///   `SealConflict`.
/// - ADR scellé, corps changé, `force = true` → réécrit l'entrée.
///
/// `adr_source` fournit à la fonction le contenu courant du fichier ADR ;
/// même motif que `plan_supersede` — la coquille lit, la fonction pure
/// calcule.
pub fn plan_seal(
    index: &DecisionIndex,
    existing_seal: &SealFile,
    id: &str,
    force: bool,
    today: &str,
    layout: &Layout,
    adr_source: impl FnOnce(&std::path::Path) -> std::io::Result<String>,
) -> Result<SealActionPlan, ActionError> {
    // Résolution : même mécanique que supersede — mais on refuse une
    // décision héritée avec un code dédié.
    let (idx, _) = resolve_old_entry(index, id)?;
    let entry = &index.entries[idx];
    if entry.qualified_id.origin != Origin::Project {
        return Err(ActionError::CannotSealInherited {
            qualified_id: entry.qualified_id.as_str(),
        });
    }

    let local_id = entry.decision.id.clone();
    let source = adr_source(&entry.path).map_err(|_| ActionError::UnknownDecisionId {
        id: local_id.clone(),
    })?;
    let body_sha256 = seal::body_hash(&source)?;

    let already = existing_seal.find(&local_id);
    let mut plan = Plan::new();

    match already {
        None => {
            // Cas 1 : pas encore scellé — insertion neuve.
            let new_seal = seal::plan_seal_new(
                existing_seal,
                local_id.clone(),
                body_sha256.clone(),
                today.to_string(),
            )?;
            plan.dir(layout.decisions_dir());
            plan.write(
                layout.decisions_seal_file(),
                seal::render_seal_file(&new_seal),
                WriteMode::Overwrite,
            );
            Ok(SealActionPlan {
                plan,
                id: local_id,
                body_sha256,
                sealed_at: today.to_string(),
                was_noop: false,
            })
        }
        Some(existing_entry) if existing_entry.body_sha256 == body_sha256 => {
            // Cas 2 : no-op — le sceau est déjà à jour.
            Ok(SealActionPlan {
                plan,
                id: local_id,
                body_sha256,
                sealed_at: existing_entry.sealed_at.clone(),
                was_noop: true,
            })
        }
        Some(_) if !force => {
            // Cas 3 : conflit sans force.
            Err(ActionError::SealConflict { id: local_id })
        }
        Some(_) => {
            // Cas 4 : force → réécriture.
            let new_seal = seal::plan_seal_force(
                existing_seal,
                local_id.clone(),
                body_sha256.clone(),
                today.to_string(),
            )?;
            plan.dir(layout.decisions_dir());
            plan.write(
                layout.decisions_seal_file(),
                seal::render_seal_file(&new_seal),
                WriteMode::Overwrite,
            );
            Ok(SealActionPlan {
                plan,
                id: local_id,
                body_sha256,
                sealed_at: today.to_string(),
                was_noop: false,
            })
        }
    }
}

/// Un statut porte-t-il un engagement d'immutabilité ?
fn status_needs_seal(status: &DecisionStatus) -> bool {
    matches!(status, DecisionStatus::Accepted | DecisionStatus::Superseded)
}

/// La ligne de citation qui remplace le corps d'un bloc `### Décision :`
/// après promotion. Emit d'ici pour rester testable sans I/O.
fn build_promote_reference(new_id: &str, new_slug: &str) -> String {
    format!(
        "\n> Promue en ADR **{new_id}** — voir `_codev/decisions/{new_id}-{new_slug}.md`.\n\n"
    )
}

/// Prépare la promotion d'un bloc `### Décision : <titre>` d'un
/// `design.md` en ADR local. Réutilise le rendu de `plan_new` avec un
/// corps précalculé — le sceau (K3) est attaché au plan.
///
/// `design_source` : contenu courant du design (la coquille l'a lu).
/// `design_path` : chemin du design côté disque, pour la write du plan.
///
/// Refus explicites — voir `ActionError` : `EmptyTitle`, `DesignMissing`,
/// `DecisionHeadingNotFound`, `AmbiguousDecisionHeading`.
#[allow(clippy::too_many_arguments)]
pub fn plan_promote(
    index: &DecisionIndex,
    existing_seal: &SealFile,
    change_name: &str,
    heading: &str,
    design_source: &str,
    design_path: PathBuf,
    today: &str,
    layout: &Layout,
) -> Result<PromotePlan, ActionError> {
    if heading.trim().is_empty() {
        return Err(ActionError::EmptyTitle);
    }

    let blocks = crate::design::extract_decision_blocks(design_source);
    let idx = crate::design::find_decision_block(&blocks, heading).map_err(|err| match err {
        crate::design::LookupError::NotFound => ActionError::DecisionHeadingNotFound {
            title: heading.to_string(),
        },
        crate::design::LookupError::Ambiguous { lines } => ActionError::AmbiguousDecisionHeading {
            title: heading.to_string(),
            lines,
        },
    })?;
    let block = &blocks[idx];

    // ─── Construction de l'ADR ───
    let next = next_local_id(index);
    let slug = slug_from_title(&block.title);
    let filename = format!("{next}-{slug}.md");
    let new_path = layout.decisions_dir().join(&filename);

    // Rendu du corps ADR : template avec placeholders pour Contexte,
    // Conséquences, Alternatives ; le bloc verbatim va sous ## Décision.
    let contents = render_promoted_adr(&next, &block.title, today, &block.body, change_name);
    let body_sha256 = seal::body_hash(&contents)?;
    let new_seal = seal::plan_seal_new(
        existing_seal,
        next.clone(),
        body_sha256.clone(),
        today.to_string(),
    )?;

    // ─── Substitution du bloc dans le design ───
    // On garde la ligne de titre `### Décision : <titre>` telle qu'elle
    // était, on remplace seulement le corps par la citation. Le titre
    // se termine au premier `\n` après byte_range.start.
    let title_line_end = design_source[block.byte_range.start..]
        .find('\n')
        .map(|off| block.byte_range.start + off + 1)
        .unwrap_or(design_source.len());
    let reference = build_promote_reference(&next, &slug);

    let mut new_design = String::with_capacity(design_source.len());
    new_design.push_str(&design_source[..title_line_end]);
    new_design.push_str(&reference);
    new_design.push_str(&design_source[block.byte_range.end..]);

    // ─── Plan atomique ───
    let mut plan = Plan::new();
    plan.dir(layout.decisions_dir());
    plan.write(new_path.clone(), contents, WriteMode::CreateOnly);
    plan.write(
        layout.decisions_seal_file(),
        seal::render_seal_file(&new_seal),
        WriteMode::Overwrite,
    );
    plan.write(design_path.clone(), new_design, WriteMode::Overwrite);

    Ok(PromotePlan {
        plan,
        new_id: next,
        new_path,
        body_sha256,
        source_change: change_name.to_string(),
        design_path,
    })
}

/// Rend le contenu complet d'un ADR issu d'une promotion depuis un
/// `design.md` — le corps du bloc va sous `## Décision`, les autres
/// sections gardent leur placeholder pour ventilation manuelle.
fn render_promoted_adr(
    id: &str,
    title: &str,
    today: &str,
    verbatim_body: &str,
    source_change: &str,
) -> String {
    let frontmatter = render_frontmatter(
        id,
        title,
        &DecisionStatus::Accepted,
        today,
        &[],
        &[],
        &[],
    );
    // Corps propre au verbatim : on retire les blancs de tête pour
    // éviter une ligne vide inutile en début de section, et on garantit
    // un `\n` final.
    let trimmed_body = verbatim_body.trim_start_matches('\n');
    let body_with_newline = if trimmed_body.ends_with('\n') {
        trimmed_body.to_string()
    } else {
        format!("{trimmed_body}\n")
    };

    format!(
        "{frontmatter}\n\n\
         <!-- ADR promu depuis _codev/changes/{source_change}/design.md.\n     \
         Ventile le corps ci-dessous entre Contexte, Décision, Conséquences\n     \
         et Alternatives écartées avant d'archiver le change. -->\n\n\
         ## Contexte\n\n\
         <!-- Le problème ou la situation qui appelle une décision. Court : deux ou\n     \
         trois phrases suffisent. -->\n\n\
         ## Décision\n\n\
         {body_with_newline}\n\
         ## Conséquences\n\n\
         <!-- Ce que cette décision impose ou permet — bonnes et mauvaises. -->\n\n\
         ## Alternatives écartées\n\n\
         <!-- Ce qu'on aurait pu faire, et pourquoi on a choisi autre chose. -->\n"
    )
}

/// Prépare la dérive locale d'une décision héritée.
///
/// Crée un ADR local `accepted` portant `deviates_from: ["<qualified>"]`
/// et scelle son corps — comme `plan_new` mais avec la référence à la
/// cible.
///
/// Refus explicites :
/// - `EmptyTitle` si `new_title` est vide ;
/// - `CannotDeviateFromLocal` si `target` est une décision `projet/…` ;
/// - `UnknownDecisionId` si `target` n'est pas indexée ;
/// - `AmbiguousDecisionId` si deux sources exposent le même qualified.
pub fn plan_deviate(
    index: &DecisionIndex,
    existing_seal: &SealFile,
    target: &str,
    new_title: &str,
    today: &str,
    layout: &Layout,
) -> Result<DeviatePlan, ActionError> {
    if new_title.trim().is_empty() {
        return Err(ActionError::EmptyTitle);
    }

    // Résolution de la cible — même mécanique que supersede et seal.
    let (idx, _) = resolve_old_entry(index, target)?;
    let entry = &index.entries[idx];
    if entry.qualified_id.origin == Origin::Project {
        return Err(ActionError::CannotDeviateFromLocal {
            qualified_id: entry.qualified_id.as_str(),
        });
    }

    let qualified = entry.qualified_id.as_str();

    // Nouveau numéro local, rendu de l'ADR, hash, sceau.
    let next = next_local_id(index);
    let slug = slug_from_title(new_title);
    let filename = format!("{next}-{slug}.md");
    let new_path = layout.decisions_dir().join(&filename);

    let contents = render_new_deviate_adr(&next, new_title, today, std::slice::from_ref(&qualified));
    let body_sha256 = seal::body_hash(&contents)?;
    let new_seal = seal::plan_seal_new(
        existing_seal,
        next.clone(),
        body_sha256.clone(),
        today.to_string(),
    )?;

    let mut plan = Plan::new();
    plan.dir(layout.decisions_dir());
    plan.write(new_path.clone(), contents, WriteMode::CreateOnly);
    plan.write(
        layout.decisions_seal_file(),
        seal::render_seal_file(&new_seal),
        WriteMode::Overwrite,
    );

    Ok(DeviatePlan {
        plan,
        new_id: next,
        new_path,
        target_qualified_id: qualified,
        body_sha256,
    })
}

// ─────────────────────────── résolution d'identifiant ───────────────────────────

/// Résout un identifiant `old_id` en une entrée de l'index.
///
/// Deux formes acceptées :
/// - `<id>` court (par exemple `0007`) — sans ambiguïté sur l'origine
/// - `<origin>/<id>` qualifié (par exemple `path:~/partage/0100`)
pub fn resolve_old_entry(
    index: &DecisionIndex,
    old_id: &str,
) -> Result<(usize, bool), ActionError> {
    // Forme qualifiée ?
    if let Some((origin_raw, id)) = split_qualified(old_id) {
        let matching: Vec<usize> = index
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                e.qualified_id.origin.as_str() == origin_raw
                    && e.decision.id == id
            })
            .map(|(i, _)| i)
            .collect();
        return match matching.len() {
            1 => Ok((matching[0], true)),
            _ => Err(ActionError::UnknownDecisionId {
                id: old_id.to_string(),
            }),
        };
    }

    // Forme courte : recherche par `decision.id`.
    let matching: Vec<usize> = index
        .entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.decision.id == old_id)
        .map(|(i, _)| i)
        .collect();
    match matching.len() {
        0 => Err(ActionError::UnknownDecisionId {
            id: old_id.to_string(),
        }),
        1 => Ok((matching[0], false)),
        _ => Err(ActionError::AmbiguousDecisionId {
            id: old_id.to_string(),
            candidates: matching
                .iter()
                .map(|&i| index.entries[i].qualified_id.as_str())
                .collect(),
        }),
    }
}

fn split_qualified(raw: &str) -> Option<(String, String)> {
    // Un identifiant qualifié se termine par `/<id>` — l'origine peut
    // contenir des `/` (le `path:~/partage/xxx` en a). On prend donc le
    // dernier `/` comme séparateur.
    let idx = raw.rfind('/')?;
    let origin_raw = &raw[..idx];
    let id = &raw[idx + 1..];
    if origin_raw.is_empty() || id.is_empty() {
        return None;
    }
    // Une origine qualifiée commence toujours par `projet` ou `path:` ou
    // `git:` — sinon c'est un id court qui contient un `/`, très
    // improbable, mais on préfère ne pas le confondre.
    if origin_raw != "projet"
        && !origin_raw.starts_with("path:")
        && !origin_raw.starts_with("git:")
    {
        return None;
    }
    Some((origin_raw.to_string(), id.to_string()))
}

// ─────────────────────────── numérotation & slug ───────────────────────────

/// `NNNN` — un de plus que le plus grand `id` numérique connu **côté projet
/// uniquement**. Les décisions héritées vivent dans leur propre espace ; on
/// ne s'y intercale pas.
fn next_local_id(index: &DecisionIndex) -> String {
    let max_local = index
        .entries
        .iter()
        .filter(|e| e.qualified_id.origin == Origin::Project)
        .filter_map(|e| e.decision.id.parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    format!("{:04}", max_local + 1)
}

/// Kebab-case ASCII à partir d'un titre libre. Un slug vide après
/// normalisation devient `decision` — fallback documenté du design.
pub fn slug_from_title(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut previous_dash = true;
    for c in title.chars() {
        let mapped = match c {
            'A'..='Z' => Some(c.to_ascii_lowercase()),
            'a'..='z' | '0'..='9' => Some(c),
            _ if c.is_ascii_whitespace() || matches!(c, '-' | '_') => Some('-'),
            _ => None,
        };
        if let Some(ch) = mapped {
            if ch == '-' {
                if !previous_dash {
                    out.push('-');
                    previous_dash = true;
                }
            } else {
                out.push(ch);
                previous_dash = false;
            }
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "decision".to_string()
    } else {
        trimmed
    }
}

// ─────────────────────────── rendu ADR ───────────────────────────

fn render_new_adr(
    id: &str,
    title: &str,
    status: &DecisionStatus,
    today: &str,
    supersedes: &[String],
) -> String {
    let frontmatter = render_frontmatter(id, title, status, today, supersedes, &[], &[]);
    DECISION_TEMPLATE.replace("{{FRONTMATTER}}", &frontmatter)
}

/// Rendu d'un ADR de dérive — même corps modèle, frontmatter avec
/// `deviates_from`.
fn render_new_deviate_adr(
    id: &str,
    title: &str,
    today: &str,
    deviates_from: &[String],
) -> String {
    let frontmatter = render_frontmatter(
        id,
        title,
        &DecisionStatus::Accepted,
        today,
        &[],
        &[],
        deviates_from,
    );
    DECISION_TEMPLATE.replace("{{FRONTMATTER}}", &frontmatter)
}

fn render_frontmatter(
    id: &str,
    title: &str,
    status: &DecisionStatus,
    date: &str,
    supersedes: &[String],
    tags: &[String],
    deviates_from: &[String],
) -> String {
    let mut out = String::from("---\n");
    out.push_str(&format!("id: \"{id}\"\n"));
    out.push_str(&format!("title: {}\n", yaml_scalar(title)));
    out.push_str(&format!("status: {}\n", status.as_str()));
    out.push_str(&format!("date: {date}\n"));
    if !tags.is_empty() {
        out.push_str(&format!(
            "tags: [{}]\n",
            tags.iter()
                .map(|t| yaml_scalar(t))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !supersedes.is_empty() {
        out.push_str(&format!(
            "supersedes: [{}]\n",
            supersedes
                .iter()
                .map(|s| format!("\"{s}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !deviates_from.is_empty() {
        out.push_str(&format!(
            "deviates_from: [{}]\n",
            deviates_from
                .iter()
                .map(|s| format!("\"{s}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    out.push_str("---");
    out
}

/// Cite un scalaire YAML entre guillemets ; échappe les caractères
/// problématiques minimaux.
fn yaml_scalar(raw: &str) -> String {
    let echappe = raw.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{echappe}\"")
}

/// Réécrit le contenu d'un ADR existant en changeant uniquement le `status`
/// du frontmatter. Le corps — tout ce qui suit le deuxième `---` — est
/// préservé au caractère près.
pub fn rewrite_frontmatter_status(
    source: &str,
    decision: &codev_core::decisions::Decision,
    new_status: DecisionStatus,
) -> String {
    let corps = &source[decision.frontmatter_span.byte_range.end..];
    let frontmatter = render_frontmatter(
        &decision.id,
        &decision.title,
        &new_status,
        &decision.date,
        &decision.supersedes,
        &decision.tags,
        &decision.deviates_from,
    );
    // Le `frontmatter_span` se termine juste après le `---\n` de fermeture.
    // On ajoute un `\n` après notre nouveau frontmatter pour restaurer la
    // même forme.
    format!("{frontmatter}\n{corps}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::decisions::index as build_index;
    use crate::ports::{FixedEnv, MemoryFileSystem};

    fn env() -> FixedEnv {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/home".into());
        env
    }

    fn adr(id: &str, status: &str) -> String {
        format!(
            "---\nid: \"{id}\"\ntitle: ADR {id}\nstatus: {status}\ndate: 2026-09-08\n---\n\n## Contexte\n\nx\n"
        )
    }

    fn empty_index_with_config(fs: &MemoryFileSystem) -> DecisionIndex {
        let layout = Layout::new("/p");
        let cfg = config::resolve(fs, &env(), &layout).unwrap();
        build_index(fs, &env(), &layout, &cfg).unwrap()
    }

    #[test]
    fn squelette_est_embarque() {
        assert!(DECISION_TEMPLATE.contains("{{FRONTMATTER}}"));
        assert!(DECISION_TEMPLATE.contains("## Contexte"));
        assert!(DECISION_TEMPLATE.contains("## Décision"));
        assert!(DECISION_TEMPLATE.contains("## Conséquences"));
        assert!(DECISION_TEMPLATE.contains("## Alternatives écartées"));
    }

    #[test]
    fn plan_new_dans_projet_vide_produit_0001() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let plan = plan_new(
            &index,
            &SealFile::empty(),
            "Un premier choix",
            DecisionStatus::Accepted,
            "2026-09-08",
            &Layout::new("/p"),
        )
        .unwrap();
        assert_eq!(plan.new_id, "0001");
        assert_eq!(
            plan.new_path,
            PathBuf::from("/p/_codev/decisions/0001-un-premier-choix.md")
        );
        // 2 writes : ADR (CreateOnly) + seal.yaml (Overwrite).
        assert_eq!(plan.plan.writes.len(), 2);
        let adr_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::CreateOnly)
            .unwrap();
        assert_eq!(adr_write.mode, WriteMode::CreateOnly);
    }

    #[test]
    fn numerotation_ignore_les_sources_heritees() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file("/p/_codev/decisions/0001-local.md", adr("0001", "accepted"))
            // Source héritée avec un id « plus grand » — ne doit pas influencer.
            .with_file(
                "/home/partage/_codev/decisions/9999-du-partage.md",
                adr("9999", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let plan = plan_new(
            &index,
            &SealFile::empty(),
            "Suivant",
            DecisionStatus::Accepted,
            "2026-09-08",
            &Layout::new("/p"),
        )
        .unwrap();
        assert_eq!(plan.new_id, "0002");
    }

    #[test]
    fn slug_du_titre() {
        assert_eq!(slug_from_title("Titre normal"), "titre-normal");
        assert_eq!(slug_from_title("émoji : 🎉"), "moji");
        assert_eq!(slug_from_title("???"), "decision");
        assert_eq!(slug_from_title(""), "decision");
        assert_eq!(slug_from_title("  espaces multiples  "), "espaces-multiples");
        assert_eq!(slug_from_title("under_score"), "under-score");
        assert_eq!(slug_from_title("ID-Kebab-Case"), "id-kebab-case");
    }

    #[test]
    fn titre_vide_refuse() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_new(
            &index,
            &SealFile::empty(),
            "   ",
            DecisionStatus::Accepted,
            "2026-09-08",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "empty_title");
    }

    #[test]
    fn plan_new_ecrit_ladr_en_create_only_et_le_sceau_en_overwrite() {
        // L'ADR ne doit jamais écraser un fichier existant (CreateOnly),
        // mais le sceau se réécrit à chaque nouvelle décision (Overwrite).
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let plan = plan_new(
            &index,
            &SealFile::empty(),
            "X",
            DecisionStatus::Accepted,
            "2026-09-08",
            &Layout::new("/p"),
        )
        .unwrap();
        // 2 writes : l'ADR + le seal.yaml.
        assert_eq!(plan.plan.writes.len(), 2);
        let create_only = plan
            .plan
            .writes
            .iter()
            .filter(|w| w.mode == WriteMode::CreateOnly)
            .count();
        assert_eq!(create_only, 1, "un seul CreateOnly attendu (l'ADR)");
    }

    #[test]
    fn plan_new_sur_statut_proposed_ne_scelle_pas() {
        // Un ADR `proposed` ne porte pas d'engagement d'immutabilité —
        // pas de sceau.
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let plan = plan_new(
            &index,
            &SealFile::empty(),
            "Piste",
            DecisionStatus::Proposed,
            "2026-09-08",
            &Layout::new("/p"),
        )
        .unwrap();
        assert_eq!(plan.plan.writes.len(), 1, "aucun sceau pour proposed");
    }

    // ─────────────── supersede ───────────────

    #[test]
    fn plan_supersede_produit_deux_ecritures() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0003-vieux.md",
                adr("0003", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let plan = plan_supersede(
            &index,
            &SealFile::empty(),
            "0003",
            "Nouveau choix",
            "2026-09-08",
            &Layout::new("/p"),
            |path| {
                Ok(fs
                    .read(path)
                    .unwrap_or_else(|| panic!("{} absent", path.display())))
            },
        )
        .unwrap();

        assert_eq!(plan.new_id, "0004");
        assert_eq!(plan.old_qualified_id, "projet/0003");
        // 3 writes : nouvel ADR (CreateOnly), ancien réécrit (Overwrite),
        // seal.yaml (Overwrite).
        assert_eq!(plan.plan.writes.len(), 3);

        // Nouvel ADR : CreateOnly, référence l'ancien.
        let new_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::CreateOnly)
            .unwrap();
        assert!(new_write.contents.contains("supersedes: [\"0003\"]"));
        assert!(new_write.contents.contains("id: \"0004\""));

        // Ancien ADR : Overwrite, status: superseded.
        let old_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::Overwrite)
            .unwrap();
        assert!(old_write.contents.contains("status: superseded"));
    }

    #[test]
    fn supersede_id_inconnu_refuse() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_supersede(
            &index,
            &SealFile::empty(),
            "9999",
            "X",
            "2026-09-08",
            &Layout::new("/p"),
            |_| Ok(String::new()),
        )
        .unwrap_err();
        assert_eq!(err.code(), "unknown_decision_id");
    }

    #[test]
    fn supersede_source_heritee_refuse() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file(
                "/home/partage/_codev/decisions/0100-partagee.md",
                adr("0100", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let err = plan_supersede(
            &index,
            &SealFile::empty(),
            "path:~/partage/0100",
            "Notre alternative",
            "2026-09-08",
            &Layout::new("/p"),
            |_| Ok(String::new()),
        )
        .unwrap_err();
        assert_eq!(err.code(), "cannot_supersede_inherited");
        assert!(err.to_string().contains("déviation"));
    }

    #[test]
    fn frontmatter_supersede_conserve_champs_dorigine() {
        // On construit un source explicite avec tags et supersedes existants,
        // et on vérifie que la réécriture les conserve tous.
        let source = "---\nid: \"0003\"\ntitle: \"Vieux titre\"\nstatus: accepted\ndate: 2025-01-01\ntags: [\"a\", \"b\"]\nsupersedes: [\"0001\"]\n---\n\n## Corps\n\ninchangé\n";
        let parsed = codev_core::decisions::parse_decision(source);
        let decision = parsed.value.expect("décision valide");
        let out = rewrite_frontmatter_status(source, &decision, DecisionStatus::Superseded);

        assert!(out.contains("status: superseded"));
        assert!(out.contains("id: \"0003\""));
        assert!(out.contains("title: \"Vieux titre\""));
        assert!(out.contains("date: 2025-01-01"));
        assert!(out.contains("tags: [\"a\", \"b\"]"));
        assert!(out.contains("supersedes: [\"0001\"]"));
        // Corps préservé au caractère près.
        assert!(out.ends_with("## Corps\n\ninchangé\n"));
    }

    #[test]
    fn supersede_ne_touche_pas_au_corps_du_predecesseur() {
        // Golden : après supersession, tout ce qui est après le frontmatter
        // reste identique à l'octet près.
        let source = "---\nid: \"0003\"\ntitle: T\nstatus: accepted\ndate: 2026-09-08\n---\n\n## Contexte\n\nUn contenu avec des `caractères` spéciaux — comme ça.\n\n## Décision\n\nOK.\n";
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file("/p/_codev/decisions/0003.md", source);
        let index = empty_index_with_config(&fs);
        let plan = plan_supersede(
            &index,
            &SealFile::empty(),
            "0003",
            "Nouveau",
            "2026-09-08",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap();

        let old_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::Overwrite)
            .unwrap();

        // Extrait ce qui suit le deuxième `---\n` — c'est le corps.
        fn body_after_frontmatter(text: &str) -> &str {
            let end_first = text.find("---\n").expect("premier ---") + 4;
            let end_second_rel = text[end_first..]
                .find("---\n")
                .expect("second ---")
                + 4;
            &text[end_first + end_second_rel..]
        }
        assert_eq!(
            body_after_frontmatter(&old_write.contents),
            body_after_frontmatter(source),
            "corps au caractère près"
        );
    }

    #[test]
    fn plan_supersede_inclut_la_nouvelle_decision() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0003-vieux.md",
                adr("0003", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let plan = plan_supersede(
            &index,
            &SealFile::empty(),
            "0003",
            "Nouveau choix",
            "2026-09-08",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap();
        assert_eq!(
            plan.new_path,
            PathBuf::from("/p/_codev/decisions/0004-nouveau-choix.md")
        );
    }

    // ─────────────── plan_seal ───────────────

    #[test]
    fn plan_seal_neuf_ajoute_lentree() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0001-x.md",
                adr("0001", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let plan = plan_seal(
            &index,
            &SealFile::empty(),
            "0001",
            false,
            "2026-09-09",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap();
        assert!(!plan.was_noop);
        assert_eq!(plan.id, "0001");
        assert!(plan.body_sha256.starts_with("sha256:"));
        assert_eq!(plan.plan.writes.len(), 1);
    }

    #[test]
    fn plan_seal_noop_si_le_hash_correspond_deja() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0001-x.md",
                adr("0001", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let source = fs.read(std::path::Path::new("/p/_codev/decisions/0001-x.md")).unwrap();
        let hash = seal::body_hash(&source).unwrap();
        let mut existing = SealFile::empty();
        existing.seals.push(codev_core::decisions::seal::Seal {
            id: "0001".into(),
            body_sha256: hash,
            sealed_at: "2026-09-08".into(),
        });
        let plan = plan_seal(
            &index,
            &existing,
            "0001",
            false,
            "2026-09-09",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap();
        assert!(plan.was_noop);
        assert!(plan.plan.writes.is_empty(), "no-op → aucune écriture");
    }

    #[test]
    fn plan_seal_refuse_sans_force_si_le_corps_a_change() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0001-x.md",
                adr("0001", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let mut existing = SealFile::empty();
        existing.seals.push(codev_core::decisions::seal::Seal {
            id: "0001".into(),
            body_sha256: "sha256:un-vieux-hash".into(),
            sealed_at: "2026-09-08".into(),
        });
        let err = plan_seal(
            &index,
            &existing,
            "0001",
            false,
            "2026-09-09",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap_err();
        assert_eq!(err.code(), "seal_conflict");
    }

    #[test]
    fn plan_seal_avec_force_reecrit_lentree() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0001-x.md",
                adr("0001", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let mut existing = SealFile::empty();
        existing.seals.push(codev_core::decisions::seal::Seal {
            id: "0001".into(),
            body_sha256: "sha256:un-vieux-hash".into(),
            sealed_at: "2026-09-08".into(),
        });
        let plan = plan_seal(
            &index,
            &existing,
            "0001",
            true,
            "2026-09-09",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap();
        assert!(!plan.was_noop);
        assert!(plan.body_sha256.starts_with("sha256:"));
        assert_ne!(plan.body_sha256, "sha256:un-vieux-hash");
        assert_eq!(plan.plan.writes.len(), 1);
    }

    #[test]
    fn plan_seal_refuse_une_decision_heritee() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file(
                "/home/partage/_codev/decisions/0100-partagee.md",
                adr("0100", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let err = plan_seal(
            &index,
            &SealFile::empty(),
            "path:~/partage/0100",
            false,
            "2026-09-09",
            &Layout::new("/p"),
            |path| Ok(fs.read(path).unwrap()),
        )
        .unwrap_err();
        assert_eq!(err.code(), "cannot_seal_inherited");
    }

    // ─────────────── plan_deviate ───────────────

    #[test]
    fn plan_deviate_ecrit_ladr_et_le_sceau() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file(
                "/home/partage/_codev/decisions/0100-partagee.md",
                adr("0100", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let plan = plan_deviate(
            &index,
            &SealFile::empty(),
            "path:~/partage/0100",
            "Notre alternative locale",
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap();

        assert_eq!(plan.new_id, "0001");
        assert_eq!(plan.target_qualified_id, "path:~/partage/0100");
        assert_eq!(plan.plan.writes.len(), 2, "ADR + sceau");

        let adr_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::CreateOnly)
            .unwrap();
        assert!(adr_write.contents.contains("deviates_from: [\"path:~/partage/0100\"]"));
        assert!(adr_write.contents.contains("status: accepted"));
        assert!(adr_write.contents.contains("id: \"0001\""));
    }

    #[test]
    fn plan_deviate_refuse_une_cible_locale() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/config.yaml", "")
            .with_file(
                "/p/_codev/decisions/0003-locale.md",
                adr("0003", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let err = plan_deviate(
            &index,
            &SealFile::empty(),
            "projet/0003",
            "…",
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "cannot_deviate_from_local");
        assert!(err.to_string().contains("supersede"));
    }

    #[test]
    fn plan_deviate_refuse_une_cible_inconnue() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_deviate(
            &index,
            &SealFile::empty(),
            "path:~/inconnue/0100",
            "…",
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "unknown_decision_id");
    }

    #[test]
    fn plan_deviate_refuse_titre_vide() {
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file(
                "/home/partage/_codev/decisions/0100-p.md",
                adr("0100", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let err = plan_deviate(
            &index,
            &SealFile::empty(),
            "path:~/partage/0100",
            "   ",
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "empty_title");
    }

    #[test]
    fn plan_deviate_rend_le_frontmatter_avec_deviates_from() {
        // Contrat exact du frontmatter — les tests plus haut vérifient déjà
        // le contenu, on golden ici le format YAML précis pour éviter
        // qu'un refactor casse la ligne rendue.
        let fs = MemoryFileSystem::new()
            .with_file(
                "/p/_codev/config.yaml",
                "inherits:\n  - path: ~/partage\n",
            )
            .with_file(
                "/home/partage/_codev/decisions/0100-p.md",
                adr("0100", "accepted"),
            );
        let index = empty_index_with_config(&fs);
        let plan = plan_deviate(
            &index,
            &SealFile::empty(),
            "path:~/partage/0100",
            "Choix local",
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap();
        let adr_content = &plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::CreateOnly)
            .unwrap()
            .contents;
        // Ligne exacte : `deviates_from: ["<qualified>"]`.
        assert!(adr_content.contains("\ndeviates_from: [\"path:~/partage/0100\"]\n"));
    }

    // ─────────────── plan_promote ───────────────

    const DESIGN_A_UN_BLOC: &str = "\
# Design : x

## Contexte

Voir proposal.

## Décisions

### Décision : Utiliser JWT

Le rationale du choix.

Deuxième paragraphe.

## Risques

y
";

    #[test]
    fn plan_promote_produit_un_adr_et_reference_le_design() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let plan = plan_promote(
            &index,
            &SealFile::empty(),
            "add-auth",
            "Utiliser JWT",
            DESIGN_A_UN_BLOC,
            PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap();

        assert_eq!(plan.new_id, "0001");
        assert_eq!(
            plan.new_path,
            PathBuf::from("/p/_codev/decisions/0001-utiliser-jwt.md")
        );
        assert!(plan.body_sha256.starts_with("sha256:"));
        // 3 writes : ADR + sceau + design.md.
        assert_eq!(plan.plan.writes.len(), 3);

        // Le nouvel ADR contient bien le corps verbatim.
        let adr = plan
            .plan
            .writes
            .iter()
            .find(|w| w.mode == WriteMode::CreateOnly)
            .unwrap();
        assert!(adr.contents.contains("Le rationale du choix."));
        assert!(adr.contents.contains("Deuxième paragraphe."));
        assert!(adr.contents.contains("<!-- ADR promu depuis"));
        assert!(adr.contents.contains("_codev/changes/add-auth/design.md"));

        // Le design.md réécrit contient la référence textuelle.
        let new_design = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == std::path::Path::new("/p/_codev/changes/add-auth/design.md"))
            .unwrap();
        assert!(new_design
            .contents
            .contains("### Décision : Utiliser JWT\n\n> Promue en ADR **0001**"));
        // Le corps original a bien disparu.
        assert!(!new_design.contents.contains("Le rationale du choix."));
        // Les autres sections (Contexte, Risques) sont préservées.
        assert!(new_design.contents.contains("## Risques\n\ny\n"));
    }

    #[test]
    fn plan_promote_refuse_titre_absent() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_promote(
            &index,
            &SealFile::empty(),
            "add-auth",
            "Titre fantôme",
            DESIGN_A_UN_BLOC,
            PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "decision_heading_not_found");
    }

    #[test]
    fn plan_promote_refuse_titre_ambigu() {
        let design_ambigu = "\
## Décisions

### Décision : X

v1

### Décision : X

v2

## Fin
";
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_promote(
            &index,
            &SealFile::empty(),
            "add-auth",
            "X",
            design_ambigu,
            PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "ambiguous_decision_heading");
        assert!(err.to_string().contains("plusieurs blocs"));
    }

    #[test]
    fn plan_promote_refuse_titre_vide() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let err = plan_promote(
            &index,
            &SealFile::empty(),
            "add-auth",
            "   ",
            DESIGN_A_UN_BLOC,
            PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap_err();
        assert_eq!(err.code(), "empty_title");
    }

    #[test]
    fn plan_promote_produit_un_adr_scelle() {
        // Le hash exposé correspond bien au SHA-256 du corps de l'ADR
        // rendu.
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "");
        let index = empty_index_with_config(&fs);
        let plan = plan_promote(
            &index,
            &SealFile::empty(),
            "add-auth",
            "Utiliser JWT",
            DESIGN_A_UN_BLOC,
            PathBuf::from("/p/_codev/changes/add-auth/design.md"),
            "2026-09-09",
            &Layout::new("/p"),
        )
        .unwrap();

        // Le seal.yaml qu'on va écrire porte l'entrée avec le même hash.
        let seal_write = plan
            .plan
            .writes
            .iter()
            .find(|w| w.path == Layout::new("/p").decisions_seal_file())
            .unwrap();
        assert!(seal_write.contents.contains(&plan.body_sha256));
    }
}
