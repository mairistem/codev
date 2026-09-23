use std::path::{Path, PathBuf};

use codev_core::{Plan, WriteMode};
use codev_engine::FileSystem;

use crate::target::{AgentTarget, SkillsPlan};
use crate::workflows::Workflow;

/// Claude Code.
///
/// Une skill déposée dans `.claude/skills/<nom>/SKILL.md` est découverte
/// automatiquement **et** invocable par l'utilisateur en tapant `/<nom>`. C'est
/// pourquoi codev ne génère pas de fichiers de commandes séparés : ils
/// feraient deux fichiers à garder cohérents pour un seul workflow.
/// Voir `_codev/decisions/0004-une-seule-identite-skill-et-commande.md`.
///
/// L'instance porte un `RenderCtx` : le CLI l'alimente depuis la config
/// projet (`_codev/config.yaml.mcp.jira_tool`, …), et il alimente la
/// substitution des placeholders au moment du rendering.
#[derive(Debug, Default)]
pub struct ClaudeCode {
    ctx: RenderCtx,
}

impl ClaudeCode {
    /// Crée une instance avec un contexte de rendu vide — utilisé quand
    /// aucune config projet n'est disponible (tests, chemin scaffolding
    /// bas niveau).
    pub fn new() -> Self {
        Self::default()
    }

    /// Crée une instance avec le contexte fourni. C'est le chemin
    /// utilisé par le CLI, alimenté par `ResolvedConfig.mcp`.
    pub fn with_ctx(ctx: RenderCtx) -> Self {
        Self { ctx }
    }
}

/// Contexte de rendu du frontmatter d'une skill.
///
/// Porte les informations projet-spécifiques dont la substitution des
/// placeholders a besoin. Aujourd'hui : le nom du tool MCP Jira. Un
/// futur MCP (Design, Confluence…) ajoutera son propre champ.
///
/// Un `Default` renvoie tous les champs `None` — utilisé par les tests
/// et le chemin « projet sans MCP configuré ».
#[derive(Debug, Clone, Default)]
pub struct RenderCtx {
    /// Nom du tool MCP Jira à injecter dans les skills. Alimenté par
    /// `_codev/config.yaml.mcp.jira_tool`.
    pub jira_mcp_tool: Option<String>,
}

/// Placeholder textuel à substituer dans `allowed_tools` et dans le
/// body de chaque workflow. Choisi pour ne pas apparaître naturellement
/// dans un body markdown.
const JIRA_MCP_PLACEHOLDER: &str = "{{JIRA_MCP_TOOL}}";

/// Fallback affiché dans le body quand aucun MCP Jira n'est configuré —
/// l'agent qui lit la skill voit clairement que la détection de tickets
/// n'aboutira pas.
const JIRA_MCP_FALLBACK_BODY: &str = "(MCP Jira non configuré)";

/// Substitue le placeholder `{{JIRA_MCP_TOOL}}` selon la config.
///
/// Deux passes quand la config est absente : on retire d'abord
/// `, {{JIRA_MCP_TOOL}}` (avec la virgule qui le précède, pour ne pas
/// laisser `allowed_tools` finir par `", "`), puis on remplace ce qu'il
/// reste par le fallback textuel (utile dans le body).
pub fn substitute_jira_mcp(source: &str, jira_tool: Option<&str>) -> String {
    match jira_tool {
        Some(tool) => source.replace(JIRA_MCP_PLACEHOLDER, tool),
        None => source
            .replace(&format!(", {JIRA_MCP_PLACEHOLDER}"), "")
            .replace(JIRA_MCP_PLACEHOLDER, JIRA_MCP_FALLBACK_BODY),
    }
}

const SKILLS_ROOT: &str = ".claude/skills";

/// Préfixe des noms de skills. `propose` devient `codev-propose`, invocable
/// `/codev-propose`.
const SKILL_PREFIX: &str = "codev";

impl ClaudeCode {
    pub fn ctx(&self) -> &RenderCtx {
        &self.ctx
    }

    pub fn skill_name(workflow_id: &str) -> String {
        format!("{SKILL_PREFIX}-{workflow_id}")
    }

    pub fn skill_file(project_root: &Path, workflow_id: &str) -> PathBuf {
        project_root
            .join(SKILLS_ROOT)
            .join(Self::skill_name(workflow_id))
            .join("SKILL.md")
    }

