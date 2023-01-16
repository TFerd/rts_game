/*
    THIS CLASS IS FOR COMPONENTS/SYSTEMS THAT CAN BE SHARED BETWEEN BUILDINGS AND UNITS
    @TODO: rename to globals.rs or global.rs ?
*/

use bevy::{app::PluginGroupBuilder, prelude::*};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Range(pub u32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AttackCooldown(pub Timer);

// Sparse-set storage because this is added/removed frequently
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Target {
    pub target: Entity,
}

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<Range>()
            .register_type::<AttackCooldown>();
    }
}

fn tick_timers(mut timers: Query<&mut AttackCooldown>, time: Res<Time>) {
    for mut timer in timers.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn aquire_target(
    mut commands: Commands,
    shooter: Query<Entity, (With<Range>, Without<Target>)>,
    targets: Query<Entity, With<Health>>,
) {
}

// if target moves out of range, remove the target
fn remove_target() {}

fn attack_target() {}
