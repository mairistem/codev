# Design : `/codev-sync` et `/codev-archive`

## Contexte

Voir `proposal.md` pour la motivation. Le pattern est celui déjà éprouvé par
`propose`, `explore` et `apply` : deux couples {entrée dans `CATALOG`,
fichier markdown sous `assets/workflows/`}. La différence de nature avec
`apply` est que ces skills n'écrivent rien elles-mêmes — elles délèguent
l'intégralité de l'effet au binaire codev, dont le pattern « plan puis
exécution » cadré par la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md)
garantit l'atomicité côté disque.

## Objectifs / Hors objectifs

Ce design cadre :

- le contenu des deux skills (structure, garde-fous, gestion des erreurs
  remontées par le CLI) ;
- le contrat `allowed-tools` restreint et pourquoi il l'est ;
- le test d'invariant qui verrouille la restriction.

Il ne cadre **pas** l'ajout à `DEFAULT_WORKFLOWS`, ni la skill `update`, ni
un parsing du JSON de sortie des commandes.

## Décisions

### Décision : `allowed-tools` = `Bash(codev:*), Read`

Le seul effet de ces skills est un appel `codev sync` ou `codev archive` —
le reste n'est que texte affiché à l'utilisateur. Aucun `Write`, aucun
`Edit`, aucun `Bash` général. Le `Read` reste utile pour répondre à une
question de contexte de l'utilisateur (« que dit `tasks.md` ? »), sans jamais
écrire.

**Rationale** : chaque outil supplémentaire dans `allowed-tools` élargit ce
que la skill peut faire par erreur. Restreindre est plus sûr que d'ouvrir
« au cas où ». L'invariant testable — que le `Bash` général n'apparaît que
dans `apply` — devient la garantie structurelle.

**Alternative écartée** : ajouter `Bash(git:*)` pour que la skill puisse
suggérer un `git status` après archive. Reportable — l'utilisateur sait
lancer `git status` seul. Si le besoin monte, on ouvrira à ce moment-là,
pas avant.

### Décision : la skill lit le JSON, pas la sortie humaine

`codev sync` et `codev archive` produisent tous deux un `--json` structuré,
contrat versionné et testé par snapshot (`SyncReportV1`, `ArchiveReportV1`).
La skill l'invoque et lit cette forme plutôt que le texte humain.

**Rationale** :

1. Le contrat JSON est **précisément fait pour être consommé** — c'est son
   raison d'être. Le tester par snapshot dans `contract::tests` sans avoir
   de consommateur premier serait un gâchis d'invariant.
2. Le rendu structuré permet à la skill de composer proprement — « ✓ 2 specs
   créées, 1 mise à jour, 0 inchangée, déplacé vers … » plutôt que de
   relayer un bloc de texte à trous.
3. Le code stable dans `status[0].code` est directement testable côté agent —
   `validation_failed` déclenche exactement la branche « renvoie vers
   `codev validate` », sans à devoir grepper le message humain.

**Alternative écartée** : lire le texte humain. Séduisant pour son couplage
faible, mais fragile en pratique : n'importe quelle reformulation par
gentillesse ferait dériver l'interprétation. Le JSON est stable *par
contrat*, et c'est cette stabilité qui vaut d'être exploitée.

**Coût accepté** : la skill devient dépendante de la forme du JSON. Le
contrat étant versionné (v1), un changement de forme demandera une nouvelle
version — c'est justement ce que le versioning garantit.

### Décision : `archive` renvoie explicitement vers `validate` en cas de refus

Quand `codev archive` refuse pour cause de pré-flight validate, la skill ne
retente pas, ne devine pas, ne « corrige » pas. Elle dit exactement :
« Le change a des erreurs de validation ; lance `codev validate <nom>` pour
voir le détail. »

C'est cohérent avec le design de `codev archive` lui-même, qui refuse déjà
de dupliquer les messages de règles. La skill perpétue cette division du
travail : `codev archive` traite le geste, `codev validate` explique.

### Décision : `sync` termine par une invitation non-injonctive à archiver

Quand la fusion a produit du changement (au moins un `created` ou
`updated`), la skill ajoute en fin de rendu **une seule ligne** :

> Le change est prêt à être archivé si tu veux clore le cycle.

Elle **ne suggère rien** quand la fusion est un no-op (tout `unchanged`) —
il n'y a alors rien de nouveau à archiver que ce qui l'était déjà.

**Rationale** : l'archive est le pas naturel après un sync qui a modifié
les specs principales, et rappeler l'existence de `/codev-archive` évite à
l'utilisateur d'avoir à se souvenir seul du prochain verbe. La formulation
« si tu veux » retire le paternalisme — c'est un rappel, pas une injonction.

**Coût accepté** : la skill devient sensible au champ `updated`/`created`
du rapport JSON. C'est aligné avec la décision précédente : puisqu'on lit
le JSON, autant s'en servir pour prendre cette décision de rendu.

## Risques et compromis

- **Un utilisateur pourrait invoquer `/codev-archive` sans avoir lu le
  résultat de son `/codev-apply`**. → **Compromis assumé** : le pré-flight
  `validate` intégré à `codev archive` remonte les erreurs, et le refus
  d'archiver interrompt le processus. Un archivage à l'aveugle qui échoue
  est plus sûr qu'un archivage qui passerait en silence.
- **Deux changes actifs déclarant `skills` comme « nouvelle capacité »** —
  cf. proposal. À l'archivage, le premier crée la spec, le second l'enrichit
  par ADDED. Rien à faire côté design. Un test unitaire du merger couvre
  déjà le cas « ADDED sur main spec existante ».

## Plan de migration

Sans objet — deux nouvelles skills. Un projet existant qui a déjà
`workflows: [propose, explore, apply]` doit ajouter `- sync` et `- archive`,
puis relancer `codev update`.
