use crate::domain::models::SessionResult;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct SummaryView;

impl SummaryView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        session_result: &SessionResult,
        colors: &Colors,
        is_word_mode: bool,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Line 1: CPM | WPM | Time
                Constraint::Length(1), // Line 2: Keystrokes | Mistakes | Accuracy
            ])
            .split(area);

        let (cpm_label, wpm_label, time_label, key_label, err_label, acc_label) = if is_word_mode {
            ("字/分", "词/分", "用时", "击键", "错误", "正确率")
        } else {
            ("CPM", "WPM", "Time", "Keystrokes", "Mistakes", "Accuracy")
        };

        // Line 1: CPM | WPM | Time
        let line1 = Line::from(vec![
            Span::styled(
                format!("{cpm_label}: "),
                Style::default().fg(colors.cpm_wpm()),
            ),
            Span::styled(
                format!("{:.0}", session_result.overall_cpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{wpm_label}: "),
                Style::default().fg(colors.cpm_wpm()),
            ),
            Span::styled(
                format!("{:.0}", session_result.overall_wpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{time_label}: "),
                Style::default().fg(colors.duration()),
            ),
            Span::styled(
                format!("{:.1}s", session_result.session_duration.as_secs_f64()),
                Style::default().fg(colors.text()),
            ),
        ]);
        let line1_widget = Paragraph::new(line1).alignment(Alignment::Center);
        frame.render_widget(line1_widget, chunks[0]);

        // Line 2: Keystrokes | Mistakes | Accuracy
        let total_keystrokes = session_result.valid_keystrokes + session_result.invalid_keystrokes;
        let total_mistakes = session_result.valid_mistakes + session_result.invalid_mistakes;

        let line2 = Line::from(vec![
            Span::styled(
                format!("{key_label}: "),
                Style::default().fg(colors.stage_info()),
            ),
            Span::styled(
                format!("{}", total_keystrokes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{err_label}: "),
                Style::default().fg(colors.error()),
            ),
            Span::styled(
                format!("{}", total_mistakes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{acc_label}: "),
                Style::default().fg(colors.accuracy()),
            ),
            Span::styled(
                format!("{:.1}%", session_result.overall_accuracy),
                Style::default().fg(colors.text()),
            ),
        ]);
        let line2_widget = Paragraph::new(line2).alignment(Alignment::Center);
        frame.render_widget(line2_widget, chunks[1]);
    }
}
