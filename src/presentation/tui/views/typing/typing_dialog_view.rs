use crate::presentation::ui::Colors;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub struct TypingDialogView;

impl TypingDialogView {
    pub fn render(
        frame: &mut Frame,
        skips_remaining: usize,
        colors: &Colors,
        is_word_mode: bool,
    ) {
        // Calculate dialog size and position
        let area = frame.area();
        let dialog_width = 50.min(area.width - 4);
        let dialog_height = 9;

        let dialog_area = Rect {
            x: (area.width - dialog_width) / 2,
            y: (area.height - dialog_height) / 2,
            width: dialog_width,
            height: dialog_height,
        };

        // Clear the area behind the dialog
        frame.render_widget(Clear, dialog_area);

        let (title, choose, skip_label, no_skips_label, quit_label, back_label) = if is_word_mode
        {
            ("游戏选项", "选择一个操作:", "跳过", "无跳过次数", "退出", "返回游戏")
        } else {
            (
                "Game Options",
                "Choose an option:",
                "Skip challenge",
                "No skips remaining",
                "Quit (fail)",
                "Back to game",
            )
        };

        // Create dialog content
        let dialog_lines = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                choose,
                Style::default()
                    .fg(colors.text())
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                if skips_remaining > 0 {
                    Span::styled(
                        "[S] ",
                        Style::default()
                            .fg(colors.info())
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::styled("[S] ", Style::default().fg(colors.text_secondary()))
                },
                if skips_remaining > 0 {
                    Span::styled(
                        format!("{skip_label} ({})", skips_remaining),
                        Style::default().fg(colors.text()),
                    )
                } else {
                    Span::styled(
                        no_skips_label,
                        Style::default().fg(colors.text_secondary()),
                    )
                },
            ]),
            Line::from(vec![
                Span::styled(
                    "[Q] ",
                    Style::default()
                        .fg(colors.error())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(quit_label, Style::default().fg(colors.text())),
            ]),
            Line::from(vec![
                Span::styled(
                    "[ESC] ",
                    Style::default()
                        .fg(colors.key_action())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(back_label, Style::default().fg(colors.text())),
            ]),
            Line::from(""),
        ];

        let dialog = Paragraph::new(dialog_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_style(
                        Style::default()
                            .fg(colors.key_action())
                            .add_modifier(Modifier::BOLD),
                    )
                    .border_style(Style::default().fg(colors.border())),
            )
            .alignment(ratatui::layout::Alignment::Center);

        frame.render_widget(dialog, dialog_area);
    }
}
