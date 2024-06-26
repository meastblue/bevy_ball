use bevy::prelude::*;

use events::*;

use crate::config::GameConfig;
use crate::enemy::EnemyPlugin;
use crate::events::GameOverEvent;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::star::StarPlugin;
use crate::systems::{exit_game, handle_game_over};

mod config;
mod events;
mod player;
mod score;
mod enemy;
mod star;

mod systems;

fn main() {
    let config = GameConfig::default();

    App::new()
        .insert_resource(config.clone())
        .add_event::<GameOverEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(ScorePlugin)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