    /// Rend le `SKILL.md` complet : frontmatter engendré, corps tel quel.
    ///
    /// `ctx` porte les substitutions projet-spécifiques (nom du MCP Jira,
    /// …). Un `RenderCtx::default()` fait le rendu « sans MCP » — le
    /// placeholder est retiré proprement et remplacé par un fallback
    /// informatif dans le body.
    pub fn render(workflow: &Workflow, version: &str, ctx: &RenderCtx) -> String {
        let tools = substitute_jira_mcp(workflow.allowed_tools, ctx.jira_mcp_tool.as_deref());
        let body = substitute_jira_mcp(workflow.body, ctx.jira_mcp_tool.as_deref());
        format!(
            "---\n\
             name: {name}\n\
             description: {description}\n\
             allowed-tools: {tools}\n\
             license: MIT\n\
             metadata:\n\
             \x20 generator: codev\n\
             \x20 version: \"{version}\"\n\
             ---\n\
             \n\
             {body}",
            name = Self::skill_name(workflow.id),
            description = yaml_scalar(workflow.description),
            tools = yaml_scalar(&tools),
            body = body.trim_end(),
        )
    }
}

impl AgentTarget for ClaudeCode {
    fn id(&self) -> &'static str {
        "claude"
    }

    fn label(&self) -> &'static str {
        "Claude Code"
    }

    fn detect(&self, fs: &dyn FileSystem, project_root: &Path) -> bool {
        fs.exists(&project_root.join(".claude")) || fs.exists(&project_root.join("CLAUDE.md"))
    }

    fn plan_skills(
        &self,
        fs: &dyn FileSystem,
        project_root: &Path,
        workflows: &[&Workflow],
        version: &str,
        force: bool,
    ) -> SkillsPlan {
        let mut plan = Plan::new();
        let mut preserved = Vec::new();

        for workflow in workflows {
            let path = Self::skill_file(project_root, workflow.id);
            let desired = Self::render(workflow, version, &self.ctx);

            // Même version inscrite, contenu différent : quelqu'un a édité le
            // fichier à la main. L'écraser lui ferait perdre son travail sans
            // prévenir ; on le signale et on passe.
            let edite_a_la_main = !force
                && fs.exists(&path)
                && fs.read_to_string(&path).is_ok_and(|current| {
                    current != desired && stamped_version(&current).as_deref() == Some(version)
                });
            if edite_a_la_main {
                preserved.push(path);
                continue;
            }

            plan.dir(path.parent().unwrap_or(project_root));
            plan.write(path, desired, WriteMode::Overwrite);
        }

        SkillsPlan { plan, preserved }
    }
}

/// Rend un scalaire sûr pour du frontmatter YAML.
///
/// Les descriptions contiennent des deux-points et des virgules, qui changent
/// le sens d'un scalaire nu. On cite systématiquement plutôt que de deviner au
/// cas par cas.
fn yaml_scalar(raw: &str) -> String {
    let echappe = raw.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{echappe}\"")
}

