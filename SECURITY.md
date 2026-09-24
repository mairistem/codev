# Politique de sécurité

## Versions supportées

| Version | Support |
|---------|---------|
| 0.x     | Toutes les versions récentes tant que codev est en 0.x |

Une fois codev passé en 1.x, seule la branche majeure la plus
récente sera supportée activement. Cette table sera mise à jour à ce
moment-là.

## Signaler une faille

Merci de **ne pas ouvrir d'issue publique** pour signaler une faille
de sécurité.

Utilise le mécanisme privé de GitHub :

1. Va sur [github.com/mairistem/codev/security/advisories](https://github.com/mairistem/codev/security/advisories).
2. Clique **« Report a vulnerability »**.
3. Décris la faille, un scénario reproductible, et l'impact attendu.

Ce canal est privé, versionné et permet la coordination d'une CVE si
nécessaire.

**Délai de première réponse cible : 72 heures.** Si tu ne reçois
aucune réponse dans ce délai, tu peux ouvrir une issue publique
minimale intitulée « Re: vulnerability report » sans donner le
détail, pour signaler qu'un rapport privé attend traitement.

## Périmètre

Sont dans le périmètre les failles qui affectent :

- Le binaire `codev` (crates `codev-core`, `codev-engine`,
  `codev-cli`).
- Les scripts `install.sh` et `install.ps1` (intégrité, chaîne de
  téléchargement, vérification SHA-256).
- La chaîne de publication (workflow `.github/workflows/release.yml`).

Sont hors périmètre :

- Les vulnérabilités des dépendances tierces qui n'ont pas d'impact
  démontré sur codev (à signaler en amont au projet concerné).
- Les problèmes d'utilisation qui ne relèvent pas d'une faille de
  sécurité (à ouvrir en issue publique).
