# API guidelines

- Clients ignore fields they do not know: adding an optional field is not a
  breaking change.
- A field is never renamed or removed without a deprecation period of two
  releases, announced in the changelog.
