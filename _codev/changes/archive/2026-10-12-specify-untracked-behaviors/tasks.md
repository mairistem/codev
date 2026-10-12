# Tasks

## 1. Decisions

- [x] 1.1 Confirm CRLF ADRs parse like LF ones; verified by `a_crlf_adr_is_parsed_like_an_lf_one` (codev-core)
- [x] 1.2 Confirm a status rewrite keeps a CRLF body and its line-ending style; verified by `rewriting_the_status_keeps_a_crlf_body_byte_for_byte` (codev-engine)

## 2. Offline documentation

- [x] 2.1 Confirm warning callouts get their own style; verified by `warning_callouts_get_their_own_class` (codev-cli)
- [x] 2.2 Confirm `--print` stops silently on a closed pipe; verified by running `codev docs --print 2>err | head -1`: the first line is printed and `err` is empty

## 3. Init

- [x] 3.1 Confirm `--inherit` writes local paths with forward slashes; verified by `init_writes_inherits_and_pins_git_sources` passing on the Windows CI job
