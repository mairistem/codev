# Rôles

Une skill codev est un seul agent, mais il n'aborde pas chaque étape dans le
même état d'esprit. Chaque étape reçoit un rôle : une question dont il a la
charge, et des critères « Done when » que l'agent vérifie avant de passer à
la suite. Vous et votre équipe validez à des moments définis — l'agent ne
décide jamais à votre place.

Les rôles sont des critères, pas un jeu de rôle. L'agent ne se fait pas
passer pour un product owner : il vérifie la proposal à l'aide de la liste
du product owner. Un rôle dont les critères sont remplis est terminé, quel
que soit le ton de la prose.

Les quatre rôles d'artefact — product owner, analyste QA, architecte, tech
lead — sont écrits dans le schéma : l'instruction de chaque artefact commence
par une ligne `Role:` et une liste `Done when:`, que `codev instructions`
transmet à l'agent. Un [schéma personnalisé](guides/custom-schemas.md)
définit les siens. Les autres rôles ci-dessous nomment les étapes du workflow
de chaque skill, telles que les [skills](skills.md) les exécutent.

## Planifier un change (`/codev-propose`)

`/codev-propose` tient tous les rôles ci-dessous, dans cet ordre, en une
seule passe. Elle ne rédige que des artefacts de planification, puis
s'arrête pour votre relecture.

