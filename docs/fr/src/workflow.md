# Comment ça marche

Dans un projet Compagnon, chaque modification passe par les cinq mêmes
étapes, dans le même ordre : le [déroulé](glossary.md#déroulé) de
Compagnon. Vous décidez à deux d'entre elles ; Claude travaille aux autres
et s'arrête dès qu'il a besoin de vous.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 170" role="img" aria-label="Cinq étapes : vous demandez avec vos mots ; Claude rédige un plan ; vous l'approuvez ou demandez des corrections ; Claude le réalise tâche par tâche ; la modification est classée et le plan reste avec le code.">
<defs><marker id="dg-workflow-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<g class="st"><text x="70" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-you-text)">VOUS</text><rect x="4" y="32" width="132" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/><circle cx="30" cy="58" r="13" style="fill:var(--dg-you)"/><text x="30" y="63" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-on-you)">1</text><text x="20" y="98" font-size="16" font-weight="600" style="fill:var(--dg-ink)">Demander</text><text x="20" y="120" font-size="13" style="fill:var(--dg-muted)">avec vos mots</text></g>
<line x1="138" y1="91" x2="154" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-arrow)"/>
<g class="st"><text x="223" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="157" y="32" width="132" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="183" cy="58" r="13" style="fill:var(--dg-claude)"/><text x="183" y="63" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">2</text><text x="173" y="98" font-size="16" font-weight="600" style="fill:var(--dg-ink)">Planifier</text><text x="173" y="120" font-size="13" style="fill:var(--dg-muted)">pourquoi, quoi,</text><text x="173" y="137" font-size="13" style="fill:var(--dg-muted)">comment, tâches</text></g>
<line x1="291" y1="91" x2="307" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-arrow)"/>
<g class="st"><text x="376" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-you-text)">VOUS</text><rect x="310" y="32" width="132" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/><circle cx="336" cy="58" r="13" style="fill:var(--dg-you)"/><text x="336" y="63" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-on-you)">3</text><text x="326" y="98" font-size="16" font-weight="600" style="fill:var(--dg-ink)">Approuver</text><text x="326" y="120" font-size="13" style="fill:var(--dg-muted)">ou demander des</text><text x="326" y="137" font-size="13" style="fill:var(--dg-muted)">corrections</text></g>
<line x1="444" y1="91" x2="460" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-arrow)"/>
<g class="st"><text x="529" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="463" y="32" width="132" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="489" cy="58" r="13" style="fill:var(--dg-claude)"/><text x="489" y="63" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">4</text><text x="479" y="98" font-size="16" font-weight="600" style="fill:var(--dg-ink)">Réaliser</text><text x="479" y="120" font-size="13" style="fill:var(--dg-muted)">tâche par tâche,</text><text x="479" y="137" font-size="13" style="fill:var(--dg-muted)">testé</text></g>
<line x1="597" y1="91" x2="613" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-arrow)"/>
<g class="st"><text x="682" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-muted)">CONSERVÉ</text><rect x="616" y="32" width="140" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2" stroke-dasharray="5 4"/><circle cx="642" cy="58" r="13" style="fill:var(--dg-line)"/><text x="642" y="63" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">5</text><text x="632" y="98" font-size="16" font-weight="600" style="fill:var(--dg-ink)">Classer</text><text x="632" y="120" font-size="13" style="fill:var(--dg-muted)">le plan reste</text><text x="632" y="137" font-size="13" style="fill:var(--dg-muted)">avec le code</text></g>
</svg>
</div>
<figcaption>Les cinq étapes : celles en corail sont les vôtres, celles en bleu sont celles de Claude, et celle en pointillés est conservée pour plus tard. Rien n'est réalisé avant l'étape 3.</figcaption>
</figure>

<div class="diagram-steps">
<details><summary>1. Vous demandez</summary>

