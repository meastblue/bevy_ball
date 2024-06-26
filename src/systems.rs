use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::prelude::{Camera2dBundle, Commands, default, EventReader, EventWriter, KeyCode, Query, Res, Transform, Window, With};
use bevy::window::PrimaryWindow;

use crate::events::GameOverEvent;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn exit_game(
    mut app_exit_event_writer: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOverEvent>) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score.to_string());
    }
}