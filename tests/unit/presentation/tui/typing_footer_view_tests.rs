use gittype::domain::models::typing::ProcessingOptions;
use gittype::domain::services::typing_core::TypingCore;
use gittype::presentation::tui::views::typing::TypingFooterView;

fn word_typing_core() -> TypingCore {
    let text = "science  ['saɪəns]  科学\n  → museum  [mjuː'zɪəm]  博物馆";
    let word_chars = "science".chars().count();
    let comment_start = word_chars + "  ".chars().count();
    TypingCore::new(
        text,
        &[(comment_start, text.chars().count())],
        ProcessingOptions {
            preserve_empty_lines: true,
            ..Default::default()
        },
    )
}

#[test]
fn progress_uses_typeable_text_not_display_text() {
    let mut core = word_typing_core();

    "scienc".chars().for_each(|ch| {
        core.process_character_input(ch);
    });

    assert_eq!(TypingFooterView::progress_percent(false, false, &core), 85);
}

#[test]
fn progress_reaches_full_when_typeable_word_is_complete() {
    let mut core = word_typing_core();

    "science".chars().for_each(|ch| {
        core.process_character_input(ch);
    });

    assert_eq!(TypingFooterView::progress_percent(false, false, &core), 100);
}

#[test]
fn progress_is_zero_before_typing_starts() {
    let core = word_typing_core();

    assert_eq!(TypingFooterView::progress_percent(true, false, &core), 0);
    assert_eq!(TypingFooterView::progress_percent(false, true, &core), 0);
}