Dites ce que vous voulez avec vos mots : « ajoute un mode sombre », ou
« Prépare un plan pour PROJ-123 ». Claude choisit la compétence qui
correspond.
Pour aller plus loin : [Parler à Claude](skills.md).

</details>
<details><summary>2. Claude planifie</summary>

Claude étudie le code, puis rédige quatre fichiers courts : pourquoi,
quoi, comment et les tâches. Il remet son propre plan en question avant de
vous le montrer.
Pour aller plus loin : [Un exemple de plan commenté](guides/example-plan.md).

</details>
<details><summary>3. Vous approuvez</summary>

Vous lisez le plan, demandez des corrections, puis l'approuvez. Vous
pouvez le faire dans la conversation ou sur GitHub et GitLab.
Pour aller plus loin : [Relire un plan](guides/review-a-plan.md).

</details>
<details><summary>4. Claude réalise</summary>

Claude réalise les tâches dans l'ordre et lance le test de chacune. Il
s'arrête au premier doute au lieu de deviner.
Pour aller plus loin : [Rôles](roles.md).

</details>
<details><summary>5. Classé</summary>

Les changements du plan rejoignent les spécifications du projet, et le
plan est archivé. La personne suivante sait pourquoi le code est ainsi.
Pour aller plus loin : [Concepts](concepts.md).

</details>
</div>

À chaque étape, Claude suit une compétence (voir
[Parler à Claude](skills.md)). Cette page dit ce que font les quatre
principales : proposer, réaliser, synchroniser et archiver.

## Les fichiers du plan

Une modification vit dans son propre dossier, `_compagnon/changes/<name>/`.
Chaque fichier qu'il contient est un [artefact](glossary.md#artefact), et
chacun a un seul auteur :

