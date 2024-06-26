mod player;
mod enemy;
mod game_config;
mod star;


use bevy::prelude::*;
use game_config::*;
// use crate::game_config::*;


fn main() {
    let config = GameConfig::default();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(config.clone())
        .insert_resource(Score::default())
        .insert_resource(StarSpawnTimer::from_config(&config))
        .insert_resource(EnemySpawnTimer::from_config(&config))
        .add_systems(Startup, player::spawn_player)
        .add_systems(Startup, player::spawn_camera)
        .add_systems(Update, player::player_movement)
        .add_systems(Update, player::confine_player_movement)
        .add_systems(Startup, enemy::spawn_enemies)
        .add_systems(Update, enemy::enemy_movement)
        .add_systems(Update, enemy::update_enemy_movement)
        .add_systems(Update, enemy::enemy_hit_player)
        .add_systems(Update, enemy::tick_enemy_spawn_timer)
        .add_systems(Update, enemy::spawn_enemies_over_time)
        .add_systems(Startup, star::spawn_stars)
        .add_systems(Update, star::player_hit_star)
        .add_systems(Update, star::update_score)
        .add_systems(Update, star::tick_star_spawn_timer)
        .add_systems(Update, star::spawn_stars_over_time)
        .add_systems(Update, exit_game)
        .run();
}


