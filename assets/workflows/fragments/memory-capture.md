## The author's corrections

When the author corrects what you did with an instruction about how this
project does things — a convention, a technical choice, an expectation on
an artifact — note it once in the change's `corrections.md`: the line
`<!-- codev-corrections: 1 -->` first when the file is new, then
`### C-<n> <the instruction in a few words>`, `<n>` the next free number,
and a `yaml` block with `step` (`propose`, `apply` or `update`), `where`
(the file or artifact), `did` (what you had done) and `asked` (what the
author asked instead), one line each. Read nothing else of the file. Then
quote the entry to the author, who may have it changed or removed.

It counts when it would apply to the next change too: "errors go through
`thiserror` in the libraries, `anyhow` stays in the CLI"; "every scenario
is covered by a test named after the behavior"; "a new JSON field is
additive, never renamed". It does not: a one-off preference about this
change, a typo, a change of scope or of the request, a misunderstanding
of the request, or a preference about how you talk or work with this
author.

A project instruction goes here, not to your personal memory; a personal
preference stays in your personal memory, never here. An entry names no
person and quotes no personal data or secret.