| Fichier | Ce qu'il dit | Qui l'écrit |
|---|---|---|
| `proposal.md` | Pourquoi la modification est nécessaire, ce qui change, quelles [capacités](glossary.md#capacité) (domaines du produit) elle touche, et une note de version pour les utilisateurs | Claude en responsable produit |
| `specs/<capability>/spec.md` | Les [deltas](glossary.md#delta) : le comportement ajouté, modifié, supprimé ou renommé, chacun avec ses scénarios | Claude en analyste qualité |
| `design.md` | Comment ce sera construit : décisions, solutions écartées, risques | Claude en architecte |
| `tasks.md` | La liste des tâches, chacune avec sa vérification | Claude en référent technique ; `/compagnon-apply` coche les cases |
| `review.md` | Le [compte rendu de relecture](guides/review-record.md) : chaque constat, la réponse de l'auteur, la vérification du relecteur | Les rôles relecteurs de Claude, pendant la planification et la réalisation |
| `corrections.md` | Ce sur quoi vous avez corrigé Claude, pour la modification suivante | Claude, quand vous le corrigez ([Les corrections de l'auteur](skills.md#les-corrections-de-lauteur)) |
| `change.yaml` | Les réglages de la modification, comme `skip_specs` | Compagnon le crée ; vous le modifiez |

Le [schéma](glossary.md#schéma) fixe les fichiers que contient un plan ;
celui par défaut, `spec-driven`, demande les quatre premiers. Les
spécifications et la conception demandent d'abord la proposition ; les
tâches demandent les deux. Vous approuvez un plan en lisant la
proposition ; les autres fichiers sont là quand vous voulez le détail.

<details>
<summary>Sous le capot : comment `compagnon status` présente chaque fichier</summary>

`compagnon status` présente chaque artefact comme terminé (`[x]`), prêt à
être rédigé (`[ ]`), bloqué par une dépendance (`[-]`) ou ignoré (`[~]`). Cet
état se déduit uniquement des fichiers présents : Compagnon ne tient aucun
fichier d'état.

</details>

## Proposer

`/compagnon-propose` transforme votre demande en plan complet, et rien de
plus : même si vous dites « construis » ou « corrige », la compétence
s'arrête une fois le plan rédigé et attend votre retour.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 170" role="img" aria-label="Comment se construit un plan, en six étapes : Claude comprend la demande et lit le ticket ; enquête sur ce que fait déjà le code ; rédige les quatre fichiers, un rôle chacun ; vérifie que les fichiers concordent ; remet le plan en question avec un contradicteur et des experts ; puis vous l'approuvez.">
<defs><marker id="dg-workflow-plan-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<g class="st"><text x="59" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="4" y="32" width="110" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="28" cy="56" r="12" style="fill:var(--dg-claude)"/><text x="28" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">1</text><text x="16" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Comprendre</text><text x="16" y="117" font-size="12" style="fill:var(--dg-muted)">la demande,</text><text x="16" y="133" font-size="12" style="fill:var(--dg-muted)">le ticket</text></g>
<line x1="116" y1="91" x2="128" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-plan-arrow)"/>
<g class="st"><text x="185" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="130" y="32" width="110" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="154" cy="56" r="12" style="fill:var(--dg-claude)"/><text x="154" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">2</text><text x="142" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Enquêter</text><text x="142" y="117" font-size="12" style="fill:var(--dg-muted)">ce que fait déjà</text><text x="142" y="133" font-size="12" style="fill:var(--dg-muted)">le code</text></g>
<line x1="242" y1="91" x2="254" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-plan-arrow)"/>
<g class="st"><text x="311" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="256" y="32" width="110" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="280" cy="56" r="12" style="fill:var(--dg-claude)"/><text x="280" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">3</text><text x="268" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Rédiger</text><text x="268" y="117" font-size="12" style="fill:var(--dg-muted)">quatre fichiers,</text><text x="268" y="133" font-size="12" style="fill:var(--dg-muted)">un rôle chacun</text></g>
<line x1="368" y1="91" x2="380" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-plan-arrow)"/>
<g class="st"><text x="437" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="382" y="32" width="110" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="406" cy="56" r="12" style="fill:var(--dg-claude)"/><text x="406" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">4</text><text x="394" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Tracer</text><text x="394" y="117" font-size="12" style="fill:var(--dg-muted)">les fichiers</text><text x="394" y="133" font-size="12" style="fill:var(--dg-muted)">concordent-ils ?</text></g>
<line x1="494" y1="91" x2="506" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-plan-arrow)"/>
<g class="st"><text x="563" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-claude)">CLAUDE</text><rect x="508" y="32" width="110" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/><circle cx="532" cy="56" r="12" style="fill:var(--dg-claude)"/><text x="532" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-box)">5</text><text x="520" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Contester</text><text x="520" y="117" font-size="12" style="fill:var(--dg-muted)">contradicteur et</text><text x="520" y="133" font-size="12" style="fill:var(--dg-muted)">experts relisent</text></g>
<line x1="620" y1="91" x2="632" y2="91" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-workflow-plan-arrow)"/>
<g class="st"><text x="695" y="22" text-anchor="middle" class="mono" font-size="12" style="fill:var(--dg-you-text)">VOUS</text><rect x="634" y="32" width="122" height="118" rx="14" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/><circle cx="658" cy="56" r="12" style="fill:var(--dg-you)"/><text x="658" y="61" text-anchor="middle" class="mono" font-size="13" font-weight="700" style="fill:var(--dg-on-you)">6</text><text x="646" y="96" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Vous approuvez</text><text x="646" y="117" font-size="12" style="fill:var(--dg-muted)">ou demandez des</text><text x="646" y="133" font-size="12" style="fill:var(--dg-muted)">corrections</text></g>
</svg>
</div>
<figcaption>Comment se construit un plan : Claude franchit seul cinq étapes, et vérifie deux fois son plan, avant que vous le voyiez.</figcaption>
</figure>

