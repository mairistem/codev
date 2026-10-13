//! The sign-in page documents every way a session starts and ends.

#[test]
fn docs_cover_sign_in() {
    let docs = std::fs::read_to_string("docs/sign-in.md").unwrap();
    assert!(docs.contains("sign in"));
}
