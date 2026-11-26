# Link Checking Specification

## Purpose

What `linkcheck` reports about the links of a Markdown file.

## Requirements

### Requirement: Broken links are listed with their line

The command SHALL check every `http` and `https` link of the file it is
given, print each link that answers 400 or more, or does not answer, with
its line number, and exit with code 1 when at least one link is broken.

#### Scenario: One broken link

- **GIVEN** a file whose line 3 links to a page answering 404
- **WHEN** the user runs `linkcheck notes.md`
- **THEN** it prints `notes.md:3 404 <the link>` and exits with code 1
