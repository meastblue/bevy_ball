use bevy::prelude::*;

use crate::config::GameConfig;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

pub trait FromConfig {
    fn from_config(config: &GameConfig) -> Self;
}

impl FromConfig for StarSpawnTimer {
    fn from_config(config: &GameConfig) -> Self {
        Self {
            timer: Timer::from_seconds(config.star_spawn_time, TimerMode::Repeating),
        }
    }
}