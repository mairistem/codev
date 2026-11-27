# Migrer depuis codev

Jusqu'à la 0.6, Compagnon s'appelait **codev**. La version 0.7.0 renomme
tout ; un projet créé avec codev continue de fonctionner une fois converti,
en une commande.

## En bref

1. **Installez Compagnon** — `codev self-update` s'en charge (vous obtenez
   alors un exécutable `codev` qui vous demande de le renommer
   `compagnon`), ou lancez l'installeur du
   [guide d'installation](../installation.md).
2. **Prévisualisez la conversion** dans chaque projet :

   ```sh
   compagnon migrate --dry-run
   ```

3. **Convertissez-le**, puis enregistrez le résultat :

   ```sh
   compagnon migrate
   git add -A && git commit -m "Passage à Compagnon"
   ```

Tant qu'un projet n'est pas converti, toutes les autres commandes le
refusent avec `legacy_root` et vous indiquent de lancer
`compagnon migrate` : rien n'est lu ni écrit sous `_codev/` à votre insu.

## Ce qui est renommé

| Avant | Après |
|---|---|
| commande `codev` | `compagnon` |
| `_codev/` | `_compagnon/` |
| `_codev/codev.lock` | `_compagnon/compagnon.lock` |
| `codev_version:` | `compagnon_version:` |
| `/codev-propose`, `/codev-apply`… | `/compagnon-propose`, `/compagnon-apply`… |
| `CODEV_NO_UPDATE_CHECK`, `CODEV_VERSION`… | `COMPAGNON_NO_UPDATE_CHECK`, `COMPAGNON_VERSION`… |
| `.github/workflows/codev.yml` | `.github/workflows/compagnon.yml` |
| codes JSON `no_codev_root`, `codev_version_unsatisfied`… | `no_compagnon_root`, `compagnon_version_unsatisfied`… |
| github.com/mairistem/codev | github.com/mairistem/compagnon |

`compagnon migrate` s'occupe des fichiers du projet. Ce qui vit en dehors
du projet reste à votre charge : les scripts qui appellent `codev`, les
alias de votre shell et les variables d'environnement ci-dessus.

## Ce que `compagnon migrate` conserve

- **Votre prose.** Les spécifications, décisions et modifications sont
  déplacées, jamais reformulées : une spécification qui dit « lancez `codev validate` » le dit toujours.
- **Ce que vous avez modifié à la main.** Une compétence `codev-*` ou un
  fichier de déroulé `codev.yml` qui ne correspond plus à ce que codev avait généré
  reste en place, signalé comme conservé. Reportez vos modifications dans le
  nouveau fichier, puis supprimez l'ancien. Les compétences installées par codev
  0.6 ne portent pas d'empreinte : une modification n'y est pas détectable,
  elles sont donc remplacées. Si vous en aviez modifié une, prévisualisez
  avec `--dry-run` et récupérez votre modification dans git après la
  migration.

## Cas particuliers

**Une exigence de version à élargir.** Un projet épinglé sur les versions
de codev — `codev_version: "^0.6"` — refuserait tout verdict une fois
converti. `compagnon migrate` s'arrête avant d'écrire quoi que ce soit
(`migration_version_unsatisfied`) : élargissez l'exigence dans
`_codev/config.yaml`, par exemple à `">=0.6"`, puis relancez-le.

**Un dépôt de sources partagé.** Un projet qui hérite d'un dépôt encore
organisé pour codev est refusé avec `legacy_source`. Migrez d'abord le
dépôt de sources. Ses mainteneurs peuvent conserver une copie de `_codev/`
à côté de `_compagnon/` pendant la transition, pour que les projets encore
sous codev continuent de la lire. Ensuite, dans chaque projet qui en
hérite, lancez `compagnon sources update` pour épingler un enregistrement organisé
pour Compagnon — sinon, l'enregistrement épinglé précède la migration de la
source.

**Les jobs de CI.** Un job qui installe codev avec `CODEV_VERSION=0.6.1`
continue de fonctionner : les installeurs servent toujours les versions de
codev sous leur ancien nom. Une fois le projet converti, passez le job à
`COMPAGNON_VERSION` et à la commande `compagnon`. `compagnon migrate`
remplace le `.github/workflows/codev.yml` généré ; un job GitLab, ou un
fichier de déroulé que vous avez écrit vous-même, reste à mettre à jour par vos soins.

**Une migration interrompue.** Si un fichier ne peut pas être écrit ou
supprimé, la commande s'arrête avec `migration_incomplete`, indique le
chemin en cause et liste ce qu'elle a déjà fait. Corrigez la cause, puis
relancez `compagnon migrate` : elle termine ce qui reste.

**Revenir en arrière.** Avant d'enregistrer, `git checkout` restaure le
projet. Après, annulez l'enregistrement. Pour réinstaller codev lui-même :

```sh
compagnon self-update --version 0.6.1
```
