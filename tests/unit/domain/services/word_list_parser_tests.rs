use gittype::domain::services::word_list_parser::WordListParser;
use std::io::Write;

fn create_temp_tsv(content: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir();
    let file_path = dir.join(format!("gittype_test_{}.tsv", uuid::Uuid::new_v4()));
    let mut file = std::fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file_path
}

#[test]
fn parse_valid_entries() {
    let tsv =
        "category\tschool-grade6\tapple\t['æpəl]\t苹果\ncategory\tschool-grade6\tbook\t[bʊk]\t书\n";
    let path = create_temp_tsv(tsv);
    let entries = WordListParser::parse_anki_tsv(&path).unwrap();
    std::fs::remove_file(&path).ok();

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].deck, "school-grade6");
    assert_eq!(entries[0].word, "apple");
    assert_eq!(entries[0].phonetic, "['æpəl]");
    assert_eq!(entries[0].meaning, "苹果");
    assert_eq!(entries[1].word, "book");
}

#[test]
fn skip_comment_lines() {
    let tsv = "# This is a comment\ncategory\tschool-grade6\tapple\t['æpəl]\t苹果\n";
    let path = create_temp_tsv(tsv);
    let entries = WordListParser::parse_anki_tsv(&path).unwrap();
    std::fs::remove_file(&path).ok();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].word, "apple");
}

#[test]
fn skip_empty_lines() {
    let tsv = "category\tschool-grade6\tapple\t['æpəl]\t苹果\n\n\ncategory\tschool-grade6\tbook\t[bʊk]\t书\n";
    let path = create_temp_tsv(tsv);
    let entries = WordListParser::parse_anki_tsv(&path).unwrap();
    std::fs::remove_file(&path).ok();

    assert_eq!(entries.len(), 2);
}

#[test]
fn empty_phonetic() {
    let tsv = "category\tschool-grade6\tBDS\t\t\n";
    let path = create_temp_tsv(tsv);
    let entries = WordListParser::parse_anki_tsv(&path).unwrap();
    std::fs::remove_file(&path).ok();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].word, "BDS");
    assert_eq!(entries[0].phonetic, "");
    assert_eq!(entries[0].meaning, "");
}

#[test]
fn multi_word_phrase() {
    let tsv = "category\tschool-grade6\tpost office\t['pəust ɔfis]\t邮局\n";
    let path = create_temp_tsv(tsv);
    let entries = WordListParser::parse_anki_tsv(&path).unwrap();
    std::fs::remove_file(&path).ok();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].word, "post office");
}

#[test]
fn skip_insufficient_columns() {
    let tsv = "col1\tcol2\tcol3\ncategory\tschool-grade6\tapple\t['æpəl]\t苹果\n";
    let path = create_temp_tsv(tsv);
    let entries = WordListParser::parse_anki_tsv(&path).unwrap();
    std::fs::remove_file(&path).ok();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].word, "apple");
}

#[test]
fn empty_file() {
    let tsv = "";
    let path = create_temp_tsv(tsv);
    let entries = WordListParser::parse_anki_tsv(&path).unwrap();
    std::fs::remove_file(&path).ok();

    assert!(entries.is_empty());
}

#[test]
fn only_comments_and_empty_lines() {
    let tsv = "# comment 1\n\n# comment 2\n  \n";
    let path = create_temp_tsv(tsv);
    let entries = WordListParser::parse_anki_tsv(&path).unwrap();
    std::fs::remove_file(&path).ok();

    assert!(entries.is_empty());
}
