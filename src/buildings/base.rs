use bevy::prelude::*;

use crate::utils::Health;
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Base;

#[derive(Bundle)]
pub struct BaseBundle {
    pub building_type: Base,
    pub health: Health,
}

pub struct BaseDeathEvent; // gameover?

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Base>().add_system(base_death);
    }
}

fn base_death(bases: Query<&Health, With<Base>>, mut base_death_ew: EventWriter<BaseDeathEvent>) {
    for health in bases.iter() {
        if health.0 <= 0.0 {
            // base destroyed
            // gameover?
            base_death_ew.send(BaseDeathEvent);

            // play death animation
        }
    }
}

fn build_base() {
    todo!();
}
