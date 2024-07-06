mod events;
mod game;
mod menu;
mod systems;

use bevy::prelude::*;
use game::GamePlugin;
use menu::MenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_plugins(MenuPlugin)
        .run();
}
