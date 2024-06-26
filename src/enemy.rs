use bevy::{
    prelude::*,
    window::PrimaryWindow,
    audio::{AudioBundle},
};
use rand::random;
use crate::game_config::{EnemySpawnTimer, GameConfig};
use crate::player::Player;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..config.number_of_enemies {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec3::new(random::<f32>(), random::<f32>(), 0.0).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction * config.enemy_speed * time.delta_seconds();
    }
}

pub fn update_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    config: Res<GameConfig>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = config.enemy_size / 2.0;
    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    config: Res<GameConfig>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        let player_radius = config.player_size / 2.0;
        let enemy_radius = config.enemy_size / 2.0;

        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);

            if distance < player_radius + enemy_radius {
                commands.spawn((
                  AudioBundle {
                      source: asset_server.load("audio/explosionCrunch_000.ogg"),
                      ..default()
                  },
                  Player
               ));

                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn tick_enemy_spawn_timer(
    time: Res<Time>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec3::new(random::<f32>(), random::<f32>(), 0.0).normalize(),
            },
        ));
    }
}
