# linkcheck

A command-line tool that reads a Markdown file and reports the links whose
target answers with an HTTP error.

    go build ./cmd/linkcheck
    linkcheck README.md

Exit code 0 when every link answers, 1 when one fails, 2 on a usage error.
Errors are returned and wrapped with `fmt.Errorf("...: %w", err)`; only
`main` prints them.
