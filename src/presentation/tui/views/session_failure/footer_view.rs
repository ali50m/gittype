use crate::presentation::ui::Colors;
use ratatui::{
    layout::Alignment,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct FooterView;

impl FooterView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        colors: &Colors,
        is_word_mode: bool,
    ) {
        let (retry_label, title_label, exit_label) = if is_word_mode {
            ("重试", "返回", "总结 & 退出")
        } else {
            ("Retry", "Back to Title", "Session Summary & Exit")
        };

        let nav_line = Line::from(vec![
            Span::styled("[R]", Style::default().fg(colors.success())),
            Span::styled(
                format!(" {retry_label} | "),
                Style::default().fg(colors.text()),
            ),
            Span::styled("[T]", Style::default().fg(colors.success())),
            Span::styled(
                format!(" {title_label} | "),
                Style::default().fg(colors.text()),
            ),
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(format!(" {exit_label}"), Style::default().fg(colors.text())),
        ]);
        let navigation = Paragraph::new(nav_line).alignment(Alignment::Center);
        frame.render_widget(navigation, area);
    }
}
