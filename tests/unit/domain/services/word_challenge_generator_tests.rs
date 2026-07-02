use gittype::domain::models::typing::ProcessingOptions;
use gittype::domain::models::word_entry::WordEntry;
use gittype::domain::services::typing_core::TypingCore;
use gittype::domain::services::word_challenge_generator::WordChallengeGenerator;

fn make_entry(word: &str, phonetic: &str, meaning: &str) -> WordEntry {
    WordEntry {
        deck: "test-deck".to_string(),
        word: word.to_string(),
        phonetic: phonetic.to_string(),
        meaning: meaning.to_string(),
    }
}

#[test]
fn generates_correct_comment_ranges() {
    let entries = vec![make_entry("science", "['saɪəns]", "科学")];
    let challenges = WordChallengeGenerator::generate(entries);
    assert_eq!(challenges.len(), 1);

    let challenge = &challenges[0];
    // display = "science  ['saɪəns]  科学"
    // word = "science" (7 chars), separator = "  " (2 chars)
    // comment_ranges should start after separator
    assert_eq!(challenge.language.as_deref(), Some("word"));
    assert_eq!(challenge.source_file_path.as_deref(), Some("test-deck"));
    assert!(!challenge.comment_ranges.is_empty());
    assert_eq!(challenge.comment_ranges[0].0, 9); // 7 (word) + 2 (separator)
}

#[test]
fn typing_core_comment_ranges_match_display() {
    let entries = vec![make_entry("science", "['saɪəns]", "科学")];
    let challenges = WordChallengeGenerator::generate(entries);
    let challenge = &challenges[0];

    let typing_core = TypingCore::new(
        &challenge.code_content,
        &challenge.comment_ranges,
        ProcessingOptions::default(),
    );

    let display_ranges = typing_core.display_comment_ranges();
    assert!(
        !display_ranges.is_empty(),
        "Expected non-empty display_comment_ranges"
    );

    // The comment text should be shown in gray (matching display_comment_ranges)
    let text_to_type = typing_core.text_to_type();
    let text_to_display = typing_core.text_to_display();

    assert_eq!(text_to_type, "science");
    assert!(text_to_display.contains("['saɪəns]"));
    assert!(text_to_display.contains("科学"));
}

#[test]
fn empty_phonetic_and_meaning() {
    let entries = vec![make_entry("BDS", "", "")];
    let challenges = WordChallengeGenerator::generate(entries);
    assert_eq!(challenges.len(), 1);

    let challenge = &challenges[0];
    // No hint, so display = word only, no comment ranges
    assert_eq!(challenge.code_content, "BDS");
    assert!(challenge.comment_ranges.is_empty());
}

#[test]
fn empty_phonetic_only() {
    let entries = vec![make_entry("hello", "", "你好")];
    let challenges = WordChallengeGenerator::generate(entries);
    assert_eq!(challenges.len(), 1);

    let challenge = &challenges[0];
    // display = "hello  你好" (word + separator + meaning only)
    assert!(challenge.code_content.contains("你好"));
}

#[test]
fn multi_word_phrase() {
    let entries = vec![make_entry("post office", "['pəust ɔfis]", "邮局")];
    let challenges = WordChallengeGenerator::generate(entries);
    assert_eq!(challenges.len(), 1);

    let challenge = &challenges[0];
    // word = "post office" (11 chars), separator = 2 chars
    assert_eq!(challenge.comment_ranges[0].0, 13);

    let typing_core = TypingCore::new(
        &challenge.code_content,
        &challenge.comment_ranges,
        ProcessingOptions::default(),
    );
    assert_eq!(typing_core.text_to_type(), "post office");
}

#[test]
fn empty_word_skipped() {
    let entries = vec![make_entry("", "phonetic", "meaning")];
    let challenges = WordChallengeGenerator::generate(entries);
    assert!(challenges.is_empty());
}

#[test]
fn whitespace_only_word_skipped() {
    let entries = vec![make_entry("   ", "phonetic", "meaning")];
    let challenges = WordChallengeGenerator::generate(entries);
    assert!(challenges.is_empty());
}
