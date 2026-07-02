use crate::domain::models::challenge::Challenge;
use crate::domain::models::word_entry::WordEntry;

pub struct WordChallengeGenerator;

impl WordChallengeGenerator {
    const SEPARATOR: &'static str = "  ";

    pub fn generate(entries: Vec<WordEntry>) -> Vec<Challenge> {
        let len = entries.len();
        entries
            .iter()
            .enumerate()
            .filter_map(|(i, entry)| {
                let next = (i + 1 < len).then(|| &entries[i + 1]);
                Self::build_challenge(entry.clone(), next)
            })
            .collect()
    }

    fn build_challenge(entry: WordEntry, next_entry: Option<&WordEntry>) -> Option<Challenge> {
        let word = entry.word.trim();
        if word.is_empty() {
            return None;
        }

        let hint = [entry.phonetic.as_str(), entry.meaning.as_str()]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(Self::SEPARATOR);

        let mut display = if hint.is_empty() {
            word.to_string()
        } else {
            format!("{word}{}{hint}", Self::SEPARATOR)
        };

        let word_chars = word.chars().count();
        let display_chars_before_next = display.chars().count();
        let comment_start = word_chars + Self::SEPARATOR.chars().count();
        let mut comment_ranges: Vec<(usize, usize)> = if comment_start < display_chars_before_next {
            vec![(comment_start, display_chars_before_next)]
        } else {
            Vec::new()
        };

        if let Some(next) = next_entry {
            let next_hint = [next.phonetic.as_str(), next.meaning.as_str()]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(Self::SEPARATOR);

            let next_line = if next_hint.is_empty() {
                format!("\n  → {}", next.word)
            } else {
                format!("\n  → {}  {}", next.word, next_hint)
            };

            let next_hint_start = display_chars_before_next;
            display.push_str(&next_line);
            let next_hint_end = display.chars().count();
            comment_ranges.push((next_hint_start, next_hint_end));
        }

        Some(
            Challenge::new(uuid::Uuid::new_v4().to_string(), display)
                .with_language("word".to_string())
                .with_comment_ranges(comment_ranges)
                .with_source_file_path(entry.deck)
                .with_word(word.to_string()),
        )
    }
}
