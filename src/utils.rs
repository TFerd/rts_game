use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Range(pub f32);
