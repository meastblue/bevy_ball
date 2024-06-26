use bevy::{
    window::PrimaryWindow,
    prelude::*
};
use rand::prelude::*;
use crate::game_config::{GameConfig, Score, StarSpawnTimer};
use crate::player::Player;

#[derive(Component)]
pub struct Star;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..config.number_of_stars {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star,
        ));
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>,
    mut score: ResMut<Score>
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < config.player_size / 2.0 + config.star_size / 2.0 {
                println!("Player hit star!");
                score.value += 1;

                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("audio/impactMining_000.ogg"),
                        ..default()
                    },
                    Star,
                ));

                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star,
        ));
    }
}