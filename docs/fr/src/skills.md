# Parler à Claude

Une compétence (skill) est un ensemble d'instructions que Claude Code suit
pour un type de tâche. Compagnon vous en fournit huit, une pour chaque
étape de son [déroulé](glossary.md#déroulé) : planifier une modification,
la réaliser, la classer, etc. Claude Code les trouve tout seul : il vous
suffit de demander avec vos mots. Vous pouvez aussi en lancer une
directement en tapant son nom, par exemple `/compagnon-propose`.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 140" role="img" aria-label="Parler à Claude en quatre étapes : 1, vous dites ce que vous voulez avec vos mots ; 2, Claude choisit la compétence qui correspond, ici celle qui planifie ; 3, Claude fait cette étape, écrit le plan, puis s'arrête ; 4, vous décidez : vous le corrigez ou le validez.">
<g stroke-width="2" style="fill:var(--dg-box)">
<rect x="4" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-you)"/>
<rect x="198" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-claude)"/>
<rect x="392" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-claude)"/>
<rect x="586" y="16" width="168" height="118" rx="14" style="stroke:var(--dg-you)"/>
</g>
<path d="M175,75h18m-6,-5l6,5l-6,5M369,75h18m-6,-5l6,5l-6,5M563,75h18m-6,-5l6,5l-6,5" fill="none" stroke-width="2" style="stroke:var(--dg-line)"/>
<g class="mono" font-size="12" style="fill:var(--dg-muted)">
<text x="20" y="44">1 · VOUS</text><text x="214" y="44">2 · CLAUDE</text><text x="408" y="44">3 · CLAUDE</text><text x="602" y="44">4 · VOUS</text>
</g>
<g font-size="16" font-weight="600" style="fill:var(--dg-ink)">
<text x="20" y="78">Vos mots</text><text x="214" y="78">Choisit</text><text x="408" y="78">Fait l'étape</text><text x="602" y="78">Décidez</text>
</g>
<g font-size="13" style="fill:var(--dg-muted)">
<text x="20" y="102">« prépare un plan</text><text x="20" y="120">de mode sombre »</text><text x="214" y="102">la compétence</text><text x="214" y="120">qui planifie</text><text x="408" y="102">écrit le plan,</text><text x="408" y="120">puis s'arrête</text><text x="602" y="102">corrigez-le,</text><text x="602" y="120">ou validez-le</text>
</g>
</svg>
</div>
<figcaption>Aucune commande à connaître : Claude choisit la compétence d'après vos mots.</figcaption>
</figure>

<div class="example">
<p class="example-title">Exemple</p>

```text
> Prépare un plan pour un mode sombre qui suit le réglage du système
⏺ Skill(compagnon-propose)
  ⎿ Successfully loaded skill
⏺ J'écris le plan dans _compagnon/changes/add-dark-mode/.
⏺ Le plan est prêt. Lisez d'abord proposal.md ; rien n'est construit
  avant votre accord.
```

Vous n'avez jamais tapé `/compagnon-propose` : Claude a choisi la compétence d'après vos mots.

</div>

<details>
<summary>Sous le capot : les huit compétences, où elles vivent et ce qu'elles peuvent modifier</summary>

Vous n'avez rien à installer à la main : `compagnon init` et
`compagnon update` installent chaque compétence dans son propre dossier,
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

Chaque compétence ne reçoit que les outils dont sa tâche a besoin : seule
`/compagnon-apply` peut lancer n'importe quelle commande, parce qu'elle
lance vos [tests](glossary.md#test). En mode de relecture isolé,
`/compagnon-propose` et `/compagnon-apply` utilisent aussi `Agent`,
l'outil de sous-agent de Claude Code. Voir
[Permissions des outils](reference/configuration.md#permissions-des-outils).

</details>

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
- « Prépare un plan pour PROJ-123. »

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

## Annonce des mises à jour

Cinq compétences vérifient, avant de commencer, que vous utilisez la bonne
version de Compagnon : `/compagnon-propose`, `/compagnon-apply`,
`/compagnon-update`, `/compagnon-sync` et `/compagnon-archive`. Si votre
équipe exige une version que vous n'avez pas, ou si une plus récente est
sortie, la compétence le dit en une ligne et vous demande **Update now**
(mettre à jour maintenant) ou **Later** (plus tard), une fois par demande,
avant toute autre chose :

- **Update now** installe la nouvelle version, puis reprend ;
- **Later** met l'annonce de cette version en sourdine pendant 7 jours,
  puis reprend.

Quand c'est votre équipe qui exige la version, **Later** ne met rien en
sourdine, et `/compagnon-sync` et `/compagnon-archive` s'arrêtent : le
message donne la version à installer.

<details>
<summary>Sous le capot : comment une compétence apprend qu'une version existe</summary>

Les cinq compétences lisent les objets `update` et `versionRequirement` de leur
premier `compagnon status --json` ; aucune compétence n'a besoin de plus que
`Bash(compagnon:*)` pour cela. **Update now** lance `compagnon self-update --yes` ;
**Later** lance `compagnon self-update --snooze`. L'exigence de l'équipe est
`compagnon_version` dans la [configuration](reference/configuration.md#compagnon_version) :
`compagnon sync` et `compagnon archive` refusent de s'exécuter avec la mauvaise
version (`compagnon_version_unsatisfied`). `/compagnon-apply` pose la question
avant la première tâche. `/compagnon-explore`, `/compagnon-onboard` et
`/compagnon-configure` ne la posent jamais ; `/compagnon-onboard` relaie
plutôt le résultat de `compagnon doctor`, dont la vérification
`compagnon_version` dit la même chose.

</details>

## Les corrections de l'auteur

Il vous arrive de corriger Claude sur la manière de faire de votre projet :
une convention, un choix technique, ce que vous attendez d'une partie du
plan. Quand cela arrive pendant `/compagnon-propose`, `/compagnon-apply` ou
`/compagnon-update`, Claude note la correction une fois, dans le
[`corrections.md`](reference/file-formats.md#correctionsmd) de la
modification : où c'était, ce qu'il avait fait, ce que vous avez demandé,
une ligne chacun. Il vous montre ensuite l'entrée, pour que vous puissiez
la faire modifier ou retirer avant
l'[enregistrement](glossary.md#enregistrement) (commit).
`compagnon review` liste les corrections, et `compagnon validate` vérifie
leur fichier.

Claude ne note qu'une consigne qui vaudrait aussi pour la modification
suivante. Il ne note pas une préférence ponctuelle sur cette modification,
une faute de frappe, un changement de périmètre ni un malentendu sur votre
demande.

L'endroit où elle va dépend de qui elle concerne :

- une consigne sur le projet va dans `corrections.md`, que l'équipe partage
  par [git](glossary.md#git). Elle ne va pas dans la mémoire personnelle de
  Claude Code : les notes que Claude garde pour vous seul, sur votre
  ordinateur ;
- une préférence sur la façon dont Claude travaille avec vous reste dans
  cette mémoire personnelle.

Une entrée ne nomme personne, et ne cite ni donnée personnelle ni secret.

## Étape suivante

[Partir d'un ticket Jira](guides/mcp.md) Votre travail est déjà un ticket Jira ? Partez-en.
