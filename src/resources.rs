use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct GameConfig {
    pub player_speed: f32,
    pub player_size: f32,
    pub enemy_speed: f32,
    pub enemy_size: f32,
    pub number_of_enemies: u32,
    pub enemy_spawn_time: f32,
    pub star_size: f32,
    pub number_of_stars: u32,
    pub star_spawn_time: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player_speed: 500.0,
            player_size: 64.0,
            enemy_speed: 200.0,
            enemy_size: 64.0,
            number_of_enemies: 4,
            enemy_spawn_time: 5.0,
            star_size: 30.0,
            number_of_stars: 10,
            star_spawn_time: 1.0,
        }
    }
}