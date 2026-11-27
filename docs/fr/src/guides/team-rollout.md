# Déployer Compagnon pour une équipe

Cette page s'adresse à la personne qui installe Compagnon pour une équipe :
tout le monde installe la même version, le [dépôt](../glossary.md#dépôt)
contient ce que l'équipe partage, une vérification bloque une
[demande de fusion](../glossary.md#demande-de-fusion) dont le plan n'est
pas valide, et chacun sait qui approuve quoi.

## L'installer sur chaque poste

Chaque développeur qui fait coder Claude a besoin de la commande
`compagnon` et de Claude Code. [Installation](../installation.md) donne les
commandes d'installation en une ligne pour macOS, Linux et Windows, et la
façon d'installer une version précise (`COMPAGNON_VERSION=0.6.1`).

Les personnes qui ne font que relire les plans n'installent rien : elles
lisent et approuvent le plan dans la demande de fusion, sur GitHub ou
GitLab — voir [Approuver un plan sur GitHub ou GitLab](approve-a-plan.md).

## Une seule version pour tous

Indiquez les versions que l'équipe accepte dans `_compagnon/config.yaml` :

```yaml
compagnon_version: ">=0.6, <0.8"
```

Pour exiger une seule version précise, écrivez `"=0.6.1"`. Mettez la
valeur entre guillemets : YAML lit `>` de façon particulière. Avec une
version hors de cet intervalle, les compétences proposent de mettre à jour
avant de commencer, et `compagnon validate`, `compagnon sync` et
`compagnon archive` refusent de s'exécuter tant que le développeur n'a pas
mis à jour — voir
[`compagnon_version`](../reference/configuration.md#compagnon_version).

La tâche d'[intégration continue](../glossary.md#intégration-continue) que
`compagnon init` écrit installe une version fixée (`COMPAGNON_VERSION=`
dans `.github/workflows/compagnon.yml`). Quand vous relevez
`compagnon_version` au-dessus, relevez cette version dans le même
[enregistrement](../glossary.md#enregistrement). Compagnon 0.5 et les
versions antérieures ne connaissent pas cette clé et refusent le fichier :
ajoutez-la une fois que tout le monde utilise une version plus récente.
Voir [Annonce des mises à jour](../skills.md#annonce-des-mises-à-jour).

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

La vérification que met en place `compagnon init` fait échouer un envoi ou
une demande de fusion dont une spécification ou une modification est
invalide, ou qui retouche une décision acceptée ; sur GitLab, vous collez
la tâche qu'affiche `compagnon init`. Voir
[Intégration au dépôt et compagnon doctor](repository-integration.md).

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
`compagnon doctor` ne signale plus rien.

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

Avant qu'une personne lise le plan ou le code, les rôles de relecture de
Compagnon l'ont mis à l'épreuve, et leurs constats sont dans le fichier
`review.md` de la modification. Quels rôles s'exécutent, avec quels
critères et dans quel mode, se règle pour toute l'équipe dans
`_compagnon/config.yaml` — voir [Rôles](../roles.md).

## Pour aller plus loin

- [Intégration au dépôt et compagnon doctor](repository-integration.md)
- [Rôles](../roles.md)
- [Configuration](../reference/configuration.md)
- [Sources héritées](inherited-sources.md)
- [Approuver un plan sur GitHub ou GitLab](approve-a-plan.md)
