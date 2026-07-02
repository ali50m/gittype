use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct HeaderView;

impl HeaderView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        colors: &Colors,
        is_word_mode: bool,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Session title
                Constraint::Length(2), // Spacing
                Constraint::Length(1), // YOU'RE label
            ])
            .split(area);

        let title = if is_word_mode {
            "=== 练习完成 ==="
        } else {
            "=== SESSION COMPLETE ==="
        };
        let session_title = Paragraph::new(Line::from(vec![Span::styled(
            title,
            Style::default()
                .fg(colors.info())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(session_title, chunks[0]);

        let youre_text = if is_word_mode { "你的评级:" } else { "YOU'RE:" };
        let youre_label = Paragraph::new(Line::from(vec![Span::styled(
            youre_text,
            Style::default()
                .fg(colors.info())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(youre_label, chunks[2]);
    }
}
