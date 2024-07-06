use crate::game::config::events::GameOverEvent;
use crate::game::config::resources::Config;
use crate::game::config::systems::handle_game_over;
use bevy::app::{Last, Startup, Update};
use bevy::prelude::{App, Plugin, States};
use systems::{exit_game, spawn_camera, transition_to_game_state, transition_to_menu_state};

pub mod events;
pub mod resources;
mod systems;

pub struct ConfigPlugin;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .insert_resource(Config::default())
            .init_state::<AppState>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Last, handle_game_over)
            .add_systems(Last, exit_game)
            .add_systems(Update, transition_to_game_state)
            .add_systems(Update, transition_to_menu_state);
    }
}
