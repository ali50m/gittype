use crate::domain::models::TotalResult;
use crate::presentation::ui::Colors;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct StatisticsView;

impl StatisticsView {
    pub fn render(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        total_summary: &TotalResult,
        colors: &Colors,
        is_word_mode: bool,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Line 1: CPM, WPM, Accuracy
                Constraint::Length(1), // Line 2: Sessions and Stages
                Constraint::Length(1), // Line 3: Keystrokes, Mistakes, Skipped
                Constraint::Length(1), // Line 4: Best/Worst sessions
            ])
            .split(area);

        let (
            overall_label,
            sessions_label,
            completed_label,
            stages_label,
            keystrokes_label,
            mistakes_label,
            skipped_label,
            best_label,
            worst_label,
        ) = if is_word_mode {
            (
                "总计",
                "练习次数",
                "已完成",
                "单词数",
                "击键",
                "错误",
                "跳过",
                "最佳",
                "最差",
            )
        } else {
            (
                "Overall",
                "Total Sessions",
                "Completed",
                "Stages",
                "Total Keystrokes",
                "Mistakes",
                "Skipped",
                "Best Session",
                "Worst",
            )
        };

        // Line 1: Overall CPM, WPM, Accuracy
        let line1 = Line::from(vec![
            Span::styled(
                format!("{overall_label} "),
                Style::default().fg(colors.text()),
            ),
            Span::styled(
                format!("{}: ", if is_word_mode { "字/分" } else { "CPM" }),
                Style::default().fg(colors.cpm_wpm()),
            ),
            Span::styled(
                format!("{:.1}", total_summary.overall_cpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{}: ", if is_word_mode { "词/分" } else { "WPM" }),
                Style::default().fg(colors.cpm_wpm()),
            ),
            Span::styled(
                format!("{:.1}", total_summary.overall_wpm),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled(
                format!(
                    "{}: ",
                    if is_word_mode {
                        "正确率"
                    } else {
                        "Accuracy"
                    }
                ),
                Style::default().fg(colors.accuracy()),
            ),
            Span::styled(
                format!("{:.1}%", total_summary.overall_accuracy),
                Style::default().fg(colors.text()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(line1).alignment(Alignment::Center),
            chunks[0],
        );

        // Line 2: Sessions and Stages
        let line2 = Line::from(vec![
            Span::styled(
                format!("{sessions_label}: "),
                Style::default().fg(colors.stage_info()),
            ),
            Span::styled(
                format!("{}", total_summary.total_sessions_attempted),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{completed_label}: "),
                Style::default().fg(colors.success()),
            ),
            Span::styled(
                format!("{}", total_summary.total_sessions_completed),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{stages_label}: "),
                Style::default().fg(colors.stage_info()),
            ),
            Span::styled(
                format!(
                    "{}/{}",
                    total_summary.total_stages_completed, total_summary.total_stages_attempted
                ),
                Style::default().fg(colors.text()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(line2).alignment(Alignment::Center),
            chunks[1],
        );

        // Line 3: Keystrokes, Mistakes, Skipped
        let line3 = Line::from(vec![
            Span::styled(
                format!("{keystrokes_label}: "),
                Style::default().fg(colors.stage_info()),
            ),
            Span::styled(
                format!("{}", total_summary.total_keystrokes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{mistakes_label}: "),
                Style::default().fg(colors.error()),
            ),
            Span::styled(
                format!("{}", total_summary.total_mistakes),
                Style::default().fg(colors.text()),
            ),
            Span::styled(" | ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{skipped_label}: "),
                Style::default().fg(colors.warning()),
            ),
            Span::styled(
                format!("{}", total_summary.total_stages_skipped),
                Style::default().fg(colors.text()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(line3).alignment(Alignment::Center),
            chunks[2],
        );

        // Line 4: Best/Worst sessions
        let line4 = Line::from(vec![
            Span::styled(
                format!("{best_label}: "),
                Style::default().fg(colors.text()),
            ),
            Span::styled(
                format!("{:.0} CPM", total_summary.best_session_wpm * 5.0),
                Style::default().fg(colors.cpm_wpm()),
            ),
            Span::styled(", ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{:.1}%", total_summary.best_session_accuracy),
                Style::default().fg(colors.accuracy()),
            ),
            Span::styled(
                format!(" | {worst_label}: "),
                Style::default().fg(colors.text()),
            ),
            Span::styled(
                format!("{:.0} CPM", total_summary.worst_session_wpm * 5.0),
                Style::default().fg(colors.cpm_wpm()),
            ),
            Span::styled(", ", Style::default().fg(colors.text())),
            Span::styled(
                format!("{:.1}%", total_summary.worst_session_accuracy),
                Style::default().fg(colors.accuracy()),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(line4).alignment(Alignment::Center),
            chunks[3],
        );
    }
}