1. **Comprendre.** Claude s'assure de ce qu'il faut construire. Si votre
   demande cite un ticket, comme `PROJ-123`, et que Jira est connecté,
   Claude lit ce ticket une fois, sans le modifier, et le cite en tête de
   la proposition. Voir [Partir d'un ticket Jira](guides/mcp.md).
2. **Enquêter.** Avant d'écrire, Claude lit le code, les tests et la
   documentation que concerne votre demande, et fonde le périmètre dessus.
   Si votre demande contredit les spécifications du projet, il vous le dit
   au lieu de trancher seul.
3. **Rédiger.** Claude rédige les quatre fichiers, chacun dans le rôle
   que donne le tableau ci-dessus, dans
   [la langue que votre projet utilise pour les plans](guides/artifact-language.md).
   Il modifie les exigences existantes que touche la modification au lieu
   d'en ajouter de nouvelles à côté. La conception respecte les
   [décisions](concepts.md#décision) en vigueur, ou propose ouvertement
   d'en remplacer une.
4. **Tracer.** Claude vérifie que rien ne manque entre les fichiers :
   chaque exigence a ses scénarios, chaque scénario une tâche avec un test
   automatisé, et la note de version concorde avec le plan. Les manques
   sont corrigés dans le plan.
5. **Contester.** Claude relit le plan comme le ferait un relecteur
   sceptique : est-ce utile, le périmètre est-il juste, les décisions
   tiennent-elles ? Des experts se joignent à lui pour les points sensibles
   que le plan touche, comme la sécurité ou l'accessibilité (chacun est un
   [domaine](glossary.md#domaine)). Chaque constat va dans le compte rendu
   de relecture ; l'auteur le corrige ou le conteste, et le relecteur
   vérifie chaque réponse une fois. Cette relecture n'élargit jamais le
   périmètre.
6. **Vous approuvez.** Le résumé se termine par au plus cinq **points à
   discuter**, les plus graves d'abord, puis ce que **vous seul pouvez
   décider**. Commencez votre propre relecture par là.

Quel rôle travaille à chaque étape, et ce qu'il vérifie : voir
[Rôles](roles.md).

Certaines modifications ne changent aucun comportement : un nettoyage,
une mise à niveau, de la documentation. Une telle modification déclare
`skip_specs: true` dans son `change.yaml`, et son plan n'a pas de
spécifications.

<details>
<summary>Sous le capot : les commandes que lance la compétence de proposition</summary>

La création du dossier lance `compagnon new change <name>`. Puis, pour
chaque artefact prêt :

```bash
compagnon instructions <artifact> --change <name> --json
```

Le rôle de chaque artefact est déclaré par le schéma et renvoyé par
`compagnon instructions`. À chaque étape, la compétence demande
`compagnon roles --at <step>` les rôles de l'étape et applique leurs
critères, y compris les critères ajoutés par votre projet ; une étape dont
votre configuration désactive le rôle est sautée, et le résumé le signale
(voir [Configurer les rôles](roles.md#configurer-les-rôles)). Votre demande
n'autorise que la planification : la compétence n'écrit rien hors du
dossier de la modification.

Une fois la proposition écrite, Compagnon donne à la compétence les
exigences existantes que la modification touche probablement. Cette liste
est un point de départ : il peut en manquer.

</details>

<details>
<summary>Sous le capot : les deux relectures en détail</summary>

1. **Traçabilité** — chaque capacité a son fichier de spécification, chaque exigence
   un scénario nominal et un scénario d'erreur ou de cas limite, chaque
   scénario une tâche dont la vérification le couvre par un test
   automatisé, une tâche met à jour la documentation lorsque la section
   Impact touche un comportement documenté, et la
   [note de version](reference/file-formats.md#release-note) de la proposition
   concorde avec le plan — chaque élément incompatible de What Changes, et
   chaque exigence supprimée ou modifiée de façon incompatible, a son élément
   **BREAKING**. Les manques sont corrigés dans les artefacts.
2. **Relecture contradictoire** — la compétence relit le plan en relecteur
   sceptique : besoin, périmètre, spécifications, décisions, hypothèses, tâches,
   et si `## Domains` coche chaque domaine que le plan touche, ainsi que
   les [`rules`](reference/configuration.md#rules) de votre projet. Le
   contradicteur ne relit lui-même aucun domaine : sécurité, données
   personnelles, migration, exploitation, performance et accessibilité
   reviennent à leurs experts. Les
   [experts](roles.md#experts-et-domaines) dont la proposition coche le
   domaine dans `## Domains` rejoignent cette relecture avec leurs propres
   connaissances. Avant d'écrire `## Domains`, la compétence lit les
   [domaines que suggère le diff de la modification](roles.md#les-déclencheurs-et-les-domaines-suggérés)
   et coche chacun ou le garde non coché avec une raison ; le contradicteur
   consigne un constat pour chacun resté `unticked`. Chaque constat est écrit
   dans le [compte rendu de relecture](guides/review-record.md), avec sa
   gravité et sa preuve citée. En
   [mode isolé](roles.md#relecture-en-ligne-ou-isolée), le contrôleur de
   traçabilité et chaque relecteur travaillent comme des sous-agents
   distincts qui ne voient jamais la conversation, et la compétence
   fusionne leurs constats avant de les consigner.

Chaque point à discuter dit quel est le problème, pourquoi il compte, quel
fichier il concerne, quel rôle l'a relevé, et l'identifiant et le statut
du constat. S'il n'y en a aucun, le résumé le dit.

</details>

## Réaliser

`/compagnon-apply <name>` construit ce que décrit le plan :

1. La compétence vérifie que le plan est complet.
2. Elle prend dans l'ordre chaque tâche non cochée de `tasks.md`
   (`- [ ]`), la réalise, lance la vérification que la tâche indique, et ne
   coche la case qu'une fois cette vérification réussie.
3. Elle écrit les tests à la manière de vos tests existants, puis selon
   les habitudes de votre langage et vos
   [`rules`](reference/configuration.md#rules), et vous demande comment les
   lancer quand votre projet ne le dit pas.
4. Une fois toutes les tâches cochées, Claude relit le code qu'il a
   touché, comme le ferait un nouvel arrivant sur le projet, rejoint par
   les [experts](roles.md#experts-et-domaines) des domaines que coche la
   proposition. Il simplifie ce qui ne fait pas de doute, sans élargir le
   périmètre, et liste au plus cinq notes de relecture. Un constat grave de
   sécurité ou de données personnelles est toujours signalé.
5. Enfin, elle lance une fois tous les tests du projet.

Chaque constat va dans le compte rendu de relecture, avec la réponse de
l'auteur et la vérification du relecteur. Le résumé se termine par ce que
vous seul pouvez trancher : par exemple, le code touche désormais des
données personnelles que le plan ne listait pas.

Claude s'arrête au premier blocage ou au premier doute pour vous
consulter, plutôt que de deviner. Il ne travaille que sur la modification
désignée, et ne synchronise ni n'archive jamais : c'est à vous de demander
ces étapes. La réalisation est la seule compétence autorisée à lancer
n'importe quelle commande dans le terminal, parce qu'elle doit lancer vos
tests.

<details>
<summary>Sous le capot : les corrections des experts et le mode isolé</summary>

Claude trouve comment lancer vos tests dans la configuration du projet,
son manifeste ou son [intégration continue](glossary.md#intégration-continue).
Un expert corrige un défaut manifeste dans le périmètre de la modification,
avec un test. Tout autre changement de comportement, il le signale au lieu
de le faire. Les critères et les connaissances de ces deux étapes viennent
de `compagnon roles` ; lorsqu'aucun rôle n'est actif pour la relecture,
celle-ci est sautée et le résumé le signale. En mode isolé, chaque
relecteur travaille d'abord seul comme [sous-agent](glossary.md#sous-agent)
(une copie distincte de Claude qui ne voit pas votre conversation), et la
compétence fusionne leurs constats.

</details>

## Synchroniser

```bash
compagnon sync --change <name>
```

`/compagnon-sync` recopie les deltas de la modification dans les
spécifications du projet, sous `_compagnon/specs/`, **sans** déplacer la
modification. Elle sert quand une autre modification en cours a besoin de
voir une nouvelle capacité, ou pour vérifier le résultat avant
l'archivage. Elle vous dit quelles spécifications ont été créées, mises à
jour ou laissées inchangées. En général, vous sautez cette étape :
l'archivage synchronise pour vous.

## Archiver

```bash
compagnon archive --change <name>
```

`/compagnon-archive` clôt une modification terminée, en quatre temps :

1. Elle valide la modification, et **refuse** de continuer sur une erreur.
2. Elle vérifie le [compte rendu de relecture](guides/review-record.md),
   et **refuse** de continuer si une relecture était attendue mais n'a pas
   été consignée, ou si un constat grave n'est pas encore réglé
   (`review_blocked`). `compagnon review --change <name>` montre ce qui
   reste. Claude ne consigne jamais une décision à votre place.
3. Elle recopie les deltas dans les spécifications du projet, comme la
   synchronisation.
4. Elle déplace le dossier de la modification dans
   `_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`.

Rien ne change sur le disque tant que toute la mise à jour n'a pas été
vérifiée. Ensuite, les spécifications du projet décrivent ce que vous
avez livré, et le dossier archivé garde le pourquoi et le comment.
Enregistrez les deux dans [Git](glossary.md#git) : c'est un
[enregistrement](glossary.md#enregistrement) (commit). Si la modification
a une note de version, `compagnon changelog --write` l'ajoute à
`CHANGELOG.md` (voir [Notes de version et CHANGELOG](guides/release-notes.md)).

<details>
<summary>Sous le capot : quand l'archivage refuse à cause du compte rendu</summary>

L'archivage signale `review_blocked` quand le compte rendu manque pour une
étape où des rôles de relecture étaient actifs, ou quand un constat de
gravité `high` est encore `open` ou `disputed`. La synchronisation, comme
l'archivage, valide d'abord : sur une erreur, elle n'écrit rien et renvoie
vers `compagnon validate <name>`.

</details>

Ce que vérifie `compagnon validate`, et comment `--change` est choisi
quand plusieurs modifications sont en cours, se trouvent dans la
référence de l'[Interface en ligne de commande](reference/cli.md#compagnon-validate)
et ses [conventions](reference/cli.md#conventions).

## Les autres compétences

- `/compagnon-explore` mûrit une idée avec vous et n'écrit rien.
- `/compagnon-update` révise un fichier d'un plan, vous signale les
  répercussions sur les autres, et se termine par `compagnon validate`.
  Elle ne touche pas au code.
- `/compagnon-onboard` fait visiter, liste jusqu'à cinq éléments
  d'installation manquants signalés par `compagnon doctor`, et propose une
  prochaine action. Elle ne modifie rien.
- `/compagnon-configure` lit votre projet et propose le `context:` et les
  `rules:` de `_compagnon/config.yaml`, règles de test comprises, et
  n'écrit qu'après votre accord. `compagnon init` et `compagnon status` la
  recommandent tant qu'il n'y a aucune `rules:`.

Leurs postures sont décrites dans [Rôles](roles.md).

## Pour aller plus loin

- [Parler à Claude](skills.md) : comment solliciter chaque compétence.
- [Relire un plan](guides/review-a-plan.md) : ce qu'on vérifie dans chaque
  fichier.
- [Rôles](roles.md) : les critères de chaque rôle, à chaque étape.
- [Compte rendu de relecture](guides/review-record.md) : la vie d'un
  constat.
- [Interface en ligne de commande](reference/cli.md) : chaque commande et
  chaque option.
