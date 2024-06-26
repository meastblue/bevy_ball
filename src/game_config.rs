use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::ecs::event::{EventWriter};
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

#[derive(Resource)]
pub struct Score {
    pub value: i32,
}
impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl StarSpawnTimer {
    pub fn from_config(config: &GameConfig) -> Self {
        Self {
            timer: Timer::from_seconds(config.star_spawn_time, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl EnemySpawnTimer {
    pub fn from_config(config: &GameConfig) -> Self {
        Self {
            timer: Timer::from_seconds(config.enemy_spawn_time, TimerMode::Repeating),
        }
    }
}

pub fn exit_game(
    mut app_exit_event_writer: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

// #[derive(Event, Debug)]
// pub struct GameOver {
//     pub score: u32,
// }
//
// pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
//     println!("{:?}", game_over_event_reader);
//     for event in game_over_event_reader.iter_mut() {
//         println!("Your final score is: {}", event.score);
//     }
// }


// #[derive(Resource, Debug)]
// pub struct HighScores {
//     pub scores: Vec<(String, u32)>,
// }
//
// pub fn update_high_scores(
//     mut game_over_event_reader: EventReader<GameOver>,
//     mut high_scores: ResMut<HighScores>,
// ) {
//     for event in game_over_event_reader.iter_mut() {
//         println!("{:?}", event);
//     }
//     for event in game_over_event_reader.iter_mut() {
//         high_scores.scores.push(("Player".to_string(), event.score));
//     }
// }


// pub fn high_scores_updated(high_scores: Res<HighScores>) {
//     if high_scores.is_changed() {
//         println!("High Scores: {:?}", high_scores);
//     }
// }
