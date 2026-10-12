**Update check — once per action, before acting.** In this JSON:

- `versionRequirement.satisfied` is `false` → say in one line that this
  project requires codev `<versionRequirement.requirement>`;
- otherwise, `update.available` is `true` and `update.snoozed` is `false`
  → say in one line that codev `<update.latest>` is available, with
  `<update.url>`;
- otherwise, say nothing about updates.

In the first two cases, ask one question with two answers:

- **Update now** → run `codev self-update --yes`, report its outcome in one
  line, then continue the action.
- **Later** → for an available update, run `codev self-update --snooze`;
  for a requirement, run nothing. Then continue the action.

Never ask again during this action, and never in the middle of it.
