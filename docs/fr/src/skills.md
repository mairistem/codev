# Les compétences Claude Code

Une compétence (skill) est un ensemble d'instructions que Claude Code suit
pour un type de tâche. Compagnon vous en fournit huit, une pour chaque
étape de son [déroulé](glossary.md#déroulé) : planifier une modification,
la réaliser, la classer, etc. Claude Code les trouve tout seul : il vous
suffit de demander avec vos mots. Vous pouvez aussi en lancer une
directement en tapant son nom, par exemple `/compagnon-propose`.

Vous n'avez rien à installer à la main : `compagnon init` et
`compagnon update` écrivent chacune dans son propre dossier,
`.claude/skills/compagnon-<workflow>/SKILL.md`.

| Compétence | Ce qu'elle fait | Ce qu'elle peut modifier |
|---|---|---|
| `/compagnon-propose` | Crée une modification et rédige tout son plan en une fois | Seulement le dossier de la modification, `_compagnon/changes/<name>/` |
| `/compagnon-explore` | Vous aide à mûrir une idée, comparer des approches, clarifier un besoin | Rien |
| `/compagnon-onboard` | Présente Compagnon, vous dit où en est le projet, propose la prochaine action | Rien |
| `/compagnon-configure` | Lit votre projet et propose comment le décrire à Compagnon : un `context:` plus riche et des `rules:` pour chaque partie du plan | `_compagnon/config.yaml`, après votre accord |
| `/compagnon-apply` | Réalise une modification planifiée, tâche par tâche, en cochant chacune | Le code de votre projet et `tasks.md` |
| `/compagnon-update` | Révise une partie d'un plan déjà rédigé, et garde le reste cohérent | Seulement le dossier de la modification, `_compagnon/changes/<name>/` |
| `/compagnon-sync` | Met à jour les spécifications du projet avec ce que la modification apporte, sans la clore | Rien directement : elle passe par `compagnon sync` |
| `/compagnon-archive` | Vérifie une modification terminée, met à jour les spécifications du projet avec elle, et la classe | Rien directement : elle passe par `compagnon archive` |

## Comment demander

Inutile de retenir les commandes. Claude Code lit la description de chaque
compétence installée et lance celle qui correspond à votre demande : dites
ce que vous voulez, avec vos mots et dans votre langue. La commande
`/compagnon-…` n'est qu'un raccourci qui nomme directement la compétence.
Si Claude choisit la mauvaise compétence, ou aucune, précisez laquelle
(« utilise Compagnon pour planifier ça ») ou tapez le raccourci.

### Pour planifier une modification (`/compagnon-propose`)

L'indice : vous demandez un plan, une proposition, ou quelque chose de
nouveau à construire ou à corriger « avec Compagnon ».

- « Prépare un plan pour un mode sombre qui suit le réglage du système. »
- « Rédige une proposition pour exporter les factures en PDF. »
- « Prépare une modification pour le ticket PROJ-123. »

### Pour réfléchir à une idée (`/compagnon-explore`)

L'indice : vous ne savez pas encore quoi construire, ou voulez comparer des
options. Rien n'est écrit.

- « Explorons comment mettre en cache les résultats de recherche. »
- « J'hésite entre deux façons de stocker les sessions : compare-les. »

### Pour une visite guidée (`/compagnon-onboard`)

L'indice : vous découvrez Compagnon ou le projet, et voulez savoir où en
sont les choses.

- « Fais-moi visiter Compagnon. »
- « Où en est ce projet, et que dois-je faire ensuite ? »

### Pour adapter l'installation (`/compagnon-configure`)

L'indice : vous voulez que Compagnon connaisse les conventions de votre
projet.

- « Configure Compagnon pour ce projet. »
- « Lis notre README et notre code, et propose des règles pour les plans. »

### Pour réaliser une modification planifiée (`/compagnon-apply`)

L'indice : un plan existe et vous le validez.

- « Le plan me va, réalise la modification. »
- « Réalise les tâches de add-dark-mode. »

### Pour réviser un plan (`/compagnon-update`)

L'indice : vous voulez modifier un plan déjà rédigé, avant sa réalisation.

- « Dans le plan, garde le choix de l'utilisateur quand il choisit un thème à la main. »
- « Mets à jour la conception de add-dark-mode : on stocke le thème dans un cookie. »

### Pour publier les spécifications plus tôt (`/compagnon-sync`)

L'indice : une autre modification a besoin des spécifications de celle-ci
avant son archivage.

- « Reporte les spécifications de add-dark-mode dans celles du projet, sans archiver. »
- « Mets à jour maintenant les spécifications du projet avec cette modification. »

### Pour classer une modification terminée (`/compagnon-archive`)

L'indice : le travail est fini et vérifié.

- « La modification est terminée, archive-la. »
- « Archive add-dark-mode. »

## Permissions des outils

Chaque compétence ne peut utiliser que les outils dont sa tâche a besoin.
Une compétence qui ne doit pas écrire de fichiers ne reçoit pas les outils
pour le faire. Cela fait partie de ce que chaque compétence vous promet.

- Toutes les compétences peuvent lancer des commandes `compagnon` et lire
  des fichiers.
- Seule `/compagnon-apply` peut lancer n'importe quelle commande, parce
  qu'elle doit lancer vos tests et compiler votre projet.
- `/compagnon-sync` et `/compagnon-archive` ne peuvent écrire aucun
  fichier : toutes leurs modifications passent par Compagnon lui-même.
- `/compagnon-explore` et `/compagnon-onboard` n'écrivent rien.

<details>
<summary>Sous le capot : les outils autorisés de chaque compétence</summary>

Chaque skill déclare les outils qu'elle peut utiliser dans le champ
`allowed-tools` de son frontmatter.

| Skill | `allowed-tools` |
|---|---|
| `compagnon-propose` | `Bash(compagnon:*), Read, Write, Edit, Glob, Grep`, plus l'outil MCP Jira s'il est configuré, plus `Agent` en mode de revue isolé |
| `compagnon-explore` | `Bash(compagnon:*), Read, Glob, Grep` |
| `compagnon-onboard` | `Bash(compagnon:*), Read, Glob` |
| `compagnon-configure` | `Bash(compagnon:*), Read, Write, Edit, Glob, Grep` |
| `compagnon-apply` | `Bash(compagnon:*), Read, Write, Edit, Glob, Grep, Bash`, plus `Agent` en mode de revue isolé ou quand le `developer` a un modèle |
| `compagnon-update` | `Bash(compagnon:*), Read, Write, Edit, Glob, Grep` |
| `compagnon-sync` | `Bash(compagnon:*), Read` |
| `compagnon-archive` | `Bash(compagnon:*), Read` |

`Bash(compagnon:*)` permet à une skill de lancer des commandes `compagnon`, et rien
d'autre. Seule `compagnon-apply` obtient un accès général à `Bash`, parce qu'elle
doit lancer vos tests et vos commandes de build. `compagnon-sync` et
`compagnon-archive` ne peuvent écrire aucun fichier : toutes leurs modifications
passent par la CLI.

`Agent` est l'outil de sous-agent de Claude Code. `compagnon-propose` et
`compagnon-apply` le demandent lorsque `_compagnon/config.yaml` définit
`review: { mode: isolated }`
([Revue en ligne ou isolée](roles.md#revue-en-ligne-ou-isolée)) : chaque
rôle relecteur travaille alors comme sous-agent. `compagnon-apply` le demande
aussi quand le `developer` a un modèle : il lui confie alors les tâches
dans un sous-agent ([Le modèle du développeur](roles.md#le-modèle-du-développeur)).
Aucune autre skill ne le demande jamais. Après avoir changé de mode, lancez `compagnon update` ;
`compagnon doctor` signale `skills_review_mode` tant que vous ne l'avez pas
fait. Les deux skills décrivent la revue isolée dans les deux modes, car
le `change.yaml` d'un change peut remplacer le mode du projet : sans
l'outil pré-autorisé, Claude Code demande avant de lancer un sous-agent,
et s'il ne peut pas en lancer, les rôles relisent en ligne.

</details>

## Annonce des mises à jour

Cinq compétences vérifient, avant de commencer, que vous utilisez la bonne
version de Compagnon : `/compagnon-propose`, `/compagnon-apply`,
`/compagnon-update`, `/compagnon-sync` et `/compagnon-archive`. Elles le
signalent dans deux cas :

- votre équipe exige une version de Compagnon (`compagnon_version` dans la
  configuration) que vous n'avez pas ;
- un Compagnon plus récent est disponible, et vous ne l'avez pas mis en
  sourdine.

La compétence le dit alors en une ligne, avec le lien vers la nouvelle
version, et vous demande **Update now** (mettre à jour maintenant) ou
**Later** (plus tard) avant toute autre chose :

- **Update now** installe la nouvelle version
  (`compagnon self-update --yes`), puis reprend ce que vous avez demandé ;
- **Later** met l'annonce de cette version en sourdine pendant 7 jours
  (`compagnon self-update --snooze`), puis reprend.

Quand c'est votre équipe qui exige la version, **Later** ne met rien en
sourdine : c'est l'équipe qui l'a demandé. `/compagnon-sync` et
`/compagnon-archive` s'arrêtent alors, car `compagnon sync` et
`compagnon archive` refusent de s'exécuter avec la mauvaise version
(`compagnon_version_unsatisfied`). Le message donne la correction :
`compagnon self-update`, ou la version à installer.

La question est posée une fois par demande, jamais au milieu du travail :
`/compagnon-apply` la pose avant la première tâche. `/compagnon-explore`,
`/compagnon-onboard` et `/compagnon-configure` ne la posent jamais.
`/compagnon-onboard` relaie plutôt le résultat de `compagnon doctor`, dont
la vérification `compagnon_version` dit la même chose.

<details>
<summary>Sous le capot : comment une compétence apprend qu'une version existe</summary>

Les cinq skills lisent les objets `update` et `versionRequirement` de leur
premier `compagnon status --json`. Aucune skill n'a besoin de plus que
`Bash(compagnon:*)` pour cela.

</details>

## Les corrections de l'auteur

Il vous arrive de corriger Claude sur la manière de faire de votre projet :
une convention, un choix technique, ce que vous attendez d'une partie du
plan. Quand cela arrive pendant `/compagnon-propose`, `/compagnon-apply` ou
`/compagnon-update`, Claude note la correction une fois, dans le
[`corrections.md`](reference/file-formats.md#correctionsmd) de la
modification : où c'était, ce qu'il avait fait, ce que vous avez demandé,
une ligne chacun. Il vous montre ensuite l'entrée, pour que vous puissiez
la faire modifier ou retirer avant l'enregistrement (commit).
`compagnon review` liste les corrections, et `compagnon validate` vérifie
leur fichier.

Claude ne note qu'une consigne qui vaudrait aussi pour la modification
suivante. Il ne note pas une préférence ponctuelle sur cette modification,
une faute de frappe, un changement de périmètre ni un malentendu sur votre
demande.

L'endroit où elle va dépend de qui elle concerne :

- une consigne sur le projet va dans `corrections.md`, que l'équipe partage
  par git, et non dans la mémoire personnelle de Claude Code ;
- une préférence sur la façon dont Claude travaille avec vous reste dans
  cette mémoire personnelle.

Une entrée ne nomme personne, et ne cite ni donnée personnelle ni secret.

## Les compétences en détail

### Propose (`/compagnon-propose`)

Cette compétence transforme votre demande en plan complet, en quatre
parties : la proposition, les modifications des spécifications (appelées
[deltas](glossary.md#delta)), la conception et les tâches.

Avant d'écrire, elle lit le code que votre demande concerne, et fonde le
périmètre sur ce qu'elle y trouve. Si votre demande contredit les
spécifications existantes, elle vous le dit au lieu de trancher seule. Elle
rédige tout le plan, son texte comme ses mots-clés de structure, dans
[la langue des plans de votre projet](guides/artifact-language.md),
exactement comme ses instructions les donnent.

Une fois la proposition écrite, Compagnon donne à la compétence la liste
des exigences existantes que la modification touche probablement. La
compétence les lit, et modifie celles que la modification altère, au lieu
d'en ajouter de nouvelles à côté. Cette liste est un point de départ : il
peut en manquer.

Votre demande n'autorise que la planification, même si elle dit
« construis » ou « corrige ». La compétence s'arrête une fois le plan
rédigé, et attend votre retour.

**Qui écrit quoi.** Chaque partie du plan est rédigée par un rôle, avec des
critères « Done when » (terminé quand) que Claude vérifie avant de passer à
la suite :

- le product owner rédige la proposition, y compris sa note de version pour
  les utilisateurs, classée par catégorie, ou `None` pour une modification
  que les utilisateurs ne verront pas ;
- l'analyste QA rédige les spécifications ;
- l'architecte rédige la conception ;
- le tech lead rédige les tâches. Ses critères exigent une tâche qui met à
  jour la documentation concernée, et un test automatisé pour chaque
  scénario, qui suit les conventions de test de votre projet.

Ces rôles viennent du [schéma](glossary.md#schéma) qu'utilise votre
projet. Un [schéma personnalisé](guides/custom-schemas.md) définit les
siens.

**La relecture avant que vous le voyiez.** Avant de vous présenter le plan,
la compétence le relit en deux étapes :

1. **Traçabilité** : elle vérifie que rien ne manque entre les parties.
   Chaque exigence a ses scénarios, chaque scénario une tâche avec un test
   automatisé, et la note de version concorde avec le plan. Les manques
   sont corrigés dans le plan.
2. **Relecture contradictoire** : la compétence relit le plan comme le
   ferait un relecteur sceptique : est-ce utile, le périmètre est-il juste,
   les décisions tiennent-elles ? Des experts rejoignent cette relecture
   pour les domaines que le plan touche, comme la sécurité ou
   l'accessibilité. Chaque constat est noté, avec sa gravité et sa preuve.
   L'auteur le corrige ou le conteste, et le relecteur vérifie chaque
   réponse une fois. Cette relecture n'élargit jamais le périmètre.

Le résumé de la compétence se termine alors par deux listes. Les **points
à challenger** : au plus cinq points d'une ligne, tous rôles confondus, les
plus graves d'abord, chacun marqué du rôle qui l'a relevé : identifiant du
constat, statut, quoi, pourquoi c'est important, quelle partie du plan.
S'il n'y en a aucun, le résumé le dit. **À décider** : les contestations
que le relecteur a maintenues, et les constats graves encore ouverts.
Commencez votre propre relecture par là.

**Les tickets.** Si votre demande mentionne un numéro de ticket comme
`PROJ-123` (de la forme `[A-Z]{2,}-\d+`) et qu'un outil MCP Jira est
configuré, la compétence lit le premier ticket : en lecture seule, en un
seul appel. Elle le cite ensuite en tête de la proposition. Voir
[Jira et autres serveurs MCP](guides/mcp.md).

Rôles : analyste, enquêteur, product owner, analyste QA, architecte, tech
lead, contrôleur de traçabilité et contradicteur — voir
[Rôles intégrés](roles.md#rôles-intégrés).

<details>
<summary>Sous le capot : les rôles de chaque étape et les deux relectures en détail</summary>

Le rôle de chaque artefact est déclaré par le schéma et renvoyé par
`compagnon instructions`. À chaque étape, la skill demande
`compagnon roles --at <step>` les rôles de l'étape et applique leurs
critères, y compris les critères ajoutés par votre projet ; une étape dont
votre configuration désactive le rôle est sautée, et le résumé le signale
(voir [Configurer les rôles](roles.md#configurer-les-rôles)).

1. **Traçabilité** — chaque capacité a son fichier de spec, chaque exigence
   un scénario nominal et un scénario d'erreur ou de cas limite, chaque
   scénario une tâche dont la vérification le couvre par un test
   automatisé, une tâche met à jour la documentation lorsque la section
   Impact touche un comportement documenté, et la
   [note de version](reference/file-formats.md#release-note) de la proposal
   concorde avec le plan — chaque élément incompatible de What Changes, et
   chaque exigence supprimée ou modifiée de façon incompatible, a son élément
   **BREAKING**. Les manques sont corrigés dans les artefacts.
2. **Relecture contradictoire** — la skill relit le plan en relecteur
   sceptique : besoin, périmètre, specs, décisions, hypothèses, tâches,
   et si `## Domains` coche chaque domaine que le plan touche, ainsi que
   les [`rules`](reference/configuration.md#rules) de votre projet. Le
   contradicteur ne relit lui-même aucun domaine : sécurité, données
   personnelles, migration, exploitation, performance et accessibilité
   reviennent à leurs experts. Les
   [experts](roles.md#experts-et-domaines) dont la proposal coche le
   domaine dans `## Domains` — sécurité, données personnelles,
   accessibilité, performance, exploitation, migration — rejoignent cette
   relecture avec leurs propres connaissances. Avant d'écrire
   `## Domains`, la skill lit les
   [domaines que suggère le diff du change](roles.md#les-déclencheurs-et-les-domaines-suggérés)
   et coche chacun ou le garde non coché avec une raison ; le contradicteur
   consigne un constat pour chacun resté `unticked`. Chaque constat est écrit
   dans le [compte rendu de revue](guides/review-record.md) du change, avec
   sa gravité et sa preuve citée ; l'auteur le corrige ou le conteste, le
   relecteur vérifie chaque réponse une fois, et la relecture n'élargit
   jamais le périmètre. En [mode isolé](roles.md#revue-en-ligne-ou-isolée),
   le contrôleur de traçabilité et chaque relecteur travaillent comme des
   sous-agents distincts qui ne voient jamais la conversation, et la skill
   fusionne leurs constats avant de les consigner.

</details>

### Explore (`/compagnon-explore`)

Un partenaire de réflexion pour les demandes floues, les enquêtes et les
comparaisons d'approches. Cette compétence lit le code et les
spécifications, mais ne crée ni modification, ni plan, ni fichier. Une fois
l'idée claire, elle propose `/compagnon-propose`.

Posture : [partenaire de réflexion](roles.md#partenaire-de-réflexion-compagnon-explore).

### Onboard (`/compagnon-onboard`)

Une visite guidée pour qui découvre Compagnon. Elle ne modifie rien. Elle
vous dit ce que fait l'outil, ce que contient le projet (spécifications,
modifications en cours, décisions), et la seule chose à faire ensuite : par
exemple `/compagnon-configure` sur un projet tout juste installé.

Elle lance aussi une vérification de votre installation,
`compagnon doctor --json`, et liste jusqu'à cinq éléments manquants avec la
façon de corriger chacun. Elle n'applique elle-même aucune de ces
corrections.

Posture : [guide](roles.md#guide-compagnon-onboard).

### Configure (`/compagnon-configure`)

Cette compétence lit votre README, votre guide de contribution, votre
documentation et un échantillon de votre code. Elle en tire une
description de votre projet (le bloc `context:`) et des règles pour chaque
partie du plan (`rules:`), pour `_compagnon/config.yaml`. Elle vous montre
la différence, et n'écrit qu'après votre accord. Elle ne touche jamais à
`schema`, `language`, `workflows`, `mcp` ni `inherits`, et garde vos
commentaires existants.

Ses règles pour les tâches (`tasks`) comprennent des règles de test pour
votre langage de programmation : outil de test, emplacement des tests,
nommage, commandes. Elle les tire de vos tests existants. C'est seulement
lorsque le projet n'a encore aucun test qu'elle se rabat sur les
conventions courantes de Rust, TypeScript/JavaScript, Python, Go ou Java.

Tant que la configuration ne comporte aucune `rules:`, `compagnon init` et
`compagnon status` recommandent de lancer cette compétence.

Posture : [analyste de configuration](roles.md#analyste-de-configuration-compagnon-configure).

### Apply (`/compagnon-apply`)

Cette compétence réalise une modification planifiée. Elle procède ainsi :

1. Elle vérifie que le plan est complet.
2. Elle prend dans l'ordre chaque tâche non cochée de `tasks.md`, la
   réalise, la vérifie, et ne la coche qu'une fois la vérification réussie.
3. Elle écrit les tests à la manière de vos tests existants, puis selon les
   conventions de votre langage et vos `rules`. Elle trouve la commande de
   test dans votre configuration, votre manifeste ou votre CI, et vous la
   demande si elle n'en trouve aucune.
4. Une fois toutes les tâches cochées, elle relit le code touché par la
   modification comme le ferait un nouvel arrivant, rejointe par les
   experts des domaines que coche la proposition. Elle simplifie ce qui ne
   fait pas de doute, sans élargir le périmètre, et signale au plus cinq
   notes de relecture, chacune marquée de son rôle. Un constat grave de
   sécurité ou de données personnelles est toujours signalé.
5. Enfin, elle lance une fois la suite de tests complète.

Chaque constat va dans le [compte rendu de revue](guides/review-record.md),
et suit la même boucle de réponse que ceux du plan. En mode isolé, chaque
relecteur travaille d'abord seul comme sous-agent, et la compétence
fusionne leurs constats. Le résumé liste ce qu'il vous revient de décider,
y compris tout domaine que le code suggère désormais et que la proposition
ne coche ni ne justifie : cochez-le, et son expert relit le code, ou
justifiez-le.

Un expert corrige un défaut manifeste dans le périmètre de la modification,
avec un test. Tout autre changement de comportement, il le signale au lieu
de le faire. Les critères et les connaissances de ces deux étapes viennent
de `compagnon roles` ; lorsqu'aucun rôle n'est actif pour la relecture,
celle-ci est sautée et le résumé le signale.

La compétence s'arrête au premier blocage ou au premier doute. Elle ne
touche jamais à une autre modification, ne synchronise jamais et n'archive
jamais.

Rôles : [développeur](roles.md#développeur-compagnon-apply),
[nouvel arrivant et experts](roles.md#nouvel-arrivant-et-experts-compagnon-apply).

### Update (`/compagnon-update`)

Cette compétence révise une partie d'un plan qui existe déjà : par
exemple, une exigence que vous voulez changer après avoir lu le plan. Elle
cherche ensuite les répercussions sur les autres parties et vous les
signale.

Elle ne rédige pas les parties manquantes (c'est le rôle de
`/compagnon-propose`), ne modifie pas le code (celui de `/compagnon-apply`)
et ne touche pas aux modifications archivées. Elle se termine en vérifiant
le plan avec `compagnon validate`.

Posture : [éditeur](roles.md#éditeur-compagnon-update).

### Sync (`/compagnon-sync`)

Cette compétence reporte les spécifications d'une modification dans les
spécifications du projet, sans archiver la modification. Elle lance
`compagnon sync`, puis vous dit quelles spécifications du projet ont été
créées, mises à jour ou laissées inchangées.

Posture : [mainteneur des spécifications vivantes](roles.md#mainteneur-des-specs-vivantes-compagnon-sync-compagnon-archive).

### Archive (`/compagnon-archive`)

Cette compétence clôt une modification terminée : elle lance
`compagnon archive`. Si la vérification trouve des erreurs, elle s'arrête
et vous renvoie vers `compagnon validate`, au lieu de chercher à les
contourner. Si le compte rendu de revue bloque l'archivage
(`review_blocked`), elle vous renvoie vers `compagnon review`, et ne
consigne jamais une décision à votre place.

Posture : [mainteneur des spécifications vivantes](roles.md#mainteneur-des-specs-vivantes-compagnon-sync-compagnon-archive).

## Choisir les compétences à installer

Par défaut, les huit compétences sont installées. Pour en installer moins,
vous avez deux façons de faire :

- choisir le préréglage **minimal** à l'installation de Compagnon :
  `propose`, `explore`, `onboard` et `configure` ;
- ou lister les compétences souhaitées sous `workflows:` dans
  `_compagnon/config.yaml` :

```yaml
workflows:
  - propose
  - explore
  - apply
  - archive
```

Lancez ensuite `compagnon update`. Un nom que Compagnon ne connaît pas
produit un avertissement et est ignoré. `compagnon update` ne supprime pas
les compétences que vous ne listez plus : supprimez vous-même leurs
dossiers dans `.claude/skills/`.

## Modifier une compétence générée

Vous pouvez modifier à la main le `SKILL.md` d'une compétence.
`compagnon update` s'en aperçoit, et laisse votre fichier modifié en place
au lieu de l'écraser. `compagnon update --force` le remplace par la version
générée : vérifiez d'abord vos modifications (`git diff`), car il les
écrase, puis réappliquez-les.

<details>
<summary>Sous le capot : comment Compagnon repère une modification à la main</summary>

Chaque `SKILL.md` généré enregistre la version de Compagnon qui l'a produit et
une empreinte (`checksum`) de son propre contenu. Si vous modifiez une skill
à la main, son contenu ne correspond plus à son empreinte : `compagnon update`
détecte la modification et laisse le fichier en place :

```text
  Left in place because edited by hand — rerun with --force to overwrite:
    /home/you/acme-app/.claude/skills/compagnon-apply/SKILL.md
```

`compagnon update --force` l'écrase avec la version générée. Les skills issues
d'une autre version de Compagnon sont toujours régénérées, tout comme les skills
de cette version que Compagnon a rendues lui-même — une compilation antérieure, ou
l'autre mode de revue —, puisqu'elles correspondent encore à leur empreinte.
Une skill de cette version générée avant les empreintes est préservée comme
une modification : lancez une fois `compagnon update --force` pour lui donner son
empreinte — vérifiez d'abord `git diff`, car il écrase aussi vos vraies
modifications, puis réappliquez-les.

</details>