| Rôle | Sa question | Done when | Où |
|---|---|---|---|
| Analyste métier | Que faut-il construire ? | La demande est comprise — si elle n'est pas claire, une question ouverte est posée ; une ambiguïté qui changerait sensiblement le périmètre, le comportement observable, la compatibilité ou les critères d'acceptation est levée avant de créer le change ; un détail mineur devient une hypothèse raisonnable consignée dans les artefacts ; un identifiant de ticket comme `PROJ-123` est lu par un seul appel en lecture seule au MCP Jira lorsqu'il est configuré, et cité en tête de la proposal | Étapes 0–1 : ticket, compréhension de la demande |
| Enquêteur de code | Que fait déjà le projet ? | `context` et `rules` sont lus ; l'implémentation concernée, les tests voisins, la configuration et la documentation sont examinés, en lecture seule et à la mesure du change ; périmètre, approche et tâches s'appuient sur ce qui a été trouvé ; comportement observé, hypothèses et propositions restent distincts ; les contradictions avec les specs existantes sont signalées, pas tranchées seul ; aucune tâche « explorer le code » n'est laissée à l'implémentation | Étape 4c, avant de rédiger chaque artefact |
| Product owner | Pourquoi ce change, et qu'est-ce qui change ? | Why énonce le problème et pourquoi maintenant en une ou deux phrases ; What Changes est une liste de points concrets ; chaque changement incompatible est marqué **BREAKING** ; chaque capacité listée a son fichier de spec (ou `skip_specs: true`) ; la note de version est écrite pour les utilisateurs, classée par catégorie, avec les éléments incompatibles marqués, ou `None` pour un change interne ; aucun détail d'implémentation | [`proposal.md`](reference/file-formats.md#proposalmd) |
| Analyste QA | Quel comportement observable doit tenir ? | Chaque exigence emploie SHALL ou MUST sur un comportement observable ; chaque exigence a au moins un scénario nominal et un scénario d'erreur ou de cas limite, ou explique pourquoi aucun ne s'applique ; ni noms internes, ni bibliothèques, ni étapes d'implémentation | [Deltas de spec](reference/file-formats.md#delta-de-spec) |
| Architecte | Comment le construire ? | Chaque décision nomme l'alternative qu'elle écarte et pourquoi ; chaque risque a sa mitigation ; les décisions acceptées qui contraignent l'approche sont citées et suivies, ou un remplacement est explicitement proposé ; les Open Questions ne contiennent que ce qui peut réellement attendre | [`design.md`](reference/file-formats.md#designmd) — rédigé seulement si le change le demande |
| Tech lead | Dans quel ordre, et comment vérifier chaque étape ? | Chaque tâche tient en une séance de travail et suit l'ordre des dépendances ; chaque tâche nomme sa vérification (test, commande, comportement observable) ; chaque scénario de spec est couvert par la vérification d'une tâche, avec un test automatisé — une vérification manuelle seulement si l'automatisation est impossible, avec la raison ; la documentation concernée est mise à jour dans une tâche, ou le plan indique qu'aucune ne l'est ; les tests suivent la suite existante du projet, puis les conventions de son langage et les `rules` du projet | [`tasks.md`](reference/file-formats.md#tasksmd) |
| Contrôleur de traçabilité | Les artefacts concordent-ils entre eux ? | Chaque capacité de la proposal a son fichier de spec, et chaque fichier de spec est listé dans la proposal ; chaque exigence a un scénario nominal et un scénario d'erreur ou de cas limite ; chaque scénario est couvert par une tâche dont la vérification nomme un test automatisé, ou explique pourquoi l'automatisation est impossible ; une tâche met à jour la documentation lorsque la section Impact touche un comportement documenté ; la note de version concorde avec le plan ; chaque vérification est concrète. Les manques sont corrigés dans les artefacts | Étape 6 |
| Relecteur sceptique (contradicteur) | Le plan résisterait-il à un relecteur qui ne l'a pas écrit ? | Le plan est relu sous l'angle du besoin, du périmètre, des specs, des décisions, des hypothèses et des tâches — plus sécurité et confidentialité, compatibilité et migration, exploitabilité, performance, accessibilité et UX lorsque la section Impact les touche, et les `rules` du projet ; ce qui ne fait pas de doute est corrigé ; ce qui demande une décision humaine est consigné dans les Open Questions du design (ou de la proposal, sans design) ; le périmètre n'est jamais élargi ; au plus cinq **points à challenger**, classés par impact, ou la mention qu'aucun n'a été relevé | Étape 7 |

Le dernier rôle est le vôtre : la skill présente le plan avec ses points à
challenger et attend. Voir [Là où vous décidez](#là-où-vous-décidez).

## Les autres skills

| Skill | Rôle | Sa question | Ce qu'elle écrit |
|---|---|---|---|
| `/codev-explore` | [Partenaire de réflexion](#partenaire-de-réflexion-codev-explore) | Cela vaut-il la peine d'être construit, et comment ? | Rien |
| `/codev-apply` | [Développeur](#développeur-codev-apply) | Chaque tâche fonctionne-t-elle, vérification à l'appui ? | Code du projet, cases de `tasks.md` |
| `/codev-sync`, `/codev-archive` | [Mainteneur des specs vivantes](#mainteneur-des-specs-vivantes-codev-sync-codev-archive) | Les specs principales décrivent-elles ce qui a été planifié et livré ? | Uniquement via la CLI |
| `/codev-update` | [Éditeur](#éditeur-codev-update) | L'artefact révisé reste-t-il cohérent avec les autres ? | `_codev/changes/<name>/` uniquement |
| `/codev-onboard` | [Guide](#guide-codev-onboard) | Où en est ce projet, et que faire ensuite ? | Rien |
| `/codev-configure` | [Analyste de configuration](#analyste-de-configuration-codev-configure) | Que doit savoir un agent sur ce projet avant d'écrire ? | `_codev/config.yaml`, après confirmation |

### Partenaire de réflexion (`/codev-explore`)

Explore une idée, un problème ou un besoin, sans engager quoi que ce soit.
Done when :

- ce que le dépôt sait déjà passe en premier — specs, changes en cours,
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
  `/codev-propose`), une information manque (laquelle, et qui peut la
  fournir), ou la piste ne valait pas la peine.

Elle n'écrit aucun fichier, ne crée aucun change et n'implémente pas, même
si on le lui demande.

### Développeur (`/codev-apply`)

Implémente les tâches d'un change planifié, dans l'ordre du fichier. Done
when, pour chaque tâche :

- les fichiers concernés sont lus avant d'écrire ;
- le code, les tests, la configuration et la documentation que la tâche
  demande sont écrits — rien de plus ;
- les tests sont écrits à la manière de la suite existante du projet, puis
  selon les conventions de son langage et les `rules` du projet ;
- la vérification citée par la tâche est lancée et réussit — la commande de
  test vient de `context`, des `rules`, du manifeste ou de la configuration
  de CI, et la skill vous la demande si elle n'en trouve aucune ;
- alors seulement, la case est cochée.

Elle s'arrête sur une ambiguïté importante ou un blocage technique sans
cocher la tâche, et décrit les options. Une fois toutes les cases cochées,
elle lance une fois la suite de tests complète du projet ; un échec est
signalé à la place du résumé. Elle ne contourne jamais un test en échec, ne
touche jamais un autre change, ne synchronise jamais et n'archive jamais.

### Mainteneur des specs vivantes (`/codev-sync`, `/codev-archive`)

Veille à ce que `_codev/specs/` décrive le système. `/codev-sync` fusionne
les deltas d'un change sans le déplacer ; `/codev-archive` les fusionne et
déplace le change dans `_codev/changes/archive/<date>-<name>/`. Done when :

- la planification est complète — sinon rien n'est lancé ;
- la fusion passe par `codev sync` ou `codev archive`, jamais par un fichier
  que la skill écrirait elle-même ;
- les specs créées, mises à jour et inchangées sont indiquées (et, pour
  l'archivage, l'endroit où le change a été déplacé).

Sur une erreur de validation, les deux refusent : elles répondent que le
change comporte des erreurs et renvoient vers `codev validate <name>`, sans
réessayer ni corriger le change elles-mêmes. Toute autre erreur est relayée
telle que la CLI l'a écrite.

### Éditeur (`/codev-update`)

Révise un artefact de planification qui existe déjà dans un change actif.
Done when :

- l'artefact et les autres qu'il peut affecter sont lus sur le disque avant
  d'écrire ;
- la révision respecte le contrat de l'artefact — sections attendues,
  format des scénarios, cases à cocher ;
- les répercussions sont annoncées : un fichier de spec orphelin ou manquant
  après une modification de la proposal, une tâche qui cite une exigence
  supprimée ou un scénario renommé, une décision du design qui ne
  correspond plus, une tâche qui contredit une exigence — avec une action
  suivante suggérée, appliquée seulement après votre confirmation ;
- `codev validate` a été lancé et son rapport est relayé.

Elle refuse de créer un artefact manquant, de toucher à un change archivé
ou de modifier le code.

### Guide (`/codev-onboard`)

Présente codev à qui le découvre. Done when elle a affiché trois blocs : ce
qu'est codev, l'état du projet (nombre de specs, de décisions et de changes,
et jusqu'à cinq éléments d'installation manquants signalés par
`codev doctor`, avec leur correction), et exactement une action recommandée,
citée sous la forme de la commande à taper. Elle est strictement en lecture
seule : elle ne lance aucune correction, pas même `codev init`.

### Analyste de configuration (`/codev-configure`)

Propose le `context:` et les `rules:` par artefact de `_codev/config.yaml` à
partir du projet lui-même. Done when :

- la lecture reste dans son budget — README et CONTRIBUTING en entier, au
  plus 5 fichiers de documentation, 8 fichiers source et 3 fichiers de
  test ;
- `context:` compte 2 à 5 lignes factuelles qui complètent la pile
  détectée ;
- `rules:` compte 1 à 2 règles pour `specs`, `design` et `tasks`, chacune
  formulée positivement, vérifiable en relecture et fondée sur ce que fait
  le projet, plus des règles de test tirées des tests existants du projet ;
- le diff est affiché et votre accord est demandé avant toute écriture.

Elle n'écrit qu'après votre oui, ne modifie que `context:` et `rules:`,
préserve tous les autres champs et commentaires, et refuse d'agir si
`_codev/config.yaml` n'existe pas.

## Là où vous décidez

L'agent s'arrête et vous passe la main à ces moments :

- **Questions de clarification.** `/codev-propose` vous interroge avant de
  créer un change lorsqu'une ambiguïté changerait sensiblement le résultat,
  et de nouveau lorsqu'un artefact demande une décision ou qu'une Open
  Question change ce qu'il faut construire. `/codev-explore` pose les
  questions auxquelles le code ne peut pas répondre. `/codev-apply` s'arrête
  sur une ambiguïté ou un blocage et vous laisse décider.
- **Valider le plan.** `/codev-propose` s'arrête une fois les artefacts
  rédigés. Commencez votre relecture par les points à challenger et les
  Open Questions du design ; quand le plan vous convient, demandez
  `/codev-apply`. Pour modifier d'abord un artefact, utilisez
  `/codev-update`.
- **Accepter les décisions.** Un design suit les décisions en vigueur ou
  propose explicitement d'en remplacer une. Une décision ne prend effet que
  lorsque vous lancez [`codev decision accept`](concepts.md#décision).
- **Confirmer la configuration.** `/codev-configure` n'écrit qu'après votre
  oui à son diff.
- **Appliquer une répercussion.** `/codev-update` signale ce que la révision
  affecte par ailleurs ; vous acceptez, refusez ou reportez chaque suite.
- **Mettre à jour codev.** Lorsqu'un codev plus récent est disponible ou que
  le projet exige une autre version, la skill demande **Update now** ou
  **Later** avant d'agir (voir
  [Annonce des mises à jour](skills.md#annonce-des-mises-à-jour)).
- **Archiver.** `/codev-apply` n'archive jamais : une fois l'implémentation
  relue, lancez `/codev-archive` ou `codev archive`.
