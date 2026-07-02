use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::domain::models::{DifficultyLevel, GameMode, SessionConfig};
use crate::domain::services::audio_service::AudioServiceInterface;
use crate::domain::services::session_manager_service::{SessionManager, SessionManagerInterface};
use crate::domain::services::stage_builder_service::{StageRepository, StageRepositoryInterface};
use crate::domain::services::word_challenge_generator::WordChallengeGenerator;
use crate::domain::services::word_list_parser::WordListParser;
use crate::domain::stores::ChallengeStoreInterface;
use crate::presentation::di::AppModule;
use crate::presentation::signal_handler::setup_signal_handlers;
use crate::presentation::tui::{ScreenManagerFactory, ScreenManagerImpl, ScreenType};
use crate::Result;
use shaku::HasComponent;

pub fn run_word_session(
    word_file: PathBuf,
    shuffle: bool,
    audio_url: Option<String>,
) -> Result<()> {
    let entries = WordListParser::parse_anki_tsv(&word_file)?;
    if entries.is_empty() {
        eprintln!("No valid word entries found in file.");
        std::process::exit(1);
    }

    let challenges = WordChallengeGenerator::generate(entries);
    let total = challenges.len();

    let container = AppModule::builder().build();

    let challenge_store: &dyn ChallengeStoreInterface = container.resolve_ref();
    challenge_store.set_challenges(challenges);

    let stage_repository: &dyn StageRepositoryInterface = container.resolve_ref();
    let game_mode = if shuffle {
        GameMode::Normal
    } else {
        GameMode::Sequential
    };
    let stage_repo = stage_repository
        .as_any()
        .downcast_ref::<StageRepository>()
        .expect("Failed to downcast StageRepository");
    stage_repo.set_mode(game_mode.clone());
    stage_repo.build_difficulty_indices();

    if let Some(ref url) = audio_url {
        let audio_service: &dyn AudioServiceInterface = container.resolve_ref();
        audio_service.set_base_url(url.clone());
    }

    let session_manager_trait: Arc<dyn SessionManagerInterface> = container.resolve();

    // Downcast to concrete SessionManager for initialization
    {
        let sm = session_manager_trait
            .as_any()
            .downcast_ref::<SessionManager>()
            .expect("Failed to downcast SessionManager");
        sm.initialize(Some(SessionConfig {
            max_stages: total,
            game_mode,
            difficulty: DifficultyLevel::Normal,
            ..Default::default()
        }))?;
        sm.reduce(crate::domain::models::SessionAction::Start)?;
    }

    // Setup event subscriptions (requires owned Arc)
    {
        let session_manager_arc = unsafe {
            Arc::from_raw(Arc::into_raw(session_manager_trait.clone()) as *const SessionManager)
        };
        SessionManager::setup_event_subscriptions(session_manager_arc);
    }

    let factory: &dyn ScreenManagerFactory = container.resolve_ref();
    let screen_manager_impl = factory.create(&container);
    let screen_manager = Arc::new(Mutex::new(screen_manager_impl));

    setup_signal_handlers(screen_manager.clone());

    // Set up event subscriptions for screen navigation (required for Esc+q, Ctrl+C, etc.)
    ScreenManagerImpl::setup_event_subscriptions(&screen_manager);

    {
        let mut manager = screen_manager.lock().unwrap();
        manager.initialize_terminal()?;
        manager.set_current_screen(ScreenType::Typing)?;
    }

    screen_manager.lock().unwrap().run()?;
    Ok(())
}
