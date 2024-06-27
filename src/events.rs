use bevy::prelude::Event;

#[derive(Event)]
pub struct GameOverEvent {
    pub score: u32,
}
