use gittype::infrastructure::audio::audio_player::pronunciation_tokens;

#[test]
fn splits_phrase_on_common_separators() {
    assert_eq!(
        pronunciation_tokens("go for a walk"),
        vec!["go", "for", "a", "walk"]
    );
    assert_eq!(pronunciation_tokens("have...class"), vec!["have", "class"]);
    assert_eq!(pronunciation_tokens("go swimming."), vec!["go", "swimming"]);
}

#[test]
fn splits_mixed_separator_phrase() {
    assert_eq!(
        pronunciation_tokens("read/write, listen-and_say (again)"),
        vec!["read", "write", "listen", "and", "say", "again"]
    );
}

#[test]
fn ignores_repeated_or_outer_separators() {
    assert_eq!(
        pronunciation_tokens("  /take///a---dancing___class!! "),
        vec!["take", "a", "dancing", "class"]
    );
}
