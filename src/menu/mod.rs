use bevy::app::Startup;
use bevy::prelude::{App, Plugin};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub fn setup() {
    println!("Menu setup");
}
