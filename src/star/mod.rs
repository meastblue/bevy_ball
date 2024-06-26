use bevy::prelude::*;

use crate::star::{
    resources::*,
    systems::*,
};

mod systems;
mod resources;
mod components;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, tick_star_spawn_timer)
            .add_systems(Update, spawn_stars_over_time);
    }
}