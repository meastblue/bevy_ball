// use bevy::app::AppExit;
// use bevy::input::keyboard::Key;
// use bevy::prelude::*;
// use bevy::window::PrimaryWindow;

// use crate::AppState;

// pub fn transition_to_game_state(
//     mut commands: Commands,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     app_state: Res<State<AppState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::G) {
//         if app_state.get() != AppState::Game {
//             commads.insert_resource(NextState(Some(AppState::Game)));
//             println!("Transitioning to Game state");
//         }
//     }
// }

// pub fn transition_to_menu_state(
//     mut commands: Commands,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     app_state: Res<State<AppState>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::M) {
//         if app_state.get() != AppState::Menu {
//             commands.insert_resource(NextState(Some(AppState::Menu)));
//             println!("Transitioning to Menu state");
//         }
//     }
// }

// pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
//     let window = window_query.get_single().unwrap();

//     commands.spawn(Camera2dBundle {
//         transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
//         ..default()
//     });
// }

// pub fn exit_game(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut app_exit_event_writer: EventWriter<AppExit>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Escape) {
//         app_exit_event_writer.send(AppExit);
//     }
// }
