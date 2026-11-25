# Moving from codev

Until 0.6, Compagnon was named **codev**. Version 0.7.0 renames everything;
a project set up with codev keeps working once converted, in one command.

## In short

1. **Install Compagnon** — `codev self-update` does it (you then get a
   `codev` executable that asks to be renamed `compagnon`), or run the
   installer of the [installation guide](../installation.md).
2. **Preview the conversion** in each project:

   ```sh
   compagnon migrate --dry-run
   ```

3. **Convert it**, then commit the result:

   ```sh
   compagnon migrate
   git add -A && git commit -m "Move to Compagnon"
   ```

Until a project is converted, every other command refuses it with
`legacy_root` and tells you to run `compagnon migrate`: nothing is read or
written under `_codev/` behind your back.

## What is renamed

| Before | After |
|---|---|
| `codev` command | `compagnon` |
| `_codev/` | `_compagnon/` |
| `_codev/codev.lock` | `_compagnon/compagnon.lock` |
| `codev_version:` | `compagnon_version:` |
| `/codev-propose`, `/codev-apply`… | `/compagnon-propose`, `/compagnon-apply`… |
| `CODEV_NO_UPDATE_CHECK`, `CODEV_VERSION`… | `COMPAGNON_NO_UPDATE_CHECK`, `COMPAGNON_VERSION`… |
| `.github/workflows/codev.yml` | `.github/workflows/compagnon.yml` |
| JSON codes `no_codev_root`, `codev_version_unsatisfied`… | `no_compagnon_root`, `compagnon_version_unsatisfied`… |
| github.com/mairistem/codev | github.com/mairistem/compagnon |

`compagnon migrate` handles the project's files. What lives outside the
project is yours to update: scripts that call `codev`, shell aliases, and
the environment variables above.

## What `compagnon migrate` keeps

- **Your prose.** Specs, decisions and changes are moved, never reworded:
  a spec that says "run `codev validate`" still says so.
- **What you edited by hand.** A `codev-*` skill or a `codev.yml` workflow
  that no longer matches what codev generated stays in place, reported as
  kept. Bring your edits over to the new file, then delete the old one.
  Skills installed by codev 0.6 carry no fingerprint, so an edit to them
  cannot be detected: they are replaced. If you edited one, preview with
  `--dry-run` and recover your edit from git after the migration.

## Particular cases

**A version requirement to widen.** A project pinned to codev releases —
`codev_version: "^0.6"` — would refuse every verdict once converted.
`compagnon migrate` stops before writing anything
(`migration_version_unsatisfied`): widen the requirement in
`_codev/config.yaml`, for example to `">=0.6"`, then run it again.

**A shared source repository.** A project that inherits from a repository
still laid out for codev is refused with `legacy_source`. Migrate the source
repository first. Its maintainers may keep a copy of `_codev/` beside
`_compagnon/` during the transition, so that projects still on codev keep
reading it. Then, in each project that inherits from it, run
`compagnon sources update` to pin a commit laid out for Compagnon — the
pinned commit, otherwise, predates the source's migration.

**CI jobs.** A job that installs codev with `CODEV_VERSION=0.6.1` keeps
working: the installers still serve codev releases under their former name.
Once the project is converted, switch the job to `COMPAGNON_VERSION` and the
`compagnon` command. `compagnon migrate` replaces the generated
`.github/workflows/codev.yml`; a GitLab job, or a workflow you wrote
yourself, is yours to update.

**A migration that stopped.** If a file cannot be written or removed, the
command stops with `migration_incomplete`, names the path and lists what it
already did. Fix the cause, then run `compagnon migrate` again: it completes
what is left.

**Going back.** Before committing, `git checkout` restores the project.
After, revert the commit. To reinstall codev itself:

```sh
compagnon self-update --version 0.6.1
```
