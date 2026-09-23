---
id: 0006
title: serde_norway pour le YAML
status: accepted
date: 2026-09-08
tags: [dependances]
---

## Contexte

Le YAML est partout dans le format : `config.yaml`, `schema.yaml`,
`change.yaml`, et le frontmatter des `SKILL.md` et des décisions. Il est écrit à
la main par des humains, donc la qualité des messages d'erreur compte autant que
la conformité au standard. Or `serde_yaml`, la référence historique, publie
désormais sa version sous le nom `0.9.34+deprecated` : elle est archivée en
amont.

## Décision

`serde_norway`, fork maintenu de `serde_yaml`, avec la même API.

## Conséquences

- Migration sans coût depuis les exemples et la documentation de `serde_yaml`,
  qui restent valables.
- Un seul crate couvre la désérialisation, la sérialisation et le frontmatter.
- Le choix est confiné : le YAML n'est lu qu'aux frontières
  (`codev-core::schema`, `codev-engine::config`). En changer plus tard touche
  quelques modules, pas le domaine.

## Alternatives écartées

- **`serde_yaml`** : archivé en amont, la version le dit elle-même.
- **`serde-saphyr`** (1.x, panic-free, bons messages d'erreur) : la proposition
  la plus intéressante sur le papier, mais implémentation récente et
  compatibilité serde moins éprouvée. À reconsidérer si les messages d'erreur de
  `serde_norway` s'avèrent insuffisants pour du YAML écrit à la main — ce serait
  alors une décision qui remplace celle-ci.
- **`yaml-rust2` brut** : nous ferait écrire nous-mêmes la couche
  désérialisation.
