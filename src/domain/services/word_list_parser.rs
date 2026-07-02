use crate::domain::models::word_entry::WordEntry;
use crate::Result;
use std::path::Path;

pub struct WordListParser;

impl WordListParser {
    pub fn parse_anki_tsv(path: &Path) -> Result<Vec<WordEntry>> {
        let content = std::fs::read_to_string(path)?;

        Ok(content
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#')
            })
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                (parts.len() >= 5).then(|| WordEntry {
                    deck: parts[1].to_string(),
                    word: parts[2].to_string(),
                    phonetic: parts[3].to_string(),
                    meaning: parts[4].to_string(),
                })
            })
            .collect())
    }
}
