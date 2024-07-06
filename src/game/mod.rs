use bevy::{
    app::Update,
    prelude::{in_state, App, IntoSystemConfigs, Plugin, States},
};

mod config;
pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;

use crate::game::systems::toggle_simulation;
use config::{AppState, ConfigPlugin};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            // Plugins
            .add_plugins(ConfigPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            // Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}
