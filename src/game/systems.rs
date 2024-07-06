use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    state: Res<State<SimulationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match state.get() {
            SimulationState::Running => {
                commands.insert_resource(NextState(Some(SimulationState::Running)));
                println!("Simulation Paused");
            }
            SimulationState::Paused => {
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
                println!("Simulation Running");
            }
        }
    }
}
