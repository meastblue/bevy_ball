use bevy::prelude::*;
use crate::resources::GameConfig;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

pub trait FromConfig {
    fn from_config(config: &GameConfig) -> Self;
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl FromConfig for StarSpawnTimer {
    fn from_config(config: &GameConfig) -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(config.star_spawn_time, TimerMode::Repeating),
        }
    }
}
