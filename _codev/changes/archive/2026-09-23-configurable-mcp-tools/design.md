# Design : configurer les noms de MCP côté projet

## Contexte

Voir `proposal.md`. Le premier vrai usage a montré que les noms de
MCP sont **spécifiques à l'environnement Claude Code**. Ce change
retire les noms du code source et les déplace vers
`_codev/config.yaml`.

## Objectifs / Hors objectifs

Ce design cadre la structure de configuration, la mécanique de
substitution de placeholders, la propagation dans les crates, et le
fallback quand aucun MCP n'est configuré. Il ne cadre pas d'autres
MCP (Design, Confluence…), ni l'héritage inter-projets du bloc `mcp:`,
ni un mode debug de rendering.

## Décisions

### Décision : `mcp:` est une struct typée, pas un `HashMap<String, String>`

Deux options :

| Option | Pro | Contre |
|---|---|---|
| **A. `HashMap<String, String>` libre** | Extension à zéro coût, chaque skill lit sa clef | Invite à mettre n'importe quoi ; erreurs de nommage silencieuses ; `deny_unknown_fields` inutile |
| **B. `struct McpConfig { jira_tool: Option<String>, … }`** | Champs connus, validation par serde, autocomplétion IDE, warnings en cas de faute de frappe | Chaque nouveau MCP demande une PR côté codev |

**Choisi : B.** Cohérent avec la philosophie du reste de
`ProjectConfig` (`inherits`, `rules`) — champs typés,
`deny_unknown_fields`. Ajouter un champ par futur MCP est un coût
négligeable, et le typage rend les erreurs de config immédiatement
visibles.

### Décision : substitution de placeholders par `str::replace`, pas de moteur de template

Le rendering d'un `SKILL.md` est simple : quelques substitutions
statiques (`{{FRONTMATTER}}` existe déjà pour les décisions).
Introduire un moteur de template (handlebars, tera) serait
disproportionné.

**Alternative écartée** : `handlebars-rust`. Utile un jour si les
substitutions deviennent conditionnelles/complexes ; aujourd'hui,
`s.replace("{{JIRA_MCP_TOOL}}", tool)` suffit. Reportable.

### Décision : le placeholder est retiré **avec la virgule qui le précède** dans `allowed_tools`

Sans cette précaution, `allowed_tools` finirait par `", "` à la fin
quand la config est absente — Claude Code refuserait probablement le
frontmatter. La substitution est faite en deux temps :

```rust
fn substitute_jira_mcp(source: &str, jira_tool: Option<&str>) -> String {
    match jira_tool {
        Some(tool) => source.replace("{{JIRA_MCP_TOOL}}", tool),
        None => source
            .replace(", {{JIRA_MCP_TOOL}}", "")   // dans allowed_tools
            .replace("{{JIRA_MCP_TOOL}}", "(MCP Jira non configuré)"), // ailleurs
    }
}
```

L'ordre importe : on retire d'abord `, {{JIRA_MCP_TOOL}}` (avec la
virgule) avant le fallback sur `{{JIRA_MCP_TOOL}}` seul. Sinon le
premier `replace` sans virgule laisserait `", "` orphelin.

### Décision : `RenderCtx` porte l'unique champ, pas une struct extensible

Pour la V1 : `RenderCtx { jira_mcp_tool: Option<String> }`. Si un
autre MCP arrive plus tard, on ajoute un champ. Un `HashMap<String,
Option<String>>` serait trop générique pour un besoin restreint.

### Décision : la configuration MCP est projet-spécifique, pas héritée

Un projet qui déclare `inherits: [{path: ~/partage}]` **ne** hérite
**pas** du bloc `mcp:` de la source. Chaque projet configure son
propre MCP — parce que le nom du MCP dépend de la config Claude Code
de l'utilisateur, pas du projet source.

**Alternative écartée** : propager `mcp:` par héritage. Une source
partagée entre plusieurs équipes forcerait toutes ces équipes à
utiliser le même MCP, ce qui n'est pas ce qu'on veut.

### Décision : le fallback affiche `(MCP Jira non configuré)` dans le body

Quand `mcp.jira_tool` est absent, l'agent qui lit le body voit
`(MCP Jira non configuré)` là où le nom du tool aurait été. Ça
lui dit clairement :

1. Qu'il n'a **pas** à essayer d'appeler un MCP (qui n'existe pas).
2. Que si l'utilisateur mentionne un ticket, il doit afficher le
   message informatif documenté et rédiger sans le contenu.

**Alternative écartée** : laisser le body avec le placeholder brut
`{{JIRA_MCP_TOOL}}`. Rejeté — l'agent n'aurait aucun repère et
pourrait halluciner un nom de tool.

## Risques et compromis

- **Substitution naïve casse si un utilisateur écrit accidentellement
  `{{JIRA_MCP_TOOL}}` dans son body de skill.** → **Compromis
  assumé** : le CATALOG est du code Rust maintenu par nous ; personne
  n'écrit de body de skill à la main. Si un jour le catalogue devient
  éditable projet par projet, la substitution deviendra une source
  d'ambiguïté — on la remplacera par un vrai template engine.
- **Le nom du MCP dépend d'une convention Claude Code sur laquelle
  on n'a pas la main.** Si Claude Code renomme encore ses tools, la
  config projet cassera. → **Atténuation** : le message informatif
  documente ce qui se passe ; l'utilisateur met à jour son
  `config.yaml` en une ligne. Pas de changement de code.
- **Deux configs qui déclarent le même MCP diffèrent d'un caractère
  près.** → **Compromis assumé** : validation stricte (serde
  `deny_unknown_fields` sur `McpConfig` + type `String`) attrape le
  parsing YAML mais pas les fautes de frappe dans le nom lui-même.
  Un test de sanité pourrait vérifier que le nom matche
  `^mcp__[a-zA-Z0-9_]+$` — reportable.

## Plan de migration

Pour ce dépôt :

1. Éditer `_codev/config.yaml` — ajouter :
   ```yaml
   mcp:
     jira_tool: mcp__claude_ai_Atlassian_Rovo__getJiraIssue
   ```
2. `cargo install --path crates/codev-cli`
3. `codev update --force` — le SKILL.md installé porte maintenant le
   nom Rovo.
4. Vérification : `head -5 .claude/skills/codev-propose/SKILL.md`
   montre `allowed-tools: "..., mcp__claude_ai_Atlassian_Rovo__getJiraIssue"`.

Pour un projet qui découvre codev :

- Sans MCP branché ni configuré : le skill s'installe avec
  `allowed-tools` sans MCP et un body qui dit « (MCP Jira non
  configuré) ». Comportement bit-identique à celui d'avant
  `propose-detects-jira-tickets`.
- Avec MCP branché : ajouter le bloc `mcp:` dans `config.yaml` (le
  `DEFAULT_CONFIG` du scaffold contient un exemple commenté),
  relancer `codev update --force`.
