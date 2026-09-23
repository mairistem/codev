# Tâches

## 1. Body de la skill `propose`

- [x] 1.1 Éditer `assets/workflows/propose.md` — ajouter une nouvelle
      section **Étape 0 : détection d'un ticket externe**, insérée
      **avant** l'étape 1 « Comprendre la demande ». Contenu attendu :
      - Explication : la skill scanne le prompt de l'utilisateur pour
        y détecter un pattern `[A-Z]{2,}-\d+`.
      - Branche « pas de pattern » : passer directement à l'étape 1,
        comportement inchangé.
      - Branche « pattern détecté, MCP Atlassian disponible » :
        appeler `mcp__claude_ai_Atlassian__getJiraIssue({issueIdOrKey:
        "<ID>"})` pour le premier ticket ; utiliser le résultat pour
        enrichir le contexte de rédaction ; citer le ticket en tête du
        proposal.
      - Branche « pattern détecté, MCP absent » : afficher le message
        informatif documenté dans la spec ; continuer sans le
        contenu ; le proposal cite le ticket avec la mention « contenu
        non récupéré ».
      - Cas multi-tickets : le premier est récupéré ; les autres
        listés sous « autre(s) ticket(s) mentionné(s) ».
- [x] 1.2 Éditer l'étape « écriture du proposal » (étape 4d actuelle)
      pour préciser : « si un ticket a été détecté à l'étape 0,
      insérer la ligne `> Source : ticket **<ID>** — …` juste après le
      `# Proposal : <titre>`, avant `## Pourquoi` ».
- [x] 1.3 Ajouter un garde-fou en fin de section « Garde-fous » : la
      skill n'écrit **jamais** dans Jira, ne fait **jamais** de
      recherche JQL — seul un `getJiraIssue` par invocation, sur
      l'identifiant explicitement mentionné.

## 2. Entrée `Workflow { id: "propose" }` dans le CATALOG

- [x] 2.1 Étendre `allowed_tools` de l'entrée `propose` dans
      `crates/codev-agents/src/workflows.rs` : passer de
      `"Bash(codev:*), Read, Write, Edit, Glob, Grep"` à
      `"Bash(codev:*), Read, Write, Edit, Glob, Grep, mcp__claude_ai_Atlassian__getJiraIssue"`.
- [x] 2.2 Actualiser le doc-comment de l'entrée `propose` pour dire
      que la skill peut appeler le MCP Atlassian sur détection d'un
      ticket, sans être obligatoire.

## 3. Tests d'invariant

- [x] 3.1 Nouveau test `propose_declare_le_mcp_atlassian_getJiraIssue`
      dans `crates/codev-agents/src/workflows.rs::tests` : vérifie
      que `find("propose").allowed_tools.contains("mcp__claude_ai_Atlassian__getJiraIssue")`.
- [x] 3.2 Test complémentaire
      `propose_ne_declare_pas_dautre_mcp_atlassian` : parcourt
      `allowed_tools` de `propose`, vérifie qu'aucune autre chaîne
      `mcp__claude_ai_Atlassian__` (préfixe) n'est présente. Garde-fou
      contre un élargissement silencieux du périmètre d'accès.
- [x] 3.3 Test
      `propose_cite_la_detection_de_ticket_dans_son_body` : le body
      de la skill contient les chaînes `[A-Z]{2,}-\d+` (le pattern)
      et `mcp__claude_ai_Atlassian__getJiraIssue` (l'appel), pour
      qu'une refonte accidentelle du body remonte via `grep`.

## 4. Vérifications et dogfooding

- [x] 4.1 `cargo test --workspace` reste vert, gagne au moins 3 tests
      nouveaux.
- [x] 4.2 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 4.3 `codev validate --strict` sur ce dépôt reste vert.
- [x] 4.4 Après `cargo install --path crates/codev-cli` puis
      `codev update --force`, vérifier que
      `.claude/skills/codev-propose/SKILL.md` porte bien
      `mcp__claude_ai_Atlassian__getJiraIssue` dans son
      frontmatter `allowed-tools`.
- [x] 4.5 Vérification à la main (dans cette session Claude Code où
      le MCP Atlassian est disponible) : simuler l'invocation en
      relisant le body de la skill et en confirmant que le flow
      décrit fonctionne — cite bien un ID, appelle bien le MCP,
      injecte bien le contenu.
