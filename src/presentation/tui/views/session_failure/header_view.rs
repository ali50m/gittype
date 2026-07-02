use crate::presentation::ui::Colors;
use ratatui::{
    layout::Alignment,
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
        let title = if is_word_mode {
            "=== 练习中断 ==="
        } else {
            "=== SESSION FAILED ==="
        };
        let header = Paragraph::new(Line::from(vec![Span::styled(
            title,
            Style::default()
                .fg(colors.error())
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(header, area);
    }
}
