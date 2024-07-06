use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystem;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, MovementSystemSet.before(ConfinementSystem))
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement.in_set(MovementSystemSet))
            .add_systems(Update, confine_player_movement.in_set(ConfinementSystem))
            .add_systems(Update, enemy_hit_player)
            .add_systems(Update, player_hit_star);
    }
}
