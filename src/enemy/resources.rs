use bevy::prelude::*;

use crate::config::GameConfig;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

pub trait FromConfig {
    fn from_config(config: &GameConfig) -> Self;
}


impl FromConfig for EnemySpawnTimer {
    fn from_config(config: &GameConfig) -> Self {
        Self {
            timer: Timer::from_seconds(config.enemy_spawn_time, TimerMode::Repeating),
        }
    }
}
