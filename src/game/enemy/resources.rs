use bevy::prelude::*;
use crate::game::config::resources::Config;


#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

pub trait FromConfig {
    fn from_config(config: &Config) -> Self;
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

impl FromConfig for EnemySpawnTimer {
    fn from_config(config: &Config) -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(config.enemy_spawn_time, TimerMode::Repeating),
        }
    }
}
