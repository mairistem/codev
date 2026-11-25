# Tasks

## 1. Skill body

- [x] 1.1 Edit `assets/workflows/onboard.md`, block 2 ("here you
      have"): after the list of active changes, add a bullet
      "Q archived change(s)" displayed **only if Q > 0**. The skill
      counts the folders under `_codev/changes/archive/` via `ls` or
      an equivalent command (Glob). A new project or one without an
      archive adds nothing.
- [x] 1.2 Edit block 3 ("what's next"), branch "project initialized,
      no active change": new wording that invites reading `README.md`
      first, then suggests `/codev-propose <an-idea>`, and mentions
      `/codev-explore <topic>` as an alternative. Example of expected
      output:
      > **What's next**: start by reading `README.md` to get a feel
      > for the project. Then, when an idea emerges, type
      > `/codev-propose <an-idea>`. Alternative if you have a question
      > but no idea for an action yet: `/codev-explore <topic>`.
- [x] 1.3 The other branches (active change, several changes,
      `codev init` missing) stay unchanged.

## 2. Checks

- [x] 2.1 The invariant `onboard_cite_ses_trois_blocs` keeps
      passing — the keywords `codev is` / `here you have` / `what's
      next` remain present in the body. Verified by
      `cargo test -p codev-agents onboard_cite_ses_trois_blocs`.
- [x] 2.2 `cargo test --workspace` stays green.
- [x] 2.3 `cargo clippy --workspace --all-targets` stays free of
      warnings.
- [x] 2.4 `codev validate --strict` stays green.

## 3. Dogfooding

- [x] 3.1 After `cargo install --path crates/codev-cli` then
      `codev update`, rerun `/codev-onboard` on this repository and
      check visually:
      - the line "11 archived change(s)" appears in "here you
        have";
      - the "what's next" block cites `README.md` before
        `/codev-propose`.
