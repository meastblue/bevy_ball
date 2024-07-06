use bevy::prelude::*;
use crate::game::config::resources::Config;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

pub trait FromConfig {
    fn from_config(config: &Config) -> Self;
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl FromConfig for StarSpawnTimer {
    fn from_config(config: &Config) -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(config.star_spawn_time, TimerMode::Repeating),
        }
    }
}