/// Extrait la version inscrite dans le frontmatter d'un `SKILL.md`.
///
/// C'est ce tampon qui distingue « fichier d'une version antérieure, à
/// régénérer » de « fichier de la version courante que l'utilisateur a
/// modifié, à préserver ».
fn stamped_version(contents: &str) -> Option<String> {
    if !contents.starts_with("---") {
        return None;
    }
    contents
        .lines()
        .skip(1)
        .take_while(|line| *line != "---")
        .find_map(|line| line.trim().strip_prefix("version:"))
        .map(|value| value.trim().trim_matches('"').to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use codev_engine::ports::MemoryFileSystem;

    use crate::workflows;

    const VERSION: &str = "0.1.0";

    fn propose() -> &'static Workflow {
        workflows::find("propose").unwrap()
    }

    #[test]
    fn le_nom_de_la_skill_est_la_slash_command() {
        assert_eq!(ClaudeCode::skill_name("propose"), "codev-propose");
        assert_eq!(
            ClaudeCode::skill_file(Path::new("/p"), "propose"),
            PathBuf::from("/p/.claude/skills/codev-propose/SKILL.md")
        );
    }

    #[test]
    fn le_frontmatter_est_complet_et_le_corps_intact() {
        let rendu = ClaudeCode::render(propose(), VERSION, &RenderCtx::default());

        assert!(rendu.starts_with("---\n"));
        assert!(rendu.contains("name: codev-propose\n"));
        // Rendu sans MCP configuré : le placeholder et la virgule qui
        // le précède ont été retirés.
        assert!(rendu.contains("allowed-tools: \"Bash(codev:*), Read, Write, Edit, Glob, Grep\"\n"));
        assert!(rendu.contains("  version: \"0.1.0\"\n"));
        assert!(
            rendu.contains("Frontière de planification"),
            "le corps du workflow doit être présent tel quel"
        );
    }

    #[test]
    fn le_frontmatter_de_chaque_skill_est_du_yaml_valide() {
        // Le garde-fou qui compte : une description contenant un deux-points,
        // un tiret cadratin ou une apostrophe ne doit pas produire un
        // frontmatter que Claude Code refusera de lire — et l'erreur serait
        // silencieuse, la skill simplement absente.
        for workflow in workflows::CATALOG {
            let rendu = ClaudeCode::render(workflow, VERSION, &RenderCtx::default());
            let frontmatter = rendu
                .split("---\n")
                .nth(1)
                .unwrap_or_else(|| panic!("« {} » n'a pas de frontmatter", workflow.id));

            let parse: serde_norway::Value = serde_norway::from_str(frontmatter)
                .unwrap_or_else(|e| panic!("frontmatter de « {} » illisible : {e}", workflow.id));

            assert_eq!(
                parse["name"].as_str(),
                Some(ClaudeCode::skill_name(workflow.id).as_str()),
                "le nom de la skill doit correspondre à son dossier"
            );
            assert_eq!(parse["metadata"]["version"].as_str(), Some(VERSION));
            assert!(parse["description"].as_str().is_some_and(|d| d.len() > 60));
        }
    }

    #[test]
    fn la_description_est_citee_car_elle_contient_de_la_ponctuation_yaml() {
        let rendu = ClaudeCode::render(propose(), VERSION, &RenderCtx::default());
        let ligne = rendu
            .lines()
            .find(|l| l.starts_with("description:"))
            .unwrap();
        assert!(
            ligne.starts_with("description: \"") && ligne.ends_with('"'),
            "{ligne}"
        );
    }

    #[test]
    fn installe_les_skills_demandees() {
        let fs = MemoryFileSystem::new();
        let workflows = vec![propose()];
        let planifie = ClaudeCode::new().plan_skills(&fs, Path::new("/p"), &workflows, VERSION, false);

        assert_eq!(planifie.plan.writes.len(), 1);
        assert_eq!(
            planifie.plan.writes[0].path,
            PathBuf::from("/p/.claude/skills/codev-propose/SKILL.md")
        );
        assert!(planifie.preserved.is_empty());
    }

    #[test]
    fn regenere_une_skill_dune_version_anterieure() {
        let ancienne = ClaudeCode::render(propose(), "0.0.1", &RenderCtx::default());
        let fs = MemoryFileSystem::new()
            .with_file("/p/.claude/skills/codev-propose/SKILL.md", ancienne);

        let planifie = ClaudeCode::new().plan_skills(&fs, Path::new("/p"), &[propose()], VERSION, false);

        assert_eq!(planifie.plan.writes.len(), 1, "la mise à jour doit écrire");
        assert!(planifie.preserved.is_empty());
    }

    #[test]
    fn preserve_une_skill_editee_a_la_main() {
        let editee = format!(
            "{}\n\nMa consigne maison ajoutée à la fin.\n",
            ClaudeCode::render(propose(), VERSION, &RenderCtx::default())
        );
        let fs =
            MemoryFileSystem::new().with_file("/p/.claude/skills/codev-propose/SKILL.md", editee);

        let planifie = ClaudeCode::new().plan_skills(&fs, Path::new("/p"), &[propose()], VERSION, false);

        assert!(
            planifie.plan.writes.is_empty(),
            "un fichier édité à la main ne doit pas être écrasé"
        );
        assert_eq!(
            planifie.preserved,
            [PathBuf::from("/p/.claude/skills/codev-propose/SKILL.md")]
        );
    }

    #[test]
    fn force_ecrase_meme_une_skill_editee() {
        let editee = format!(
            "{}\n\nMa consigne.\n",
            ClaudeCode::render(propose(), VERSION, &RenderCtx::default())
        );
        let fs =
            MemoryFileSystem::new().with_file("/p/.claude/skills/codev-propose/SKILL.md", editee);

        let planifie = ClaudeCode::new().plan_skills(&fs, Path::new("/p"), &[propose()], VERSION, true);

        assert_eq!(planifie.plan.writes.len(), 1);
        assert!(planifie.preserved.is_empty());
    }

    #[test]
    fn une_skill_deja_conforme_est_replanifiee_sans_dommage() {
        // Le plan la contient, mais l'exécution la reconnaîtra identique et
        // n'écrira rien : c'est `apply::execute` qui tranche.
        let fs = MemoryFileSystem::new().with_file(
            "/p/.claude/skills/codev-propose/SKILL.md",
            ClaudeCode::render(propose(), VERSION, &RenderCtx::default()),
        );
        let planifie = ClaudeCode::new().plan_skills(&fs, Path::new("/p"), &[propose()], VERSION, false);
        assert_eq!(planifie.plan.writes.len(), 1);
        assert!(planifie.preserved.is_empty());
    }

    #[test]
    fn lit_le_tampon_de_version() {
        assert_eq!(
            stamped_version(&ClaudeCode::render(propose(), "1.2.3", &RenderCtx::default())).as_deref(),
            Some("1.2.3")
        );
        assert_eq!(stamped_version("pas de frontmatter"), None);
        assert_eq!(
            stamped_version("---\nname: x\n---\nversion: 9.9.9\n"),
            None,
            "une version hors frontmatter ne compte pas"
        );
    }

    #[test]
    fn detecte_claude_code_a_ses_traces() {
        let root = Path::new("/p");
        assert!(!ClaudeCode::new().detect(&MemoryFileSystem::new(), root));
        assert!(ClaudeCode::new().detect(
            &MemoryFileSystem::new().with_file("/p/CLAUDE.md", "#"),
            root
        ));
        assert!(ClaudeCode::new().detect(
            &MemoryFileSystem::new().with_file("/p/.claude/settings.json", "{}"),
            root
        ));
    }

    // ─────────────── substitute_jira_mcp ───────────────

    #[test]
    fn substitute_avec_tool_remplace_le_placeholder() {
        let src = "Bash(codev:*), Read, {{JIRA_MCP_TOOL}}";
        let out = substitute_jira_mcp(src, Some("mcp__foo__bar"));
        assert_eq!(out, "Bash(codev:*), Read, mcp__foo__bar");
    }

    #[test]
    fn substitute_sans_tool_retire_placeholder_et_virgule_dans_allowed_tools() {
        // La virgule qui précède doit partir aussi, sinon `allowed_tools`
        // finirait par `", "` — malformé.
        let src = "Bash(codev:*), Read, Write, Grep, {{JIRA_MCP_TOOL}}";
        let out = substitute_jira_mcp(src, None);
        assert_eq!(out, "Bash(codev:*), Read, Write, Grep");
    }

    #[test]
    fn substitute_sans_tool_remplace_par_fallback_dans_le_body() {
        // Sans virgule qui précède (cas typique du body), le placeholder
        // est remplacé par la mention informative.
        let src = "Appelle l'outil `{{JIRA_MCP_TOOL}}` avec l'id du ticket.";
        let out = substitute_jira_mcp(src, None);
        assert_eq!(
            out,
            "Appelle l'outil `(MCP Jira non configuré)` avec l'id du ticket."
        );
    }

    #[test]
    fn substitute_sans_placeholder_ne_touche_a_rien() {
        let src = "Bash(codev:*), Read";
        assert_eq!(substitute_jira_mcp(src, None), src);
        assert_eq!(substitute_jira_mcp(src, Some("mcp__x__y")), src);
    }

    #[test]
    fn render_avec_jira_tool_substitue_partout() {
        let ctx = RenderCtx {
            jira_mcp_tool: Some("mcp__x__y".into()),
        };
        let rendu = ClaudeCode::render(propose(), VERSION, &ctx);
        assert!(!rendu.contains("{{JIRA_MCP_TOOL}}"));
        assert!(!rendu.contains("(MCP Jira non configuré)"));
        assert!(rendu.contains("mcp__x__y"));
    }

    #[test]
    fn render_sans_jira_tool_retire_placeholder_et_utilise_fallback() {
        let rendu = ClaudeCode::render(propose(), VERSION, &RenderCtx::default());
        assert!(!rendu.contains("{{JIRA_MCP_TOOL}}"));
        // Le body cite le fallback (au moins une occurrence).
        assert!(rendu.contains("(MCP Jira non configuré)"));
        // `allowed-tools` ne finit pas par une virgule orpheline.
        let line = rendu
            .lines()
            .find(|l| l.starts_with("allowed-tools:"))
            .unwrap();
        assert!(!line.ends_with(", \""));
        assert!(line.ends_with('"'));
    }
}
