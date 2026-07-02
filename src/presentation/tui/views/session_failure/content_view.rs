use crate::domain::models::SessionResult;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct ContentView;

impl ContentView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        session_result: &SessionResult,
        total_stages: usize,
        colors: &Colors,
        is_word_mode: bool,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Stage progress
                Constraint::Length(1), // Spacing
                Constraint::Length(1), // Metrics line 1
                Constraint::Length(1), // Metrics line 2
                Constraint::Length(1), // Spacing
                Constraint::Length(1), // Failure message
            ])
            .split(area);

        // Stage progress
        let stage_text = if is_word_mode {
            format!(
                "已完成: {}/{} 个单词",
                session_result.stages_completed, total_stages
            )
        } else {
            format!(
                "Stages: {}/{}",
                session_result.stages_completed, total_stages
            )
        };
        let stage_progress = Paragraph::new(Line::from(vec![Span::styled(
            stage_text,
            Style::default().fg(colors.info()),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(stage_progress, chunks[0]);

        // Metrics
        let total_keystrokes = session_result.valid_keystrokes + session_result.invalid_keystrokes;
        let total_mistakes = session_result.valid_mistakes + session_result.invalid_mistakes;

        let (cpm_label, wpm_label, time_label, key_label, err_label, acc_label) = if is_word_mode {
            ("字/分", "词/分", "用时", "击键", "错误", "正确率")
        } else {
            ("CPM", "WPM", "Time", "Keystrokes", "Mistakes", "Accuracy")
        };

        // Line 1: CPM | WPM | Time
        let metrics_line1 = Line::from(vec![
            Span::styled(format!("{cpm_label}: "), Style::default().fg(colors.cpm_wpm())),
            Span::styled(
                format!("{:.0}", session_result.overall_cpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(format!(" | {wpm_label}: "), Style::default().fg(colors.cpm_wpm())),
            Span::styled(
                format!("{:.0}", session_result.overall_wpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(format!(" | {time_label}: "), Style::default().fg(colors.duration())),
            Span::styled(
                format!("{:.1}s", session_result.session_duration.as_secs_f64()),
                Style::default().fg(colors.text()),
            ),
        ]);
        let metrics_widget1 = Paragraph::new(metrics_line1).alignment(Alignment::Center);
        frame.render_widget(metrics_widget1, chunks[2]);

        // Line 2: Keystrokes | Mistakes | Accuracy
        let metrics_line2 = Line::from(vec![
            Span::styled(format!("{key_label}: "), Style::default().fg(colors.stage_info())),
            Span::styled(
                format!("{}", total_keystrokes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(format!(" | {err_label}: "), Style::default().fg(colors.error())),
            Span::styled(
                format!("{}", total_mistakes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(format!(" | {acc_label}: "), Style::default().fg(colors.accuracy())),
            Span::styled(
                format!("{:.1}%", session_result.overall_accuracy),
                Style::default().fg(colors.text()),
            ),
        ]);
        let metrics_widget2 = Paragraph::new(metrics_line2).alignment(Alignment::Center);
        frame.render_widget(metrics_widget2, chunks[3]);

        // Failure message
        let fail_msg_text = if is_word_mode {
            "练习中断，再接再厉！"
        } else {
            "Challenge failed. Better luck next time!"
        };
        let fail_msg = Paragraph::new(Line::from(vec![Span::styled(
            fail_msg_text,
            Style::default().fg(colors.error()),
        )]))
        .alignment(Alignment::Center);
        frame.render_widget(fail_msg, chunks[5]);
    }
}
