use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}
