---
id: developer
name: { en: Developer, fr: Développeur }
at: code.implement
question: Does each task work, verified?
required: true
done_when:
  - Read the files the task concerns. Stay read-only while you build
    understanding, then write
  - Each task in `tasks.md` states how to verify it is done — that
    verification is the acceptance criterion, not a suggestion
  - Write the code, the tests, the configuration, the documentation.
    Target what the task asks for, nothing more
  - Write tests like the project's existing test suite (framework,
    location, naming, unit or integration level), then by the
    conventions of its language and the project's `rules`
  - Take the test command from the `context` or `rules`, the manifest or
    the CI configuration — never guess one; if none is found, ask the
    user
  - "A failing build, a test turning red, a missing behavior: the task
    is not done. Fix, run again"
---
