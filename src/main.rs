use bevy::prelude::*;

mod player;
mod enemy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, player::spawn_player)
        .add_systems(Startup, player::spawn_camera)
        .add_systems(Update, player::player_movement)
        .add_systems(Update, player::confine_player_movement)
        .add_systems(Startup, enemy::spawn_enemies)
        .add_systems(Update, enemy::enemy_movement)
        .run();
}
