use std::time::Duration;

use crate::domain::models::DifficultyLevel;
use crate::domain::models::GameMode;

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub max_stages: usize,
    pub session_timeout: Option<Duration>,
    pub difficulty: DifficultyLevel,
    pub max_skips: usize,
    pub game_mode: GameMode,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_stages: 3,
            session_timeout: None,
            difficulty: DifficultyLevel::Normal,
            max_skips: 3,
            game_mode: GameMode::Normal,
        }
    }
}
