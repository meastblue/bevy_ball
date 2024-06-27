pub mod events;
mod systems;

pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod resources;

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use bevy::prelude::*;
use crate::resources::GameConfig;

fn main() {
    let config = GameConfig::default();
    
    App::new()
        .insert_resource(config.clone())
        .add_plugins(DefaultPlugins)
        .add_event::<GameOverEvent>()
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(StarPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Last, exit_game)
        .add_systems(Last, handle_game_over)
        .run();
}
