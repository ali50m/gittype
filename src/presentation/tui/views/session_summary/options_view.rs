use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct OptionsView;

impl OptionsView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        colors: &Colors,
        is_word_mode: bool,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Row 1
                Constraint::Length(1), // Row 2
            ])
            .split(area);

        let (detail_label, share_label, retry_label, title_label, quit_label) = if is_word_mode {
            ("详情", "分享", "再练一次", "返回", "退出")
        } else {
            ("Show Detail", "Share Result", "Retry", "Back to Title", "Quit")
        };

        // Row 1: [D] Show Detail  [S] Share Result
        let row1 = Line::from(vec![
            Span::styled("[D]", Style::default().fg(colors.info())),
            Span::styled(format!(" {detail_label}"), Style::default().fg(colors.text())),
            Span::styled("  ", Style::default().fg(colors.text())),
            Span::styled("[S]", Style::default().fg(colors.info())),
            Span::styled(format!(" {share_label}"), Style::default().fg(colors.text())),
        ]);
        let row1_widget = Paragraph::new(row1).alignment(Alignment::Center);
        frame.render_widget(row1_widget, chunks[0]);

        // Row 2: [R] Retry  [T] Back to Title  [ESC] Quit
        let row2 = Line::from(vec![
            Span::styled("[R]", Style::default().fg(colors.success())),
            Span::styled(format!(" {retry_label}"), Style::default().fg(colors.text())),
            Span::styled("  ", Style::default().fg(colors.text())),
            Span::styled("[T]", Style::default().fg(colors.success())),
            Span::styled(format!(" {title_label}"), Style::default().fg(colors.text())),
            Span::styled("  ", Style::default().fg(colors.text())),
            Span::styled("[ESC]", Style::default().fg(colors.error())),
            Span::styled(format!(" {quit_label}"), Style::default().fg(colors.text())),
        ]);
        let row2_widget = Paragraph::new(row2).alignment(Alignment::Center);
        frame.render_widget(row2_widget, chunks[1]);
    }
}
