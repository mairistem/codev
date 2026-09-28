# Design : codev init interactif avec détection

## Contexte

Voir `proposal.md`. Nouvelle capacité `init`, modification de `skills`
pour retourner `DEFAULT_WORKFLOWS`, refonte de `codev-cli::commands::init`,
nouveau module `codev-core::detect`, nouveau module
`codev-core::config::generate`. Nouvelle dépendance `dialoguer`.

## Décisions

### Décision : `dialoguer` plutôt que `inquire`

Les deux libs Rust de prompts sont matures. `dialoguer` est plus
petite (une dizaine de dépendances, pas de tokio), stable depuis
2018, utilisée par `cargo`, `rustup`, `clap` et `k8s`. `inquire` est
plus riche (validation inline, autocomplete) mais ces features ne
nous servent pas ici — deux questions, aucune validation
sophistiquée.

Coût pour codev : ~10 dépendances transitives supplémentaires. Coût
`inquire` : ~20. Pour un CLI de dev tool, `dialoguer` gagne.

**Alternative écartée A** : `inquire`. Rejeté au motif du poids
disproportionné pour les besoins.

**Alternative écartée B** : rouler notre propre prompt via `println!`
+ `io::stdin().read_line()`. Rejeté — la gestion propre du TTY, de
la sélection au clavier, du redraw sur backspace, coûte plus qu'une
dépendance battle-tested.

### Décision : détection dans `codev-core`, prompts dans `codev-cli`

La détection est **pure** — elle transforme des `&[u8]` (contenus de
manifestes) en un `struct Detected`. Elle n'a pas besoin d'I/O
au-delà de la lecture des fichiers, qui passe par le port `FileSystem`
existant. Elle vit dans `codev-core::detect` (submodules `stack`,
`mcp`, `license`, `ci`, `git`).

Les prompts sont **impurs** — ils lisent stdin, écrivent stdout,
sondent l'état du TTY. Ils vivent dans `codev-cli::init_prompts`
(nouveau module).

L'orchestration (sniff → prompt → generate → scaffold → install)
vit dans `codev-cli::commands::init`.

**Alternative écartée** : ranger la détection dans `codev-engine`
parce qu'elle « fait de l'I/O ». Rejeté — la lecture d'un
`Cargo.toml` via le port `FileSystem` est de la lecture pure vue
depuis le domaine. Rangée dans `codev-engine`, elle mélangerait
détection et effets, alors que la détection est justement le calcul
qu'un cœur pur sait faire.

### Décision : la génération de `_codev/config.yaml` est un **rendu YAML manuel**, pas `serde_norway::to_string`

Le fichier généré porte des **commentaires de provenance** au-dessus
de certaines clés (`# détecté depuis Cargo.toml`). Aucune lib de
sérialisation YAML ne préserve les commentaires — c'est un
non-problème pour la lecture (elles les ignorent), mais un problème
insurmontable pour l'écriture.

La fonction `codev-core::config::generate` MUST donc rendre le YAML
**caractère par caractère** : elle assemble une string à partir d'un
`GeneratedConfig` typé qui porte, pour chaque champ, sa valeur ET
son commentaire optionnel de provenance.

Le rendu est déterministe (ordre des champs stable) et testable par
golden : mêmes entrées, même bytes en sortie.

**Alternative écartée** : générer le YAML nu avec `serde_norway`,
puis post-traiter la string pour insérer les commentaires par
regex/split. Rejeté — trop fragile et pas plus court que le rendu
manuel.

**Contrôle croisé** : le YAML généré MUST passer par
`serde_norway::from_str::<ProjectConfig>()` dans un test unitaire, pour
garantir qu'il reste lisible par le reste de codev. On génère à la
main, on relit avec la lib.

### Décision : `Detected` inclut des `Option` — jamais d'invention

Chaque champ de `Detected` est un `Option<T>`. Un manifeste absent
ou illisible produit `None`, jamais une valeur inventée par défaut.
La génération de `context:` ne mentionne que les champs `Some`. Un
projet sans manifeste connu et sans `.github/workflows/` produit un
`context:` minimal — ce n'est pas une erreur.

**Alternative écartée** : produire un `context:` par défaut (« projet
générique ») quand rien n'est détecté. Rejeté — préférable de rien
dire que d'affirmer faux.

### Décision : ordre de résolution MCP — projet gagne sur utilisateur

Sources MCP scannées, dans cet ordre :

1. `<projet>/.mcp.json`
2. `~/.claude.json`
3. `<projet>/.claude/settings.json`
4. `<projet>/.claude/settings.local.json`

