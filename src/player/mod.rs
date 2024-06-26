use bevy::app::App;
use bevy::prelude::*;

use crate::player::systems::*;

mod components;
pub mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, confine_player_movement);
    }
}