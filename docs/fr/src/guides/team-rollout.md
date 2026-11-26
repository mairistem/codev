# Déployer Compagnon pour une équipe

Cette page s'adresse à la personne qui installe Compagnon pour une équipe :
tout le monde installe la même version, le [dépôt](../glossary.md#dépôt)
contient ce que l'équipe partage, une vérification bloque une
[demande de fusion](../glossary.md#demande-de-fusion) dont le plan n'est
pas valide, et chacun sait qui approuve quoi.

## L'installer sur chaque poste

Chaque développeur qui fait coder Claude a besoin de la commande
`compagnon` et de Claude Code. Les scripts d'installation ne demandent
aucun outil de développement :

```bash
# macOS et Linux
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh
```

```powershell
# Windows, dans PowerShell
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

`COMPAGNON_VERSION=0.6.1` placé avant `sh` installe une version précise
(sous Windows, tapez d'abord `$env:COMPAGNON_VERSION = '0.6.1'`) ;
le téléchargement manuel et `cargo install` sont les deux autres voies.
Voir [Installation](../installation.md).

Les personnes qui ne font que relire les plans n'installent rien : elles
lisent et approuvent le plan dans la demande de fusion, sur GitHub ou
GitLab — voir [Approuver un plan sur GitHub ou GitLab](approve-a-plan.md).

## Une seule version pour tous

Indiquez les versions que l'équipe accepte dans `_compagnon/config.yaml` :

```yaml
compagnon_version: ">=0.6, <0.8"
```

Pour exiger une seule version précise, écrivez `"=0.6.1"`.

Mettez la valeur entre guillemets : YAML lit `>` de façon particulière.
Avec une version hors de cet intervalle :

- les compétences qui agissent sur le projet (`/compagnon-propose`,
  `/compagnon-apply`, `/compagnon-update`, `/compagnon-sync`,
  `/compagnon-archive`) le signalent avant de commencer et proposent
  de mettre à jour maintenant (**Update now**) ou plus tard (**Later**) ;
  quand l'équipe exige la version, « plus tard » ne fait pas taire
  l'alerte ;
- `compagnon validate`, `compagnon sync` et `compagnon archive` refusent de
  s'exécuter, avec `compagnon_version_unsatisfied` et le correctif :
  `compagnon self-update`, ou `compagnon self-update --version <X.Y.Z>`
  quand la version installée est trop récente ;
- `compagnon doctor` signale la vérification `compagnon_version` comme
  manquante.

La tâche d'[intégration continue](../glossary.md#intégration-continue) que
`compagnon init` écrit installe une version fixée (`COMPAGNON_VERSION=`
dans `.github/workflows/compagnon.yml`). Quand vous relevez
`compagnon_version` au-dessus, relevez cette version dans le même
[enregistrement](../glossary.md#enregistrement). Compagnon 0.5 et les
versions antérieures ne connaissent pas cette clé et refusent le fichier :
ajoutez-la une fois que tout le monde utilise une version plus récente.
Voir [`compagnon_version`](../reference/configuration.md#compagnon_version)
et [Annonce des mises à jour](../skills.md#annonce-des-mises-à-jour).

## Ce qu'il faut enregistrer

Lancez `compagnon init` une fois, sur un seul poste, et enregistrez ce
qu'il a écrit :

| Chemin | Pourquoi |
|---|---|
| `_compagnon/` | les spécifications, les décisions, les modifications et la configuration, y compris `_compagnon/decisions/seal.yaml` et `_compagnon/compagnon.lock` |
| `.claude/skills/` | chacun reçoit les mêmes compétences sans lancer `compagnon init` |
| `CLAUDE.md` | demande à Claude Code, à chaque session, de planifier avant de changer un comportement |
| `.github/workflows/compagnon.yml` | la vérification à chaque envoi et à chaque demande de fusion (GitHub) |
| le modèle de demande de fusion (`.github/PULL_REQUEST_TEMPLATE.md` ou le vôtre) | la ligne où le développeur nomme la modification (GitHub) |

Après une mise à jour de Compagnon, `compagnon update` rafraîchit les
compétences, les sections gérées et la version fixée dans l'intégration
continue : enregistrez aussi le résultat. Voir
[Faut-il enregistrer `_compagnon/` et `.claude/skills/` dans
git ?](../faq.md#faut-il-enregistrer-_compagnon-et-claudeskills-dans-git-) dans la FAQ.

## La vérification qui bloque une demande de fusion

Sur GitHub, `.github/workflows/compagnon.yml` lance
`compagnon validate --strict` à chaque envoi et à chaque demande de
fusion : une spécification invalide, une modification invalide ou une
décision acceptée retouchée la fait échouer. Sur GitLab, collez dans
`.gitlab-ci.yml` la tâche que `compagnon init` affiche ; Compagnon ne
modifie jamais ce fichier. Ailleurs, ajoutez `compagnon validate --strict`
à votre chaîne d'intégration.

Une vérification en échec ne bloque la fusion qu'une fois rendue
obligatoire, dans les réglages de votre site :

- **GitHub :** une règle de protection de la branche principale (ou un
  jeu de règles) qui exige que les vérifications passent (« Require status
  checks to pass before merging »), avec la vérification
  `compagnon validate`.
- **GitLab :** dans les réglages des demandes de fusion du projet,
  l'option qui exige une chaîne d'intégration réussie (« Pipelines must succeed », ou sa
  traduction si GitLab est en français).

Vous pouvez aussi lancer `compagnon doctor --strict` dans l'intégration
continue : il échoue quand une vérification de l'installation manque — une
compétence pas à jour, la section de `CLAUDE.md` retirée — et jamais sur un
avertissement. Un nouveau projet commence sans spécification, ce que
`doctor` signale comme manquant : ajoutez cette étape une fois que
`compagnon doctor` ne signale plus rien. Voir
[Intégration au dépôt et compagnon doctor](repository-integration.md).

## Qui approuve quoi

Compagnon s'arrête aux moments où une personne décide ; il ne sait pas qui,
dans votre équipe, doit être cette personne. Une règle qui fonctionne :

| Quoi | Qui approuve | Où |
|---|---|---|
| Le plan : proposition, spécifications, conception, tâches | le responsable produit ou l'expert du domaine, avec le développeur | dans la demande de fusion, avant que Claude construise — voir [Approuver un plan sur GitHub ou GitLab](approve-a-plan.md) |
| Le code | les développeurs, comme pour toute demande de fusion | dans la demande de fusion, une fois les tâches faites |
| Une [décision](../concepts.md#décision) | l'équipe | acceptée avec `compagnon decision accept` dans une demande de fusion que relisent ceux qu'elle engage |
| Un risque que la relecture a laissé ouvert | la personne qui en répond | la compétence pose la question et note la réponse avec le nom de cette personne |

C'est une suggestion, pas une règle que Compagnon impose : ce sont les
règles de votre site — approbations obligatoires, un fichier `CODEOWNERS`
qui désigne qui doit approuver `_compagnon/changes/` ou
`_compagnon/decisions/` — qui la rendent contraignante. La page
[Rôles](../roles.md#là-où-vous-décidez) liste chaque moment où Compagnon
passe la main.

## Les rôles de relecture

Avant qu'une personne lise le plan ou le code, les
[rôles](../roles.md) de relecture de Compagnon l'ont déjà mis à l'épreuve :
produit, qualité, architecture, et des experts en sécurité, données
personnelles, accessibilité, performance, exploitation et migration, qui
interviennent quand la proposition d'une modification coche leur domaine.
Leurs constats et les réponses sont dans le fichier `review.md` de la
modification.

Deux réglages de `_compagnon/config.yaml` décident de leur fonctionnement,
pour toute l'équipe :

- `roles:` active ou désactive un rôle, ou lui ajoute vos propres
  critères — par exemple `security: always` pour que chaque modification
  soit relue sous l'angle de la sécurité, ou `accessibility: never` sans
  interface utilisateur. Voir
  [Configurer les rôles](../roles.md#configurer-les-rôles).
- `review: { mode: isolated }` fait travailler chaque rôle de relecture
  seul, sans voir la conversation ; `inline`, le réglage par défaut, les
  enchaîne dans la même conversation. Voir
  [`review`](../reference/configuration.md#review).

## Pour aller plus loin

- [Intégration au dépôt et compagnon doctor](repository-integration.md) :
  chaque fichier qu'écrit `compagnon init`, et chaque vérification de
  `compagnon doctor`.
- [Rôles](../roles.md) : les critères de chaque rôle, à chaque étape.
- [Configuration](../reference/configuration.md) : chaque réglage.
- [Sources héritées](inherited-sources.md) : partager règles et décisions
  entre les projets de l'équipe.
