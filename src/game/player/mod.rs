use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use super::{config::AppState, SimulationState};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystem;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, MovementSystemSet.before(ConfinementSystem))
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystem),
                    enemy_hit_player,
                    player_hit_star,
                )
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(AppState::Game)),
            );
        // .add_systems(StateTransition, despawn_player);
    }
}
