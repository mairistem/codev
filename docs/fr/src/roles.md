# Rôles

Une compétence Compagnon est un seul agent, mais il n'aborde pas chaque étape dans le
même état d'esprit. Chaque étape reçoit un rôle : une question dont il a la
charge, et des critères de fin (« Done when ») que l'agent vérifie avant de passer à
la suite. Vous et votre équipe validez à des moments définis — l'agent ne
décide jamais à votre place.

Les rôles sont des critères, pas un jeu de rôle. L'agent ne se fait pas
passer pour un responsable produit (product owner) : il vérifie la proposition à l'aide de la liste
du responsable produit. Un rôle dont les critères sont remplis est terminé, quel
que soit le ton de la prose. Certains rôles portent aussi des
**connaissances** — les signaux concrets, les pièges par langage et les
références dont ils ont besoin pour bien vérifier leurs critères — voir
[Experts et domaines](#experts-et-domaines).

Les rôles sont des données, pas de la prose enfouie dans les compétences. Compagnon
les sert : à chaque étape, les compétences demandent `compagnon roles --at <step>` et
appliquent ce qui leur revient, et `compagnon instructions` transmet à l'agent
le rôle de chaque artefact. Votre projet peut désactiver un rôle, ajouter
ses propres critères et changer l'activation pour une seule modification — voir
[Configurer les rôles](#configurer-les-rôles).

## Rôles intégrés

| Id | Nom | Nature | Étape | Sa question |
|---|---|---|---|---|
| `analyst` | Analyste | étape | `plan.understand` | Que faut-il construire ? |
| `investigator` | Enquêteur | étape | `plan.investigate` | Que fait déjà le projet ? |
| `product` | Product owner | artefact (`proposal`) | `plan.write` | Pourquoi cette modification, et qu'est-ce qui change ? |
| `qa` | Analyste QA | artefact (`specs`) | `plan.write` | Quel comportement observable doit tenir ? |
| `architect` | Architecte | artefact (`design`) | `plan.write` | Comment le construire ? |
| `lead` | Tech lead | artefact (`tasks`) | `plan.write` | Dans quel ordre, et comment vérifier chaque étape ? |
| `tracer` | Contrôleur de traçabilité | étape | `plan.trace` | Les artefacts concordent-ils entre eux ? |
| `skeptic` | Contradicteur | étape | `plan.challenge` | Le plan résisterait-il à un relecteur qui ne l'a pas écrit ? |
| `security` | Sécurité | expert (domaine `security`) | `plan.challenge`, `code.review` | Un attaquant peut-il détourner ce que la modification ajoute ? |
| `privacy` | Données personnelles | expert (domaine `personal-data`) | `plan.challenge`, `code.review` | La modification ne collecte-t-elle, ne conserve-t-elle et ne partage-t-elle des données personnelles que dans la mesure nécessaire ? |
| `accessibility` | Accessibilité | expert (domaine `user-interface`) | `plan.challenge`, `code.review` | Tout le monde peut-il utiliser ce que la modification ajoute, au clavier ou avec une technologie d'assistance ? |
| `performance` | Performance | expert (domaine `performance`) | `plan.challenge`, `code.review` | La modification reste-t-elle rapide quand les données et la charge augmentent ? |
| `ops` | Exploitation | expert (domaine `operations`) | `plan.challenge`, `code.review` | L'équipe peut-elle exploiter, observer et annuler ce que la modification ajoute ? |
| `migration` | Migration | expert (domaine `migration`) | `plan.challenge`, `code.review` | Les données, la configuration et les consommateurs existants peuvent-ils passer à cette modification sans risque ? |
| `developer` | Développeur | étape | `code.implement` | Chaque tâche fonctionne-t-elle, vérification à l'appui ? |
| `newcomer` | Nouvel arrivant | étape | `code.review` | Un développeur qui n'a pas vu le plan peut-il suivre ce code ? |

Les rôles d'étape et les experts sont intégrés à Compagnon. Les rôles
d'artefact viennent du schéma par défaut `spec-driven`, qui déclare un rôle
par artefact ; un [schéma personnalisé](guides/custom-schemas.md) déclare
les siens, ou aucun.

Pour lire les critères « Done when » d'un rôle, interrogez Compagnon — c'est la
liste que reçoit l'agent :

```bash
compagnon roles --at plan.challenge
```

Sans `--at`, `compagnon roles` liste tous les rôles avec leur activation ;
ajoutez `--knowledge` pour lire aussi les connaissances d'un rôle. Voir
[`compagnon roles`](reference/cli.md#compagnon-roles).

## Étapes

Une modification passe par sept étapes, dans deux compétences. À chacune,
certains rôles rédigent ou construisent, d'autres relisent, et vous
décidez après les relectures :

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 406" role="img" aria-label="Qui relit quand. Les étapes d'une modification, de gauche à droite : plan.understand, plan.investigate, plan.write, plan.trace et plan.challenge dans /compagnon-propose, puis code.implement et code.review dans /compagnon-apply. Les rôles, de haut en bas : l'analyste travaille à plan.understand ; l'enquêteur à plan.investigate ; les quatre rôles d'artefact rédigent à plan.write ; le contrôleur de traçabilité relit à plan.trace ; le contradicteur relit à plan.challenge ; les six experts relisent à plan.challenge et à code.review quand la modification coche leur domaine ; le développeur travaille à code.implement ; le nouvel arrivant relit à code.review ; vous validez le plan après plan.challenge et tranchez ce qui reste après code.review.">
<text x="365" y="20" text-anchor="middle" class="mono" font-size="13" style="fill:var(--dg-muted)">/compagnon-propose</text>
<line x1="154" y1="30" x2="576" y2="30" style="stroke:var(--dg-line)" stroke-width="2"/>
<text x="666" y="20" text-anchor="middle" class="mono" font-size="13" style="fill:var(--dg-muted)">/compagnon-apply</text>
<line x1="584" y1="30" x2="748" y2="30" style="stroke:var(--dg-line)" stroke-width="2"/>
<text x="193" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">plan.</text>
<text x="193" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">understand</text>
<text x="279" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">plan.</text>
<text x="279" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">investigate</text>
<text x="365" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">plan.</text>
<text x="365" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">write</text>
<text x="451" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">plan.</text>
<text x="451" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">trace</text>
<text x="537" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">plan.</text>
<text x="537" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">challenge</text>
<text x="623" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">code.</text>
<text x="623" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">implement</text>
<text x="709" y="50" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">code.</text>
<text x="709" y="66" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-ink)">review</text>
<rect x="4" y="78" width="752" height="30" rx="6" style="fill:var(--dg-box)"/>
<text x="14" y="98" font-size="14" style="fill:var(--dg-ink)">Analyste</text>
<text x="14" y="128" font-size="14" style="fill:var(--dg-ink)">Enquêteur</text>
<rect x="4" y="138" width="752" height="30" rx="6" style="fill:var(--dg-box)"/>
<text x="14" y="158" font-size="14" style="fill:var(--dg-ink)">Rôles d'artefact (4)</text>
<text x="14" y="181" font-size="13" style="fill:var(--dg-ink)">Contrôleur de</text>
<text x="14" y="195" font-size="13" style="fill:var(--dg-ink)">traçabilité</text>
<rect x="4" y="198" width="752" height="30" rx="6" style="fill:var(--dg-box)"/>
<text x="14" y="218" font-size="14" style="fill:var(--dg-ink)">Contradicteur</text>
<text x="14" y="248" font-size="14" style="fill:var(--dg-ink)">Experts (6)</text>
<rect x="4" y="258" width="752" height="30" rx="6" style="fill:var(--dg-box)"/>
<text x="14" y="278" font-size="14" style="fill:var(--dg-ink)">Développeur</text>
<text x="14" y="308" font-size="14" style="fill:var(--dg-ink)">Nouvel arrivant</text>
<rect x="4" y="318" width="752" height="44" rx="6" style="fill:var(--dg-box)"/>
<text x="14" y="338" font-size="14" font-weight="600" style="fill:var(--dg-ink)">Vous</text>
<line x1="580" y1="40" x2="580" y2="362" style="stroke:var(--dg-line)" stroke-width="1.5" stroke-dasharray="4 4"/>
<circle cx="193" cy="93" r="8" style="fill:var(--dg-claude)"/>
<circle cx="279" cy="123" r="8" style="fill:var(--dg-claude)"/>
<circle cx="365" cy="153" r="8" style="fill:var(--dg-claude)"/>
<circle cx="451" cy="183" r="7" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3"/>
<circle cx="537" cy="213" r="7" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3"/>
<circle cx="537" cy="243" r="7" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3" stroke-dasharray="3 2.5"/>
<circle cx="709" cy="243" r="7" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3" stroke-dasharray="3 2.5"/>
<circle cx="623" cy="273" r="8" style="fill:var(--dg-claude)"/>
<circle cx="709" cy="303" r="7" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3"/>
<circle cx="537" cy="333" r="8" style="fill:var(--dg-you)"/>
<text x="537" y="357" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">validez</text>
<circle cx="709" cy="333" r="8" style="fill:var(--dg-you)"/>
<text x="709" y="357" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">tranchez</text>
<circle cx="20" cy="387" r="7" style="fill:var(--dg-claude)"/>
<text x="34" y="392" font-size="13" style="fill:var(--dg-muted)">travaille</text>
<circle cx="120" cy="387" r="6" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3"/>
<text x="134" y="392" font-size="13" style="fill:var(--dg-muted)">relit</text>
<circle cx="240" cy="387" r="6" style="fill:var(--dg-bg);stroke:var(--dg-claude)" stroke-width="3" stroke-dasharray="3 2.5"/>
<text x="254" y="392" font-size="13" style="fill:var(--dg-muted)">relit si son domaine est coché</text>
<circle cx="520" cy="387" r="7" style="fill:var(--dg-you)"/>
<text x="534" y="392" font-size="13" style="fill:var(--dg-muted)">vous décidez</text>
</svg>
</div>
<figcaption>Qui relit quand : les rôles de Claude rédigent et relisent à chaque étape ; les experts n'interviennent que si la modification touche leur domaine, et vous décidez après chaque relecture.</figcaption>
</figure>

| Étape | Compétence | Ce qui s'y passe |
|---|---|---|
| `plan.understand` | `/compagnon-propose` | La demande est comprise avant que la modification soit créée ; un ticket comme `PROJ-123` est lu via le MCP Jira lorsqu'il est configuré |
| `plan.investigate` | `/compagnon-propose` | Le code, les tests, la configuration et la documentation sont étudiés avant la rédaction de chaque artefact |
| `plan.write` | `/compagnon-propose` | Chaque artefact est rédigé, puis vérifié à l'aide de son rôle, que `compagnon instructions` renvoie dans `role` |
| `plan.trace` | `/compagnon-propose` | Les artefacts sont confrontés entre eux ; les manques sont corrigés dans les artefacts |
| `plan.challenge` | `/compagnon-propose` | Le contradicteur et les experts actifs relisent le plan ; leurs constats vont dans le [compte rendu de relecture](#gravité-confiance-et-compte-rendu-de-relecture) ; au plus cinq **points à discuter**, chacun marqué de son rôle, puis ce qu'il vous revient de décider terminent le résumé |
| `code.implement` | `/compagnon-apply` | Chaque tâche est implémentée, vérifiée, puis cochée |
| `code.review` | `/compagnon-apply` | Le nouvel arrivant et les experts actifs relisent le code de la modification ; ce qui ne fait pas de doute est simplifié ; au plus cinq **notes de relecture**, chacune marquée de son rôle, puis ce qu'il vous revient de décider terminent le résumé — un constat de sécurité ou de données personnelles de gravité haute n'est jamais écarté |

La procédure de chaque étape — ce qu'il faut écrire, les plafonds, les
garde-fous — reste dans la compétence et se lit dans
[Comment ça marche](workflow.md). Les critères et les connaissances
viennent du rôle.

## Experts et domaines

Six experts rejoignent les étapes de relecture lorsqu'une modification touche leur
domaine. Chacun est actif à `plan.challenge` — la conception traite-t-elle le
domaine ? — et à `code.review` — l'implémentation le respecte-t-elle ?

| Domaine | Expert | Ce qu'il examine |
|---|---|---|
| `security` | `security` | Validation des entrées aux frontières de confiance, autorisation côté serveur, secrets, injections, désérialisation non sûre, nouvelles dépendances |
| `personal-data` | `privacy` | Finalité et minimisation, conservation et suppression, droits des personnes, données personnelles dans les journaux, sous-traitants et transferts |
| `user-interface` | `accessibility` | Accès au clavier, focus, alternatives textuelles, étiquettes et erreurs de formulaire, contraste, sémantique et ARIA |
| `performance` | `performance` | Requêtes dans des boucles, résultats non bornés, E/S bloquantes sur les chemins critiques, allocations dans les boucles, coût des nouvelles dépendances |
| `operations` | `ops` | Journaux structurés sans secret, santé et métriques, délais d'expiration et nouvelles tentatives, chemin de retour arrière, erreurs exploitables |
| `migration` | `migration` | Changements incompatibles annoncés, chemins de migration, ordre de déploiement, migrations de schéma réversibles, dépréciation |

**Domaines.** La proposition liste les domaines que touche la modification dans sa
section `## Domains` — les six ci-dessus, à cocher, ou la seule ligne
`None` :

```markdown
## Domains

- [x] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [x] operations — nouveau worker en arrière-plan
- [ ] migration
```

Le responsable produit vérifie que chaque domaine touché par la modification est coché.
`compagnon validate` signale un domaine inconnu (`unknown_domain`) ou une
section mal formée (`domains_invalid`) comme une erreur, et une section
absente sur une modification active (`domains_missing`) comme un avertissement —
voir [Formats de fichiers](reference/file-formats.md#proposalmd).

**Activation `on_domain`.** Un expert n'est actif à ses étapes que lorsque
la proposition de la modification coche son domaine. Sans modification — `compagnon roles` sans
`--change` — ou tant que la proposition n'a pas de section `## Domains`, il
est inactif. `compagnon roles --json` indique pourquoi chaque rôle actif l'est
dans `activatedBy` : `always`, `project`, `change` ou `domain:<nom>`. Vous
pouvez toujours forcer un expert, pour le projet ou pour une modification — voir
[Configurer les rôles](#configurer-les-rôles).

**Connaissances.** Les connaissances d'un expert se répartissent en cinq
sections, toujours dans cet ordre :

- **Périmètre** (`Scope`) — ce qu'il examine, ce qu'il laisse de côté, et ce qu'il ne
  signale pas — par exemple, l'expert exploitation ne réclame pas
  d'interrupteur quand redéployer la version précédente annule la modification,
  et l'expert performance ne signale pas un gain d'un facteur constant ;
- **Signaux** (`Signals`) — des motifs concrets à repérer, chacun avec un exemple
  d'une ligne ;
- **Par pile technique** (`By stack`) — les pièges en Rust, TypeScript/JavaScript, Python, Go et
  Java ; Compagnon sert la partie qui concerne les langages de votre projet
  (voir [Filtre par pile technique](#les-déclencheurs-et-les-domaines-suggérés)) ;
- **Références** (`References`) — les référentiels qui font autorité, comme l'OWASP
  Top 10, les WCAG 2.2, le RGAA 4 ou le RGPD ;
- **Signalement** (`Reporting`) — comment signaler un constat, avec sa gravité, et ce
  qu'il doit nommer : la panne que subit l'exploitant, l'attaque, la
  donnée et qui la reçoit, le fichier ou le consommateur existant qui
  casse.

Le `newcomer` porte des signaux de complexité par langage, et regroupe en
un seul constat ce qui rend une fonction difficile à suivre ; il juge la
lisibilité, non la justesse, et les tests que la modification contient, non
ceux qui lui manquent. Le `skeptic` porte des signaux de planification
fragile : il conteste la logique et les preuves du plan — besoin,
périmètre, spécifications, décisions, hypothèses, tâches, et étapes sans retour
possible — et laisse chaque domaine à son expert, en vérifiant seulement
que la section `## Domains` de la proposition coche chaque domaine que le
plan touche, pour que son expert le relise. Les autres rôles d'étape n'en
portent pas. Les connaissances ne sont servies qu'aux rôles actifs : un
expert inactif ne coûte rien à l'agent. Lisez-les avec
`compagnon roles --at code.review --change <nom> --knowledge`.

## Les déclencheurs et les domaines suggérés

Cocher les domaines est un jugement porté pendant la planification, avant
que le code existe. Pour qu'un domaine que la modification finit par toucher ne
soit pas oublié en silence, chaque expert porte des **déclencheurs** — des
indices qu'une modification touche son domaine — et Compagnon les confronte aux
différences (diff) de la modification.

**Trois sortes de déclencheurs**, en groupes qui nomment chacun le domaine
qu'ils suggèrent :

- `paths` — des motifs de chemin (glob) sur les fichiers modifiés, relatifs à la racine du
  projet, sans tenir compte de la casse : `**/auth/**`, `**/*.tsx`. `*`
  reste dans un dossier, `**` traverse les dossiers ;
- `manifests` — des noms de manifestes de dépendances, à toute
  profondeur : `Cargo.toml`, `package.json`, `go.mod` ;
- `patterns` — des expressions régulières sur les lignes que les
  différences **ajoutent** : `(?i)\b(ALTER|DROP)\s+(TABLE|COLUMN)\b`.

| Expert | Ses déclencheurs, en résumé |
|---|---|
| `security` | dossiers `auth`, `security`, `crypto`, fichiers `.env`, clés ; chaque manifeste de dépendances ; noms d'identifiants secrets, `eval`, `innerHTML`, `pickle`, `shell=True`, `unsafe {` |
| `privacy` | modèles `user`, `customer`, `profile`, `account` ; e-mail, téléphone, date de naissance, adresse postale, code postal, numéro de sécurité sociale, IBAN, adresse IP, géolocalisation |
| `accessibility` | balisage, feuilles de style, composants, gabarits serveur ; `aria-*`, `role=`, `alt=`, éléments de formulaire et de média |
| `performance` | `*.sql`, dossiers de requêtes et de dépôts, bancs d'essai ; requêtes SQL, `Promise.all`, `tokio::spawn`, caches, pagination |
| `ops` | Dockerfiles, fichiers compose, déroulés d'intégration continue, Kubernetes, Helm, Terraform, dossiers de déploiement, configuration des journaux ; variables d'environnement, délais, nouvelles tentatives, sondes de santé, métriques |
| `migration` | migrations, schémas, `*.proto`, `*.graphql`, OpenAPI, `*.d.ts` ; `ALTER`/`DROP`/`RENAME`, dépréciations, renommages `serde` |

Les motifs intégrés privilégient la précision : `access_token` plutôt que
`token` (un analyseur lexical a des jetons), `street_address` plutôt
qu'`address` (une connexion réseau — socket — en a une). Ce qu'ils manquent pour votre projet,
ajoutez-le avec [`triggers:`](reference/configuration.md#triggers) dans
`_compagnon/config.yaml`.

**Les différences.** Compagnon demande à git, localement, ce que la branche a changé
depuis qu'elle a quitté la branche par défaut — `origin/HEAD`, sinon
`main`, sinon `master` —, enregistré ou non, plus les fichiers non suivis,
dans le seul dossier du projet. Il ne contacte jamais de dépôt distant.
Sans différences, Compagnon le dit et ne suggère rien, et aucune commande
n'échoue pour autant.

<details>
<summary>Sous le capot : quand il n'y a pas de différences, et ce qu'elles laissent de côté</summary>

Hors de git, sans enregistrement ou sans branche par défaut — ou dans un clone
superficiel d'intégration continue sans la base commune ; utilisez-y `fetch-depth: 0` —, il
n'y a pas de différences.

Certains fichiers échappent à tout déclencheur : le dossier de
planification `_compagnon/` et les compétences Compagnon (`planning`), les dépendances
embarquées — `node_modules`, `vendor`, `third_party` (`vendored`) —, les
fichiers de verrouillage comme `Cargo.lock` ou `package-lock.json`
(`lockfile`), et les fichiers générés comme `*.min.js` ou `*.pb.go`
(`generated`). Les fichiers binaires, la prose (`.md`, `.txt`…) et les
très gros fichiers ne sont confrontés que par leur chemin (`binary`,
`prose`, `large`). L'analyse s'arrête à 2000 fichiers, 5000 lignes
ajoutées par fichier, 1 Mio par fichier non suivi et 2 Mio de lignes
ajoutées au total, et ignore les lignes de plus de 2000 caractères ; les
différences sont alors marquées comme tronquées.

</details>

**Les suggestions.** `compagnon roles --change <nom>` présente les différences et
chaque domaine suggéré avec son état :

- `ticked` — la proposition le coche ;
- `justified` — la proposition le laisse décoché avec une raison sur sa ligne ;
- `unticked` — ni l'un ni l'autre : la proposition doit y répondre.

```markdown
## Domains

- [x] security
- [ ] personal-data — the fields matched are test fixtures
```

`compagnon validate` signale chaque suggestion `unticked` par l'avertissement
`domain_suggested`, à la ligne du domaine, en nommant le déclencheur et le
fichier concernés ; `compagnon validate --strict` échoue alors. `compagnon
archive` et `compagnon doctor` ne lisent pas les différences. Ce sont celles de
l'arbre de travail : deux modifications actives sur une même branche les
partagent. Un rôle réglé sur `never` n'apporte aucune suggestion.

`/compagnon-propose` lit les suggestions avant d'écrire `## Domains`, et le
contradicteur (`skeptic`) consigne un constat pour chacune restée `unticked`. Avant que le
code existe, les différences sont en général vides : c'est la relecture qui tranche.
`/compagnon-apply` liste chaque suggestion encore `unticked` sous **À
décider** — cochez-la, et son expert relit le code, ou justifiez-la.

**Filtre par pile technique.** Compagnon détecte les langages de votre projet à partir
de ses manifestes de dépendances — à la racine et jusqu'à trois dossiers
en dessous, si bien qu'un dépôt multiprojet (monorepo) obtient toutes ses piles techniques — et la
partie `By stack` des connaissances qu'il sert ne garde que ces langages.
`compagnon roles --json` les indique dans `stacks` ; sans aucun manifeste,
tout est conservé. `compagnon roles --all-stacks` sert tous les langages.

## Gravité, confiance et compte rendu de relecture

Ce que trouvent les rôles de relecture — le contradicteur et les experts à
`plan.challenge`, le nouvel arrivant et les experts à `code.review` — est
écrit dans le **compte rendu de relecture** de la modification,
`_compagnon/changes/<name>/review.md`, avant d'être corrigé ou signalé. Chaque
constat cite sa preuve mot pour mot, à un emplacement que Compagnon peut
vérifier, et porte une gravité et une confiance sur une échelle commune :

| Gravité | Signification |
|---|---|
| `high` | La modification casserait une exigence, perdrait ou exposerait des données, ou ouvrirait une vulnérabilité, et ne doit pas être archivée tant qu'elle n'est pas corrigée ou qu'une personne n'en accepte pas le risque. |
| `medium` | Un défaut ou un risque réel, à l'impact limité ou contournable, à corriger dans cette modification ou à reporter en connaissance de cause. |
| `low` | Une amélioration — lisibilité, cohérence, coût mineur — qui ne décide pas de la justesse de la modification. |

| Confiance | Signification |
|---|---|
| `high` | La preuve montre le problème directement. |
| `low` | Le problème dépend de quelque chose que le relecteur n'a pas pu voir ou vérifier. |

Un constat de confiance basse reste dans le compte rendu et reçoit une
réponse comme les autres, mais il ne vous est présenté que si l'on vous
demande de le trancher.

Tous les rôles de relecture notent de la même façon. Un constat `medium`
ou `high` qui vous est présenté avec une confiance `high` cite la ligne
qu'il vise et décrit le scénario qui échoue ; une inquiétude qui dépend
de ce que le relecteur n'a pas pu voir — la charge, un appelant, une
configuration — et une préférence sans défaillance sont notées avec une
confiance `low`. Un rôle ne signale pas une question qui relève d'un
autre rôle, du code ou des données que la modification ne touche pas, ce que le
plan exclut explicitement, un test manquant que ses critères ne demandent
pas, ni une affaire de goût. Chaque défaut est signalé une fois, ses
conséquences dans sa recommandation ; deux défauts qui demandent deux
corrections restent deux constats.

Ensuite, l'auteur corrige ou conteste chaque constat, le relecteur vérifie
chaque réponse une fois, et ce qui reste contesté vous revient sous
**À décider**. Vous seul acceptez un risque ou reportez un constat de
gravité haute. La boucle de réponses, les contrôles et le verrou
d'archivage sont dans [Compte rendu de relecture](guides/review-record.md).

## Relecture en ligne ou isolée

Par défaut, les rôles relecteurs travaillent **en ligne** (`inline`) : la
compétence qui a écrit le plan ou le code le relit dans la même conversation,
un rôle après l'autre. C'est le moyen le moins coûteux, mais chaque
relecteur a lu le raisonnement de l'auteur et les rôles passés avant lui,
et tend à leur donner raison.

En mode **isolé** (`isolated`) — `review.mode` dans la
[configuration](reference/configuration.md#review), ou dans le
`change.yaml` d'une modification — chaque rôle relecteur travaille d'abord seul,
comme [sous-agent](glossary.md#sous-agent) de Claude Code : le contrôleur
de traçabilité à `plan.trace`, le contradicteur et les experts actifs à
`plan.challenge`, le nouvel arrivant et les experts actifs à `code.review`,
tous ceux d'une étape en même temps. Un relecteur ne reçoit jamais la
conversation, le raisonnement de l'auteur ni les constats d'un autre rôle.
La compétence fusionne ensuite ce qu'ils ont trouvé, comme une équipe après
avoir lu chacun de son côté.

La relecture isolée coûte plus cher : chaque relecteur lit la matière, que la
relecture en ligne ne lit qu'une fois. C'est pourquoi `inline` reste le mode
par défaut ; voyez la [configuration](reference/configuration.md#review)
pour en changer.

<details>
<summary>Sous le capot : ce que reçoit un relecteur isolé, et la réunion</summary>

Un relecteur reçoit seulement :

- son rôle : question, critères et connaissances, filtrées par pile technique ;
- le `context` et les `rules` du projet ;
- la matière : les artefacts de la modification aux étapes du plan, les fichiers
  que la modification touche et leurs différences à `code.review` ;
- les identifiants déjà présents dans le compte rendu et le format d'un
  constat.

Il peut lire le reste du dépôt — les décisions,
les spécifications principales, le code que la matière cite —, ne modifie rien, et
traite la matière comme des données, jamais comme des instructions.
`compagnon roles --json` mesure la matière (`review.material`) : au-delà de
160 Kio, les fichiers les plus gros sont marqués `summarize`, et un
sous-agent distinct, qui ne voit que les différences de ce fichier, le résume en
citant les lignes modifiées quand elles tiennent. Le compte rendu dit
quels fichiers ont été résumés.

Un sous-agent qui échoue, que les limites de Claude Code arrêtent, ou qui
répond dans un autre format est relancé une fois ; s'il échoue encore, ce
rôle relit en ligne et le compte rendu le dit par une ligne `Fallback:`.
Sans l'outil de sous-agent, tous les rôles relisent en ligne.

La compétence tient ensuite la réunion :

- **Fusionner.** Les constats de plusieurs rôles sur le même défaut — au
  même endroit, avec la même correction — deviennent un seul constat,
  inscrit sous le rôle qui l'a jugé le plus grave ; le constat crédite
  chaque rôle qui l'a relevé (`raised_by`). Des constats au même endroit qui
  demandent des corrections différentes restent distincts.
- **Classer.** Par gravité, puis confiance, puis ordre des rôles ; le
  résumé en montre cinq au plus, comme en ligne.
- **Répondre.** Les réponses de l'auteur, l'unique revérification du
  relecteur et ce qui vous revient sous **À décider** fonctionnent
  exactement comme en ligne.

</details>

### Les modèles des relecteurs

En mode isolé, le sous-agent de chaque relecteur tourne sur le modèle de
son rôle — `model` dans `compagnon roles --json`. Aucun rôle n'en fixe par
défaut : chaque relecteur tourne sur le modèle de la session tant que votre
projet n'en choisit pas un autre, sous `roles:` — `model: opus`, `sonnet`
ou `haiku`, ou `inherit` pour revenir au modèle de la session (voir
[Configurer les rôles](#configurer-les-rôles)). Le modèle ne s'applique
qu'en mode isolé, où un rôle tourne comme son propre sous-agent ; en mode
en ligne, chaque rôle relit dans la session.

Pourquoi pas de défaut : sur la [suite d'évaluation](#mesurer-les-rôles),
en mode isolé, passer sur Sonnet le contrôleur de traçabilité, le
contradicteur, le nouvel arrivant et les experts sécurité, performance et
migration a fait gagner 7 % par rapport à Opus, avec le même rappel, mais
un faux positif de plus que la référence pour `migration` et le
contradicteur. L'essentiel du coût d'une relecture isolée vient de la
session — lire la compétence, préparer les consignes, fusionner les constats, écrire
le compte rendu —, si bien que le modèle d'un relecteur pèse peu sur la
facture. La même mesure montre que le mode isolé coûte environ 73 % de
plus que le mode en ligne sur les mêmes cas, avec le même rappel.

### Le modèle du développeur

Le `developer` implémente les tâches à `code.implement`. Quand votre
projet lui donne un modèle — `roles: { developer: { model: sonnet } }` —,
`/compagnon-apply` confie les tâches à un sous-agent sur ce modèle, quel que
soit le mode de relecture. La session garde la relecture et la synthèse sur son
propre modèle. Si le sous-agent s'arrête sur une ambiguïté ou un
blocage, la session vous pose la question, puis lance un nouveau
sous-agent sur le même modèle avec votre réponse. Si le sous-agent ne
peut pas tourner, la compétence réessaie une fois, puis implémente dans la
session et le signale. Sans modèle, `/compagnon-apply` implémente dans la
session.

Nous recommandons `developer: sonnet` ([décision 0012](reference/configuration.md#roles)).
Sur l'[évaluation de l'implémentation](#mesurer-les-rôles), avec une
session Opus et le développeur sur Sonnet, chaque mesure de qualité est
restée à 1,00 : tâches cochées, format, vérification de style (lint), tests et scripts
d'acceptation cachés. Le coût a baissé d'environ 30 %, mais l'exécution a
duré plus longtemps, car le travail du sous-agent s'ajoute à celui de la
session. Avec toute la session sur Sonnet, l'économie atteint 56 %,
mais la relecture et la synthèse tournent alors aussi sur Sonnet. La
planification reste sur le modèle le plus fort.

## Configurer les rôles

Tous les rôles sont actifs par défaut. Dans `_compagnon/config.yaml`, `roles:`
associe un identifiant de rôle à son réglage :

```yaml
roles:
  newcomer: never            # forme courte : always | never
  skeptic:
    when: always             # facultatif, always par défaut
    done_when:               # ajoutés après les critères propres du rôle
      - Le plan de retour arrière est indiqué
    model: inherit           # opus | sonnet | haiku | inherit
```

- **Activation.** `always`, `never` ou `on_domain`. Une étape dont tous les
  rôles sont inactifs est sautée, et le résumé de la compétence indique qu'elle
  est désactivée par la configuration. `on_domain` est la valeur par défaut
  des experts ; elle est refusée sur un rôle lié à aucun domaine. Indiquez
  `security: always` pour que l'expert sécurité relise chaque modification, ou
  `accessibility: never` dans un projet sans interface utilisateur.
- **Critères supplémentaires.** Les entrées de `done_when` s'ajoutent après
  les critères propres du rôle, sans jamais les remplacer. Elles portent
  l'origine `project`, comme votre
  [`context` et vos `rules`](reference/configuration.md#context).
- **Modèle.** `opus`, `sonnet` ou `haiku` remplace le modèle par défaut du
  rôle, et `inherit` garde celui de la session — voir
  [Les modèles des relecteurs](#les-modèles-des-relecteurs). Il s'applique
  aux rôles de `plan.trace`, `plan.challenge` et `code.review` en mode
  isolé, et au `developer` dans les deux modes — voir
  [Le modèle du développeur](#le-modèle-du-développeur). Relevez d'abord
  [`compagnon_version`](reference/configuration.md#compagnon_version) : un Compagnon
  plus ancien refuse la clé.
- **Par modification.** Le `change.yaml` d'une modification peut changer l'activation
  pour cette modification seulement :

  ```yaml
  roles:
    newcomer: always
  ```

- **Ce qui ne peut pas être désactivé.** Les rôles d'artefact (`product`,
  `qa`, `architect`, `lead`), l'`analyst` — le garde-fou contre un plan
  bâti sur un malentendu — et le `developer` : leur artefact est toujours
  rédigé, la demande toujours comprise, leurs tâches toujours
  implémentées. Leurs critères peuvent être complétés.
- **Les sources héritées** n'apportent aucun rôle : seul le `roles:` de
  votre projet est lu.

`compagnon validate` signale un identifiant de rôle inconnu (`unknown_role`),
une activation autre que `always`, `never` ou `on_domain`, ou `on_domain`
sur un rôle sans domaine (`invalid_role_activation`), et `never` sur un
rôle qui ne peut pas être désactivé (`role_not_disableable`), et un modèle
autre que `opus`, `sonnet`, `haiku` ou `inherit` (`invalid_role_model`) ;
`compagnon status`, `compagnon instructions` et `compagnon roles` échouent avec le même
code. Un modèle sur un rôle qui ne sert aucune étape de relecture et
n'est pas le `developer` ne s'applique jamais : `compagnon validate` le signale par l'avertissement
`role_model_unused`. `compagnon doctor` liste les rôles actifs, les rôles actifs selon le
domaine avec leurs domaines, et les rôles désactivés.

## Implémenter une modification (`/compagnon-apply`)

Le développeur implémente les tâches dans l'ordre, et ne coche une tâche
qu'une fois sa vérification réussie. Puis le nouvel arrivant et les
experts actifs relisent le code que la modification a ajouté ou modifié,
avant que la suite de tests complète tourne une fois.
[Comment ça marche](workflow.md) raconte toute l'étape ; les critères des
rôles sont ci-dessous.

<details>
<summary>Sous le capot : les critères du développeur et des relecteurs</summary>

### Développeur (`/compagnon-apply`)

Implémente les tâches d'une modification planifiée, dans l'ordre du fichier.
« Done when », pour chaque tâche :

- les fichiers concernés sont lus avant d'écrire ;
- le code, les tests, la configuration et la documentation que la tâche
  demande sont écrits — rien de plus ;
- les tests sont écrits à la manière de la suite existante du projet, puis
  selon les conventions de son langage et les `rules` du projet ;
- la vérification citée par la tâche est lancée et réussit — la commande de
  test vient de `context`, des `rules`, du manifeste ou de la configuration
  d'intégration continue, et la compétence vous la demande si elle n'en trouve aucune ;
- alors seulement, la case est cochée.

Elle s'arrête sur une ambiguïté importante ou un blocage technique sans
cocher la tâche, et décrit les options. Une fois toutes les cases cochées,
et après la [relecture](#nouvel-arrivant-et-experts-compagnon-apply),
elle lance une fois la suite de tests complète du projet ; un échec est
signalé à la place du résumé. Elle ne contourne jamais un test en échec, ne
touche jamais une autre modification, ne synchronise jamais et n'archive jamais.

### Nouvel arrivant et experts (`/compagnon-apply`)

Une fois toutes les tâches cochées, et avant la suite de tests complète,
relit uniquement le code que la modification a ajouté ou modifié : le nouvel
arrivant, comme un développeur qui rejoint le projet sans avoir vu ni le
plan ni la conversation, et les experts qu'activent les domaines de la modification,
chacun à l'aide de ses critères et de ses connaissances. Chaque constat va
d'abord dans le [compte rendu de
relecture](#gravité-confiance-et-compte-rendu-de-relecture). Ce qui ne
fait pas de doute est simplifié et les tests concernés sont relancés ; le
reste devient au plus cinq **notes de relecture** dans le résumé final —
chacune marquée de son rôle et de l'identifiant du constat : quoi, où,
pourquoi — ou la mention qu'aucune n'a été relevée, suivies de ce qu'il vous
revient de décider. Un constat de sécurité ou de données personnelles de gravité haute
est toujours listé, même au-delà de cinq.

La relecture n'élargit jamais le périmètre de la modification et ne touche jamais au
code que la modification n'a ni ajouté ni modifié. Le nouvel arrivant ne change
jamais le comportement. Un expert ne le modifie jamais en silence : une
correction qui change le comportement devient une note de relecture, sauf
s'il s'agit d'un défaut manifeste dans le périmètre de la modification, alors
corrigé avec un test et mentionné dans le résumé.

</details>

## Les autres compétences

Les autres compétences gardent une posture fixe, écrite dans la compétence elle-même.
Ces postures ne sont pas des rôles : elles ne figurent pas au catalogue et
ne se configurent pas. Ce que fait chaque compétence est dans
[Parler à Claude](skills.md).

| Compétence | Posture | Sa question | Ce qu'elle écrit |
|---|---|---|---|
| `/compagnon-explore` | [Partenaire de réflexion](#partenaire-de-réflexion-compagnon-explore) | Cela vaut-il la peine d'être construit, et comment ? | Rien |
| `/compagnon-sync`, `/compagnon-archive` | [Mainteneur des spécifications vivantes](#mainteneur-des-spécifications-vivantes-compagnon-sync-compagnon-archive) | Les spécifications principales décrivent-elles ce qui a été planifié et livré ? | Uniquement via la ligne de commande |
| `/compagnon-update` | [Éditeur](#éditeur-compagnon-update) | L'artefact révisé reste-t-il cohérent avec les autres ? | `_compagnon/changes/<name>/` uniquement |
| `/compagnon-onboard` | [Guide](#guide-compagnon-onboard) | Où en est ce projet, et que faire ensuite ? | Rien |
| `/compagnon-configure` | [Analyste de configuration](#analyste-de-configuration-compagnon-configure) | Que doit savoir un agent sur ce projet avant d'écrire ? | `_compagnon/config.yaml`, après confirmation |

<details>
<summary>Sous le capot : les critères de chaque posture</summary>

### Partenaire de réflexion (`/compagnon-explore`)

Explore une idée, un problème ou un besoin, sans engager quoi que ce soit.
« Done when » :

- ce que le dépôt sait déjà passe en premier — spécifications, modifications en cours,
  décisions en vigueur, code concerné — de sorte qu'on ne vous demande
  jamais ce que le code peut dire ;
- les questions posées portent sur une dépendance, une contrainte ou un
  compromis, chacune avec une réponse recommandée par défaut et sa raison ;
- deux ou trois options sont comparées, chacune avec ce qu'elle coûte et ce
  qu'elle exclut, et une recommandation est faite ;
- une direction qui s'écarte d'une décision acceptée est signalée tôt ;
- les hypothèses sont présentées comme des hypothèses ;
- l'exploration se termine de l'une de trois façons, nommée : elle se
  cristallise (problème, approche, périmètre, et la proposition de lancer
  `/compagnon-propose`), une information manque (laquelle, et qui peut la
  fournir), ou la piste ne valait pas la peine.

Elle n'écrit aucun fichier, ne crée aucune modification et n'implémente pas, même
si on le lui demande.

### Mainteneur des spécifications vivantes (`/compagnon-sync`, `/compagnon-archive`)

Veille à ce que `_compagnon/specs/` décrive le système. `/compagnon-sync` fusionne
les deltas d'une modification sans la déplacer ; `/compagnon-archive` les fusionne et
déplace la modification dans `_compagnon/changes/archive/<date>-<name>/`. « Done when » :

- la planification est complète — sinon rien n'est lancé ;
- la fusion passe par `compagnon sync` ou `compagnon archive`, jamais par un fichier
  que la compétence écrirait elle-même ;
- les spécifications créées, mises à jour et inchangées sont indiquées (et, pour
  l'archivage, l'endroit où la modification a été déplacée).

Sur une erreur de validation, les deux refusent : elles répondent que la
modification comporte des erreurs et renvoient vers `compagnon validate <name>`, sans
réessayer ni corriger la modification elles-mêmes. Toute autre erreur est relayée
telle que la ligne de commande l'a écrite.

### Éditeur (`/compagnon-update`)

Révise un artefact de planification qui existe déjà dans une modification active.
« Done when » :

- l'artefact et les autres qu'il peut affecter sont lus sur le disque avant
  d'écrire ;
- la révision respecte le contrat de l'artefact — sections attendues,
  format des scénarios, cases à cocher ;
- les répercussions sont annoncées : un fichier de spécification orphelin ou manquant
  après une modification de la proposition, une tâche qui cite une exigence
  supprimée ou un scénario renommé, une décision de la conception qui ne
  correspond plus, une tâche qui contredit une exigence — avec une action
  suivante suggérée, appliquée seulement après votre confirmation ;
- `compagnon validate` a été lancé et son rapport est relayé.

Elle refuse de créer un artefact manquant, de toucher à une modification archivée
ou de modifier le code.

### Guide (`/compagnon-onboard`)

Présente Compagnon à qui le découvre. Elle a terminé (« Done when ») quand elle a affiché trois blocs : ce
qu'est Compagnon, l'état du projet (nombre de spécifications, de décisions et de modifications,
et jusqu'à cinq éléments d'installation manquants signalés par
`compagnon doctor`, avec leur correction), et exactement une action recommandée,
citée sous la forme de la commande à taper. Elle est strictement en lecture
seule : elle ne lance aucune correction, pas même `compagnon init`.

### Analyste de configuration (`/compagnon-configure`)

Propose le `context:` et les `rules:` par artefact de `_compagnon/config.yaml` à
partir du projet lui-même. « Done when » :

- la lecture reste dans son budget — README et CONTRIBUTING en entier, au
  plus 5 fichiers de documentation, 8 fichiers source et 3 fichiers de
  test ;
- `context:` compte 2 à 5 lignes factuelles qui complètent la pile
  technique détectée ;
- `rules:` compte 1 à 2 règles pour `specs`, `design` et `tasks`, chacune
  formulée positivement, vérifiable en relecture et fondée sur ce que fait
  le projet, plus des règles de test tirées des tests existants du projet ;
- les différences sont affichées et votre accord est demandé avant toute écriture.

Elle n'écrit qu'après votre oui, ne modifie que `context:` et `rules:`,
préserve tous les autres champs et commentaires, et refuse d'agir si
`_compagnon/config.yaml` n'existe pas.

</details>

## Mesurer les rôles

Le dépôt de Compagnon mesure ses rôles de relecture intégrés — le
contradicteur, le contrôleur de traçabilité, le nouvel arrivant et les six
experts — avec une suite d'évaluation, `evals/` : de petits projets
ensemencés de défauts que chaque rôle devrait trouver, et des témoins
propres où il ne devrait rien trouver. La suite ne tourne jamais dans
l'intégration continue habituelle et ne fait pas partie du binaire
`compagnon`. [Fiabilité des rôles](reliability.md) présente les scores rôle
par rôle, la façon de les mesurer, les seuils que chaque rôle respecte ou
dépasse et les limites de la mesure.

## Là où vous décidez

L'agent s'arrête et vous passe la main à ces moments :

- **Questions de clarification.** `/compagnon-propose` vous interroge avant de
  créer une modification lorsqu'une ambiguïté changerait sensiblement le résultat,
  et de nouveau lorsqu'un artefact demande une décision ou qu'une question
  ouverte (`Open Question`) change ce qu'il faut construire. `/compagnon-explore` pose les
  questions auxquelles le code ne peut pas répondre. `/compagnon-apply` s'arrête
  sur une ambiguïté ou un blocage et vous laisse décider.
- **Valider le plan.** `/compagnon-propose` s'arrête une fois les artefacts
  rédigés. Commencez votre relecture par les points à discuter, ce qui
  reste à décider et les questions ouvertes (`Open Questions`) de la conception ; quand le plan vous convient, demandez
  `/compagnon-apply`. Pour modifier d'abord un artefact, utilisez
  `/compagnon-update`.
- **Accepter les décisions.** Une conception suit les décisions en vigueur ou
  propose explicitement d'en remplacer une. Une décision ne prend effet que
  lorsque vous lancez [`compagnon decision accept`](concepts.md#décision).
- **Confirmer la configuration.** `/compagnon-configure` n'écrit qu'après votre
  oui aux différences qu'elle affiche.
- **Appliquer une répercussion.** `/compagnon-update` signale ce que la révision
  affecte par ailleurs ; vous acceptez, refusez ou reportez chaque suite.
- **Mettre à jour Compagnon.** Lorsqu'un Compagnon plus récent est disponible ou que
  le projet exige une autre version, la compétence propose de mettre à jour
  maintenant (**Update now**) ou plus tard (**Later**) avant d'agir (voir
  [Annonce des mises à jour](skills.md#annonce-des-mises-à-jour)).
- **Configurer les rôles.** Les rôles de relecture qui s'exécutent, et les
  critères qu'ils ajoutent, relèvent du choix de votre projet dans
  `roles:` — voir [Configurer les rôles](#configurer-les-rôles).
- **Trancher la relecture.** Une contestation que le relecteur a maintenue, ou
  un constat de gravité haute que personne n'a pu corriger, vous revient
  sous **À décider** : le corriger, en accepter le risque avec une
  justification, ou le reporter vers une autre modification ou un ticket. La compétence
  consigne votre décision avec votre nom ; elle n'accepte jamais un risque à
  votre place. `compagnon review` montre ce qui reste.
- **Archiver.** `/compagnon-apply` n'archive jamais : une fois l'implémentation
  relue, en commençant par ses notes de relecture, dites « archive la modification »
  (raccourci : `/compagnon-archive`, ou `compagnon archive` dans un terminal).

## Pour aller plus loin

- [Compte rendu de relecture](guides/review-record.md)
- [Comment ça marche](workflow.md)
- [Configuration](reference/configuration.md)
- [Schémas personnalisés](guides/custom-schemas.md)
- [Fiabilité des rôles](reliability.md)
