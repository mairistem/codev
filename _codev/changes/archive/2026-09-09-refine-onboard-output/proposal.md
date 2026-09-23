# Proposal : affiner la sortie de `/codev-onboard`

## Pourquoi

Le smoke-test de `/codev-onboard` sur ce dépôt (2026-09-09) a fait
remonter deux frictions à l'usage :

1. Le bloc « ici, tu as » compte les changes **actifs** (0 pour ce
   dépôt) mais ignore les **archivés** — 11 aujourd'hui. Un nouvel
   arrivant sur un projet mûr voit « 0 change actif » et pense que le
   projet n'a rien produit ; l'historique reste invisible.
2. La recommandation par défaut sans change actif est
   `/codev-propose <une-idée>` — abstrait. Un nouvel arrivant qui
   n'a pas d'idée précise a besoin de prendre le pouls du projet
   d'abord ; le premier réflexe naturel est d'ouvrir `README.md` avant
   d'inventer un change.

## Ce qui change

- **Le bloc « ici, tu as » compte aussi les changes archivés** — une
  ligne supplémentaire « `X change(s) archivé(s)` » quand ce nombre est
  non nul. Silencieux si zéro (pas de bruit sur un projet neuf).
- **La recommandation par défaut cite explicitement `README.md`** —
  « prends le pouls du projet en lisant `README.md`, puis
  `/codev-propose <une-idée>` ». La suggestion `/codev-explore
  <sujet>` reste mentionnée comme alternative si l'utilisateur a une
  question mais pas encore d'idée d'action.
- **Aucun changement de frontière** — reste strictement en lecture, les
  `allowed-tools` restent inchangés (`Bash(codev:*), Read, Glob`).

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `skills` — l'exigence `Skill onboard présente codev et recommande
  la prochaine action` est modifiée pour couvrir ces deux ajouts. Le
  reste (frontière lecture, `allowed-tools`, présence catalogue par
  défaut) est inchangé.

### Capacités retirées

Aucune.

## Impact

- **Code** : édition du body `assets/workflows/onboard.md` (bloc 2 et
  bloc 3). L'entrée `CATALOG` reste identique — même id, même
  description, mêmes `allowed-tools`.
- **Tests** : l'invariant `onboard_cite_ses_trois_blocs` continue de
  passer (les mots-clés `codev, c'est` / `ici, tu as` / `la suite`
  restent présents). Aucun test à ajouter — les nouveaux détails
  vivent dans le body, la spec cadre les grandes lignes.
- **Contrat JSON** : rien. La skill ne parse aucun JSON, ne produit
  aucune sortie structurée.
- **Migration** : aucune. Le comportement change à la prochaine session
  Claude Code après `codev update`.
- **Hors périmètre** :
  - **Distinguer les archivés par date/ancienneté** — un simple
    compte suffit ; une hiérarchie temporelle pourrait venir plus tard
    si un besoin apparaît.
  - **Détecter la présence d'un `README.md`** — la skill le suggère
    inconditionnellement ; si le dépôt n'en a pas, l'utilisateur
    l'apprend en essayant. Coût nul, robustesse suffisante.
