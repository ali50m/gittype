use crate::domain::services::scoring::RealTimeCalculator;
use crate::{
    domain::services::scoring::tracker::stage::StageTracker,
    domain::services::typing_core::TypingCore, presentation::ui::Colors,
};
use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

pub struct TypingFooterView;

impl TypingFooterView {
    #[allow(clippy::too_many_arguments)]
    pub fn render_metrics(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        waiting_to_start: bool,
        countdown_active: bool,
        skips_remaining: usize,
        stage_tracker: &StageTracker,
        typing_core: &TypingCore,
        colors: &Colors,
        is_word_mode: bool,
    ) {
        let (
            cpm_label,
            wpm_label,
            acc_label,
            err_label,
            streak_label,
            time_label,
            skips_label,
            title,
        ) = if is_word_mode {
            (
                "字/分",
                "词/分",
                "正确率",
                "错误",
                "连击",
                "用时",
                "跳过",
                "统计",
            )
        } else {
            (
                "CPM", "WPM", "Accuracy", "Mistakes", "Streak", "Time", "Skips", "Metrics",
            )
        };

        let metrics_line = if waiting_to_start || countdown_active {
            format!(
                "{wpm_label}: 0 | {cpm_label}: 0 | {acc_label}: 0% | {err_label}: 0 | {streak_label}: 0 | {time_label}: 0s | {skips_label}: {}",
                skips_remaining
            )
        } else {
            let elapsed_time = stage_tracker.get_data().elapsed_time;
            let current_position = typing_core.current_position_to_type();
            let mistakes = typing_core.mistakes();
            let metrics = RealTimeCalculator::calculate(current_position, mistakes, elapsed_time);
            let elapsed_secs = elapsed_time.as_secs();
            let streak = stage_tracker.get_data().current_streak;
            format!(
                "{wpm_label}: {:.0} | {cpm_label}: {:.0} | {acc_label}: {:.0}% | {err_label}: {} | {streak_label}: {} | {time_label}: {}s | {skips_label}: {}",
                metrics.wpm, metrics.cpm, metrics.accuracy, metrics.mistakes, streak, elapsed_secs, skips_remaining
            )
        };

        let metrics_widget = Paragraph::new(vec![Line::from(vec![Span::styled(
            metrics_line,
            Style::default().fg(colors.text_secondary()),
        )])])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.border()))
                .title(title)
                .title_style(Style::default().fg(colors.text_secondary()))
                .padding(ratatui::widgets::Padding::horizontal(1)),
        );
        frame.render_widget(metrics_widget, area);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_progress(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        waiting_to_start: bool,
        countdown_active: bool,
        typing_core: &TypingCore,
        colors: &Colors,
        is_word_mode: bool,
    ) {
        let progress_percent =
            Self::progress_percent(waiting_to_start, countdown_active, typing_core);

        let title = if is_word_mode { "进度" } else { "Progress" };
        let progress_widget = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border()))
                    .title(title)
                    .title_style(Style::default().fg(colors.text_secondary())),
            )
            .gauge_style(Style::default().fg(colors.text_secondary()))
            .percent(progress_percent as u16)
            .label(format!("{}%", progress_percent));
        frame.render_widget(progress_widget, area);
    }

    pub fn progress_percent(
        waiting_to_start: bool,
        countdown_active: bool,
        typing_core: &TypingCore,
    ) -> u8 {
        if waiting_to_start || countdown_active {
            return 0;
        }

        let total_chars = typing_core.text_to_type().chars().count();
        if total_chars == 0 {
            return 0;
        }

        ((typing_core.current_position_to_type() as f32 / total_chars as f32) * 100.0).min(100.0)
            as u8
    }
}
