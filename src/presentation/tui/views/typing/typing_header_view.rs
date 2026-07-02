use crate::domain::models::Languages;
use crate::{
    domain::models::{Challenge, GitRepository},
    presentation::ui::Colors,
};
use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct TypingHeaderView;

impl TypingHeaderView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        challenge: Option<&Challenge>,
        git_repository: Option<&GitRepository>,
        colors: &Colors,
    ) {
        Self::render_with_stage_info(frame, area, challenge, git_repository, colors, None);
    }

    pub fn render_with_stage_info(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        challenge: Option<&Challenge>,
        git_repository: Option<&GitRepository>,
        colors: &Colors,
        stage_info: Option<(usize, usize)>,
    ) {
        let is_word_mode = challenge
            .map(|c| c.language.as_deref() == Some("word"))
            .unwrap_or(false);

        let header_text = if let Some(challenge) = challenge {
            let base_title = if is_word_mode {
                let deck = challenge.source_file_path.as_deref().unwrap_or("Words");
                match stage_info {
                    Some((current, total)) => format!("{} {}/{}", deck, current, total),
                    None => deck.to_string(),
                }
            } else {
                challenge.get_display_title_with_repo(&git_repository.cloned())
            };

            let difficulty_text = match &challenge.difficulty_level {
                Some(difficulty) => format!("{:?}", difficulty),
                None => "Unknown".to_string(),
            };

            // Create spans for colored language display before difficulty
            let mut spans = vec![Span::styled(
                base_title,
                Style::default().fg(colors.text_secondary()),
            )];

            // In word mode, skip language and difficulty tags
            if !is_word_mode {
                // Add language with color if available
                if let Some(ref language) = challenge.language {
                    let display_name = Languages::get_display_name(Some(language));
                    spans.push(Span::styled(
                        " ",
                        Style::default().fg(colors.text_secondary()),
                    ));
                    spans.push(Span::styled(
                        format!("[{}]", display_name),
                        Style::default().fg(colors.info()),
                    ));
                }

                // Add difficulty at the end
                spans.push(Span::styled(
                    format!(" [{}]", difficulty_text),
                    Style::default().fg(colors.text_secondary()),
                ));
            }

            Line::from(spans)
        } else {
            Line::from(vec![Span::styled(
                "[Challenge]",
                Style::default().fg(colors.text_secondary()),
            )])
        };

        let header_title = if is_word_mode {
            "单词练习"
        } else {
            "Challenge"
        };

        let header = Paragraph::new(vec![header_text]).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.border()))
                .title(header_title)
                .title_style(Style::default().fg(colors.border()))
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );
        frame.render_widget(header, area);
    }
}
