//! The command line's documentation names every flag the parser accepts.

#[test]
fn docs_mention_every_flag() {
    let docs = std::fs::read_to_string("docs/cli.md").unwrap();
    for flag in ledger::cli::import::FLAGS {
        assert!(docs.contains(flag), "docs/cli.md does not mention {flag}");
    }
}