Les entrées `mcpServers` sont fusionnées ; si un même nom apparaît
dans deux fichiers, **le premier trouvé gagne** — la config projet
prime sur la config utilisateur globale. Ce choix suit la convention
Claude Code (les fichiers projet ont priorité).

`~/.claude.json` est lu **best-effort** — si absent, silence ; si
mal formé, warning silencieux, on continue.

**Alternative écartée** : fusion inverse (globale gagne). Rejeté —
contre-intuitif pour un dev qui a paramétré un MCP spécifiquement
pour ce projet.

### Décision : Détection de la stack — premier manifeste gagne, pas de vote

Ordre de recherche : `Cargo.toml` → `package.json` → `pyproject.toml`
→ `go.mod` → `pom.xml`. Le **premier trouvé** fixe la stack primaire ;
les autres sont ignorés. Un dépôt polyglotte affiche la stack du
manifeste racine.

Cette convention est arbitraire mais stable. L'utilisateur peut
toujours amender le `context:` généré à la main.

**Alternative écartée** : mentionner **tous** les manifestes trouvés
dans le `context:`. Rejeté — bruit typique d'un `node_modules/` ou
d'un submodule qui polluerait la détection.

### Décision : Le prompt « Contexte » propose l'ouverture de `$EDITOR`

Un contexte utile fait souvent plus d'une phrase. Forcer l'utilisateur
à taper la ligne au prompt le pousse à minimiser. La convention Git
est d'ouvrir `$EDITOR` sur un fichier temporaire préampli, qui devient
l'entrée après fermeture.

`dialoguer` supporte cela via `Editor::new().edit()`. On l'utilise
uniquement à ce prompt-là (les workflows restent en `Select`).

**Alternative écartée** : n'accepter que la ligne courte au prompt.
Rejeté — trop de friction pour un champ qui gagne à être détaillé.

### Décision : Retour de `DEFAULT_WORKFLOWS` = 7 workflows — bascule non-graduelle

Le nouveau défaut casse la lecture strictement chronologique de la
spec `skills` : un test existant (« Catalogue par défaut inclut
onboard » avec exactement 3 workflows) devient faux. La spec est
donc **MODIFIED**, pas complétée par un ADDED — le contrat change,
pas une extension. Le test correspondant est réécrit.

Effet secondaire pour les projets existants : `codev update` sur un
projet qui n'a pas de clé `workflows:` explicite installe d'un coup
les 4 skills manquantes. C'est le comportement voulu — un projet
sous-configuré rattrape ce qu'il aurait dû avoir. Un projet qui
tient à rester sur les 3 skills initiales peut ajouter la clé
`workflows:` explicite (voie opt-out).

Documenté dans le `CHANGELOG.md` comme **changement de comportement
notable** (mais non-breaking : l'ancien comportement reste
accessible par déclaration explicite).

## Risques et compromis

- **Prompt bloque sur un TTY faux positif** — un émulateur exotique
  qui prétend être un TTY mais n'accepte pas de saisie fait bloquer
  la commande. → **Atténuation** : `--yes` reste le kill switch. Le
  bloc « stdin non-TTY implique --yes » attrape le cas courant
  (CI, pipe).
- **Détection MCP faux positif** — un serveur nommé
  `"my-jira-mock"` est faussement matché comme un vrai MCP Jira. →
  **Atténuation** : la détection ne fait que proposer ; l'utilisateur
  confirme en interactif. En `--yes`, un faux positif produit un
  `mcp.jira_tool:` qui pointe vers un tool inexistant — les skills
  qui l'appellent échoueront proprement avec un message clair (déjà
  couvert par la spec `skills` propose).
- **Changement du défaut casse un test existant** — c'est le prix
  attendu, pas un risque. Le test « autres opt-in restent opt-in »
  est retiré dans le MODIFIED.
- **Génération YAML manuelle est fastidieuse à maintenir** — chaque
  nouveau champ demande d'être ajouté au renderer. → **Atténuation** :
  le renderer est court (~80 lignes), typé, testé par golden. Une
  clé oubliée est visible tout de suite.

## Plan de migration

Aucun outil de migration nécessaire. Un projet existant qui n'a pas
de clé `workflows:` explicite verra, au prochain `codev update`, ses
skills complétées. Un projet qui la tient veut peut-être ajouter les
workflows manquants (ADR interne — pas de contrainte).

Le `CHANGELOG.md` documente le changement avec une phrase courte :
« Le défaut de `codev init`/`update` passe de 3 à 7 skills — les
projets qui veulent moins déclarent `workflows:` explicite. »
